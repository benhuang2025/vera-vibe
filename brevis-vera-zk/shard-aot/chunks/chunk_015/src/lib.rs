pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2156440u32;
pub const PC_MAX: u32 = 2159176u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 158usize] = [
        block_0x0020e798,
        block_0x0020e7b4,
        block_0x0020e7f4,
        block_0x0020e930,
        block_0x0020e938,
        block_0x0020e948,
        block_0x0020e99c,
        block_0x0020e9a4,
        block_0x0020e9b0,
        block_0x0020e9b8,
        block_0x0020e9c4,
        block_0x0020e9d4,
        block_0x0020e9dc,
        block_0x0020e9e4,
        block_0x0020e9f0,
        block_0x0020e9f4,
        block_0x0020e9fc,
        block_0x0020ea08,
        block_0x0020ea18,
        block_0x0020ea20,
        block_0x0020ea28,
        block_0x0020ea2c,
        block_0x0020ea34,
        block_0x0020ea44,
        block_0x0020ea48,
        block_0x0020ea4c,
        block_0x0020eabc,
        block_0x0020eac4,
        block_0x0020eae4,
        block_0x0020eafc,
        block_0x0020eb04,
        block_0x0020eb08,
        block_0x0020eb28,
        block_0x0020eb30,
        block_0x0020eb38,
        block_0x0020eb4c,
        block_0x0020eb5c,
        block_0x0020eb64,
        block_0x0020eb6c,
        block_0x0020eb84,
        block_0x0020eba0,
        block_0x0020eba4,
        block_0x0020ebc0,
        block_0x0020ebc8,
        block_0x0020ebd8,
        block_0x0020ec00,
        block_0x0020ec1c,
        block_0x0020ec68,
        block_0x0020ec70,
        block_0x0020ec78,
        block_0x0020ecac,
        block_0x0020ecb4,
        block_0x0020ecb8,
        block_0x0020ecd4,
        block_0x0020ece0,
        block_0x0020ece4,
        block_0x0020ecf0,
        block_0x0020ecf4,
        block_0x0020ecfc,
        block_0x0020ed08,
        block_0x0020ed14,
        block_0x0020ed18,
        block_0x0020ed1c,
        block_0x0020ed38,
        block_0x0020ed44,
        block_0x0020ed48,
        block_0x0020ed54,
        block_0x0020ed58,
        block_0x0020ed60,
        block_0x0020ed6c,
        block_0x0020ed78,
        block_0x0020ed7c,
        block_0x0020ed84,
        block_0x0020ed88,
        block_0x0020ed98,
        block_0x0020edac,
        block_0x0020edb4,
        block_0x0020edb8,
        block_0x0020edc0,
        block_0x0020eddc,
        block_0x0020ede4,
        block_0x0020edec,
        block_0x0020edf8,
        block_0x0020ee10,
        block_0x0020ee18,
        block_0x0020ee1c,
        block_0x0020ee34,
        block_0x0020ee3c,
        block_0x0020ee40,
        block_0x0020ee50,
        block_0x0020ee54,
        block_0x0020ee60,
        block_0x0020ee64,
        block_0x0020ee78,
        block_0x0020ee80,
        block_0x0020ee84,
        block_0x0020ee8c,
        block_0x0020ee94,
        block_0x0020eea4,
        block_0x0020eeb8,
        block_0x0020eec0,
        block_0x0020eec4,
        block_0x0020eecc,
        block_0x0020eee8,
        block_0x0020eef0,
        block_0x0020eef8,
        block_0x0020ef04,
        block_0x0020ef1c,
        block_0x0020ef24,
        block_0x0020ef28,
        block_0x0020ef40,
        block_0x0020ef48,
        block_0x0020ef4c,
        block_0x0020ef5c,
        block_0x0020ef60,
        block_0x0020ef6c,
        block_0x0020ef70,
        block_0x0020ef84,
        block_0x0020ef8c,
        block_0x0020ef90,
        block_0x0020ef98,
        block_0x0020efb4,
        block_0x0020efbc,
        block_0x0020efc4,
        block_0x0020efd4,
        block_0x0020efe8,
        block_0x0020eff0,
        block_0x0020f008,
        block_0x0020f010,
        block_0x0020f02c,
        block_0x0020f034,
        block_0x0020f03c,
        block_0x0020f050,
        block_0x0020f058,
        block_0x0020f05c,
        block_0x0020f068,
        block_0x0020f094,
        block_0x0020f09c,
        block_0x0020f0b0,
        block_0x0020f0b8,
        block_0x0020f0c0,
        block_0x0020f0d0,
        block_0x0020f0d4,
        block_0x0020f0e4,
        block_0x0020f0e8,
        block_0x0020f0f0,
        block_0x0020f0f4,
        block_0x0020f130,
        block_0x0020f154,
        block_0x0020f170,
        block_0x0020f18c,
        block_0x0020f1a8,
        block_0x0020f1c4,
        block_0x0020f1e0,
        block_0x0020f1fc,
        block_0x0020f218,
        block_0x0020f230,
        block_0x0020f248,
    ];
    const IDX: [u16; 685usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 3u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 4u16, 0u16,
        5u16, 0u16, 0u16, 0u16, 6u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 7u16,
        0u16, 8u16, 0u16, 0u16, 9u16, 0u16, 10u16, 0u16, 0u16, 11u16, 0u16, 0u16, 0u16,
        12u16, 0u16, 13u16, 0u16, 14u16, 0u16, 0u16, 15u16, 16u16, 0u16, 17u16, 0u16,
        0u16, 18u16, 0u16, 0u16, 0u16, 19u16, 0u16, 20u16, 0u16, 21u16, 22u16, 0u16,
        23u16, 0u16, 0u16, 0u16, 24u16, 25u16, 26u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 27u16, 0u16, 28u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 29u16, 0u16, 0u16, 0u16, 0u16, 0u16, 30u16, 0u16,
        31u16, 32u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 33u16, 0u16, 34u16, 0u16,
        35u16, 0u16, 0u16, 0u16, 0u16, 36u16, 0u16, 0u16, 0u16, 37u16, 0u16, 38u16, 0u16,
        39u16, 0u16, 0u16, 0u16, 0u16, 0u16, 40u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        41u16, 42u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 43u16, 0u16, 44u16, 0u16, 0u16,
        0u16, 45u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 46u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 47u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 48u16, 0u16,
        49u16, 0u16, 50u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 51u16, 0u16, 52u16, 53u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 54u16,
        0u16, 0u16, 55u16, 56u16, 0u16, 0u16, 57u16, 58u16, 0u16, 59u16, 0u16, 0u16,
        60u16, 0u16, 0u16, 61u16, 62u16, 63u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        64u16, 0u16, 0u16, 65u16, 66u16, 0u16, 0u16, 67u16, 68u16, 0u16, 69u16, 0u16,
        0u16, 70u16, 0u16, 0u16, 71u16, 72u16, 0u16, 73u16, 74u16, 0u16, 0u16, 0u16,
        75u16, 0u16, 0u16, 0u16, 0u16, 76u16, 0u16, 77u16, 78u16, 0u16, 79u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 80u16, 0u16, 81u16, 0u16, 82u16, 0u16, 0u16, 83u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 84u16, 0u16, 85u16, 86u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 87u16, 0u16, 88u16, 89u16, 0u16, 0u16, 0u16, 90u16, 91u16, 0u16, 0u16,
        92u16, 93u16, 0u16, 0u16, 0u16, 0u16, 94u16, 0u16, 95u16, 96u16, 0u16, 97u16,
        0u16, 98u16, 0u16, 0u16, 0u16, 99u16, 0u16, 0u16, 0u16, 0u16, 100u16, 0u16,
        101u16, 102u16, 0u16, 103u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 104u16, 0u16,
        105u16, 0u16, 106u16, 0u16, 0u16, 107u16, 0u16, 0u16, 0u16, 0u16, 0u16, 108u16,
        0u16, 109u16, 110u16, 0u16, 0u16, 0u16, 0u16, 0u16, 111u16, 0u16, 112u16, 113u16,
        0u16, 0u16, 0u16, 114u16, 115u16, 0u16, 0u16, 116u16, 117u16, 0u16, 0u16, 0u16,
        0u16, 118u16, 0u16, 119u16, 120u16, 0u16, 121u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 122u16, 0u16, 123u16, 0u16, 124u16, 0u16, 0u16, 0u16, 125u16, 0u16, 0u16,
        0u16, 0u16, 126u16, 0u16, 127u16, 0u16, 0u16, 0u16, 0u16, 0u16, 128u16, 0u16,
        129u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 130u16, 0u16, 131u16, 0u16, 132u16,
        0u16, 0u16, 0u16, 0u16, 133u16, 0u16, 134u16, 135u16, 0u16, 0u16, 136u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 137u16, 0u16, 138u16, 0u16,
        0u16, 0u16, 0u16, 139u16, 0u16, 140u16, 0u16, 141u16, 0u16, 0u16, 0u16, 142u16,
        143u16, 0u16, 0u16, 0u16, 144u16, 145u16, 0u16, 146u16, 147u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 148u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 149u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 150u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 151u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 152u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 153u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 154u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 155u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 156u16, 0u16, 0u16, 0u16, 0u16, 0u16, 157u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 158u16,
    ];
    if pc < 2156440u32 || pc > 2159176u32 {
        return None;
    }
    let word_offset = ((pc - 2156440u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x0020e798(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(10usize, 5usize, 10usize, 2156444u32);
    emu.xrr_no_count(12usize, 31usize, 14usize, 2156448u32);
    emu.xrr_no_count(14usize, 10usize, 16usize, 2156452u32);
    emu.orr_no_count(12usize, 12usize, 14usize, 2156456u32);
    emu.sw_no_count(31usize, 2usize, 40u32, 2156460u32)?;
    emu.sw_no_count(10usize, 2usize, 44u32, 2156464u32)?;
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2158896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f130));
    } else {
        emu.pc = 2156468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e7b4));
    }
}
#[inline]
pub fn block_0x0020e7b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 2usize, 16u32, 2156472u32)?;
    emu.sbr_no_count(10usize, 30usize, 7usize, 2156476u32);
    emu.adi_no_count(12usize, 0usize, 4294967200u32, 2156480u32);
    emu.adi_no_count(16usize, 0usize, 80u32, 2156484u32);
    let a = 0u32.wrapping_add(2068697088u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2156488u32;
    emu.update_insn_clock();
    emu.sbr_no_count(12usize, 12usize, 10usize, 2156492u32);
    emu.adi_no_count(14usize, 14usize, 4294965651u32, 2156496u32);
    emu.sli_no_count(12usize, 12usize, 16u32, 2156500u32);
    emu.sai_no_count(12usize, 12usize, 1040u32, 2156504u32);
    emu.adi_no_count(12usize, 12usize, 1087u32, 2156508u32);
    emu.mul_no_count(12usize, 12usize, 16usize, 2156512u32);
    emu.mulh_no_count(12usize, 12usize, 14usize, 2156516u32);
    emu.sri_no_count(14usize, 12usize, 31u32, 2156520u32);
    emu.sai_no_count(12usize, 12usize, 1034u32, 2156524u32);
    emu.adr_no_count(14usize, 12usize, 14usize, 2156528u32);
    emu.add_memory_rw_events(15usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a < b {
        emu.pc = 2159176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f248));
    } else {
        emu.pc = 2156532u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e7f4));
    }
}
#[inline(never)]
pub fn block_0x0020e7f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 79u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.slr_no_count(12usize, 15usize, 7usize, 2156536u32);
    emu.sli_no_count(14usize, 14usize, 4u32, 2156540u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2156544u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 4294965776u32, 2156548u32);
    emu.sbr_no_count(15usize, 0usize, 10usize, 2156552u32);
    emu.adr_no_count(16usize, 16usize, 14usize, 2156556u32);
    emu.lw_no_count(14usize, 16usize, 0u32, 2156560u32)?;
    emu.lw_no_count(7usize, 16usize, 4u32, 2156564u32)?;
    emu.anr_no_count(10usize, 5usize, 12usize, 2156568u32);
    emu.lh_no_count(12usize, 16usize, 8u32, 2156572u32)?;
    emu.mulhu_no_count(30usize, 14usize, 10usize, 2156576u32);
    emu.mul_no_count(31usize, 7usize, 10usize, 2156580u32);
    emu.mulhu_no_count(8usize, 7usize, 10usize, 2156584u32);
    emu.mul_no_count(9usize, 14usize, 11usize, 2156588u32);
    emu.mulhu_no_count(18usize, 14usize, 11usize, 2156592u32);
    emu.mul_no_count(10usize, 7usize, 11usize, 2156596u32);
    emu.mulhu_no_count(11usize, 7usize, 11usize, 2156600u32);
    emu.sbr_no_count(15usize, 15usize, 12usize, 2156604u32);
    emu.mulhu_no_count(12usize, 14usize, 6usize, 2156608u32);
    emu.mul_no_count(19usize, 7usize, 6usize, 2156612u32);
    emu.mulhu_no_count(6usize, 7usize, 6usize, 2156616u32);
    emu.mul_no_count(20usize, 14usize, 17usize, 2156620u32);
    emu.mulhu_no_count(21usize, 14usize, 17usize, 2156624u32);
    emu.mul_no_count(5usize, 7usize, 17usize, 2156628u32);
    emu.mulhu_no_count(17usize, 7usize, 17usize, 2156632u32);
    emu.mulhu_no_count(22usize, 14usize, 29usize, 2156636u32);
    emu.mul_no_count(23usize, 7usize, 29usize, 2156640u32);
    emu.mulhu_no_count(29usize, 7usize, 29usize, 2156644u32);
    emu.mul_no_count(24usize, 14usize, 28usize, 2156648u32);
    emu.mulhu_no_count(14usize, 14usize, 28usize, 2156652u32);
    emu.mul_no_count(25usize, 7usize, 28usize, 2156656u32);
    emu.mulhu_no_count(28usize, 7usize, 28usize, 2156660u32);
    emu.adr_no_count(30usize, 31usize, 30usize, 2156664u32);
    emu.sltru_no_count(7usize, 30usize, 31usize, 2156668u32);
    emu.adr_no_count(7usize, 8usize, 7usize, 2156672u32);
    emu.adr_no_count(12usize, 19usize, 12usize, 2156676u32);
    emu.sltru_no_count(31usize, 12usize, 19usize, 2156680u32);
    emu.adr_no_count(31usize, 6usize, 31usize, 2156684u32);
    emu.adr_no_count(22usize, 23usize, 22usize, 2156688u32);
    emu.sltru_no_count(6usize, 22usize, 23usize, 2156692u32);
    emu.adr_no_count(19usize, 29usize, 6usize, 2156696u32);
    emu.adr_no_count(30usize, 9usize, 30usize, 2156700u32);
    emu.sltru_no_count(6usize, 30usize, 9usize, 2156704u32);
    emu.adr_no_count(18usize, 18usize, 6usize, 2156708u32);
    emu.adr_no_count(6usize, 20usize, 12usize, 2156712u32);
    emu.sltru_no_count(12usize, 6usize, 20usize, 2156716u32);
    emu.adr_no_count(12usize, 21usize, 12usize, 2156720u32);
    emu.adr_no_count(22usize, 24usize, 22usize, 2156724u32);
    emu.sltru_no_count(29usize, 22usize, 24usize, 2156728u32);
    emu.adr_no_count(14usize, 14usize, 29usize, 2156732u32);
    emu.adr_no_count(18usize, 7usize, 18usize, 2156736u32);
    emu.sltru_no_count(7usize, 18usize, 7usize, 2156740u32);
    emu.adr_no_count(9usize, 11usize, 7usize, 2156744u32);
    emu.ani_no_count(7usize, 15usize, 63u32, 2156748u32);
    emu.adr_no_count(12usize, 31usize, 12usize, 2156752u32);
    emu.sltru_no_count(8usize, 12usize, 31usize, 2156756u32);
    emu.adr_no_count(8usize, 17usize, 8usize, 2156760u32);
    emu.adi_no_count(29usize, 7usize, 4294967264u32, 2156764u32);
    emu.sri_no_count(17usize, 30usize, 31u32, 2156768u32);
    emu.sri_no_count(30usize, 22usize, 31u32, 2156772u32);
    emu.adr_no_count(11usize, 19usize, 14usize, 2156776u32);
    emu.adr_no_count(18usize, 10usize, 18usize, 2156780u32);
    emu.adr_no_count(14usize, 5usize, 12usize, 2156784u32);
    emu.sltru_no_count(12usize, 11usize, 19usize, 2156788u32);
    emu.adr_no_count(31usize, 25usize, 11usize, 2156792u32);
    emu.sltru_no_count(19usize, 18usize, 10usize, 2156796u32);
    emu.adr_no_count(11usize, 17usize, 18usize, 2156800u32);
    emu.sltru_no_count(10usize, 14usize, 5usize, 2156804u32);
    emu.sltru_no_count(24usize, 31usize, 25usize, 2156808u32);
    emu.adr_no_count(5usize, 28usize, 12usize, 2156812u32);
    emu.adr_no_count(18usize, 30usize, 31usize, 2156816u32);
    emu.adr_no_count(9usize, 9usize, 19usize, 2156820u32);
    emu.sltru_no_count(17usize, 11usize, 17usize, 2156824u32);
    emu.adi_no_count(12usize, 11usize, 1u32, 2156828u32);
    emu.adr_no_count(17usize, 9usize, 17usize, 2156832u32);
    emu.sltiu_no_count(28usize, 12usize, 1u32, 2156836u32);
    emu.adr_no_count(21usize, 17usize, 28usize, 2156840u32);
    emu.xri_no_count(31usize, 7usize, 4294967295u32, 2156844u32);
    emu.add_memory_rw_events(78usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(29usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2156856u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e938));
    } else {
        emu.pc = 2156848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e930));
    }
}
#[inline(always)]
pub fn block_0x0020e930(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.srr_no_count(22usize, 21usize, 7usize, 2156852u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2156856u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156872u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e948));
}
#[inline(always)]
pub fn block_0x0020e938(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(28usize, 21usize, 1u32, 2156860u32);
    emu.slr_no_count(28usize, 28usize, 31usize, 2156864u32);
    emu.srr_no_count(9usize, 12usize, 15usize, 2156868u32);
    emu.orr_no_count(22usize, 9usize, 28usize, 2156872u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2156872u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e948));
}
#[inline]
pub fn block_0x0020e948(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(24usize, 5usize, 24usize, 2156876u32);
    emu.sw_no_count(18usize, 2usize, 12u32, 2156880u32)?;
    emu.sltru_no_count(18usize, 18usize, 30usize, 2156884u32);
    emu.lhu_no_count(9usize, 16usize, 10u32, 2156888u32)?;
    emu.adr_no_count(8usize, 8usize, 10usize, 2156892u32);
    emu.sai_no_count(5usize, 6usize, 1055u32, 2156896u32);
    emu.slti_no_count(10usize, 29usize, 0u32, 2156900u32);
    emu.adi_no_count(16usize, 0usize, 1u32, 2156904u32);
    emu.sri_no_count(6usize, 22usize, 4u32, 2156908u32);
    emu.adi_no_count(28usize, 0usize, 625u32, 2156912u32);
    emu.slr_no_count(30usize, 16usize, 7usize, 2156916u32);
    emu.slr_no_count(16usize, 16usize, 15usize, 2156920u32);
    emu.adi_no_count(15usize, 10usize, 4294967295u32, 2156924u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2156928u32);
    emu.anr_no_count(15usize, 15usize, 30usize, 2156932u32);
    emu.anr_no_count(16usize, 10usize, 16usize, 2156936u32);
    emu.sltiu_no_count(10usize, 16usize, 1u32, 2156940u32);
    emu.sbr_no_count(19usize, 15usize, 10usize, 2156944u32);
    emu.adi_no_count(20usize, 16usize, 4294967295u32, 2156948u32);
    emu.sw_no_count(26usize, 2usize, 20u32, 2156952u32)?;
    emu.add_memory_rw_events(20usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a >= b {
        emu.pc = 2156984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e9b8));
    } else {
        emu.pc = 2156956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e99c));
    }
}
#[inline(always)]
pub fn block_0x0020e99c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 0usize, 100u32, 2156960u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(26usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a >= b {
        emu.pc = 2157028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e9e4));
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
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(6usize, 0usize, 9u32, 2156968u32);
    emu.sltiu_no_count(10usize, 22usize, 10u32, 2156972u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a < b {
        emu.pc = 2157096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ea28));
    } else {
        emu.pc = 2156976u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e9b0));
    }
}
#[inline(always)]
pub fn block_0x0020e9b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 0usize, 1u32, 2156980u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2156984u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157100u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ea2c));
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
    let a = 0u32.wrapping_add(999424u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2156988u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 576u32, 2156992u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a >= b {
        emu.pc = 2157052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e9fc));
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
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(98304u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2157000u32;
    emu.update_insn_clock();
    emu.adi_no_count(26usize, 10usize, 1696u32, 2157004u32);
    emu.sltru_no_count(10usize, 22usize, 26usize, 2157008u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(26usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a >= b {
        emu.pc = 2157020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e9dc));
    } else {
        emu.pc = 2157012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e9d4));
    }
}
#[inline(always)]
pub fn block_0x0020e9d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2157016u32;
    emu.update_insn_clock();
    emu.adi_no_count(26usize, 6usize, 1808u32, 2157020u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2157020u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e9dc));
}
#[inline(always)]
pub fn block_0x0020e9dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(1usize, 10usize, 5u32, 2157024u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2157028u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157132u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ea4c));
}
#[inline(always)]
pub fn block_0x0020e9e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(6usize, 0usize, 1000u32, 2157032u32);
    emu.sltiu_no_count(10usize, 22usize, 1000u32, 2157036u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a < b {
        emu.pc = 2157044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e9f4));
    } else {
        emu.pc = 2157040u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e9f0));
    }
}
#[inline(always)]
pub fn block_0x0020e9f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 0usize, 1000u32, 2157044u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2157044u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e9f4));
}
#[inline(always)]
pub fn block_0x0020e9f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(1usize, 10usize, 3u32, 2157048u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2157052u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157132u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ea4c));
}
#[inline(always)]
pub fn block_0x0020e9fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(99999744u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2157056u32;
    emu.update_insn_clock();
    emu.adi_no_count(26usize, 10usize, 256u32, 2157060u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(26usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a >= b {
        emu.pc = 2157108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ea34));
    } else {
        emu.pc = 2157064u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ea08));
    }
}
#[inline(always)]
pub fn block_0x0020ea08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(9998336u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2157068u32;
    emu.update_insn_clock();
    emu.adi_no_count(26usize, 10usize, 1664u32, 2157072u32);
    emu.sltru_no_count(10usize, 22usize, 26usize, 2157076u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(26usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a >= b {
        emu.pc = 2157088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ea20));
    } else {
        emu.pc = 2157080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ea18));
    }
}
#[inline(always)]
pub fn block_0x0020ea18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(999424u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2157084u32;
    emu.update_insn_clock();
    emu.adi_no_count(26usize, 6usize, 576u32, 2157088u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2157088u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ea20));
}
#[inline(always)]
pub fn block_0x0020ea20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(1usize, 10usize, 7u32, 2157092u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2157096u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157132u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ea4c));
}
#[inline(always)]
pub fn block_0x0020ea28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 0usize, 10u32, 2157100u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2157100u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ea2c));
}
#[inline(always)]
pub fn block_0x0020ea2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(1usize, 10usize, 1u32, 2157104u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2157108u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157132u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ea4c));
}
#[inline(always)]
pub fn block_0x0020ea34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(1000001536u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2157112u32;
    emu.update_insn_clock();
    emu.adi_no_count(6usize, 10usize, 4294965760u32, 2157116u32);
    emu.sltru_no_count(10usize, 22usize, 6usize, 2157120u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a < b {
        emu.pc = 2157128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ea48));
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
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 6usize, 0u32, 2157128u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2157128u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ea48));
}
#[inline(always)]
pub fn block_0x0020ea48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(1usize, 10usize, 9u32, 2157132u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2157132u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ea4c));
}
#[inline(never)]
pub fn block_0x0020ea4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(18usize, 24usize, 18usize, 2157136u32);
    emu.sw_no_count(18usize, 2usize, 4u32, 2157140u32)?;
    emu.sw_no_count(21usize, 2usize, 8u32, 2157144u32)?;
    emu.anr_no_count(18usize, 19usize, 21usize, 2157148u32);
    emu.anr_no_count(21usize, 20usize, 12usize, 2157152u32);
    emu.sbr_no_count(6usize, 1usize, 9usize, 2157156u32);
    emu.sltru_no_count(28usize, 5usize, 14usize, 2157160u32);
    emu.sbr_no_count(30usize, 5usize, 8usize, 2157164u32);
    emu.sbr_no_count(5usize, 5usize, 14usize, 2157168u32);
    emu.adi_no_count(24usize, 0usize, 4294967295u32, 2157172u32);
    emu.sai_no_count(14usize, 29usize, 1055u32, 2157176u32);
    let a = 0u32.wrapping_add(3435974656u32);
    emu.write_reg_no_count(8usize, a);
    emu.pc = 2157180u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 0usize, 10u32, 2157184u32);
    emu.adi_no_count(6usize, 6usize, 1u32, 2157188u32);
    emu.sw_no_count(6usize, 2usize, 0u32, 2157192u32)?;
    emu.sbr_no_count(28usize, 30usize, 28usize, 2157196u32);
    emu.adr_no_count(11usize, 5usize, 11usize, 2157200u32);
    emu.adi_no_count(6usize, 8usize, 4294966477u32, 2157204u32);
    emu.sltru_no_count(5usize, 11usize, 5usize, 2157208u32);
    emu.adi_no_count(8usize, 11usize, 2u32, 2157212u32);
    emu.adr_no_count(5usize, 17usize, 5usize, 2157216u32);
    emu.sltru_no_count(9usize, 8usize, 11usize, 2157220u32);
    emu.anr_no_count(17usize, 8usize, 20usize, 2157224u32);
    emu.adr_no_count(5usize, 28usize, 5usize, 2157228u32);
    emu.adr_no_count(9usize, 5usize, 9usize, 2157232u32);
    emu.anr_no_count(5usize, 9usize, 19usize, 2157236u32);
    emu.adi_no_count(11usize, 0usize, 4294967295u32, 2157240u32);
    emu.lw_no_count(23usize, 2usize, 20u32, 2157244u32)?;
    emu.add_memory_rw_events(28usize);
    emu.pc = 2157244u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020eabc));
}
#[inline(always)]
pub fn block_0x0020eabc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(28usize, 13usize, 11usize, 2157248u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2159128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f218));
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
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 26usize, 0u32, 2157256u32);
    emu.divu_no_count(28usize, 22usize, 26usize, 2157260u32);
    emu.mul_no_count(26usize, 28usize, 26usize, 2157264u32);
    emu.adi_no_count(30usize, 28usize, 48u32, 2157268u32);
    emu.sbr_no_count(22usize, 22usize, 26usize, 2157272u32);
    emu.sb_no_count(30usize, 23usize, 0u32, 2157276u32);
    emu.slr_no_count(26usize, 22usize, 7usize, 2157280u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(29usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2157320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eb08));
    } else {
        emu.pc = 2157284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eae4));
    }
}
#[inline(always)]
pub fn block_0x0020eae4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(28usize, 14usize, 26usize, 2157288u32);
    emu.adr_no_count(27usize, 26usize, 18usize, 2157292u32);
    emu.adr_no_count(26usize, 28usize, 21usize, 2157296u32);
    emu.sltru_no_count(28usize, 26usize, 28usize, 2157300u32);
    emu.adr_no_count(27usize, 27usize, 28usize, 2157304u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(27usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a != b {
        emu.pc = 2157352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eb28));
    } else {
        emu.pc = 2157308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eafc));
    }
}
#[inline(always)]
pub fn block_0x0020eafc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 26usize, 8usize, 2157312u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2157360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eb30));
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
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2157320u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157404u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020eb5c));
}
#[inline(always)]
pub fn block_0x0020eb08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(28usize, 22usize, 1u32, 2157324u32);
    emu.srr_no_count(27usize, 28usize, 31usize, 2157328u32);
    emu.anr_no_count(28usize, 14usize, 26usize, 2157332u32);
    emu.adr_no_count(27usize, 27usize, 18usize, 2157336u32);
    emu.adr_no_count(26usize, 28usize, 21usize, 2157340u32);
    emu.sltru_no_count(28usize, 26usize, 28usize, 2157344u32);
    emu.adr_no_count(27usize, 27usize, 28usize, 2157348u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(27usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2157308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eafc));
    } else {
        emu.pc = 2157352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eb28));
    }
}
#[inline(always)]
pub fn block_0x0020eb28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 27usize, 9usize, 2157356u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a != b {
        emu.pc = 2157404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eb5c));
    } else {
        emu.pc = 2157360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eb30));
    }
}
#[inline(always)]
pub fn block_0x0020eb30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(28usize, 1usize, 11usize, 2157364u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2157420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eb6c));
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
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.mulhu_no_count(28usize, 25usize, 6usize, 2157372u32);
    emu.adi_no_count(23usize, 23usize, 1u32, 2157376u32);
    emu.sri_no_count(26usize, 28usize, 3u32, 2157380u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2157384u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a >= b {
        emu.pc = 2157244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eabc));
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
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2157392u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 56u32, 2157396u32);
    emu.apc_no_count(1usize, 2157396u32, 4096u32, 2157400u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2157404u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(396u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
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
    emu.slr_no_count(10usize, 25usize, 7usize, 2157408u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(29usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2157680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ec70));
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
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 10usize, 0u32, 2157416u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2157420u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157688u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ec78));
}
#[inline(always)]
pub fn block_0x0020eb6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 0u32, 2157424u32);
    emu.sbr_no_count(11usize, 0usize, 11usize, 2157428u32);
    emu.adi_no_count(6usize, 0usize, 1u32, 2157432u32);
    emu.adi_no_count(10usize, 0usize, 10u32, 2157436u32);
    emu.lw_no_count(23usize, 2usize, 20u32, 2157440u32)?;
    emu.add_memory_rw_events(6usize);
    let return_addr = 2157444u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157472u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020eba0));
}
#[inline(always)]
pub fn block_0x0020eb84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(30usize, 18usize, 5usize, 2157448u32);
    emu.mulhu_no_count(14usize, 8usize, 10usize, 2157452u32);
    emu.mul_no_count(6usize, 9usize, 10usize, 2157456u32);
    emu.adr_no_count(14usize, 14usize, 6usize, 2157460u32);
    emu.mul_no_count(6usize, 8usize, 10usize, 2157464u32);
    emu.adi_no_count(11usize, 11usize, 1u32, 2157468u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(30usize);
    if a != b {
        emu.pc = 2157596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ec1c));
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
        emu.pc = 2159152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f230));
    } else {
        emu.pc = 2157476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eba4));
    }
}
#[inline(always)]
pub fn block_0x0020eba4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 14usize, 0u32, 2157480u32);
    emu.adi_no_count(8usize, 6usize, 0u32, 2157484u32);
    emu.mulhu_no_count(14usize, 21usize, 10usize, 2157488u32);
    emu.mul_no_count(6usize, 18usize, 10usize, 2157492u32);
    emu.adr_no_count(14usize, 14usize, 6usize, 2157496u32);
    emu.mul_no_count(6usize, 21usize, 10usize, 2157500u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(29usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2157512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ebc8));
    } else {
        emu.pc = 2157504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ebc0));
    }
}
#[inline(always)]
pub fn block_0x0020ebc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.srr_no_count(30usize, 14usize, 7usize, 2157508u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2157512u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157528u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ebd8));
}
#[inline(always)]
pub fn block_0x0020ebc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(28usize, 14usize, 1u32, 2157516u32);
    emu.slr_no_count(28usize, 28usize, 31usize, 2157520u32);
    emu.srr_no_count(30usize, 6usize, 7usize, 2157524u32);
    emu.orr_no_count(30usize, 30usize, 28usize, 2157528u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2157528u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ebd8));
}
#[inline]
pub fn block_0x0020ebd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(18usize, 14usize, 19usize, 2157532u32);
    emu.anr_no_count(21usize, 6usize, 20usize, 2157536u32);
    emu.mulhu_no_count(14usize, 17usize, 10usize, 2157540u32);
    emu.mul_no_count(5usize, 5usize, 10usize, 2157544u32);
    emu.mul_no_count(17usize, 17usize, 10usize, 2157548u32);
    emu.adi_no_count(22usize, 30usize, 48u32, 2157552u32);
    emu.adr_no_count(5usize, 14usize, 5usize, 2157556u32);
    emu.adr_no_count(14usize, 23usize, 11usize, 2157560u32);
    emu.sb_no_count(22usize, 14usize, 0u32, 2157564u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a != b {
        emu.pc = 2157444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eb84));
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
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(30usize, 21usize, 17usize, 2157572u32);
    emu.mulhu_no_count(14usize, 8usize, 10usize, 2157576u32);
    emu.mul_no_count(6usize, 9usize, 10usize, 2157580u32);
    emu.adr_no_count(14usize, 14usize, 6usize, 2157584u32);
    emu.mul_no_count(6usize, 8usize, 10usize, 2157588u32);
    emu.adi_no_count(11usize, 11usize, 1u32, 2157592u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(30usize);
    if a == b {
        emu.pc = 2157472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eba0));
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
#[inline]
pub fn block_0x0020ec1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(7usize, 2usize, 12u32, 2157600u32)?;
    emu.sltru_no_count(10usize, 12usize, 7usize, 2157604u32);
    emu.lw_no_count(13usize, 2usize, 8u32, 2157608u32)?;
    emu.lw_no_count(28usize, 2usize, 4u32, 2157612u32)?;
    emu.sbr_no_count(13usize, 13usize, 28usize, 2157616u32);
    emu.sbr_no_count(12usize, 12usize, 7usize, 2157620u32);
    emu.sbr_no_count(13usize, 13usize, 10usize, 2157624u32);
    emu.mulhu_no_count(10usize, 6usize, 12usize, 2157628u32);
    emu.mul_no_count(31usize, 14usize, 12usize, 2157632u32);
    emu.mul_no_count(7usize, 6usize, 12usize, 2157636u32);
    emu.mul_no_count(12usize, 6usize, 13usize, 2157640u32);
    emu.adr_no_count(13usize, 7usize, 6usize, 2157644u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2157648u32);
    emu.sltru_no_count(12usize, 7usize, 6usize, 2157652u32);
    emu.adr_no_count(31usize, 10usize, 31usize, 2157656u32);
    emu.adr_no_count(12usize, 14usize, 12usize, 2157660u32);
    emu.sbr_no_count(29usize, 31usize, 12usize, 2157664u32);
    emu.sbr_no_count(30usize, 7usize, 6usize, 2157668u32);
    emu.add_memory_rw_events(18usize);
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2157848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed18));
    } else {
        emu.pc = 2157672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ec68));
    }
}
#[inline(always)]
pub fn block_0x0020ec68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 18usize, 29usize, 2157676u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2157680u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157852u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ed1c));
}
#[inline(always)]
pub fn block_0x0020ec70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 25usize, 1u32, 2157684u32);
    emu.srr_no_count(15usize, 13usize, 31usize, 2157688u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2157688u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ec78));
}
#[inline]
pub fn block_0x0020ec78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 2usize, 16u32, 2157692u32)?;
    emu.lw_no_count(16usize, 2usize, 8u32, 2157696u32)?;
    emu.lw_no_count(17usize, 2usize, 12u32, 2157700u32)?;
    emu.sltru_no_count(13usize, 12usize, 17usize, 2157704u32);
    emu.lw_no_count(5usize, 2usize, 4u32, 2157708u32)?;
    emu.sbr_no_count(16usize, 16usize, 5usize, 2157712u32);
    emu.sbr_no_count(28usize, 12usize, 17usize, 2157716u32);
    emu.sbr_no_count(17usize, 16usize, 13usize, 2157720u32);
    emu.adi_no_count(13usize, 28usize, 1u32, 2157724u32);
    emu.sltiu_no_count(12usize, 28usize, 1u32, 2157728u32);
    emu.sbr_no_count(7usize, 17usize, 12usize, 2157732u32);
    emu.adi_no_count(28usize, 28usize, 4294967295u32, 2157736u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(27usize);
    if a == b {
        emu.pc = 2157748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ecb4));
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
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 27usize, 7usize, 2157744u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2157748u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157752u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ecb8));
}
#[inline(always)]
pub fn block_0x0020ecb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 26usize, 28usize, 2157752u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2157752u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ecb8));
}
#[inline(always)]
pub fn block_0x0020ecb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(6usize, 14usize, 10usize, 2157756u32);
    emu.sltiu_no_count(29usize, 13usize, 1u32, 2157760u32);
    emu.sltru_no_count(10usize, 8usize, 26usize, 2157764u32);
    emu.sbr_no_count(12usize, 9usize, 27usize, 2157768u32);
    emu.sbr_no_count(10usize, 12usize, 10usize, 2157772u32);
    emu.sbr_no_count(5usize, 8usize, 26usize, 2157776u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2157796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ece4));
    } else {
        emu.pc = 2157780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ecd4));
    }
}
#[inline(always)]
pub fn block_0x0020ecd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(14usize, 10usize, 15usize, 2157784u32);
    emu.adr_no_count(17usize, 17usize, 29usize, 2157788u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a != b {
        emu.pc = 2157808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ecf0));
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
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2157796u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157820u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ecfc));
}
#[inline(always)]
pub fn block_0x0020ece4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(14usize, 5usize, 6usize, 2157800u32);
    emu.adr_no_count(17usize, 17usize, 29usize, 2157804u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2157820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ecfc));
    } else {
        emu.pc = 2157808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ecf0));
    }
}
#[inline(always)]
pub fn block_0x0020ecf0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2157948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed7c));
    } else {
        emu.pc = 2157812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ecf4));
    }
}
#[inline(always)]
pub fn block_0x0020ecf4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 10usize, 15usize, 2157816u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2157976u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed98));
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
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 26usize, 0u32, 2157824u32);
    emu.adi_no_count(5usize, 27usize, 0u32, 2157828u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(27usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a != b {
        emu.pc = 2158164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ee54));
    } else {
        emu.pc = 2157832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed08));
    }
}
#[inline(always)]
pub fn block_0x0020ed08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 16usize, 13usize, 2157836u32);
    emu.lw_no_count(7usize, 2usize, 20u32, 2157840u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2158176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ee60));
    } else {
        emu.pc = 2157844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed14));
    }
}
#[inline(always)]
pub fn block_0x0020ed14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2157848u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2158532u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020efc4));
}
#[inline(always)]
pub fn block_0x0020ed18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 21usize, 30usize, 2157852u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2157852u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ed1c));
}
#[inline(always)]
pub fn block_0x0020ed1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 13usize, 7usize, 2157856u32);
    emu.adr_no_count(7usize, 31usize, 14usize, 2157860u32);
    emu.sltru_no_count(12usize, 17usize, 21usize, 2157864u32);
    emu.sbr_no_count(14usize, 5usize, 18usize, 2157868u32);
    emu.sbr_no_count(14usize, 14usize, 12usize, 2157872u32);
    emu.sbr_no_count(6usize, 17usize, 21usize, 2157876u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2157896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed48));
    } else {
        emu.pc = 2157880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed38));
    }
}
#[inline(always)]
pub fn block_0x0020ed38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(31usize, 14usize, 15usize, 2157884u32);
    emu.adr_no_count(28usize, 7usize, 28usize, 2157888u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2157908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed54));
    } else {
        emu.pc = 2157892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed44));
    }
}
#[inline(always)]
pub fn block_0x0020ed44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2157896u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157920u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ed60));
}
#[inline(always)]
pub fn block_0x0020ed48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(31usize, 6usize, 16usize, 2157900u32);
    emu.adr_no_count(28usize, 7usize, 28usize, 2157904u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2157920u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed60));
    } else {
        emu.pc = 2157908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed54));
    }
}
#[inline(always)]
pub fn block_0x0020ed54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2158212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ee84));
    } else {
        emu.pc = 2157912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed58));
    }
}
#[inline(always)]
pub fn block_0x0020ed58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 14usize, 15usize, 2157916u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2158220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ee8c));
    } else {
        emu.pc = 2157920u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed60));
    }
}
#[inline(always)]
pub fn block_0x0020ed60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(6usize, 21usize, 0u32, 2157924u32);
    emu.adi_no_count(7usize, 18usize, 0u32, 2157928u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a != b {
        emu.pc = 2158432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ef60));
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
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 6usize, 13usize, 2157936u32);
    emu.lw_no_count(29usize, 2usize, 16u32, 2157940u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2158444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ef6c));
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
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2157948u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2158652u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f03c));
}
#[inline(always)]
pub fn block_0x0020ed7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 5usize, 6usize, 2157952u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2157976u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed98));
    } else {
        emu.pc = 2157956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed84));
    }
}
#[inline(always)]
pub fn block_0x0020ed84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2157960u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157820u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ecfc));
}
#[inline(always)]
pub fn block_0x0020ed88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 29usize, 15usize, 2157964u32);
    emu.adi_no_count(26usize, 16usize, 0u32, 2157968u32);
    emu.adi_no_count(27usize, 5usize, 0u32, 2157972u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2158160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ee50));
    } else {
        emu.pc = 2157976u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed98));
    }
}
#[inline(always)]
pub fn block_0x0020ed98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(16usize, 26usize, 6usize, 2157980u32);
    emu.sltru_no_count(5usize, 16usize, 26usize, 2157984u32);
    emu.adr_no_count(10usize, 27usize, 15usize, 2157988u32);
    emu.adr_no_count(5usize, 10usize, 5usize, 2157992u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2158008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020edb8));
    } else {
        emu.pc = 2157996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020edac));
    }
}
#[inline(always)]
pub fn block_0x0020edac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 5usize, 7usize, 2158000u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2158016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020edc0));
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
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2158008u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2158060u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020edec));
}
#[inline(always)]
pub fn block_0x0020edb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 16usize, 28usize, 2158012u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2158060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020edec));
    } else {
        emu.pc = 2158016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020edc0));
    }
}
#[inline(always)]
pub fn block_0x0020edc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 28usize, 26usize, 2158020u32);
    emu.sbr_no_count(12usize, 7usize, 27usize, 2158024u32);
    emu.sltru_no_count(14usize, 16usize, 28usize, 2158028u32);
    emu.sbr_no_count(10usize, 12usize, 10usize, 2158032u32);
    emu.sbr_no_count(12usize, 5usize, 7usize, 2158036u32);
    emu.sbr_no_count(12usize, 12usize, 14usize, 2158040u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2158052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ede4));
    } else {
        emu.pc = 2158044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eddc));
    }
}
#[inline(always)]
pub fn block_0x0020eddc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 28usize, 26usize, 2158048u32);
    emu.sbr_no_count(12usize, 16usize, 28usize, 2158052u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2158052u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ede4));
}
#[inline(always)]
pub fn block_0x0020ede4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 10usize, 12usize, 2158056u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2158784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f0c0));
    } else {
        emu.pc = 2158060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020edec));
    }
}
#[inline(always)]
pub fn block_0x0020edec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(30usize, 30usize, 4294967295u32, 2158064u32);
    emu.sb_no_count(30usize, 23usize, 0u32, 2158068u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2158108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ee1c));
    } else {
        emu.pc = 2158072u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020edf8));
    }
}
#[inline(always)]
pub fn block_0x0020edf8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 5usize, 7usize, 2158076u32);
    emu.sltru_no_count(12usize, 8usize, 16usize, 2158080u32);
    emu.sbr_no_count(14usize, 9usize, 5usize, 2158084u32);
    emu.sbr_no_count(29usize, 14usize, 12usize, 2158088u32);
    emu.sbr_no_count(31usize, 8usize, 16usize, 2158092u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(29usize);
    if a != b {
        emu.pc = 2158132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ee34));
    } else {
        emu.pc = 2158096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ee10));
    }
}
#[inline(always)]
pub fn block_0x0020ee10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(14usize, 31usize, 6usize, 2158100u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2158140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ee3c));
    } else {
        emu.pc = 2158104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ee18));
    }
}
#[inline(always)]
pub fn block_0x0020ee18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2158108u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2158160u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ee50));
}
#[inline(always)]
pub fn block_0x0020ee1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 16usize, 28usize, 2158112u32);
    emu.sltru_no_count(12usize, 8usize, 16usize, 2158116u32);
    emu.sbr_no_count(14usize, 9usize, 5usize, 2158120u32);
    emu.sbr_no_count(29usize, 14usize, 12usize, 2158124u32);
    emu.sbr_no_count(31usize, 8usize, 16usize, 2158128u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(29usize);
    if a == b {
        emu.pc = 2158096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ee10));
    } else {
        emu.pc = 2158132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ee34));
    }
}
#[inline(always)]
pub fn block_0x0020ee34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(14usize, 29usize, 15usize, 2158136u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2158160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ee50));
    } else {
        emu.pc = 2158140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ee3c));
    }
}
#[inline(always)]
pub fn block_0x0020ee3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2157960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed88));
    } else {
        emu.pc = 2158144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ee40));
    }
}
#[inline(always)]
pub fn block_0x0020ee40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 31usize, 6usize, 2158148u32);
    emu.adi_no_count(26usize, 16usize, 0u32, 2158152u32);
    emu.adi_no_count(27usize, 5usize, 0u32, 2158156u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2157976u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed98));
    } else {
        emu.pc = 2158160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ee50));
    }
}
#[inline(always)]
pub fn block_0x0020ee50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2157832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed08));
    } else {
        emu.pc = 2158164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ee54));
    }
}
#[inline(always)]
pub fn block_0x0020ee54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 5usize, 17usize, 2158168u32);
    emu.lw_no_count(7usize, 2usize, 20u32, 2158172u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2158532u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020efc4));
    } else {
        emu.pc = 2158176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ee60));
    }
}
#[inline(always)]
pub fn block_0x0020ee60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2158532u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020efc4));
    } else {
        emu.pc = 2158180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ee64));
    }
}
#[inline(always)]
pub fn block_0x0020ee64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(6usize, 16usize, 6usize, 2158184u32);
    emu.sltru_no_count(10usize, 6usize, 16usize, 2158188u32);
    emu.adr_no_count(15usize, 5usize, 15usize, 2158192u32);
    emu.adr_no_count(10usize, 15usize, 10usize, 2158196u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2158480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ef90));
    } else {
        emu.pc = 2158200u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ee78));
    }
}
#[inline(always)]
pub fn block_0x0020ee78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 10usize, 17usize, 2158204u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2158488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ef98));
    } else {
        emu.pc = 2158208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ee80));
    }
}
#[inline(always)]
pub fn block_0x0020ee80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2158212u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2158776u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f0b8));
}
#[inline(always)]
pub fn block_0x0020ee84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 6usize, 16usize, 2158216u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2157920u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed60));
    } else {
        emu.pc = 2158220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ee8c));
    }
}
#[inline(always)]
pub fn block_0x0020ee8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(14usize, 23usize, 11usize, 2158224u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2158228u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2158244u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020eea4));
}
#[inline(always)]
pub fn block_0x0020ee94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 18usize, 15usize, 2158232u32);
    emu.adi_no_count(21usize, 6usize, 0u32, 2158236u32);
    emu.adi_no_count(18usize, 7usize, 0u32, 2158240u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2158428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ef5c));
    } else {
        emu.pc = 2158244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eea4));
    }
}
#[inline(always)]
pub fn block_0x0020eea4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(6usize, 21usize, 16usize, 2158248u32);
    emu.sltru_no_count(7usize, 6usize, 21usize, 2158252u32);
    emu.adr_no_count(10usize, 18usize, 15usize, 2158256u32);
    emu.adr_no_count(7usize, 10usize, 7usize, 2158260u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a == b {
        emu.pc = 2158276u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eec4));
    } else {
        emu.pc = 2158264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eeb8));
    }
}
#[inline(always)]
pub fn block_0x0020eeb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 7usize, 29usize, 2158268u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2158284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eecc));
    } else {
        emu.pc = 2158272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eec0));
    }
}
#[inline(always)]
pub fn block_0x0020eec0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2158276u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2158328u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020eef8));
}
#[inline(always)]
pub fn block_0x0020eec4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 6usize, 30usize, 2158280u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2158328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eef8));
    } else {
        emu.pc = 2158284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eecc));
    }
}
#[inline(always)]
pub fn block_0x0020eecc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 30usize, 21usize, 2158288u32);
    emu.sbr_no_count(12usize, 29usize, 18usize, 2158292u32);
    emu.sltru_no_count(31usize, 6usize, 30usize, 2158296u32);
    emu.sbr_no_count(10usize, 12usize, 10usize, 2158300u32);
    emu.sbr_no_count(12usize, 7usize, 29usize, 2158304u32);
    emu.sbr_no_count(12usize, 12usize, 31usize, 2158308u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2158320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eef0));
    } else {
        emu.pc = 2158312u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eee8));
    }
}
#[inline(always)]
pub fn block_0x0020eee8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 30usize, 21usize, 2158316u32);
    emu.sbr_no_count(12usize, 6usize, 30usize, 2158320u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2158320u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020eef0));
}
#[inline(always)]
pub fn block_0x0020eef0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 10usize, 12usize, 2158324u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2158804u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f0d4));
    } else {
        emu.pc = 2158328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eef8));
    }
}
#[inline(always)]
pub fn block_0x0020eef8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(22usize, 22usize, 4294967295u32, 2158332u32);
    emu.sb_no_count(22usize, 14usize, 4294967295u32, 2158336u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a == b {
        emu.pc = 2158376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ef28));
    } else {
        emu.pc = 2158340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ef04));
    }
}
#[inline(always)]
pub fn block_0x0020ef04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 7usize, 29usize, 2158344u32);
    emu.sltru_no_count(12usize, 17usize, 6usize, 2158348u32);
    emu.sbr_no_count(31usize, 5usize, 7usize, 2158352u32);
    emu.sbr_no_count(18usize, 31usize, 12usize, 2158356u32);
    emu.sbr_no_count(19usize, 17usize, 6usize, 2158360u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2158400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ef40));
    } else {
        emu.pc = 2158364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ef1c));
    }
}
#[inline(always)]
pub fn block_0x0020ef1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(31usize, 19usize, 16usize, 2158368u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2158408u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ef48));
    } else {
        emu.pc = 2158372u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ef24));
    }
}
#[inline(always)]
pub fn block_0x0020ef24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2158376u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2158428u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ef5c));
}
#[inline(always)]
pub fn block_0x0020ef28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 6usize, 30usize, 2158380u32);
    emu.sltru_no_count(12usize, 17usize, 6usize, 2158384u32);
    emu.sbr_no_count(31usize, 5usize, 7usize, 2158388u32);
    emu.sbr_no_count(18usize, 31usize, 12usize, 2158392u32);
    emu.sbr_no_count(19usize, 17usize, 6usize, 2158396u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2158364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ef1c));
    } else {
        emu.pc = 2158400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ef40));
    }
}
#[inline(always)]
pub fn block_0x0020ef40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(31usize, 18usize, 15usize, 2158404u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2158428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ef5c));
    } else {
        emu.pc = 2158408u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ef48));
    }
}
#[inline(always)]
pub fn block_0x0020ef48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2158228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ee94));
    } else {
        emu.pc = 2158412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ef4c));
    }
}
#[inline(always)]
pub fn block_0x0020ef4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 19usize, 16usize, 2158416u32);
    emu.adi_no_count(21usize, 6usize, 0u32, 2158420u32);
    emu.adi_no_count(18usize, 7usize, 0u32, 2158424u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2158244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eea4));
    } else {
        emu.pc = 2158428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ef5c));
    }
}
#[inline(always)]
pub fn block_0x0020ef5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2157932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ed6c));
    } else {
        emu.pc = 2158432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ef60));
    }
}
#[inline(always)]
pub fn block_0x0020ef60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 7usize, 28usize, 2158436u32);
    emu.lw_no_count(29usize, 2usize, 16u32, 2158440u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2158652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f03c));
    } else {
        emu.pc = 2158444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ef6c));
    }
}
#[inline(always)]
pub fn block_0x0020ef6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2158652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f03c));
    } else {
        emu.pc = 2158448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ef70));
    }
}
#[inline(always)]
pub fn block_0x0020ef70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(16usize, 6usize, 16usize, 2158452u32);
    emu.sltru_no_count(10usize, 16usize, 6usize, 2158456u32);
    emu.adr_no_count(15usize, 7usize, 15usize, 2158460u32);
    emu.adr_no_count(10usize, 15usize, 10usize, 2158464u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2158600u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f008));
    } else {
        emu.pc = 2158468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ef84));
    }
}
#[inline(always)]
pub fn block_0x0020ef84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 10usize, 28usize, 2158472u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2158608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f010));
    } else {
        emu.pc = 2158476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ef8c));
    }
}
#[inline(always)]
pub fn block_0x0020ef8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2158480u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2158832u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f0f0));
}
#[inline(always)]
pub fn block_0x0020ef90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 6usize, 13usize, 2158484u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2158776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f0b8));
    } else {
        emu.pc = 2158488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ef98));
    }
}
#[inline(always)]
pub fn block_0x0020ef98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 13usize, 16usize, 2158492u32);
    emu.sbr_no_count(14usize, 17usize, 5usize, 2158496u32);
    emu.sbr_no_count(15usize, 10usize, 17usize, 2158500u32);
    emu.sltru_no_count(17usize, 6usize, 13usize, 2158504u32);
    emu.sbr_no_count(10usize, 14usize, 12usize, 2158508u32);
    emu.sbr_no_count(12usize, 15usize, 17usize, 2158512u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2158524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020efbc));
    } else {
        emu.pc = 2158516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020efb4));
    }
}
#[inline(always)]
pub fn block_0x0020efb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 13usize, 16usize, 2158520u32);
    emu.sbr_no_count(12usize, 6usize, 13usize, 2158524u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2158524u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020efbc));
}
#[inline(always)]
pub fn block_0x0020efbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 10usize, 12usize, 2158528u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2158776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f0b8));
    } else {
        emu.pc = 2158532u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020efc4));
    }
}
#[inline(always)]
pub fn block_0x0020efc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(10usize, 16usize, 2u32, 2158536u32);
    emu.sltiu_no_count(12usize, 5usize, 1u32, 2158540u32);
    emu.anr_no_count(10usize, 12usize, 10usize, 2158544u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2158776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f0b8));
    } else {
        emu.pc = 2158548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020efd4));
    }
}
#[inline(always)]
pub fn block_0x0020efd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 4294967292u32, 2158552u32);
    emu.sltru_no_count(12usize, 10usize, 8usize, 2158556u32);
    emu.adr_no_count(12usize, 9usize, 12usize, 2158560u32);
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2158564u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2158768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f0b0));
    } else {
        emu.pc = 2158568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020efe8));
    }
}
#[inline(always)]
pub fn block_0x0020efe8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 12usize, 5usize, 2158572u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2158776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f0b8));
    } else {
        emu.pc = 2158576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eff0));
    }
}
#[inline(always)]
pub fn block_0x0020eff0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 0usize, 11usize, 2158580u32);
    emu.sw_no_count(7usize, 18usize, 0u32, 2158584u32)?;
    emu.sw_no_count(10usize, 18usize, 4u32, 2158588u32)?;
    emu.lw_no_count(10usize, 2usize, 0u32, 2158592u32)?;
    emu.sh_no_count(10usize, 18usize, 8u32, 2158596u32)?;
    emu.add_memory_rw_events(6usize);
    let return_addr = 2158600u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2158836u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f0f4));
}
#[inline(always)]
pub fn block_0x0020f008(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 16usize, 13usize, 2158604u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2158832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f0f0));
    } else {
        emu.pc = 2158608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f010));
    }
}
#[inline(always)]
pub fn block_0x0020f010(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 13usize, 6usize, 2158612u32);
    emu.sbr_no_count(14usize, 28usize, 7usize, 2158616u32);
    emu.sbr_no_count(15usize, 10usize, 28usize, 2158620u32);
    emu.sltru_no_count(28usize, 16usize, 13usize, 2158624u32);
    emu.sbr_no_count(10usize, 14usize, 12usize, 2158628u32);
    emu.sbr_no_count(12usize, 15usize, 28usize, 2158632u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2158644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f034));
    } else {
        emu.pc = 2158636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f02c));
    }
}
#[inline(always)]
pub fn block_0x0020f02c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 13usize, 6usize, 2158640u32);
    emu.sbr_no_count(12usize, 16usize, 13usize, 2158644u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2158644u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f034));
}
#[inline(always)]
pub fn block_0x0020f034(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 10usize, 12usize, 2158648u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2158832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f0f0));
    } else {
        emu.pc = 2158652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f03c));
    }
}
#[inline(always)]
pub fn block_0x0020f03c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 20u32, 2158656u32);
    emu.mulhu_no_count(12usize, 8usize, 10usize, 2158660u32);
    emu.mul_no_count(13usize, 9usize, 10usize, 2158664u32);
    emu.adr_no_count(12usize, 12usize, 13usize, 2158668u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2158684u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f05c));
    } else {
        emu.pc = 2158672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f050));
    }
}
#[inline(always)]
pub fn block_0x0020f050(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 7usize, 12usize, 2158676u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2158696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f068));
    } else {
        emu.pc = 2158680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f058));
    }
}
#[inline(always)]
pub fn block_0x0020f058(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2158684u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2158832u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f0f0));
}
#[inline(always)]
pub fn block_0x0020f05c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.mul_no_count(10usize, 8usize, 10usize, 2158688u32);
    emu.sltru_no_count(10usize, 6usize, 10usize, 2158692u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2158832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f0f0));
    } else {
        emu.pc = 2158696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f068));
    }
}
#[inline]
pub fn block_0x0020f068(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4294967256u32, 2158700u32);
    emu.mul_no_count(12usize, 9usize, 10usize, 2158704u32);
    emu.mulhu_no_count(13usize, 8usize, 10usize, 2158708u32);
    emu.mul_no_count(10usize, 8usize, 10usize, 2158712u32);
    emu.sbr_no_count(13usize, 13usize, 8usize, 2158716u32);
    emu.adr_no_count(10usize, 17usize, 10usize, 2158720u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2158724u32);
    emu.adr_no_count(12usize, 5usize, 12usize, 2158728u32);
    emu.sltru_no_count(13usize, 10usize, 17usize, 2158732u32);
    emu.adr_no_count(12usize, 12usize, 13usize, 2158736u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a == b {
        emu.pc = 2158824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f0e8));
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
#[inline(always)]
pub fn block_0x0020f094(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 12usize, 7usize, 2158744u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2158832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f0f0));
    } else {
        emu.pc = 2158748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f09c));
    }
}
#[inline(always)]
pub fn block_0x0020f09c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(23usize, 29usize, 0u32, 2158752u32)?;
    emu.sw_no_count(11usize, 29usize, 4u32, 2158756u32)?;
    emu.lw_no_count(10usize, 2usize, 0u32, 2158760u32)?;
    emu.sh_no_count(10usize, 29usize, 8u32, 2158764u32)?;
    emu.add_memory_rw_events(5usize);
    let return_addr = 2158768u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2158836u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f0f4));
}
#[inline(always)]
pub fn block_0x0020f0b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 10usize, 16usize, 2158772u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2158576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eff0));
    } else {
        emu.pc = 2158776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f0b8));
    }
}
#[inline(always)]
pub fn block_0x0020f0b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 18usize, 0u32, 2158780u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2158784u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2158836u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f0f4));
}
#[inline(always)]
pub fn block_0x0020f0c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 0u32, 2158788u32);
    emu.adi_no_count(16usize, 26usize, 0u32, 2158792u32);
    emu.adi_no_count(5usize, 27usize, 0u32, 2158796u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(27usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a != b {
        emu.pc = 2158164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ee54));
    } else {
        emu.pc = 2158800u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f0d0));
    }
}
#[inline(always)]
pub fn block_0x0020f0d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2158804u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157832u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ed08));
}
#[inline(always)]
pub fn block_0x0020f0d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(31usize, 0usize, 0u32, 2158808u32);
    emu.adi_no_count(6usize, 21usize, 0u32, 2158812u32);
    emu.adi_no_count(7usize, 18usize, 0u32, 2158816u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a != b {
        emu.pc = 2158432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ef60));
    } else {
        emu.pc = 2158820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f0e4));
    }
}
#[inline(always)]
pub fn block_0x0020f0e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2158824u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157932u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ed6c));
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
    emu.sltru_no_count(10usize, 10usize, 6usize, 2158828u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2158748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f09c));
    } else {
        emu.pc = 2158832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f0f0));
    }
}
#[inline(always)]
pub fn block_0x0020f0f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 29usize, 0u32, 2158836u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2158836u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f0f4));
}
#[inline]
pub fn block_0x0020f0f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 124u32, 2158840u32)?;
    emu.lw_no_count(8usize, 2usize, 120u32, 2158844u32)?;
    emu.lw_no_count(9usize, 2usize, 116u32, 2158848u32)?;
    emu.lw_no_count(18usize, 2usize, 112u32, 2158852u32)?;
    emu.lw_no_count(19usize, 2usize, 108u32, 2158856u32)?;
    emu.lw_no_count(20usize, 2usize, 104u32, 2158860u32)?;
    emu.lw_no_count(21usize, 2usize, 100u32, 2158864u32)?;
    emu.lw_no_count(22usize, 2usize, 96u32, 2158868u32)?;
    emu.lw_no_count(23usize, 2usize, 92u32, 2158872u32)?;
    emu.lw_no_count(24usize, 2usize, 88u32, 2158876u32)?;
    emu.lw_no_count(25usize, 2usize, 84u32, 2158880u32)?;
    emu.lw_no_count(26usize, 2usize, 80u32, 2158884u32)?;
    emu.lw_no_count(27usize, 2usize, 76u32, 2158888u32)?;
    emu.adi_no_count(2usize, 2usize, 128u32, 2158892u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158896u32;
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
pub fn block_0x0020f130(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 2usize, 52u32, 2158900u32)?;
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2158904u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966408u32, 2158908u32);
    emu.adi_no_count(11usize, 2usize, 40u32, 2158912u32);
    emu.adi_no_count(12usize, 2usize, 24u32, 2158916u32);
    emu.adi_no_count(13usize, 2usize, 52u32, 2158920u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2158924u32);
    emu.apc_no_count(1usize, 2158924u32, 4294955008u32, 2158928u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158932u32;
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
pub fn block_0x0020f154(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2158936u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294967088u32, 2158940u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2158944u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967116u32, 2158948u32);
    emu.adi_no_count(11usize, 0usize, 28u32, 2158952u32);
    emu.apc_no_count(1usize, 2158952u32, 4294955008u32, 2158956u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158960u32;
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
pub fn block_0x0020f170(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2158964u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294967132u32, 2158968u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2158972u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967164u32, 2158976u32);
    emu.adi_no_count(11usize, 0usize, 29u32, 2158980u32);
    emu.apc_no_count(1usize, 2158980u32, 4294955008u32, 2158984u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2158988u32;
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
pub fn block_0x0020f18c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2158992u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294967180u32, 2158996u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2159000u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967208u32, 2159004u32);
    emu.adi_no_count(11usize, 0usize, 28u32, 2159008u32);
    emu.apc_no_count(1usize, 2159008u32, 4294955008u32, 2159012u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2159016u32;
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
pub fn block_0x0020f1a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2159020u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 176u32, 2159024u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2159028u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 232u32, 2159032u32);
    emu.adi_no_count(11usize, 0usize, 54u32, 2159036u32);
    emu.apc_no_count(1usize, 2159036u32, 4294955008u32, 2159040u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2159044u32;
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
pub fn block_0x0020f1c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2159048u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 104u32, 2159052u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2159056u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 160u32, 2159060u32);
    emu.adi_no_count(11usize, 0usize, 55u32, 2159064u32);
    emu.apc_no_count(1usize, 2159064u32, 4294955008u32, 2159068u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2159072u32;
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
pub fn block_0x0020f1e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2159076u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294967224u32, 2159080u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2159084u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967272u32, 2159088u32);
    emu.adi_no_count(11usize, 0usize, 45u32, 2159092u32);
    emu.apc_no_count(1usize, 2159092u32, 4294955008u32, 2159096u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2159100u32;
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
pub fn block_0x0020f1fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2159104u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294967288u32, 2159108u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2159112u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 40u32, 2159116u32);
    emu.adi_no_count(11usize, 0usize, 45u32, 2159120u32);
    emu.apc_no_count(1usize, 2159120u32, 4294955008u32, 2159124u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2159128u32;
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
pub fn block_0x0020f218(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2159132u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 72u32, 2159136u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2159140u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2159144u32);
    emu.apc_no_count(1usize, 2159144u32, 4294955008u32, 2159148u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2159152u32;
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
pub fn block_0x0020f230(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2159156u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 88u32, 2159160u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2159164u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2159168u32);
    emu.apc_no_count(1usize, 2159168u32, 4294955008u32, 2159172u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2159176u32;
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
pub fn block_0x0020f248(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2159180u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967072u32, 2159184u32);
    emu.adi_no_count(11usize, 0usize, 81u32, 2159188u32);
    emu.adi_no_count(10usize, 14usize, 0u32, 2159192u32);
    emu.apc_no_count(1usize, 2159192u32, 4294955008u32, 2159196u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2159200u32;
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
