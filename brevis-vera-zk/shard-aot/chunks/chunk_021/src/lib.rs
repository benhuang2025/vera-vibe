pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2170452u32;
pub const PC_MAX: u32 = 2172380u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 109usize] = [
        block_0x00211e54,
        block_0x00211ea4,
        block_0x00211eb8,
        block_0x00211ec8,
        block_0x00211edc,
        block_0x00211ee0,
        block_0x00211ee4,
        block_0x00211ee8,
        block_0x00211ef0,
        block_0x00211ef4,
        block_0x00211ef8,
        block_0x00211f00,
        block_0x00211f38,
        block_0x00211fa0,
        block_0x00211ffc,
        block_0x0021206c,
        block_0x0021209c,
        block_0x002120cc,
        block_0x002120e0,
        block_0x002120f0,
        block_0x00212100,
        block_0x00212110,
        block_0x00212120,
        block_0x00212124,
        block_0x00212134,
        block_0x0021214c,
        block_0x00212150,
        block_0x0021216c,
        block_0x0021217c,
        block_0x00212184,
        block_0x0021218c,
        block_0x002121a0,
        block_0x002121b0,
        block_0x002121b4,
        block_0x002121bc,
        block_0x002121c0,
        block_0x002121d4,
        block_0x00212204,
        block_0x00212208,
        block_0x00212210,
        block_0x00212224,
        block_0x00212230,
        block_0x00212234,
        block_0x0021223c,
        block_0x00212258,
        block_0x0021225c,
        block_0x00212274,
        block_0x00212288,
        block_0x00212294,
        block_0x00212298,
        block_0x002122ac,
        block_0x002122d4,
        block_0x002122d8,
        block_0x002122e0,
        block_0x002122e8,
        block_0x002122f8,
        block_0x002122fc,
        block_0x00212310,
        block_0x00212338,
        block_0x0021233c,
        block_0x00212344,
        block_0x0021234c,
        block_0x00212354,
        block_0x00212368,
        block_0x00212390,
        block_0x00212394,
        block_0x0021239c,
        block_0x002123a4,
        block_0x002123ac,
        block_0x002123bc,
        block_0x002123c0,
        block_0x002123d4,
        block_0x002123e4,
        block_0x002123f8,
        block_0x00212408,
        block_0x0021241c,
        block_0x0021242c,
        block_0x00212440,
        block_0x00212444,
        block_0x0021244c,
        block_0x0021247c,
        block_0x00212480,
        block_0x00212498,
        block_0x0021249c,
        block_0x002124b4,
        block_0x002124b8,
        block_0x002124bc,
        block_0x002124cc,
        block_0x00212500,
        block_0x00212504,
        block_0x00212514,
        block_0x00212518,
        block_0x0021251c,
        block_0x0021252c,
        block_0x00212530,
        block_0x00212534,
        block_0x00212548,
        block_0x0021254c,
        block_0x00212564,
        block_0x00212568,
        block_0x0021256c,
        block_0x0021257c,
        block_0x002125b0,
        block_0x002125b4,
        block_0x002125c4,
        block_0x002125c8,
        block_0x002125cc,
        block_0x002125d8,
        block_0x002125dc,
    ];
    const IDX: [u16; 483usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 0u16, 0u16, 0u16, 3u16,
        0u16, 0u16, 0u16, 4u16, 0u16, 0u16, 0u16, 0u16, 5u16, 6u16, 7u16, 8u16, 0u16,
        9u16, 10u16, 11u16, 0u16, 12u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 13u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 14u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 15u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 16u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 17u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 18u16, 0u16, 0u16, 0u16, 0u16, 19u16, 0u16, 0u16, 0u16, 20u16, 0u16,
        0u16, 0u16, 21u16, 0u16, 0u16, 0u16, 22u16, 0u16, 0u16, 0u16, 23u16, 24u16, 0u16,
        0u16, 0u16, 25u16, 0u16, 0u16, 0u16, 0u16, 0u16, 26u16, 27u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 28u16, 0u16, 0u16, 0u16, 29u16, 0u16, 30u16, 0u16, 31u16, 0u16,
        0u16, 0u16, 0u16, 32u16, 0u16, 0u16, 0u16, 33u16, 34u16, 0u16, 35u16, 36u16,
        0u16, 0u16, 0u16, 0u16, 37u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 38u16, 39u16, 0u16, 40u16, 0u16, 0u16, 0u16, 0u16, 41u16, 0u16,
        0u16, 42u16, 43u16, 0u16, 44u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 45u16,
        46u16, 0u16, 0u16, 0u16, 0u16, 0u16, 47u16, 0u16, 0u16, 0u16, 0u16, 48u16, 0u16,
        0u16, 49u16, 50u16, 0u16, 0u16, 0u16, 0u16, 51u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 52u16, 53u16, 0u16, 54u16, 0u16, 55u16, 0u16, 0u16, 0u16,
        56u16, 57u16, 0u16, 0u16, 0u16, 0u16, 58u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 59u16, 60u16, 0u16, 61u16, 0u16, 62u16, 0u16, 63u16, 0u16,
        0u16, 0u16, 0u16, 64u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        65u16, 66u16, 0u16, 67u16, 0u16, 68u16, 0u16, 69u16, 0u16, 0u16, 0u16, 70u16,
        71u16, 0u16, 0u16, 0u16, 0u16, 72u16, 0u16, 0u16, 0u16, 73u16, 0u16, 0u16, 0u16,
        0u16, 74u16, 0u16, 0u16, 0u16, 75u16, 0u16, 0u16, 0u16, 0u16, 76u16, 0u16, 0u16,
        0u16, 77u16, 0u16, 0u16, 0u16, 0u16, 78u16, 79u16, 0u16, 80u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 81u16, 82u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 83u16, 84u16, 0u16, 0u16, 0u16, 0u16, 0u16, 85u16, 86u16, 87u16,
        0u16, 0u16, 0u16, 88u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 89u16, 90u16, 0u16, 0u16, 0u16, 91u16, 92u16, 93u16, 0u16,
        0u16, 0u16, 94u16, 95u16, 96u16, 0u16, 0u16, 0u16, 0u16, 97u16, 98u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 99u16, 100u16, 101u16, 0u16, 0u16, 0u16, 102u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 103u16, 104u16,
        0u16, 0u16, 0u16, 105u16, 106u16, 107u16, 0u16, 0u16, 108u16, 109u16,
    ];
    if pc < 2170452u32 || pc > 2172380u32 {
        return None;
    }
    let word_offset = ((pc - 2170452u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline]
pub fn block_0x00211e54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294965904u32, 2170456u32);
    emu.sw_no_count(1usize, 2usize, 1388u32, 2170460u32)?;
    emu.sw_no_count(8usize, 2usize, 1384u32, 2170464u32)?;
    emu.sw_no_count(9usize, 2usize, 1380u32, 2170468u32)?;
    emu.sw_no_count(18usize, 2usize, 1376u32, 2170472u32)?;
    emu.sw_no_count(19usize, 2usize, 1372u32, 2170476u32)?;
    emu.sw_no_count(20usize, 2usize, 1368u32, 2170480u32)?;
    emu.sw_no_count(21usize, 2usize, 1364u32, 2170484u32)?;
    emu.sw_no_count(22usize, 2usize, 1360u32, 2170488u32)?;
    emu.sw_no_count(23usize, 2usize, 1356u32, 2170492u32)?;
    emu.sw_no_count(24usize, 2usize, 1352u32, 2170496u32)?;
    emu.sw_no_count(25usize, 2usize, 1348u32, 2170500u32)?;
    emu.sw_no_count(26usize, 2usize, 1344u32, 2170504u32)?;
    emu.sw_no_count(27usize, 2usize, 1340u32, 2170508u32)?;
    emu.adi_no_count(24usize, 13usize, 0u32, 2170512u32);
    emu.sw_no_count(12usize, 2usize, 20u32, 2170516u32)?;
    emu.lw_no_count(12usize, 11usize, 0u32, 2170520u32)?;
    emu.lw_no_count(13usize, 11usize, 4u32, 2170524u32)?;
    emu.orr_no_count(14usize, 12usize, 13usize, 2170528u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2173940u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2173940u32));
    } else {
        emu.pc = 2170532u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211ea4));
    }
}
#[inline(always)]
pub fn block_0x00211ea4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(23usize, 10usize, 0u32, 2170536u32);
    emu.lw_no_count(18usize, 11usize, 8u32, 2170540u32)?;
    emu.lw_no_count(20usize, 11usize, 12u32, 2170544u32)?;
    emu.orr_no_count(10usize, 18usize, 20usize, 2170548u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2173968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2173968u32));
    } else {
        emu.pc = 2170552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211eb8));
    }
}
#[inline(always)]
pub fn block_0x00211eb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(8usize, 11usize, 16u32, 2170556u32)?;
    emu.lw_no_count(9usize, 11usize, 20u32, 2170560u32)?;
    emu.orr_no_count(10usize, 8usize, 9usize, 2170564u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2173996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2173996u32));
    } else {
        emu.pc = 2170568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211ec8));
    }
}
#[inline(always)]
pub fn block_0x00211ec8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 12usize, 8usize, 2170572u32);
    emu.sltru_no_count(15usize, 10usize, 12usize, 2170576u32);
    emu.adr_no_count(14usize, 13usize, 9usize, 2170580u32);
    emu.adr_no_count(16usize, 14usize, 15usize, 2170584u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2170592u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211ee0));
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
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(15usize, 16usize, 13usize, 2170592u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2170592u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211ee0));
}
#[inline(always)]
pub fn block_0x00211ee0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a != b {
        emu.pc = 2174024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2174024u32));
    } else {
        emu.pc = 2170596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211ee4));
    }
}
#[inline(always)]
pub fn block_0x00211ee4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2170608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211ef0));
    } else {
        emu.pc = 2170600u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211ee8));
    }
}
#[inline(always)]
pub fn block_0x00211ee8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(15usize, 13usize, 20usize, 2170604u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2170608u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2170612u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211ef4));
}
#[inline(always)]
pub fn block_0x00211ef0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(15usize, 12usize, 18usize, 2170612u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2170612u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211ef4));
}
#[inline(always)]
pub fn block_0x00211ef4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a != b {
        emu.pc = 2174052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2174052u32));
    } else {
        emu.pc = 2170616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211ef8));
    }
}
#[inline(always)]
pub fn block_0x00211ef8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 16u32, 2170620u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2174080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2174080u32));
    } else {
        emu.pc = 2170624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211f00));
    }
}
#[inline]
pub fn block_0x00211f00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lh_no_count(19usize, 11usize, 24u32, 2170628u32)?;
    emu.sltru_no_count(15usize, 10usize, 12usize, 2170632u32);
    emu.sltiu_no_count(5usize, 10usize, 1u32, 2170636u32);
    let a = 0u32.wrapping_add(1431654400u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2170640u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(858992640u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2170644u32;
    emu.update_insn_clock();
    emu.adr_no_count(15usize, 14usize, 15usize, 2170648u32);
    let a = 0u32.wrapping_add(252645376u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2170652u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 16usize, 1365u32, 2170656u32);
    emu.adi_no_count(16usize, 6usize, 819u32, 2170660u32);
    emu.adi_no_count(14usize, 14usize, 4294967055u32, 2170664u32);
    emu.sbr_no_count(5usize, 15usize, 5usize, 2170668u32);
    let a = 0u32.wrapping_add(16842752u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2170672u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 257u32, 2170676u32);
    emu.add_memory_rw_events(13usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a != b {
        emu.pc = 2170784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211fa0));
    } else {
        emu.pc = 2170680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211f38));
    }
}
#[inline(never)]
pub fn block_0x00211f38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 26u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2170684u32);
    emu.sri_no_count(5usize, 10usize, 1u32, 2170688u32);
    emu.orr_no_count(10usize, 10usize, 5usize, 2170692u32);
    emu.sri_no_count(5usize, 10usize, 2u32, 2170696u32);
    emu.orr_no_count(10usize, 10usize, 5usize, 2170700u32);
    emu.sri_no_count(5usize, 10usize, 4u32, 2170704u32);
    emu.orr_no_count(10usize, 10usize, 5usize, 2170708u32);
    emu.sri_no_count(5usize, 10usize, 8u32, 2170712u32);
    emu.orr_no_count(10usize, 10usize, 5usize, 2170716u32);
    emu.sri_no_count(5usize, 10usize, 16u32, 2170720u32);
    emu.orr_no_count(10usize, 10usize, 5usize, 2170724u32);
    emu.xri_no_count(10usize, 10usize, 4294967295u32, 2170728u32);
    emu.sri_no_count(5usize, 10usize, 1u32, 2170732u32);
    emu.anr_no_count(17usize, 5usize, 17usize, 2170736u32);
    emu.sbr_no_count(10usize, 10usize, 17usize, 2170740u32);
    emu.anr_no_count(17usize, 10usize, 16usize, 2170744u32);
    emu.sri_no_count(10usize, 10usize, 2u32, 2170748u32);
    emu.anr_no_count(10usize, 10usize, 16usize, 2170752u32);
    emu.adr_no_count(10usize, 17usize, 10usize, 2170756u32);
    emu.sri_no_count(16usize, 10usize, 4u32, 2170760u32);
    emu.adr_no_count(10usize, 10usize, 16usize, 2170764u32);
    emu.anr_no_count(10usize, 10usize, 14usize, 2170768u32);
    emu.mul_no_count(10usize, 10usize, 15usize, 2170772u32);
    emu.sri_no_count(10usize, 10usize, 24u32, 2170776u32);
    emu.adi_no_count(10usize, 10usize, 32u32, 2170780u32);
    emu.add_memory_rw_events(26usize);
    let return_addr = 2170784u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2170876u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211ffc));
}
#[inline]
pub fn block_0x00211fa0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 23u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 5usize, 1u32, 2170788u32);
    emu.orr_no_count(10usize, 5usize, 10usize, 2170792u32);
    emu.sri_no_count(5usize, 10usize, 2u32, 2170796u32);
    emu.orr_no_count(10usize, 10usize, 5usize, 2170800u32);
    emu.sri_no_count(5usize, 10usize, 4u32, 2170804u32);
    emu.orr_no_count(10usize, 10usize, 5usize, 2170808u32);
    emu.sri_no_count(5usize, 10usize, 8u32, 2170812u32);
    emu.orr_no_count(10usize, 10usize, 5usize, 2170816u32);
    emu.sri_no_count(5usize, 10usize, 16u32, 2170820u32);
    emu.orr_no_count(10usize, 10usize, 5usize, 2170824u32);
    emu.xri_no_count(10usize, 10usize, 4294967295u32, 2170828u32);
    emu.sri_no_count(5usize, 10usize, 1u32, 2170832u32);
    emu.anr_no_count(17usize, 5usize, 17usize, 2170836u32);
    emu.sbr_no_count(10usize, 10usize, 17usize, 2170840u32);
    emu.anr_no_count(17usize, 10usize, 16usize, 2170844u32);
    emu.sri_no_count(10usize, 10usize, 2u32, 2170848u32);
    emu.anr_no_count(10usize, 10usize, 16usize, 2170852u32);
    emu.adr_no_count(10usize, 17usize, 10usize, 2170856u32);
    emu.sri_no_count(16usize, 10usize, 4u32, 2170860u32);
    emu.adr_no_count(10usize, 10usize, 16usize, 2170864u32);
    emu.anr_no_count(10usize, 10usize, 14usize, 2170868u32);
    emu.mul_no_count(10usize, 10usize, 15usize, 2170872u32);
    emu.sri_no_count(10usize, 10usize, 24u32, 2170876u32);
    emu.add_memory_rw_events(23usize);
    emu.pc = 2170876u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211ffc));
}
#[inline(never)]
pub fn block_0x00211ffc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(11usize, 11usize, 26u32, 2170880u32);
    emu.sw_no_count(11usize, 2usize, 24u32, 2170884u32)?;
    emu.sbr_no_count(10usize, 19usize, 10usize, 2170888u32);
    let a = 0u32.wrapping_add(1292914688u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2170892u32;
    emu.update_insn_clock();
    emu.sltiu_no_count(14usize, 13usize, 1u32, 2170896u32);
    emu.adi_no_count(11usize, 11usize, 4294966594u32, 2170900u32);
    emu.mulh_no_count(15usize, 10usize, 11usize, 2170904u32);
    emu.mul_no_count(10usize, 10usize, 11usize, 2170908u32);
    emu.adi_no_count(11usize, 14usize, 4294967295u32, 2170912u32);
    emu.anr_no_count(11usize, 11usize, 13usize, 2170916u32);
    emu.adi_no_count(22usize, 0usize, 2u32, 2170920u32);
    emu.sbr_no_count(13usize, 22usize, 14usize, 2170924u32);
    emu.sw_no_count(13usize, 2usize, 188u32, 2170928u32)?;
    let a = 0u32.wrapping_add(1142116352u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2170932u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 128u32, 2170936u32);
    emu.adr_no_count(13usize, 10usize, 13usize, 2170940u32);
    emu.sw_no_count(12usize, 2usize, 28u32, 2170944u32)?;
    emu.sw_no_count(11usize, 2usize, 32u32, 2170948u32)?;
    emu.sltru_no_count(10usize, 13usize, 10usize, 2170952u32);
    emu.adr_no_count(10usize, 15usize, 10usize, 2170956u32);
    emu.adi_no_count(21usize, 10usize, 19u32, 2170960u32);
    emu.sli_no_count(10usize, 21usize, 16u32, 2170964u32);
    emu.sai_no_count(25usize, 10usize, 1040u32, 2170968u32);
    emu.adi_no_count(10usize, 2usize, 36u32, 2170972u32);
    emu.adi_no_count(12usize, 0usize, 152u32, 2170976u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2170980u32);
    emu.apc_no_count(1usize, 2170980u32, 4294905856u32, 2170984u32);
    emu.add_memory_rw_events(28usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2170988u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966672u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021206c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(10usize, 20usize, 1u32, 2170992u32);
    emu.adi_no_count(11usize, 10usize, 4294967295u32, 2170996u32);
    emu.sbr_no_count(10usize, 22usize, 10usize, 2171000u32);
    emu.anr_no_count(11usize, 11usize, 20usize, 2171004u32);
    emu.sw_no_count(10usize, 2usize, 352u32, 2171008u32)?;
    emu.sw_no_count(18usize, 2usize, 192u32, 2171012u32)?;
    emu.sw_no_count(11usize, 2usize, 196u32, 2171016u32)?;
    emu.adi_no_count(10usize, 2usize, 200u32, 2171020u32);
    emu.adi_no_count(12usize, 0usize, 152u32, 2171024u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2171028u32);
    emu.apc_no_count(1usize, 2171028u32, 4294905856u32, 2171032u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2171036u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966624u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021209c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(10usize, 9usize, 1u32, 2171040u32);
    emu.adi_no_count(11usize, 10usize, 4294967295u32, 2171044u32);
    emu.sbr_no_count(10usize, 22usize, 10usize, 2171048u32);
    emu.anr_no_count(11usize, 11usize, 9usize, 2171052u32);
    emu.sw_no_count(10usize, 2usize, 516u32, 2171056u32)?;
    emu.sw_no_count(8usize, 2usize, 356u32, 2171060u32)?;
    emu.sw_no_count(11usize, 2usize, 360u32, 2171064u32)?;
    emu.adi_no_count(10usize, 2usize, 364u32, 2171068u32);
    emu.adi_no_count(12usize, 0usize, 152u32, 2171072u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2171076u32);
    emu.apc_no_count(1usize, 2171076u32, 4294905856u32, 2171080u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2171084u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966576u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002120cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 524u32, 2171088u32);
    emu.adi_no_count(12usize, 0usize, 156u32, 2171092u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2171096u32);
    emu.apc_no_count(1usize, 2171096u32, 4294905856u32, 2171100u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2171104u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966556u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002120e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2171108u32);
    emu.sw_no_count(10usize, 2usize, 680u32, 2171112u32)?;
    emu.sw_no_count(10usize, 2usize, 520u32, 2171116u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2171188u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212134));
    } else {
        emu.pc = 2171120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002120f0));
    }
}
#[inline(always)]
pub fn block_0x002120f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 28u32, 2171124u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2171128u32);
    emu.apc_no_count(1usize, 2171128u32, 4294942720u32, 2171132u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2171136u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1560u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00212100(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 192u32, 2171140u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2171144u32);
    emu.apc_no_count(1usize, 2171144u32, 4294942720u32, 2171148u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2171152u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1544u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00212110(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 356u32, 2171156u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2171160u32);
    emu.apc_no_count(1usize, 2171160u32, 4294942720u32, 2171164u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2171168u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1528u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00212120(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(25usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2171216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212150));
    } else {
        emu.pc = 2171172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212124));
    }
}
#[inline(always)]
pub fn block_0x00212124(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(11usize, 21usize, 17u32, 2171176u32);
    emu.sri_no_count(11usize, 11usize, 17u32, 2171180u32);
    emu.adi_no_count(10usize, 2usize, 520u32, 2171184u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2171188u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2171268u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212184));
}
#[inline(always)]
pub fn block_0x00212134(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 0usize, 19usize, 2171192u32);
    emu.sli_no_count(10usize, 10usize, 16u32, 2171196u32);
    emu.sai_no_count(11usize, 10usize, 1040u32, 2171200u32);
    emu.adi_no_count(10usize, 2usize, 520u32, 2171204u32);
    emu.apc_no_count(1usize, 2171204u32, 4294942720u32, 2171208u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2171212u32;
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
pub fn block_0x0021214c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(25usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2171172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212124));
    } else {
        emu.pc = 2171216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212150));
    }
}
#[inline(always)]
pub fn block_0x00212150(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 0usize, 25usize, 2171220u32);
    emu.sli_no_count(10usize, 10usize, 16u32, 2171224u32);
    emu.sri_no_count(19usize, 10usize, 16u32, 2171228u32);
    emu.adi_no_count(10usize, 2usize, 28u32, 2171232u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2171236u32);
    emu.apc_no_count(1usize, 2171236u32, 0u32, 2171240u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2171244u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965324u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021216c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 192u32, 2171248u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2171252u32);
    emu.apc_no_count(1usize, 2171252u32, 0u32, 2171256u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2171260u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965308u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021217c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 356u32, 2171264u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2171268u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2171268u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212184));
}
#[inline(always)]
pub fn block_0x00212184(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2171268u32, 0u32, 2171272u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2171276u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965292u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021218c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1176u32, 2171280u32);
    emu.adi_no_count(11usize, 2usize, 28u32, 2171284u32);
    emu.adi_no_count(12usize, 0usize, 164u32, 2171288u32);
    emu.apc_no_count(1usize, 2171288u32, 4294905856u32, 2171292u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2171296u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966612u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002121a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 2usize, 1336u32, 2171300u32)?;
    emu.lw_no_count(11usize, 2usize, 516u32, 2171304u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2171308u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2171316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002121b4));
    } else {
        emu.pc = 2171312u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002121b0));
    }
}
#[inline(always)]
pub fn block_0x002121b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 12usize, 0u32, 2171316u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2171316u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002121b4));
}
#[inline(always)]
pub fn block_0x002121b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 41u32, 2171320u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2173796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2173796u32));
    } else {
        emu.pc = 2171324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002121bc));
    }
}
#[inline(always)]
pub fn block_0x002121bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2171428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212224));
    } else {
        emu.pc = 2171328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002121c0));
    }
}
#[inline(always)]
pub fn block_0x002121c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 0u32, 2171332u32);
    emu.sli_no_count(12usize, 10usize, 2u32, 2171336u32);
    emu.adi_no_count(13usize, 2usize, 356u32, 2171340u32);
    emu.adr_no_count(14usize, 13usize, 12usize, 2171344u32);
    emu.adi_no_count(15usize, 2usize, 1176u32, 2171348u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2171348u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002121d4));
}
#[inline]
pub fn block_0x002121d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(17usize, 15usize, 0u32, 2171352u32)?;
    emu.lw_no_count(5usize, 13usize, 0u32, 2171356u32)?;
    emu.ani_no_count(16usize, 16usize, 1u32, 2171360u32);
    emu.adi_no_count(13usize, 13usize, 4u32, 2171364u32);
    emu.adr_no_count(5usize, 17usize, 5usize, 2171368u32);
    emu.sltru_no_count(17usize, 5usize, 17usize, 2171372u32);
    emu.adr_no_count(16usize, 5usize, 16usize, 2171376u32);
    emu.sltru_no_count(5usize, 16usize, 5usize, 2171380u32);
    emu.sw_no_count(16usize, 15usize, 0u32, 2171384u32)?;
    emu.orr_no_count(16usize, 17usize, 5usize, 2171388u32);
    emu.adi_no_count(15usize, 15usize, 4u32, 2171392u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2171348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002121d4));
    } else {
        emu.pc = 2171396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212204));
    }
}
#[inline(always)]
pub fn block_0x00212204(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2171428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212224));
    } else {
        emu.pc = 2171400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212208));
    }
}
#[inline(always)]
pub fn block_0x00212208(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 40u32, 2171404u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2174128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2174128u32));
    } else {
        emu.pc = 2171408u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212210));
    }
}
#[inline(always)]
pub fn block_0x00212210(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 2usize, 1176u32, 2171412u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2171416u32);
    emu.adi_no_count(13usize, 0usize, 1u32, 2171420u32);
    emu.sw_no_count(13usize, 12usize, 0u32, 2171424u32)?;
    emu.adi_no_count(10usize, 10usize, 1u32, 2171428u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2171428u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212224));
}
#[inline(always)]
pub fn block_0x00212224(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 2usize, 680u32, 2171432u32)?;
    emu.sw_no_count(10usize, 2usize, 1336u32, 2171436u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2171444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212234));
    } else {
        emu.pc = 2171440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212230));
    }
}
#[inline(always)]
pub fn block_0x00212230(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 12usize, 0u32, 2171444u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2171444u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212234));
}
#[inline(always)]
pub fn block_0x00212234(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 41u32, 2171448u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2173796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2173796u32));
    } else {
        emu.pc = 2171452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021223c));
    }
}
#[inline(always)]
pub fn block_0x0021223c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 10usize, 2u32, 2171456u32);
    emu.adi_no_count(13usize, 2usize, 1176u32, 2171460u32);
    emu.adi_no_count(14usize, 2usize, 520u32, 2171464u32);
    emu.sbr_no_count(10usize, 0usize, 12usize, 2171468u32);
    emu.adi_no_count(15usize, 12usize, 4294967292u32, 2171472u32);
    emu.adr_no_count(12usize, 13usize, 15usize, 2171476u32);
    emu.adr_no_count(13usize, 14usize, 15usize, 2171480u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2171480u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212258));
}
#[inline(always)]
pub fn block_0x00212258(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2171820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002123ac));
    } else {
        emu.pc = 2171484u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021225c));
    }
}
#[inline(always)]
pub fn block_0x0021225c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(14usize, 13usize, 0u32, 2171488u32)?;
    emu.lw_no_count(15usize, 12usize, 0u32, 2171492u32)?;
    emu.adi_no_count(10usize, 10usize, 4u32, 2171496u32);
    emu.adi_no_count(12usize, 12usize, 4294967292u32, 2171500u32);
    emu.adi_no_count(13usize, 13usize, 4294967292u32, 2171504u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2171480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212258));
    } else {
        emu.pc = 2171508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212274));
    }
}
#[inline(always)]
pub fn block_0x00212274(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 14usize, 15usize, 2171512u32);
    emu.sltru_no_count(12usize, 15usize, 14usize, 2171516u32);
    emu.sbr_no_count(10usize, 12usize, 10usize, 2171520u32);
    emu.lw_no_count(12usize, 2usize, 24u32, 2171524u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2171836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002123bc));
    } else {
        emu.pc = 2171528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212288));
    }
}
#[inline(always)]
pub fn block_0x00212288(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 188u32, 2171532u32)?;
    emu.adi_no_count(12usize, 0usize, 41u32, 2171536u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2173796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2173796u32));
    } else {
        emu.pc = 2171540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212294));
    }
}
#[inline(always)]
pub fn block_0x00212294(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2171624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002122e8));
    } else {
        emu.pc = 2171544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212298));
    }
}
#[inline(always)]
pub fn block_0x00212298(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 0u32, 2171548u32);
    emu.sli_no_count(14usize, 10usize, 2u32, 2171552u32);
    emu.adi_no_count(16usize, 2usize, 28u32, 2171556u32);
    emu.adr_no_count(12usize, 16usize, 14usize, 2171560u32);
    emu.adi_no_count(15usize, 0usize, 10u32, 2171564u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2171564u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002122ac));
}
#[inline]
pub fn block_0x002122ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(17usize, 16usize, 0u32, 2171568u32)?;
    emu.mulhu_no_count(5usize, 17usize, 15usize, 2171572u32);
    emu.mul_no_count(17usize, 17usize, 15usize, 2171576u32);
    emu.adr_no_count(13usize, 17usize, 13usize, 2171580u32);
    emu.sw_no_count(13usize, 16usize, 0u32, 2171584u32)?;
    emu.adi_no_count(16usize, 16usize, 4u32, 2171588u32);
    emu.sltru_no_count(13usize, 13usize, 17usize, 2171592u32);
    emu.adi_no_count(14usize, 14usize, 4294967292u32, 2171596u32);
    emu.adr_no_count(13usize, 5usize, 13usize, 2171600u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a != b {
        emu.pc = 2171564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002122ac));
    } else {
        emu.pc = 2171604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002122d4));
    }
}
#[inline(always)]
pub fn block_0x002122d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2171624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002122e8));
    } else {
        emu.pc = 2171608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002122d8));
    }
}
#[inline(always)]
pub fn block_0x002122d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 40u32, 2171612u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2174128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2174128u32));
    } else {
        emu.pc = 2171616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002122e0));
    }
}
#[inline(always)]
pub fn block_0x002122e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(13usize, 12usize, 0u32, 2171620u32)?;
    emu.adi_no_count(10usize, 10usize, 1u32, 2171624u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2171624u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002122e8));
}
#[inline(always)]
pub fn block_0x002122e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 2usize, 352u32, 2171628u32)?;
    emu.adi_no_count(12usize, 0usize, 41u32, 2171632u32);
    emu.sw_no_count(10usize, 2usize, 188u32, 2171636u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2173892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2173892u32));
    } else {
        emu.pc = 2171640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002122f8));
    }
}
#[inline(always)]
pub fn block_0x002122f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2171724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021234c));
    } else {
        emu.pc = 2171644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002122fc));
    }
}
#[inline(always)]
pub fn block_0x002122fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 0u32, 2171648u32);
    emu.sli_no_count(14usize, 13usize, 2u32, 2171652u32);
    emu.adi_no_count(16usize, 2usize, 192u32, 2171656u32);
    emu.adr_no_count(10usize, 16usize, 14usize, 2171660u32);
    emu.adi_no_count(15usize, 0usize, 10u32, 2171664u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2171664u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212310));
}
#[inline]
pub fn block_0x00212310(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(17usize, 16usize, 0u32, 2171668u32)?;
    emu.mulhu_no_count(5usize, 17usize, 15usize, 2171672u32);
    emu.mul_no_count(17usize, 17usize, 15usize, 2171676u32);
    emu.adr_no_count(12usize, 17usize, 12usize, 2171680u32);
    emu.sw_no_count(12usize, 16usize, 0u32, 2171684u32)?;
    emu.adi_no_count(16usize, 16usize, 4u32, 2171688u32);
    emu.sltru_no_count(12usize, 12usize, 17usize, 2171692u32);
    emu.adi_no_count(14usize, 14usize, 4294967292u32, 2171696u32);
    emu.adr_no_count(12usize, 5usize, 12usize, 2171700u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a != b {
        emu.pc = 2171664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212310));
    } else {
        emu.pc = 2171704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212338));
    }
}
#[inline(always)]
pub fn block_0x00212338(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2171724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021234c));
    } else {
        emu.pc = 2171708u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021233c));
    }
}
#[inline(always)]
pub fn block_0x0021233c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 40u32, 2171712u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2174128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2174128u32));
    } else {
        emu.pc = 2171716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212344));
    }
}
#[inline(always)]
pub fn block_0x00212344(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(12usize, 10usize, 0u32, 2171720u32)?;
    emu.adi_no_count(13usize, 13usize, 1u32, 2171724u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2171724u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021234c));
}
#[inline(always)]
pub fn block_0x0021234c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(13usize, 2usize, 352u32, 2171728u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2171812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002123a4));
    } else {
        emu.pc = 2171732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212354));
    }
}
#[inline(always)]
pub fn block_0x00212354(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 0u32, 2171736u32);
    emu.sli_no_count(13usize, 11usize, 2u32, 2171740u32);
    emu.adi_no_count(15usize, 2usize, 356u32, 2171744u32);
    emu.adr_no_count(10usize, 15usize, 13usize, 2171748u32);
    emu.adi_no_count(14usize, 0usize, 10u32, 2171752u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2171752u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212368));
}
#[inline]
pub fn block_0x00212368(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 15usize, 0u32, 2171756u32)?;
    emu.mulhu_no_count(17usize, 16usize, 14usize, 2171760u32);
    emu.mul_no_count(16usize, 16usize, 14usize, 2171764u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2171768u32);
    emu.sw_no_count(12usize, 15usize, 0u32, 2171772u32)?;
    emu.adi_no_count(15usize, 15usize, 4u32, 2171776u32);
    emu.sltru_no_count(12usize, 12usize, 16usize, 2171780u32);
    emu.adi_no_count(13usize, 13usize, 4294967292u32, 2171784u32);
    emu.adr_no_count(12usize, 17usize, 12usize, 2171788u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2171752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212368));
    } else {
        emu.pc = 2171792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212390));
    }
}
#[inline(always)]
pub fn block_0x00212390(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2171812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002123a4));
    } else {
        emu.pc = 2171796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212394));
    }
}
#[inline(always)]
pub fn block_0x00212394(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 40u32, 2171800u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2174128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2174128u32));
    } else {
        emu.pc = 2171804u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021239c));
    }
}
#[inline(always)]
pub fn block_0x0021239c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(12usize, 10usize, 0u32, 2171808u32)?;
    emu.adi_no_count(11usize, 11usize, 1u32, 2171812u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2171812u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002123a4));
}
#[inline(always)]
pub fn block_0x002123a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 2usize, 516u32, 2171816u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2171820u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2171840u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002123c0));
}
#[inline(always)]
pub fn block_0x002123ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(10usize, 10usize, 1u32, 2171824u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2171828u32);
    emu.lw_no_count(12usize, 2usize, 24u32, 2171832u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2171528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212288));
    } else {
        emu.pc = 2171836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002123bc));
    }
}
#[inline(always)]
pub fn block_0x002123bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 25usize, 1u32, 2171840u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2171840u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002123c0));
}
#[inline(always)]
pub fn block_0x002123c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 684u32, 2171844u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2171848u32);
    emu.adi_no_count(12usize, 0usize, 164u32, 2171852u32);
    emu.apc_no_count(1usize, 2171852u32, 4294905856u32, 2171856u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2171860u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966048u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002123d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 684u32, 2171864u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2171868u32);
    emu.apc_no_count(1usize, 2171868u32, 4294942720u32, 2171872u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2171876u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(820u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002123e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 848u32, 2171880u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2171884u32);
    emu.adi_no_count(12usize, 0usize, 164u32, 2171888u32);
    emu.apc_no_count(1usize, 2171888u32, 4294905856u32, 2171892u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2171896u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966012u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002123f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 848u32, 2171900u32);
    emu.adi_no_count(11usize, 0usize, 2u32, 2171904u32);
    emu.apc_no_count(1usize, 2171904u32, 4294942720u32, 2171908u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2171912u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(784u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00212408(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1012u32, 2171916u32);
    emu.adi_no_count(11usize, 2usize, 520u32, 2171920u32);
    emu.adi_no_count(12usize, 0usize, 164u32, 2171924u32);
    emu.apc_no_count(1usize, 2171924u32, 4294905856u32, 2171928u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2171932u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965976u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021241c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1012u32, 2171936u32);
    emu.adi_no_count(11usize, 0usize, 3u32, 2171940u32);
    emu.apc_no_count(1usize, 2171940u32, 4294942720u32, 2171944u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2171948u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(748u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021242c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 1172u32, 2171952u32)?;
    emu.lw_no_count(21usize, 2usize, 188u32, 2171956u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2171960u32);
    emu.sw_no_count(11usize, 2usize, 12u32, 2171964u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a < b {
        emu.pc = 2171972u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212444));
    } else {
        emu.pc = 2171968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212440));
    }
}
#[inline(always)]
pub fn block_0x00212440(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 21usize, 0u32, 2171972u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2171972u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212444));
}
#[inline(always)]
pub fn block_0x00212444(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 40u32, 2171976u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2173796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2173796u32));
    } else {
        emu.pc = 2171980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021244c));
    }
}
#[inline]
pub fn block_0x0021244c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(25usize, 2usize, 8u32, 2171984u32)?;
    emu.sw_no_count(23usize, 2usize, 4u32, 2171988u32)?;
    emu.adi_no_count(23usize, 0usize, 0u32, 2171992u32);
    emu.lw_no_count(26usize, 2usize, 1008u32, 2171996u32)?;
    emu.lw_no_count(27usize, 2usize, 844u32, 2172000u32)?;
    emu.lw_no_count(18usize, 2usize, 680u32, 2172004u32)?;
    emu.adi_no_count(22usize, 2usize, 24u32, 2172008u32);
    emu.adi_no_count(7usize, 2usize, 516u32, 2172012u32);
    emu.adi_no_count(8usize, 0usize, 41u32, 2172016u32);
    emu.adi_no_count(9usize, 0usize, 10u32, 2172020u32);
    emu.sw_no_count(24usize, 2usize, 16u32, 2172024u32)?;
    emu.add_memory_rw_events(12usize);
    let return_addr = 2172028u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2172032u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212480));
}
#[inline(always)]
pub fn block_0x0021247c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a < b {
        emu.pc = 2173796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2173796u32));
    } else {
        emu.pc = 2172032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212480));
    }
}
#[inline(always)]
pub fn block_0x00212480(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 23usize, 0u32, 2172036u32);
    emu.sli_no_count(12usize, 10usize, 2u32, 2172040u32);
    emu.sbr_no_count(11usize, 0usize, 12usize, 2172044u32);
    emu.adi_no_count(13usize, 2usize, 1008u32, 2172048u32);
    emu.adr_no_count(13usize, 13usize, 12usize, 2172052u32);
    emu.adr_no_count(14usize, 22usize, 12usize, 2172056u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2172056u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212498));
}
#[inline(always)]
pub fn block_0x00212498(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2172184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212518));
    } else {
        emu.pc = 2172060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021249c));
    }
}
#[inline(always)]
pub fn block_0x0021249c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 14usize, 0u32, 2172064u32)?;
    emu.lw_no_count(16usize, 13usize, 0u32, 2172068u32)?;
    emu.adi_no_count(11usize, 11usize, 4u32, 2172072u32);
    emu.adi_no_count(13usize, 13usize, 4294967292u32, 2172076u32);
    emu.adi_no_count(14usize, 14usize, 4294967292u32, 2172080u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2172056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212498));
    } else {
        emu.pc = 2172084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002124b4));
    }
}
#[inline(always)]
pub fn block_0x002124b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a < b {
        emu.pc = 2172188u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021251c));
    } else {
        emu.pc = 2172088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002124b8));
    }
}
#[inline(always)]
pub fn block_0x002124b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2172164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212504));
    } else {
        emu.pc = 2172092u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002124bc));
    }
}
#[inline(always)]
pub fn block_0x002124bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 2usize, 1012u32, 2172096u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2172100u32);
    emu.adr_no_count(12usize, 11usize, 12usize, 2172104u32);
    emu.adi_no_count(13usize, 2usize, 28u32, 2172108u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2172108u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002124cc));
}
#[inline]
pub fn block_0x002124cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 11usize, 0u32, 2172112u32)?;
    emu.lw_no_count(16usize, 13usize, 0u32, 2172116u32)?;
    emu.ani_no_count(14usize, 14usize, 1u32, 2172120u32);
    emu.adi_no_count(11usize, 11usize, 4u32, 2172124u32);
    emu.xri_no_count(15usize, 15usize, 4294967295u32, 2172128u32);
    emu.adr_no_count(15usize, 16usize, 15usize, 2172132u32);
    emu.sltru_no_count(16usize, 15usize, 16usize, 2172136u32);
    emu.adr_no_count(14usize, 15usize, 14usize, 2172140u32);
    emu.sltru_no_count(15usize, 14usize, 15usize, 2172144u32);
    emu.sw_no_count(14usize, 13usize, 0u32, 2172148u32)?;
    emu.orr_no_count(14usize, 16usize, 15usize, 2172152u32);
    emu.adi_no_count(13usize, 13usize, 4u32, 2172156u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2172108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002124cc));
    } else {
        emu.pc = 2172160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212500));
    }
}
#[inline(always)]
pub fn block_0x00212500(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2173864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2173864u32));
    } else {
        emu.pc = 2172164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212504));
    }
}
#[inline(always)]
pub fn block_0x00212504(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 188u32, 2172168u32)?;
    emu.adi_no_count(11usize, 0usize, 8u32, 2172172u32);
    emu.adi_no_count(13usize, 26usize, 0u32, 2172176u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(26usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2172204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021252c));
    } else {
        emu.pc = 2172180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212514));
    }
}
#[inline(always)]
pub fn block_0x00212514(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2172184u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2172208u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212530));
}
#[inline(always)]
pub fn block_0x00212518(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2172088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002124b8));
    } else {
        emu.pc = 2172188u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021251c));
    }
}
#[inline(always)]
pub fn block_0x0021251c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2172192u32);
    emu.adi_no_count(10usize, 21usize, 0u32, 2172196u32);
    emu.adi_no_count(13usize, 26usize, 0u32, 2172200u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(26usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a < b {
        emu.pc = 2172208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212530));
    } else {
        emu.pc = 2172204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021252c));
    }
}
#[inline(always)]
pub fn block_0x0021252c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 10usize, 0u32, 2172208u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2172208u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212530));
}
#[inline(always)]
pub fn block_0x00212530(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2173892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2173892u32));
    } else {
        emu.pc = 2172212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212534));
    }
}
#[inline(always)]
pub fn block_0x00212534(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 13usize, 2u32, 2172216u32);
    emu.sbr_no_count(14usize, 0usize, 12usize, 2172220u32);
    emu.adi_no_count(15usize, 2usize, 844u32, 2172224u32);
    emu.adr_no_count(15usize, 15usize, 12usize, 2172228u32);
    emu.adr_no_count(16usize, 22usize, 12usize, 2172232u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2172232u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212548));
}
#[inline(always)]
pub fn block_0x00212548(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2172360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002125c8));
    } else {
        emu.pc = 2172236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021254c));
    }
}
#[inline(always)]
pub fn block_0x0021254c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(17usize, 16usize, 0u32, 2172240u32)?;
    emu.lw_no_count(5usize, 15usize, 0u32, 2172244u32)?;
    emu.adi_no_count(14usize, 14usize, 4u32, 2172248u32);
    emu.adi_no_count(15usize, 15usize, 4294967292u32, 2172252u32);
    emu.adi_no_count(16usize, 16usize, 4294967292u32, 2172256u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2172232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212548));
    } else {
        emu.pc = 2172260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212564));
    }
}
#[inline(always)]
pub fn block_0x00212564(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a < b {
        emu.pc = 2172364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002125cc));
    } else {
        emu.pc = 2172264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212568));
    }
}
#[inline(always)]
pub fn block_0x00212568(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2172340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002125b4));
    } else {
        emu.pc = 2172268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021256c));
    }
}
#[inline(always)]
pub fn block_0x0021256c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 848u32, 2172272u32);
    emu.adi_no_count(15usize, 0usize, 1u32, 2172276u32);
    emu.adr_no_count(12usize, 10usize, 12usize, 2172280u32);
    emu.adi_no_count(14usize, 2usize, 28u32, 2172284u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2172284u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021257c));
}
#[inline]
pub fn block_0x0021257c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 10usize, 0u32, 2172288u32)?;
    emu.lw_no_count(17usize, 14usize, 0u32, 2172292u32)?;
    emu.ani_no_count(15usize, 15usize, 1u32, 2172296u32);
    emu.adi_no_count(10usize, 10usize, 4u32, 2172300u32);
    emu.xri_no_count(16usize, 16usize, 4294967295u32, 2172304u32);
    emu.adr_no_count(16usize, 17usize, 16usize, 2172308u32);
    emu.sltru_no_count(17usize, 16usize, 17usize, 2172312u32);
    emu.adr_no_count(15usize, 16usize, 15usize, 2172316u32);
    emu.sltru_no_count(16usize, 15usize, 16usize, 2172320u32);
    emu.sw_no_count(15usize, 14usize, 0u32, 2172324u32)?;
    emu.orr_no_count(15usize, 17usize, 16usize, 2172328u32);
    emu.adi_no_count(14usize, 14usize, 4u32, 2172332u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2172284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021257c));
    } else {
        emu.pc = 2172336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002125b0));
    }
}
#[inline(always)]
pub fn block_0x002125b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2173864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2173864u32));
    } else {
        emu.pc = 2172340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002125b4));
    }
}
#[inline(always)]
pub fn block_0x002125b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(13usize, 2usize, 188u32, 2172344u32)?;
    emu.ori_no_count(11usize, 11usize, 4u32, 2172348u32);
    emu.adi_no_count(10usize, 27usize, 0u32, 2172352u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(27usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2172380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002125dc));
    } else {
        emu.pc = 2172356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002125c4));
    }
}
#[inline(always)]
pub fn block_0x002125c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2172360u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2172376u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002125d8));
}
#[inline(always)]
pub fn block_0x002125c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2172264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00212568));
    } else {
        emu.pc = 2172364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002125cc));
    }
}
#[inline(always)]
pub fn block_0x002125cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 10usize, 0u32, 2172368u32);
    emu.adi_no_count(10usize, 27usize, 0u32, 2172372u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(27usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2172380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002125dc));
    } else {
        emu.pc = 2172376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002125d8));
    }
}
#[inline(always)]
pub fn block_0x002125d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 13usize, 0u32, 2172380u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2172380u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002125dc));
}
#[inline(always)]
pub fn block_0x002125dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2173796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2173796u32));
    } else {
        emu.pc = 2172384u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2172384u32));
    }
}
