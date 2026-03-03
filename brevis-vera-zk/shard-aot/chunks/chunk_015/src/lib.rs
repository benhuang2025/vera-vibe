pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2154828u32;
pub const PC_MAX: u32 = 2157352u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 105usize] = [
        block_0x0020e14c,
        block_0x0020e164,
        block_0x0020e258,
        block_0x0020e264,
        block_0x0020e270,
        block_0x0020e274,
        block_0x0020e278,
        block_0x0020e290,
        block_0x0020e2a4,
        block_0x0020e2b4,
        block_0x0020e2bc,
        block_0x0020e2c4,
        block_0x0020e310,
        block_0x0020e324,
        block_0x0020e334,
        block_0x0020e348,
        block_0x0020e34c,
        block_0x0020e350,
        block_0x0020e354,
        block_0x0020e35c,
        block_0x0020e360,
        block_0x0020e364,
        block_0x0020e36c,
        block_0x0020e37c,
        block_0x0020e3a4,
        block_0x0020e414,
        block_0x0020e424,
        block_0x0020e494,
        block_0x0020e498,
        block_0x0020e4bc,
        block_0x0020e4cc,
        block_0x0020e4e0,
        block_0x0020e4fc,
        block_0x0020e500,
        block_0x0020e520,
        block_0x0020e534,
        block_0x0020e544,
        block_0x0020e558,
        block_0x0020e574,
        block_0x0020e578,
        block_0x0020e594,
        block_0x0020e5d4,
        block_0x0020e710,
        block_0x0020e718,
        block_0x0020e728,
        block_0x0020e77c,
        block_0x0020e784,
        block_0x0020e790,
        block_0x0020e798,
        block_0x0020e7a4,
        block_0x0020e7b4,
        block_0x0020e7bc,
        block_0x0020e7c4,
        block_0x0020e7d0,
        block_0x0020e7d4,
        block_0x0020e7dc,
        block_0x0020e7e8,
        block_0x0020e7f8,
        block_0x0020e800,
        block_0x0020e808,
        block_0x0020e80c,
        block_0x0020e814,
        block_0x0020e824,
        block_0x0020e828,
        block_0x0020e82c,
        block_0x0020e89c,
        block_0x0020e8a4,
        block_0x0020e8c4,
        block_0x0020e8dc,
        block_0x0020e8e4,
        block_0x0020e8e8,
        block_0x0020e908,
        block_0x0020e910,
        block_0x0020e918,
        block_0x0020e92c,
        block_0x0020e93c,
        block_0x0020e944,
        block_0x0020e94c,
        block_0x0020e964,
        block_0x0020e980,
        block_0x0020e984,
        block_0x0020e9a0,
        block_0x0020e9a8,
        block_0x0020e9b8,
        block_0x0020e9e0,
        block_0x0020e9fc,
        block_0x0020ea48,
        block_0x0020ea50,
        block_0x0020ea58,
        block_0x0020ea8c,
        block_0x0020ea94,
        block_0x0020ea98,
        block_0x0020eab4,
        block_0x0020eac0,
        block_0x0020eac4,
        block_0x0020ead0,
        block_0x0020ead4,
        block_0x0020eadc,
        block_0x0020eae8,
        block_0x0020eaf4,
        block_0x0020eaf8,
        block_0x0020eafc,
        block_0x0020eb18,
        block_0x0020eb24,
        block_0x0020eb28,
    ];
    const IDX: [u16; 632usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 3u16, 0u16, 0u16, 4u16, 0u16, 0u16, 5u16, 6u16, 7u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 8u16, 0u16, 0u16, 0u16, 0u16, 9u16, 0u16, 0u16, 0u16, 10u16,
        0u16, 11u16, 0u16, 12u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 13u16, 0u16, 0u16, 0u16,
        0u16, 14u16, 0u16, 0u16, 0u16, 15u16, 0u16, 0u16, 0u16, 0u16, 16u16, 17u16,
        18u16, 19u16, 0u16, 20u16, 21u16, 22u16, 0u16, 23u16, 0u16, 0u16, 0u16, 24u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 25u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 26u16, 0u16,
        0u16, 0u16, 27u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 28u16, 29u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 30u16, 0u16, 0u16, 0u16, 31u16, 0u16, 0u16, 0u16, 0u16, 32u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 33u16, 34u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        35u16, 0u16, 0u16, 0u16, 0u16, 36u16, 0u16, 0u16, 0u16, 37u16, 0u16, 0u16, 0u16,
        0u16, 38u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 39u16, 40u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 41u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 42u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 43u16, 0u16, 44u16, 0u16, 0u16, 0u16,
        45u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 46u16, 0u16, 47u16, 0u16, 0u16,
        48u16, 0u16, 49u16, 0u16, 0u16, 50u16, 0u16, 0u16, 0u16, 51u16, 0u16, 52u16,
        0u16, 53u16, 0u16, 0u16, 54u16, 55u16, 0u16, 56u16, 0u16, 0u16, 57u16, 0u16,
        0u16, 0u16, 58u16, 0u16, 59u16, 0u16, 60u16, 61u16, 0u16, 62u16, 0u16, 0u16,
        0u16, 63u16, 64u16, 65u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 66u16, 0u16, 67u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 68u16, 0u16, 0u16, 0u16, 0u16, 0u16, 69u16, 0u16, 70u16, 71u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 72u16, 0u16, 73u16, 0u16, 74u16, 0u16, 0u16,
        0u16, 0u16, 75u16, 0u16, 0u16, 0u16, 76u16, 0u16, 77u16, 0u16, 78u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 79u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 80u16, 81u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 82u16, 0u16, 83u16, 0u16, 0u16, 0u16, 84u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 85u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 86u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 87u16, 0u16, 88u16, 0u16, 89u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 90u16,
        0u16, 91u16, 92u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 93u16, 0u16, 0u16, 94u16,
        95u16, 0u16, 0u16, 96u16, 97u16, 0u16, 98u16, 0u16, 0u16, 99u16, 0u16, 0u16,
        100u16, 101u16, 102u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 103u16, 0u16, 0u16,
        104u16, 105u16,
    ];
    if pc < 2154828u32 || pc > 2157352u32 {
        return None;
    }
    let word_offset = ((pc - 2154828u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x0020e14c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2154832u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2154836u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1915u32, 2154840u32);
    emu.adi_no_count(11usize, 0usize, 43u32, 2154844u32);
    emu.apc_no_count(1usize, 2154844u32, 4294959104u32, 2154848u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2154852u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(548u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0020e164(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 61u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(73728u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2154856u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2154860u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1960u32, 2154864u32);
    emu.adi_no_count(11usize, 11usize, 4294965295u32, 2154868u32);
    emu.sltru_no_count(11usize, 10usize, 11usize, 2154872u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2154876u32);
    emu.ani_no_count(11usize, 11usize, 17u32, 2154880u32);
    emu.sli_no_count(12usize, 11usize, 2u32, 2154884u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2154888u32);
    emu.lw_no_count(12usize, 12usize, 32u32, 2154892u32)?;
    emu.sli_no_count(14usize, 10usize, 11u32, 2154896u32);
    emu.sli_no_count(12usize, 12usize, 11u32, 2154900u32);
    emu.sltru_no_count(12usize, 14usize, 12usize, 2154904u32);
    emu.xri_no_count(12usize, 12usize, 1u32, 2154908u32);
    emu.sli_no_count(12usize, 12usize, 3u32, 2154912u32);
    emu.orr_no_count(11usize, 11usize, 12usize, 2154916u32);
    emu.sli_no_count(12usize, 11usize, 2u32, 2154920u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2154924u32);
    emu.lw_no_count(12usize, 12usize, 16u32, 2154928u32)?;
    emu.sli_no_count(12usize, 12usize, 11u32, 2154932u32);
    emu.sltru_no_count(12usize, 14usize, 12usize, 2154936u32);
    emu.xri_no_count(12usize, 12usize, 1u32, 2154940u32);
    emu.sli_no_count(12usize, 12usize, 2u32, 2154944u32);
    emu.orr_no_count(11usize, 11usize, 12usize, 2154948u32);
    emu.sli_no_count(12usize, 11usize, 2u32, 2154952u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2154956u32);
    emu.lw_no_count(12usize, 12usize, 8u32, 2154960u32)?;
    emu.sli_no_count(12usize, 12usize, 11u32, 2154964u32);
    emu.sltru_no_count(12usize, 14usize, 12usize, 2154968u32);
    emu.xri_no_count(12usize, 12usize, 1u32, 2154972u32);
    emu.sli_no_count(12usize, 12usize, 1u32, 2154976u32);
    emu.orr_no_count(11usize, 11usize, 12usize, 2154980u32);
    emu.sli_no_count(12usize, 11usize, 2u32, 2154984u32);
    emu.adr_no_count(12usize, 12usize, 13usize, 2154988u32);
    emu.lw_no_count(12usize, 12usize, 4u32, 2154992u32)?;
    emu.sli_no_count(12usize, 12usize, 11u32, 2154996u32);
    emu.sltru_no_count(12usize, 14usize, 12usize, 2155000u32);
    emu.xri_no_count(12usize, 12usize, 1u32, 2155004u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2155008u32);
    emu.sli_no_count(12usize, 11usize, 2u32, 2155012u32);
    emu.adr_no_count(12usize, 12usize, 13usize, 2155016u32);
    emu.lw_no_count(12usize, 12usize, 4u32, 2155020u32)?;
    emu.sli_no_count(12usize, 12usize, 11u32, 2155024u32);
    emu.sltru_no_count(12usize, 14usize, 12usize, 2155028u32);
    emu.xri_no_count(12usize, 12usize, 1u32, 2155032u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2155036u32);
    emu.sli_no_count(12usize, 11usize, 2u32, 2155040u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2155044u32);
    emu.lw_no_count(12usize, 12usize, 0u32, 2155048u32)?;
    emu.sli_no_count(12usize, 12usize, 11u32, 2155052u32);
    emu.xrr_no_count(15usize, 12usize, 14usize, 2155056u32);
    emu.sltru_no_count(12usize, 12usize, 14usize, 2155060u32);
    emu.sltiu_no_count(14usize, 15usize, 1u32, 2155064u32);
    emu.adr_no_count(11usize, 12usize, 11usize, 2155068u32);
    emu.adr_no_count(14usize, 14usize, 11usize, 2155072u32);
    emu.sli_no_count(11usize, 14usize, 2u32, 2155076u32);
    emu.adr_no_count(13usize, 13usize, 11usize, 2155080u32);
    emu.lw_no_count(11usize, 13usize, 0u32, 2155084u32)?;
    emu.adi_no_count(12usize, 0usize, 32u32, 2155088u32);
    emu.sri_no_count(11usize, 11usize, 21u32, 2155092u32);
    emu.add_memory_rw_events(60usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2155124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e274));
    } else {
        emu.pc = 2155096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e258));
    }
}
#[inline(always)]
pub fn block_0x0020e258(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 13usize, 4u32, 2155100u32)?;
    emu.sri_no_count(12usize, 12usize, 21u32, 2155104u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a != b {
        emu.pc = 2155128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e278));
    } else {
        emu.pc = 2155108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e264));
    }
}
#[inline(always)]
pub fn block_0x0020e264(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(13usize, 11usize, 4294967295u32, 2155112u32);
    emu.adr_no_count(13usize, 12usize, 13usize, 2155116u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2155152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e290));
    } else {
        emu.pc = 2155120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e270));
    }
}
#[inline(always)]
pub fn block_0x0020e270(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2155124u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2155196u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e2bc));
}
#[inline(always)]
pub fn block_0x0020e274(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 751u32, 2155128u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2155128u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e278));
}
#[inline(always)]
pub fn block_0x0020e278(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 13usize, 4294967292u32, 2155132u32)?;
    emu.sli_no_count(13usize, 13usize, 11u32, 2155136u32);
    emu.sri_no_count(14usize, 13usize, 11u32, 2155140u32);
    emu.xri_no_count(13usize, 11usize, 4294967295u32, 2155144u32);
    emu.adr_no_count(13usize, 12usize, 13usize, 2155148u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2155196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e2bc));
    } else {
        emu.pc = 2155152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e290));
    }
}
#[inline(always)]
pub fn block_0x0020e290(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 0u32, 2155156u32);
    emu.sbr_no_count(10usize, 10usize, 14usize, 2155160u32);
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2155164u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2155168u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1164u32, 2155172u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2155172u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e2a4));
}
#[inline(always)]
pub fn block_0x0020e2a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(15usize, 14usize, 11usize, 2155176u32);
    emu.lbu_no_count(15usize, 15usize, 0u32, 2155180u32);
    emu.adr_no_count(13usize, 13usize, 15usize, 2155184u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2155196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e2bc));
    } else {
        emu.pc = 2155188u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e2b4));
    }
}
#[inline(always)]
pub fn block_0x0020e2b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 11usize, 1u32, 2155192u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2155172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e2a4));
    } else {
        emu.pc = 2155196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e2bc));
    }
}
#[inline(always)]
pub fn block_0x0020e2bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 11usize, 1u32, 2155200u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2155204u32;
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
pub fn block_0x0020e2c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967168u32, 2155208u32);
    emu.sw_no_count(1usize, 2usize, 124u32, 2155212u32)?;
    emu.sw_no_count(8usize, 2usize, 120u32, 2155216u32)?;
    emu.sw_no_count(9usize, 2usize, 116u32, 2155220u32)?;
    emu.sw_no_count(18usize, 2usize, 112u32, 2155224u32)?;
    emu.sw_no_count(19usize, 2usize, 108u32, 2155228u32)?;
    emu.sw_no_count(20usize, 2usize, 104u32, 2155232u32)?;
    emu.sw_no_count(21usize, 2usize, 100u32, 2155236u32)?;
    emu.sw_no_count(22usize, 2usize, 96u32, 2155240u32)?;
    emu.sw_no_count(23usize, 2usize, 92u32, 2155244u32)?;
    emu.sw_no_count(24usize, 2usize, 88u32, 2155248u32)?;
    emu.sw_no_count(25usize, 2usize, 84u32, 2155252u32)?;
    emu.sw_no_count(26usize, 2usize, 80u32, 2155256u32)?;
    emu.sw_no_count(27usize, 2usize, 76u32, 2155260u32)?;
    emu.adi_no_count(26usize, 12usize, 0u32, 2155264u32);
    emu.lw_no_count(14usize, 11usize, 0u32, 2155268u32)?;
    emu.lw_no_count(16usize, 11usize, 4u32, 2155272u32)?;
    emu.orr_no_count(12usize, 14usize, 16usize, 2155276u32);
    emu.add_memory_rw_events(18usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2158388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2158388u32));
    } else {
        emu.pc = 2155280u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e310));
    }
}
#[inline(always)]
pub fn block_0x0020e310(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2155284u32);
    emu.lw_no_count(17usize, 11usize, 8u32, 2155288u32)?;
    emu.lw_no_count(6usize, 11usize, 12u32, 2155292u32)?;
    emu.orr_no_count(10usize, 17usize, 6usize, 2155296u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2158416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2158416u32));
    } else {
        emu.pc = 2155300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e324));
    }
}
#[inline(always)]
pub fn block_0x0020e324(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 11usize, 16u32, 2155304u32)?;
    emu.lw_no_count(5usize, 11usize, 20u32, 2155308u32)?;
    emu.orr_no_count(12usize, 10usize, 5usize, 2155312u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2158444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2158444u32));
    } else {
        emu.pc = 2155316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e334));
    }
}
#[inline(always)]
pub fn block_0x0020e334(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(15usize, 14usize, 10usize, 2155320u32);
    emu.sltru_no_count(7usize, 15usize, 14usize, 2155324u32);
    emu.adr_no_count(5usize, 16usize, 5usize, 2155328u32);
    emu.adr_no_count(12usize, 5usize, 7usize, 2155332u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2155340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e34c));
    } else {
        emu.pc = 2155336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e348));
    }
}
#[inline(always)]
pub fn block_0x0020e348(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(7usize, 12usize, 16usize, 2155340u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2155340u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e34c));
}
#[inline(always)]
pub fn block_0x0020e34c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a != b {
        emu.pc = 2158472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2158472u32));
    } else {
        emu.pc = 2155344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e350));
    }
}
#[inline(always)]
pub fn block_0x0020e350(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2155356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e35c));
    } else {
        emu.pc = 2155348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e354));
    }
}
#[inline(always)]
pub fn block_0x0020e354(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(7usize, 16usize, 6usize, 2155352u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2155356u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2155360u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e360));
}
#[inline(always)]
pub fn block_0x0020e35c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(7usize, 14usize, 17usize, 2155360u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2155360u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e360));
}
#[inline(always)]
pub fn block_0x0020e360(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a != b {
        emu.pc = 2158500u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2158500u32));
    } else {
        emu.pc = 2155364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e364));
    }
}
#[inline(always)]
pub fn block_0x0020e364(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 16u32, 2155368u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a >= b {
        emu.pc = 2158528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2158528u32));
    } else {
        emu.pc = 2155372u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e36c));
    }
}
#[inline(always)]
pub fn block_0x0020e36c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 15usize, 10usize, 2155376u32);
    emu.adr_no_count(10usize, 5usize, 10usize, 2155380u32);
    emu.sri_no_count(12usize, 10usize, 29u32, 2155384u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2158556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2158556u32));
    } else {
        emu.pc = 2155388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e37c));
    }
}
#[inline]
pub fn block_0x0020e37c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(5usize, 15usize, 1u32, 2155392u32);
    let a = 0u32.wrapping_add(1431654400u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2155396u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(858992640u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2155400u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(252645376u32);
    emu.write_reg_no_count(28usize, a);
    emu.pc = 2155404u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(16842752u32);
    emu.write_reg_no_count(31usize, a);
    emu.pc = 2155408u32;
    emu.update_insn_clock();
    emu.adi_no_count(30usize, 12usize, 1365u32, 2155412u32);
    emu.adi_no_count(29usize, 7usize, 819u32, 2155416u32);
    emu.adi_no_count(28usize, 28usize, 4294967055u32, 2155420u32);
    emu.adi_no_count(7usize, 31usize, 257u32, 2155424u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2155556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e424));
    } else {
        emu.pc = 2155428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e3a4));
    }
}
#[inline(never)]
pub fn block_0x0020e3a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.orr_no_count(12usize, 15usize, 5usize, 2155432u32);
    emu.sri_no_count(31usize, 12usize, 2u32, 2155436u32);
    emu.orr_no_count(12usize, 12usize, 31usize, 2155440u32);
    emu.sri_no_count(31usize, 12usize, 4u32, 2155444u32);
    emu.orr_no_count(12usize, 12usize, 31usize, 2155448u32);
    emu.sri_no_count(31usize, 12usize, 8u32, 2155452u32);
    emu.orr_no_count(12usize, 12usize, 31usize, 2155456u32);
    emu.sri_no_count(31usize, 12usize, 16u32, 2155460u32);
    emu.orr_no_count(12usize, 12usize, 31usize, 2155464u32);
    emu.xri_no_count(12usize, 12usize, 4294967295u32, 2155468u32);
    emu.sri_no_count(31usize, 12usize, 1u32, 2155472u32);
    emu.anr_no_count(30usize, 31usize, 30usize, 2155476u32);
    emu.sbr_no_count(12usize, 12usize, 30usize, 2155480u32);
    emu.anr_no_count(30usize, 12usize, 29usize, 2155484u32);
    emu.sri_no_count(12usize, 12usize, 2u32, 2155488u32);
    emu.anr_no_count(12usize, 12usize, 29usize, 2155492u32);
    emu.adr_no_count(12usize, 30usize, 12usize, 2155496u32);
    emu.sri_no_count(29usize, 12usize, 4u32, 2155500u32);
    emu.adr_no_count(12usize, 12usize, 29usize, 2155504u32);
    emu.anr_no_count(12usize, 12usize, 28usize, 2155508u32);
    emu.mul_no_count(12usize, 12usize, 7usize, 2155512u32);
    emu.sri_no_count(12usize, 12usize, 24u32, 2155516u32);
    emu.adi_no_count(7usize, 12usize, 32u32, 2155520u32);
    emu.lhu_no_count(30usize, 11usize, 24u32, 2155524u32)?;
    emu.ani_no_count(31usize, 7usize, 63u32, 2155528u32);
    emu.adi_no_count(9usize, 31usize, 4294967264u32, 2155532u32);
    emu.xri_no_count(8usize, 31usize, 4294967295u32, 2155536u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2155668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e494));
    } else {
        emu.pc = 2155540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e414));
    }
}
#[inline(always)]
pub fn block_0x0020e414(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.slr_no_count(10usize, 10usize, 7usize, 2155544u32);
    emu.srr_no_count(11usize, 5usize, 8usize, 2155548u32);
    emu.orr_no_count(11usize, 10usize, 11usize, 2155552u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2155556u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2155672u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e498));
}
#[inline(never)]
pub fn block_0x0020e424(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 10usize, 1u32, 2155560u32);
    emu.orr_no_count(12usize, 10usize, 12usize, 2155564u32);
    emu.sri_no_count(31usize, 12usize, 2u32, 2155568u32);
    emu.orr_no_count(12usize, 12usize, 31usize, 2155572u32);
    emu.sri_no_count(31usize, 12usize, 4u32, 2155576u32);
    emu.orr_no_count(12usize, 12usize, 31usize, 2155580u32);
    emu.sri_no_count(31usize, 12usize, 8u32, 2155584u32);
    emu.orr_no_count(12usize, 12usize, 31usize, 2155588u32);
    emu.sri_no_count(31usize, 12usize, 16u32, 2155592u32);
    emu.orr_no_count(12usize, 12usize, 31usize, 2155596u32);
    emu.xri_no_count(12usize, 12usize, 4294967295u32, 2155600u32);
    emu.sri_no_count(31usize, 12usize, 1u32, 2155604u32);
    emu.anr_no_count(30usize, 31usize, 30usize, 2155608u32);
    emu.sbr_no_count(12usize, 12usize, 30usize, 2155612u32);
    emu.anr_no_count(30usize, 12usize, 29usize, 2155616u32);
    emu.sri_no_count(12usize, 12usize, 2u32, 2155620u32);
    emu.anr_no_count(12usize, 12usize, 29usize, 2155624u32);
    emu.adr_no_count(12usize, 30usize, 12usize, 2155628u32);
    emu.sri_no_count(29usize, 12usize, 4u32, 2155632u32);
    emu.adr_no_count(12usize, 12usize, 29usize, 2155636u32);
    emu.anr_no_count(12usize, 12usize, 28usize, 2155640u32);
    emu.mul_no_count(12usize, 12usize, 7usize, 2155644u32);
    emu.sri_no_count(7usize, 12usize, 24u32, 2155648u32);
    emu.lhu_no_count(30usize, 11usize, 24u32, 2155652u32)?;
    emu.ani_no_count(31usize, 7usize, 63u32, 2155656u32);
    emu.adi_no_count(9usize, 31usize, 4294967264u32, 2155660u32);
    emu.xri_no_count(8usize, 31usize, 4294967295u32, 2155664u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2155540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e414));
    } else {
        emu.pc = 2155668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e494));
    }
}
#[inline(always)]
pub fn block_0x0020e494(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.slr_no_count(11usize, 15usize, 31usize, 2155672u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2155672u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e498));
}
#[inline]
pub fn block_0x0020e498(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sai_no_count(5usize, 9usize, 1055u32, 2155676u32);
    emu.sltru_no_count(10usize, 14usize, 17usize, 2155680u32);
    emu.sbr_no_count(12usize, 16usize, 6usize, 2155684u32);
    emu.sbr_no_count(28usize, 14usize, 17usize, 2155688u32);
    emu.sbr_no_count(10usize, 12usize, 10usize, 2155692u32);
    emu.sw_no_count(28usize, 2usize, 24u32, 2155696u32)?;
    emu.sw_no_count(10usize, 2usize, 28u32, 2155700u32)?;
    emu.sh_no_count(30usize, 2usize, 32u32, 2155704u32)?;
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2155744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e4e0));
    } else {
        emu.pc = 2155708u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e4bc));
    }
}
#[inline(always)]
pub fn block_0x0020e4bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.slr_no_count(17usize, 28usize, 31usize, 2155712u32);
    emu.slr_no_count(12usize, 28usize, 7usize, 2155716u32);
    emu.anr_no_count(6usize, 5usize, 12usize, 2155720u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2155772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e4fc));
    } else {
        emu.pc = 2155724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e4cc));
    }
}
#[inline(always)]
pub fn block_0x0020e4cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.srr_no_count(12usize, 6usize, 7usize, 2155728u32);
    emu.sli_no_count(29usize, 17usize, 1u32, 2155732u32);
    emu.slr_no_count(29usize, 29usize, 8usize, 2155736u32);
    emu.orr_no_count(29usize, 12usize, 29usize, 2155740u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2155744u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2155776u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e500));
}
#[inline(always)]
pub fn block_0x0020e4e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.slr_no_count(12usize, 10usize, 7usize, 2155748u32);
    emu.sri_no_count(17usize, 28usize, 1u32, 2155752u32);
    emu.srr_no_count(17usize, 17usize, 8usize, 2155756u32);
    emu.orr_no_count(17usize, 12usize, 17usize, 2155760u32);
    emu.slr_no_count(12usize, 28usize, 7usize, 2155764u32);
    emu.anr_no_count(6usize, 5usize, 12usize, 2155768u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2155724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e4cc));
    } else {
        emu.pc = 2155772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e4fc));
    }
}
#[inline(always)]
pub fn block_0x0020e4fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.srr_no_count(29usize, 17usize, 31usize, 2155776u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2155776u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e500));
}
#[inline(always)]
pub fn block_0x0020e500(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.srr_no_count(12usize, 17usize, 7usize, 2155780u32);
    emu.xrr_no_count(28usize, 29usize, 28usize, 2155784u32);
    emu.anr_no_count(12usize, 5usize, 12usize, 2155788u32);
    emu.xrr_no_count(10usize, 12usize, 10usize, 2155792u32);
    emu.orr_no_count(10usize, 28usize, 10usize, 2155796u32);
    emu.sw_no_count(29usize, 2usize, 40u32, 2155800u32)?;
    emu.sw_no_count(12usize, 2usize, 44u32, 2155804u32)?;
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2158352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2158352u32));
    } else {
        emu.pc = 2155808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e520));
    }
}
#[inline(always)]
pub fn block_0x0020e520(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(14usize, 2usize, 24u32, 2155812u32)?;
    emu.sw_no_count(16usize, 2usize, 28u32, 2155816u32)?;
    emu.sh_no_count(30usize, 2usize, 32u32, 2155820u32)?;
    emu.slr_no_count(10usize, 14usize, 31usize, 2155824u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2155864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e558));
    } else {
        emu.pc = 2155828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e534));
    }
}
#[inline(always)]
pub fn block_0x0020e534(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(28usize, 10usize, 0u32, 2155832u32);
    emu.anr_no_count(29usize, 5usize, 10usize, 2155836u32);
    emu.srr_no_count(10usize, 10usize, 31usize, 2155840u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2155892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e574));
    } else {
        emu.pc = 2155844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e544));
    }
}
#[inline(always)]
pub fn block_0x0020e544(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 28usize, 1u32, 2155848u32);
    emu.slr_no_count(12usize, 12usize, 8usize, 2155852u32);
    emu.srr_no_count(31usize, 29usize, 31usize, 2155856u32);
    emu.orr_no_count(31usize, 31usize, 12usize, 2155860u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2155864u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2155896u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e578));
}
#[inline(always)]
pub fn block_0x0020e558(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 14usize, 1u32, 2155868u32);
    emu.srr_no_count(12usize, 12usize, 8usize, 2155872u32);
    emu.slr_no_count(28usize, 16usize, 31usize, 2155876u32);
    emu.orr_no_count(28usize, 28usize, 12usize, 2155880u32);
    emu.anr_no_count(29usize, 5usize, 10usize, 2155884u32);
    emu.srr_no_count(10usize, 28usize, 31usize, 2155888u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2155844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e544));
    } else {
        emu.pc = 2155892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e574));
    }
}
#[inline(always)]
pub fn block_0x0020e574(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(31usize, 10usize, 0u32, 2155896u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2155896u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e578));
}
#[inline(always)]
pub fn block_0x0020e578(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(10usize, 5usize, 10usize, 2155900u32);
    emu.xrr_no_count(12usize, 31usize, 14usize, 2155904u32);
    emu.xrr_no_count(14usize, 10usize, 16usize, 2155908u32);
    emu.orr_no_count(12usize, 12usize, 14usize, 2155912u32);
    emu.sw_no_count(31usize, 2usize, 40u32, 2155916u32)?;
    emu.sw_no_count(10usize, 2usize, 44u32, 2155920u32)?;
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2158352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2158352u32));
    } else {
        emu.pc = 2155924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e594));
    }
}
#[inline]
pub fn block_0x0020e594(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 2usize, 16u32, 2155928u32)?;
    emu.sbr_no_count(10usize, 30usize, 7usize, 2155932u32);
    emu.adi_no_count(12usize, 0usize, 4294967200u32, 2155936u32);
    emu.adi_no_count(16usize, 0usize, 80u32, 2155940u32);
    let a = 0u32.wrapping_add(2068697088u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2155944u32;
    emu.update_insn_clock();
    emu.sbr_no_count(12usize, 12usize, 10usize, 2155948u32);
    emu.adi_no_count(14usize, 14usize, 4294965651u32, 2155952u32);
    emu.sli_no_count(12usize, 12usize, 16u32, 2155956u32);
    emu.sai_no_count(12usize, 12usize, 1040u32, 2155960u32);
    emu.adi_no_count(12usize, 12usize, 1087u32, 2155964u32);
    emu.mul_no_count(12usize, 12usize, 16usize, 2155968u32);
    emu.mulh_no_count(12usize, 12usize, 14usize, 2155972u32);
    emu.sri_no_count(14usize, 12usize, 31u32, 2155976u32);
    emu.sai_no_count(12usize, 12usize, 1034u32, 2155980u32);
    emu.adr_no_count(14usize, 12usize, 14usize, 2155984u32);
    emu.add_memory_rw_events(15usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a < b {
        emu.pc = 2158632u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2158632u32));
    } else {
        emu.pc = 2155988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e5d4));
    }
}
#[inline(never)]
pub fn block_0x0020e5d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 79u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.slr_no_count(12usize, 15usize, 7usize, 2155992u32);
    emu.sli_no_count(14usize, 14usize, 4u32, 2155996u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2156000u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 4294965296u32, 2156004u32);
    emu.sbr_no_count(15usize, 0usize, 10usize, 2156008u32);
    emu.adr_no_count(16usize, 16usize, 14usize, 2156012u32);
    emu.lw_no_count(14usize, 16usize, 0u32, 2156016u32)?;
    emu.lw_no_count(7usize, 16usize, 4u32, 2156020u32)?;
    emu.anr_no_count(10usize, 5usize, 12usize, 2156024u32);
    emu.lh_no_count(12usize, 16usize, 8u32, 2156028u32)?;
    emu.mulhu_no_count(30usize, 14usize, 10usize, 2156032u32);
    emu.mul_no_count(31usize, 7usize, 10usize, 2156036u32);
    emu.mulhu_no_count(8usize, 7usize, 10usize, 2156040u32);
    emu.mul_no_count(9usize, 14usize, 11usize, 2156044u32);
    emu.mulhu_no_count(18usize, 14usize, 11usize, 2156048u32);
    emu.mul_no_count(10usize, 7usize, 11usize, 2156052u32);
    emu.mulhu_no_count(11usize, 7usize, 11usize, 2156056u32);
    emu.sbr_no_count(15usize, 15usize, 12usize, 2156060u32);
    emu.mulhu_no_count(12usize, 14usize, 6usize, 2156064u32);
    emu.mul_no_count(19usize, 7usize, 6usize, 2156068u32);
    emu.mulhu_no_count(6usize, 7usize, 6usize, 2156072u32);
    emu.mul_no_count(20usize, 14usize, 17usize, 2156076u32);
    emu.mulhu_no_count(21usize, 14usize, 17usize, 2156080u32);
    emu.mul_no_count(5usize, 7usize, 17usize, 2156084u32);
    emu.mulhu_no_count(17usize, 7usize, 17usize, 2156088u32);
    emu.mulhu_no_count(22usize, 14usize, 29usize, 2156092u32);
    emu.mul_no_count(23usize, 7usize, 29usize, 2156096u32);
    emu.mulhu_no_count(29usize, 7usize, 29usize, 2156100u32);
    emu.mul_no_count(24usize, 14usize, 28usize, 2156104u32);
    emu.mulhu_no_count(14usize, 14usize, 28usize, 2156108u32);
    emu.mul_no_count(25usize, 7usize, 28usize, 2156112u32);
    emu.mulhu_no_count(28usize, 7usize, 28usize, 2156116u32);
    emu.adr_no_count(30usize, 31usize, 30usize, 2156120u32);
    emu.sltru_no_count(7usize, 30usize, 31usize, 2156124u32);
    emu.adr_no_count(7usize, 8usize, 7usize, 2156128u32);
    emu.adr_no_count(12usize, 19usize, 12usize, 2156132u32);
    emu.sltru_no_count(31usize, 12usize, 19usize, 2156136u32);
    emu.adr_no_count(31usize, 6usize, 31usize, 2156140u32);
    emu.adr_no_count(22usize, 23usize, 22usize, 2156144u32);
    emu.sltru_no_count(6usize, 22usize, 23usize, 2156148u32);
    emu.adr_no_count(19usize, 29usize, 6usize, 2156152u32);
    emu.adr_no_count(30usize, 9usize, 30usize, 2156156u32);
    emu.sltru_no_count(6usize, 30usize, 9usize, 2156160u32);
    emu.adr_no_count(18usize, 18usize, 6usize, 2156164u32);
    emu.adr_no_count(6usize, 20usize, 12usize, 2156168u32);
    emu.sltru_no_count(12usize, 6usize, 20usize, 2156172u32);
    emu.adr_no_count(12usize, 21usize, 12usize, 2156176u32);
    emu.adr_no_count(22usize, 24usize, 22usize, 2156180u32);
    emu.sltru_no_count(29usize, 22usize, 24usize, 2156184u32);
    emu.adr_no_count(14usize, 14usize, 29usize, 2156188u32);
    emu.adr_no_count(18usize, 7usize, 18usize, 2156192u32);
    emu.sltru_no_count(7usize, 18usize, 7usize, 2156196u32);
    emu.adr_no_count(9usize, 11usize, 7usize, 2156200u32);
    emu.ani_no_count(7usize, 15usize, 63u32, 2156204u32);
    emu.adr_no_count(12usize, 31usize, 12usize, 2156208u32);
    emu.sltru_no_count(8usize, 12usize, 31usize, 2156212u32);
    emu.adr_no_count(8usize, 17usize, 8usize, 2156216u32);
    emu.adi_no_count(29usize, 7usize, 4294967264u32, 2156220u32);
    emu.sri_no_count(17usize, 30usize, 31u32, 2156224u32);
    emu.sri_no_count(30usize, 22usize, 31u32, 2156228u32);
    emu.adr_no_count(11usize, 19usize, 14usize, 2156232u32);
    emu.adr_no_count(18usize, 10usize, 18usize, 2156236u32);
    emu.adr_no_count(14usize, 5usize, 12usize, 2156240u32);
    emu.sltru_no_count(12usize, 11usize, 19usize, 2156244u32);
    emu.adr_no_count(31usize, 25usize, 11usize, 2156248u32);
    emu.sltru_no_count(19usize, 18usize, 10usize, 2156252u32);
    emu.adr_no_count(11usize, 17usize, 18usize, 2156256u32);
    emu.sltru_no_count(10usize, 14usize, 5usize, 2156260u32);
    emu.sltru_no_count(24usize, 31usize, 25usize, 2156264u32);
    emu.adr_no_count(5usize, 28usize, 12usize, 2156268u32);
    emu.adr_no_count(18usize, 30usize, 31usize, 2156272u32);
    emu.adr_no_count(9usize, 9usize, 19usize, 2156276u32);
    emu.sltru_no_count(17usize, 11usize, 17usize, 2156280u32);
    emu.adi_no_count(12usize, 11usize, 1u32, 2156284u32);
    emu.adr_no_count(17usize, 9usize, 17usize, 2156288u32);
    emu.sltiu_no_count(28usize, 12usize, 1u32, 2156292u32);
    emu.adr_no_count(21usize, 17usize, 28usize, 2156296u32);
    emu.xri_no_count(31usize, 7usize, 4294967295u32, 2156300u32);
    emu.add_memory_rw_events(78usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(29usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2156312u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e718));
    } else {
        emu.pc = 2156304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e710));
    }
}
#[inline(always)]
pub fn block_0x0020e710(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.srr_no_count(22usize, 21usize, 7usize, 2156308u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2156312u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156328u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e728));
}
#[inline(always)]
pub fn block_0x0020e718(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(28usize, 21usize, 1u32, 2156316u32);
    emu.slr_no_count(28usize, 28usize, 31usize, 2156320u32);
    emu.srr_no_count(9usize, 12usize, 15usize, 2156324u32);
    emu.orr_no_count(22usize, 9usize, 28usize, 2156328u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2156328u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e728));
}
#[inline]
pub fn block_0x0020e728(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(24usize, 5usize, 24usize, 2156332u32);
    emu.sw_no_count(18usize, 2usize, 12u32, 2156336u32)?;
    emu.sltru_no_count(18usize, 18usize, 30usize, 2156340u32);
    emu.lhu_no_count(9usize, 16usize, 10u32, 2156344u32)?;
    emu.adr_no_count(8usize, 8usize, 10usize, 2156348u32);
    emu.sai_no_count(5usize, 6usize, 1055u32, 2156352u32);
    emu.slti_no_count(10usize, 29usize, 0u32, 2156356u32);
    emu.adi_no_count(16usize, 0usize, 1u32, 2156360u32);
    emu.sri_no_count(6usize, 22usize, 4u32, 2156364u32);
    emu.adi_no_count(28usize, 0usize, 625u32, 2156368u32);
    emu.slr_no_count(30usize, 16usize, 7usize, 2156372u32);
    emu.slr_no_count(16usize, 16usize, 15usize, 2156376u32);
    emu.adi_no_count(15usize, 10usize, 4294967295u32, 2156380u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2156384u32);
    emu.anr_no_count(15usize, 15usize, 30usize, 2156388u32);
    emu.anr_no_count(16usize, 10usize, 16usize, 2156392u32);
    emu.sltiu_no_count(10usize, 16usize, 1u32, 2156396u32);
    emu.sbr_no_count(19usize, 15usize, 10usize, 2156400u32);
    emu.adi_no_count(20usize, 16usize, 4294967295u32, 2156404u32);
    emu.sw_no_count(26usize, 2usize, 20u32, 2156408u32)?;
    emu.add_memory_rw_events(20usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a >= b {
        emu.pc = 2156440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e798));
    } else {
        emu.pc = 2156412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e77c));
    }
}
#[inline(always)]
pub fn block_0x0020e77c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 0usize, 100u32, 2156416u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(26usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a >= b {
        emu.pc = 2156484u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e7c4));
    } else {
        emu.pc = 2156420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e784));
    }
}
#[inline(always)]
pub fn block_0x0020e784(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(6usize, 0usize, 9u32, 2156424u32);
    emu.sltiu_no_count(10usize, 22usize, 10u32, 2156428u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a < b {
        emu.pc = 2156552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e808));
    } else {
        emu.pc = 2156432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e790));
    }
}
#[inline(always)]
pub fn block_0x0020e790(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 0usize, 1u32, 2156436u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2156440u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156556u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e80c));
}
#[inline(always)]
pub fn block_0x0020e798(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(999424u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2156444u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 576u32, 2156448u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a >= b {
        emu.pc = 2156508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e7dc));
    } else {
        emu.pc = 2156452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e7a4));
    }
}
#[inline(always)]
pub fn block_0x0020e7a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(98304u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2156456u32;
    emu.update_insn_clock();
    emu.adi_no_count(26usize, 10usize, 1696u32, 2156460u32);
    emu.sltru_no_count(10usize, 22usize, 26usize, 2156464u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(26usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a >= b {
        emu.pc = 2156476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e7bc));
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
#[inline(always)]
pub fn block_0x0020e7b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2156472u32;
    emu.update_insn_clock();
    emu.adi_no_count(26usize, 6usize, 1808u32, 2156476u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2156476u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e7bc));
}
#[inline(always)]
pub fn block_0x0020e7bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(1usize, 10usize, 5u32, 2156480u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2156484u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156588u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e82c));
}
#[inline(always)]
pub fn block_0x0020e7c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(6usize, 0usize, 1000u32, 2156488u32);
    emu.sltiu_no_count(10usize, 22usize, 1000u32, 2156492u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a < b {
        emu.pc = 2156500u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e7d4));
    } else {
        emu.pc = 2156496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e7d0));
    }
}
#[inline(always)]
pub fn block_0x0020e7d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 0usize, 1000u32, 2156500u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2156500u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e7d4));
}
#[inline(always)]
pub fn block_0x0020e7d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(1usize, 10usize, 3u32, 2156504u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2156508u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156588u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e82c));
}
#[inline(always)]
pub fn block_0x0020e7dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(99999744u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2156512u32;
    emu.update_insn_clock();
    emu.adi_no_count(26usize, 10usize, 256u32, 2156516u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(26usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a >= b {
        emu.pc = 2156564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e814));
    } else {
        emu.pc = 2156520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e7e8));
    }
}
#[inline(always)]
pub fn block_0x0020e7e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(9998336u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2156524u32;
    emu.update_insn_clock();
    emu.adi_no_count(26usize, 10usize, 1664u32, 2156528u32);
    emu.sltru_no_count(10usize, 22usize, 26usize, 2156532u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(26usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a >= b {
        emu.pc = 2156544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e800));
    } else {
        emu.pc = 2156536u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e7f8));
    }
}
#[inline(always)]
pub fn block_0x0020e7f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(999424u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2156540u32;
    emu.update_insn_clock();
    emu.adi_no_count(26usize, 6usize, 576u32, 2156544u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2156544u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e800));
}
#[inline(always)]
pub fn block_0x0020e800(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(1usize, 10usize, 7u32, 2156548u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2156552u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156588u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e82c));
}
#[inline(always)]
pub fn block_0x0020e808(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 0usize, 10u32, 2156556u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2156556u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e80c));
}
#[inline(always)]
pub fn block_0x0020e80c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(1usize, 10usize, 1u32, 2156560u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2156564u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156588u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e82c));
}
#[inline(always)]
pub fn block_0x0020e814(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(1000001536u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2156568u32;
    emu.update_insn_clock();
    emu.adi_no_count(6usize, 10usize, 4294965760u32, 2156572u32);
    emu.sltru_no_count(10usize, 22usize, 6usize, 2156576u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a < b {
        emu.pc = 2156584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e828));
    } else {
        emu.pc = 2156580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e824));
    }
}
#[inline(always)]
pub fn block_0x0020e824(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 6usize, 0u32, 2156584u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2156584u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e828));
}
#[inline(always)]
pub fn block_0x0020e828(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(1usize, 10usize, 9u32, 2156588u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2156588u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e82c));
}
#[inline(never)]
pub fn block_0x0020e82c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(18usize, 24usize, 18usize, 2156592u32);
    emu.sw_no_count(18usize, 2usize, 4u32, 2156596u32)?;
    emu.sw_no_count(21usize, 2usize, 8u32, 2156600u32)?;
    emu.anr_no_count(18usize, 19usize, 21usize, 2156604u32);
    emu.anr_no_count(21usize, 20usize, 12usize, 2156608u32);
    emu.sbr_no_count(6usize, 1usize, 9usize, 2156612u32);
    emu.sltru_no_count(28usize, 5usize, 14usize, 2156616u32);
    emu.sbr_no_count(30usize, 5usize, 8usize, 2156620u32);
    emu.sbr_no_count(5usize, 5usize, 14usize, 2156624u32);
    emu.adi_no_count(24usize, 0usize, 4294967295u32, 2156628u32);
    emu.sai_no_count(14usize, 29usize, 1055u32, 2156632u32);
    let a = 0u32.wrapping_add(3435974656u32);
    emu.write_reg_no_count(8usize, a);
    emu.pc = 2156636u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 0usize, 10u32, 2156640u32);
    emu.adi_no_count(6usize, 6usize, 1u32, 2156644u32);
    emu.sw_no_count(6usize, 2usize, 0u32, 2156648u32)?;
    emu.sbr_no_count(28usize, 30usize, 28usize, 2156652u32);
    emu.adr_no_count(11usize, 5usize, 11usize, 2156656u32);
    emu.adi_no_count(6usize, 8usize, 4294966477u32, 2156660u32);
    emu.sltru_no_count(5usize, 11usize, 5usize, 2156664u32);
    emu.adi_no_count(8usize, 11usize, 2u32, 2156668u32);
    emu.adr_no_count(5usize, 17usize, 5usize, 2156672u32);
    emu.sltru_no_count(9usize, 8usize, 11usize, 2156676u32);
    emu.anr_no_count(17usize, 8usize, 20usize, 2156680u32);
    emu.adr_no_count(5usize, 28usize, 5usize, 2156684u32);
    emu.adr_no_count(9usize, 5usize, 9usize, 2156688u32);
    emu.anr_no_count(5usize, 9usize, 19usize, 2156692u32);
    emu.adi_no_count(11usize, 0usize, 4294967295u32, 2156696u32);
    emu.lw_no_count(23usize, 2usize, 20u32, 2156700u32)?;
    emu.add_memory_rw_events(28usize);
    emu.pc = 2156700u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e89c));
}
#[inline(always)]
pub fn block_0x0020e89c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(28usize, 13usize, 11usize, 2156704u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2158584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2158584u32));
    } else {
        emu.pc = 2156708u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e8a4));
    }
}
#[inline(always)]
pub fn block_0x0020e8a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 26usize, 0u32, 2156712u32);
    emu.divu_no_count(28usize, 22usize, 26usize, 2156716u32);
    emu.mul_no_count(26usize, 28usize, 26usize, 2156720u32);
    emu.adi_no_count(30usize, 28usize, 48u32, 2156724u32);
    emu.sbr_no_count(22usize, 22usize, 26usize, 2156728u32);
    emu.sb_no_count(30usize, 23usize, 0u32, 2156732u32);
    emu.slr_no_count(26usize, 22usize, 7usize, 2156736u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(29usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2156776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e8e8));
    } else {
        emu.pc = 2156740u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e8c4));
    }
}
#[inline(always)]
pub fn block_0x0020e8c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(28usize, 14usize, 26usize, 2156744u32);
    emu.adr_no_count(27usize, 26usize, 18usize, 2156748u32);
    emu.adr_no_count(26usize, 28usize, 21usize, 2156752u32);
    emu.sltru_no_count(28usize, 26usize, 28usize, 2156756u32);
    emu.adr_no_count(27usize, 27usize, 28usize, 2156760u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(27usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a != b {
        emu.pc = 2156808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e908));
    } else {
        emu.pc = 2156764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e8dc));
    }
}
#[inline(always)]
pub fn block_0x0020e8dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 26usize, 8usize, 2156768u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2156816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e910));
    } else {
        emu.pc = 2156772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e8e4));
    }
}
#[inline(always)]
pub fn block_0x0020e8e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2156776u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156860u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e93c));
}
#[inline(always)]
pub fn block_0x0020e8e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(28usize, 22usize, 1u32, 2156780u32);
    emu.srr_no_count(27usize, 28usize, 31usize, 2156784u32);
    emu.anr_no_count(28usize, 14usize, 26usize, 2156788u32);
    emu.adr_no_count(27usize, 27usize, 18usize, 2156792u32);
    emu.adr_no_count(26usize, 28usize, 21usize, 2156796u32);
    emu.sltru_no_count(28usize, 26usize, 28usize, 2156800u32);
    emu.adr_no_count(27usize, 27usize, 28usize, 2156804u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(27usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2156764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e8dc));
    } else {
        emu.pc = 2156808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e908));
    }
}
#[inline(always)]
pub fn block_0x0020e908(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 27usize, 9usize, 2156812u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a != b {
        emu.pc = 2156860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e93c));
    } else {
        emu.pc = 2156816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e910));
    }
}
#[inline(always)]
pub fn block_0x0020e910(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(28usize, 1usize, 11usize, 2156820u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2156876u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e94c));
    } else {
        emu.pc = 2156824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e918));
    }
}
#[inline(always)]
pub fn block_0x0020e918(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.mulhu_no_count(28usize, 25usize, 6usize, 2156828u32);
    emu.adi_no_count(23usize, 23usize, 1u32, 2156832u32);
    emu.sri_no_count(26usize, 28usize, 3u32, 2156836u32);
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2156840u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(25usize);
    if a >= b {
        emu.pc = 2156700u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e89c));
    } else {
        emu.pc = 2156844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e92c));
    }
}
#[inline(always)]
pub fn block_0x0020e92c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2156848u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966872u32, 2156852u32);
    emu.apc_no_count(1usize, 2156852u32, 4096u32, 2156856u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2156860u32;
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
pub fn block_0x0020e93c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.slr_no_count(10usize, 25usize, 7usize, 2156864u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(29usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2157136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ea50));
    } else {
        emu.pc = 2156868u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e944));
    }
}
#[inline(always)]
pub fn block_0x0020e944(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 10usize, 0u32, 2156872u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2156876u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157144u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ea58));
}
#[inline(always)]
pub fn block_0x0020e94c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 0u32, 2156880u32);
    emu.sbr_no_count(11usize, 0usize, 11usize, 2156884u32);
    emu.adi_no_count(6usize, 0usize, 1u32, 2156888u32);
    emu.adi_no_count(10usize, 0usize, 10u32, 2156892u32);
    emu.lw_no_count(23usize, 2usize, 20u32, 2156896u32)?;
    emu.add_memory_rw_events(6usize);
    let return_addr = 2156900u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2156928u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e980));
}
#[inline(always)]
pub fn block_0x0020e964(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(30usize, 18usize, 5usize, 2156904u32);
    emu.mulhu_no_count(14usize, 8usize, 10usize, 2156908u32);
    emu.mul_no_count(6usize, 9usize, 10usize, 2156912u32);
    emu.adr_no_count(14usize, 14usize, 6usize, 2156916u32);
    emu.mul_no_count(6usize, 8usize, 10usize, 2156920u32);
    emu.adi_no_count(11usize, 11usize, 1u32, 2156924u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(30usize);
    if a != b {
        emu.pc = 2157052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e9fc));
    } else {
        emu.pc = 2156928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e980));
    }
}
#[inline(always)]
pub fn block_0x0020e980(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2158608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2158608u32));
    } else {
        emu.pc = 2156932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e984));
    }
}
#[inline(always)]
pub fn block_0x0020e984(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 14usize, 0u32, 2156936u32);
    emu.adi_no_count(8usize, 6usize, 0u32, 2156940u32);
    emu.mulhu_no_count(14usize, 21usize, 10usize, 2156944u32);
    emu.mul_no_count(6usize, 18usize, 10usize, 2156948u32);
    emu.adr_no_count(14usize, 14usize, 6usize, 2156952u32);
    emu.mul_no_count(6usize, 21usize, 10usize, 2156956u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(29usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2156968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e9a8));
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
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.srr_no_count(30usize, 14usize, 7usize, 2156964u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2156968u32;
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
pub fn block_0x0020e9a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(28usize, 14usize, 1u32, 2156972u32);
    emu.slr_no_count(28usize, 28usize, 31usize, 2156976u32);
    emu.srr_no_count(30usize, 6usize, 7usize, 2156980u32);
    emu.orr_no_count(30usize, 30usize, 28usize, 2156984u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2156984u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020e9b8));
}
#[inline]
pub fn block_0x0020e9b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(18usize, 14usize, 19usize, 2156988u32);
    emu.anr_no_count(21usize, 6usize, 20usize, 2156992u32);
    emu.mulhu_no_count(14usize, 17usize, 10usize, 2156996u32);
    emu.mul_no_count(5usize, 5usize, 10usize, 2157000u32);
    emu.mul_no_count(17usize, 17usize, 10usize, 2157004u32);
    emu.adi_no_count(22usize, 30usize, 48u32, 2157008u32);
    emu.adr_no_count(5usize, 14usize, 5usize, 2157012u32);
    emu.adr_no_count(14usize, 23usize, 11usize, 2157016u32);
    emu.sb_no_count(22usize, 14usize, 0u32, 2157020u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a != b {
        emu.pc = 2156900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e964));
    } else {
        emu.pc = 2157024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e9e0));
    }
}
#[inline(always)]
pub fn block_0x0020e9e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(30usize, 21usize, 17usize, 2157028u32);
    emu.mulhu_no_count(14usize, 8usize, 10usize, 2157032u32);
    emu.mul_no_count(6usize, 9usize, 10usize, 2157036u32);
    emu.adr_no_count(14usize, 14usize, 6usize, 2157040u32);
    emu.mul_no_count(6usize, 8usize, 10usize, 2157044u32);
    emu.adi_no_count(11usize, 11usize, 1u32, 2157048u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(30usize);
    if a == b {
        emu.pc = 2156928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e980));
    } else {
        emu.pc = 2157052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020e9fc));
    }
}
#[inline]
pub fn block_0x0020e9fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(7usize, 2usize, 12u32, 2157056u32)?;
    emu.sltru_no_count(10usize, 12usize, 7usize, 2157060u32);
    emu.lw_no_count(13usize, 2usize, 8u32, 2157064u32)?;
    emu.lw_no_count(28usize, 2usize, 4u32, 2157068u32)?;
    emu.sbr_no_count(13usize, 13usize, 28usize, 2157072u32);
    emu.sbr_no_count(12usize, 12usize, 7usize, 2157076u32);
    emu.sbr_no_count(13usize, 13usize, 10usize, 2157080u32);
    emu.mulhu_no_count(10usize, 6usize, 12usize, 2157084u32);
    emu.mul_no_count(31usize, 14usize, 12usize, 2157088u32);
    emu.mul_no_count(7usize, 6usize, 12usize, 2157092u32);
    emu.mul_no_count(12usize, 6usize, 13usize, 2157096u32);
    emu.adr_no_count(13usize, 7usize, 6usize, 2157100u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2157104u32);
    emu.sltru_no_count(12usize, 7usize, 6usize, 2157108u32);
    emu.adr_no_count(31usize, 10usize, 31usize, 2157112u32);
    emu.adr_no_count(12usize, 14usize, 12usize, 2157116u32);
    emu.sbr_no_count(29usize, 31usize, 12usize, 2157120u32);
    emu.sbr_no_count(30usize, 7usize, 6usize, 2157124u32);
    emu.add_memory_rw_events(18usize);
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2157304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eaf8));
    } else {
        emu.pc = 2157128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ea48));
    }
}
#[inline(always)]
pub fn block_0x0020ea48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 18usize, 29usize, 2157132u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2157136u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157308u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020eafc));
}
#[inline(always)]
pub fn block_0x0020ea50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(13usize, 25usize, 1u32, 2157140u32);
    emu.srr_no_count(15usize, 13usize, 31usize, 2157144u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2157144u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ea58));
}
#[inline]
pub fn block_0x0020ea58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 2usize, 16u32, 2157148u32)?;
    emu.lw_no_count(16usize, 2usize, 8u32, 2157152u32)?;
    emu.lw_no_count(17usize, 2usize, 12u32, 2157156u32)?;
    emu.sltru_no_count(13usize, 12usize, 17usize, 2157160u32);
    emu.lw_no_count(5usize, 2usize, 4u32, 2157164u32)?;
    emu.sbr_no_count(16usize, 16usize, 5usize, 2157168u32);
    emu.sbr_no_count(28usize, 12usize, 17usize, 2157172u32);
    emu.sbr_no_count(17usize, 16usize, 13usize, 2157176u32);
    emu.adi_no_count(13usize, 28usize, 1u32, 2157180u32);
    emu.sltiu_no_count(12usize, 28usize, 1u32, 2157184u32);
    emu.sbr_no_count(7usize, 17usize, 12usize, 2157188u32);
    emu.adi_no_count(28usize, 28usize, 4294967295u32, 2157192u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(27usize);
    if a == b {
        emu.pc = 2157204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ea94));
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
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 27usize, 7usize, 2157200u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2157204u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157208u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ea98));
}
#[inline(always)]
pub fn block_0x0020ea94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 26usize, 28usize, 2157208u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2157208u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ea98));
}
#[inline(always)]
pub fn block_0x0020ea98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(6usize, 14usize, 10usize, 2157212u32);
    emu.sltiu_no_count(29usize, 13usize, 1u32, 2157216u32);
    emu.sltru_no_count(10usize, 8usize, 26usize, 2157220u32);
    emu.sbr_no_count(12usize, 9usize, 27usize, 2157224u32);
    emu.sbr_no_count(10usize, 12usize, 10usize, 2157228u32);
    emu.sbr_no_count(5usize, 8usize, 26usize, 2157232u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2157252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eac4));
    } else {
        emu.pc = 2157236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eab4));
    }
}
#[inline(always)]
pub fn block_0x0020eab4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(14usize, 10usize, 15usize, 2157240u32);
    emu.adr_no_count(17usize, 17usize, 29usize, 2157244u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a != b {
        emu.pc = 2157264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ead0));
    } else {
        emu.pc = 2157248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eac0));
    }
}
#[inline(always)]
pub fn block_0x0020eac0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2157252u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157276u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020eadc));
}
#[inline(always)]
pub fn block_0x0020eac4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(14usize, 5usize, 6usize, 2157256u32);
    emu.adr_no_count(17usize, 17usize, 29usize, 2157260u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2157276u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eadc));
    } else {
        emu.pc = 2157264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ead0));
    }
}
#[inline(always)]
pub fn block_0x0020ead0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2157404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2157404u32));
    } else {
        emu.pc = 2157268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ead4));
    }
}
#[inline(always)]
pub fn block_0x0020ead4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 10usize, 15usize, 2157272u32);
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
        return Ok(crate::NextStep::Dynamic(2157432u32));
    } else {
        emu.pc = 2157276u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eadc));
    }
}
#[inline(always)]
pub fn block_0x0020eadc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 26usize, 0u32, 2157280u32);
    emu.adi_no_count(5usize, 27usize, 0u32, 2157284u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(27usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a != b {
        emu.pc = 2157620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2157620u32));
    } else {
        emu.pc = 2157288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eae8));
    }
}
#[inline(always)]
pub fn block_0x0020eae8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 16usize, 13usize, 2157292u32);
    emu.lw_no_count(7usize, 2usize, 20u32, 2157296u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2157632u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2157632u32));
    } else {
        emu.pc = 2157300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eaf4));
    }
}
#[inline(always)]
pub fn block_0x0020eaf4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2157304u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157988u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(2157988u32));
}
#[inline(always)]
pub fn block_0x0020eaf8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 21usize, 30usize, 2157308u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2157308u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020eafc));
}
#[inline(always)]
pub fn block_0x0020eafc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 13usize, 7usize, 2157312u32);
    emu.adr_no_count(7usize, 31usize, 14usize, 2157316u32);
    emu.sltru_no_count(12usize, 17usize, 21usize, 2157320u32);
    emu.sbr_no_count(14usize, 5usize, 18usize, 2157324u32);
    emu.sbr_no_count(14usize, 14usize, 12usize, 2157328u32);
    emu.sbr_no_count(6usize, 17usize, 21usize, 2157332u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2157352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eb28));
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
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(31usize, 14usize, 15usize, 2157340u32);
    emu.adr_no_count(28usize, 7usize, 28usize, 2157344u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2157364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2157364u32));
    } else {
        emu.pc = 2157348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020eb24));
    }
}
#[inline(always)]
pub fn block_0x0020eb24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2157352u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2157376u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(2157376u32));
}
#[inline(always)]
pub fn block_0x0020eb28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(31usize, 6usize, 16usize, 2157356u32);
    emu.adr_no_count(28usize, 7usize, 28usize, 2157360u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2157376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2157376u32));
    } else {
        emu.pc = 2157364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Dynamic(2157364u32));
    }
}
