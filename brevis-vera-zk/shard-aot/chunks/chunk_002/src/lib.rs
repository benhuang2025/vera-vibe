pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2105276u32;
pub const PC_MAX: u32 = 2108892u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 137usize] = [
        block_0x00201fbc,
        block_0x00202020,
        block_0x0020202c,
        block_0x00202098,
        block_0x002020a4,
        block_0x002020e8,
        block_0x002020ec,
        block_0x002020f0,
        block_0x002020f8,
        block_0x00202110,
        block_0x00202114,
        block_0x00202140,
        block_0x0020215c,
        block_0x00202164,
        block_0x00202180,
        block_0x00202190,
        block_0x00202194,
        block_0x002021a4,
        block_0x002021c0,
        block_0x002021e0,
        block_0x002021ec,
        block_0x0020221c,
        block_0x00202234,
        block_0x00202278,
        block_0x0020227c,
        block_0x00202284,
        block_0x00202290,
        block_0x002022a8,
        block_0x002022ac,
        block_0x002022c4,
        block_0x002022cc,
        block_0x002022f0,
        block_0x00202314,
        block_0x0020231c,
        block_0x0020232c,
        block_0x00202334,
        block_0x00202338,
        block_0x00202344,
        block_0x00202354,
        block_0x00202358,
        block_0x0020236c,
        block_0x00202384,
        block_0x002023b0,
        block_0x002023b8,
        block_0x002023d4,
        block_0x002023e4,
        block_0x002023f0,
        block_0x00202400,
        block_0x00202408,
        block_0x0020240c,
        block_0x00202424,
        block_0x00202440,
        block_0x00202460,
        block_0x00202488,
        block_0x00202494,
        block_0x002024bc,
        block_0x002024c8,
        block_0x002024f0,
        block_0x002024fc,
        block_0x00202518,
        block_0x00202534,
        block_0x0020255c,
        block_0x00202568,
        block_0x00202584,
        block_0x002025a0,
        block_0x002025c4,
        block_0x002025d0,
        block_0x002025f8,
        block_0x002025fc,
        block_0x00202600,
        block_0x0020260c,
        block_0x0020261c,
        block_0x0020262c,
        block_0x00202630,
        block_0x0020263c,
        block_0x00202644,
        block_0x0020265c,
        block_0x00202660,
        block_0x00202690,
        block_0x0020269c,
        block_0x002026a0,
        block_0x002026b4,
        block_0x002026c4,
        block_0x002026cc,
        block_0x002026d4,
        block_0x002026ec,
        block_0x002026f8,
        block_0x002026fc,
        block_0x00202700,
        block_0x00202708,
        block_0x00202710,
        block_0x00202718,
        block_0x0020271c,
        block_0x00202730,
        block_0x00202744,
        block_0x00202794,
        block_0x002027a8,
        block_0x002027ac,
        block_0x002027b8,
        block_0x00202808,
        block_0x0020281c,
        block_0x00202830,
        block_0x0020283c,
        block_0x00202850,
        block_0x00202858,
        block_0x00202868,
        block_0x00202874,
        block_0x002028ac,
        block_0x002028c4,
        block_0x00202950,
        block_0x00202964,
        block_0x00202970,
        block_0x00202984,
        block_0x00202998,
        block_0x002029e0,
        block_0x002029f4,
        block_0x002029fc,
        block_0x00202b50,
        block_0x00202b60,
        block_0x00202b70,
        block_0x00202b80,
        block_0x00202b90,
        block_0x00202ba0,
        block_0x00202bb0,
        block_0x00202bc0,
        block_0x00202bcc,
        block_0x00202c1c,
        block_0x00202c54,
        block_0x00202c8c,
        block_0x00202cc4,
        block_0x00202cfc,
        block_0x00202d34,
        block_0x00202d6c,
        block_0x00202da4,
        block_0x00202db0,
        block_0x00202dcc,
        block_0x00202ddc,
    ];
    const IDX: [u16; 905usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16,
        0u16, 0u16, 3u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 4u16, 0u16, 0u16, 5u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 6u16, 7u16, 8u16,
        0u16, 9u16, 0u16, 0u16, 0u16, 0u16, 0u16, 10u16, 11u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 12u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        13u16, 0u16, 14u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 15u16, 0u16, 0u16, 0u16,
        16u16, 17u16, 0u16, 0u16, 0u16, 18u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 19u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 20u16, 0u16, 0u16, 21u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 22u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 23u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 24u16, 25u16, 0u16, 26u16, 0u16, 0u16, 27u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 28u16, 29u16, 0u16, 0u16, 0u16, 0u16, 0u16, 30u16,
        0u16, 31u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 32u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 33u16, 0u16, 34u16, 0u16, 0u16, 0u16, 35u16,
        0u16, 36u16, 37u16, 0u16, 0u16, 38u16, 0u16, 0u16, 0u16, 39u16, 40u16, 0u16,
        0u16, 0u16, 0u16, 41u16, 0u16, 0u16, 0u16, 0u16, 0u16, 42u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 43u16, 0u16, 44u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 45u16, 0u16, 0u16, 0u16, 46u16, 0u16, 0u16, 47u16, 0u16, 0u16,
        0u16, 48u16, 0u16, 49u16, 50u16, 0u16, 0u16, 0u16, 0u16, 0u16, 51u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 52u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 53u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 54u16, 0u16, 0u16, 55u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 56u16, 0u16, 0u16, 57u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 58u16, 0u16, 0u16, 59u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 60u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        61u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 62u16, 0u16, 0u16,
        63u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 64u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 65u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 66u16, 0u16, 0u16,
        67u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 68u16, 69u16, 70u16,
        0u16, 0u16, 71u16, 0u16, 0u16, 0u16, 72u16, 0u16, 0u16, 0u16, 73u16, 74u16, 0u16,
        0u16, 75u16, 0u16, 76u16, 0u16, 0u16, 0u16, 0u16, 0u16, 77u16, 78u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 79u16, 0u16, 0u16, 80u16,
        81u16, 0u16, 0u16, 0u16, 0u16, 82u16, 0u16, 0u16, 0u16, 83u16, 0u16, 84u16, 0u16,
        85u16, 0u16, 0u16, 0u16, 0u16, 0u16, 86u16, 0u16, 0u16, 87u16, 88u16, 89u16,
        0u16, 90u16, 0u16, 91u16, 0u16, 92u16, 93u16, 0u16, 0u16, 0u16, 0u16, 94u16,
        0u16, 0u16, 0u16, 0u16, 95u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 96u16, 0u16,
        0u16, 0u16, 0u16, 97u16, 98u16, 0u16, 0u16, 99u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 100u16, 0u16, 0u16, 0u16, 0u16, 101u16, 0u16, 0u16, 0u16, 0u16, 102u16,
        0u16, 0u16, 103u16, 0u16, 0u16, 0u16, 0u16, 104u16, 0u16, 105u16, 0u16, 0u16,
        0u16, 106u16, 0u16, 0u16, 107u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 108u16, 0u16, 0u16, 0u16, 0u16, 0u16, 109u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 110u16, 0u16, 0u16, 0u16, 0u16, 111u16,
        0u16, 0u16, 112u16, 0u16, 0u16, 0u16, 0u16, 113u16, 0u16, 0u16, 0u16, 0u16,
        114u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 115u16, 0u16, 0u16, 0u16, 0u16, 116u16, 0u16,
        117u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 118u16, 0u16, 0u16, 0u16, 119u16, 0u16,
        0u16, 0u16, 120u16, 0u16, 0u16, 0u16, 121u16, 0u16, 0u16, 0u16, 122u16, 0u16,
        0u16, 0u16, 123u16, 0u16, 0u16, 0u16, 124u16, 0u16, 0u16, 0u16, 125u16, 0u16,
        0u16, 126u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 127u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 128u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 129u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 130u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 131u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        132u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 133u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 134u16, 0u16, 0u16, 135u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        136u16, 0u16, 0u16, 0u16, 137u16,
    ];
    if pc < 2105276u32 || pc > 2108892u32 {
        return None;
    }
    let word_offset = ((pc - 2105276u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(never)]
pub fn block_0x00201fbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 25u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2105280u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2105284u32)?;
    emu.sw_no_count(11usize, 2usize, 12u32, 2105288u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2105292u32)?;
    let a = 0u32.wrapping_add(2129920u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2105296u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966816u32, 2105300u32);
    emu.adi_no_count(12usize, 2usize, 12u32, 2105304u32);
    let a = 0u32.wrapping_add(2101248u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2105308u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966384u32, 2105312u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2105316u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 732u32, 2105320u32);
    emu.adi_no_count(15usize, 0usize, 2u32, 2105324u32);
    emu.sw_no_count(0usize, 2usize, 36u32, 2105328u32)?;
    emu.sw_no_count(10usize, 2usize, 44u32, 2105332u32)?;
    emu.sw_no_count(11usize, 2usize, 48u32, 2105336u32)?;
    emu.sw_no_count(12usize, 2usize, 52u32, 2105340u32)?;
    emu.sw_no_count(13usize, 2usize, 56u32, 2105344u32)?;
    emu.adi_no_count(10usize, 2usize, 44u32, 2105348u32);
    emu.sw_no_count(14usize, 2usize, 20u32, 2105352u32)?;
    emu.sw_no_count(15usize, 2usize, 24u32, 2105356u32)?;
    emu.sw_no_count(10usize, 2usize, 28u32, 2105360u32)?;
    emu.sw_no_count(15usize, 2usize, 32u32, 2105364u32)?;
    emu.adi_no_count(10usize, 2usize, 20u32, 2105368u32);
    emu.apc_no_count(1usize, 2105368u32, 0u32, 2105372u32);
    emu.add_memory_rw_events(25usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2105376u32;
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
pub fn block_0x00202020(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2105380u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2105384u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2105388u32;
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
pub fn block_0x0020202c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2105392u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2105396u32)?;
    emu.sw_no_count(10usize, 2usize, 8u32, 2105400u32)?;
    emu.sw_no_count(11usize, 2usize, 12u32, 2105404u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2105408u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2105412u32);
    let a = 0u32.wrapping_add(2146304u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2105416u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966704u32, 2105420u32);
    emu.adi_no_count(12usize, 2usize, 12u32, 2105424u32);
    let a = 0u32.wrapping_add(2101248u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2105428u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966384u32, 2105432u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2105436u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 764u32, 2105440u32);
    emu.adi_no_count(15usize, 0usize, 2u32, 2105444u32);
    emu.sw_no_count(0usize, 2usize, 36u32, 2105448u32)?;
    emu.sw_no_count(10usize, 2usize, 44u32, 2105452u32)?;
    emu.sw_no_count(11usize, 2usize, 48u32, 2105456u32)?;
    emu.sw_no_count(12usize, 2usize, 52u32, 2105460u32)?;
    emu.sw_no_count(13usize, 2usize, 56u32, 2105464u32)?;
    emu.adi_no_count(10usize, 2usize, 44u32, 2105468u32);
    emu.sw_no_count(14usize, 2usize, 20u32, 2105472u32)?;
    emu.sw_no_count(15usize, 2usize, 24u32, 2105476u32)?;
    emu.sw_no_count(10usize, 2usize, 28u32, 2105480u32)?;
    emu.sw_no_count(15usize, 2usize, 32u32, 2105484u32)?;
    emu.adi_no_count(10usize, 2usize, 20u32, 2105488u32);
    emu.apc_no_count(1usize, 2105488u32, 0u32, 2105492u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2105496u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1344u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202098(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2105500u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2105504u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2105508u32;
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
pub fn block_0x002020a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2105512u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2105516u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2105520u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2105524u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2105528u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2105532u32)?;
    emu.sw_no_count(20usize, 2usize, 40u32, 2105536u32)?;
    emu.sw_no_count(21usize, 2usize, 36u32, 2105540u32)?;
    emu.sw_no_count(22usize, 2usize, 32u32, 2105544u32)?;
    emu.sw_no_count(23usize, 2usize, 28u32, 2105548u32)?;
    emu.sw_no_count(24usize, 2usize, 24u32, 2105552u32)?;
    emu.adi_no_count(9usize, 12usize, 0u32, 2105556u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2105560u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2105564u32);
    let a = 0u32.wrapping_add(1048576u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2105568u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 12usize, 0u32, 2105572u32);
    emu.add_memory_rw_events(16usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2105580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002020ec));
    } else {
        emu.pc = 2105576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002020e8));
    }
}
#[inline(always)]
pub fn block_0x002020e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(1048576u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2105580u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(1usize);
    emu.pc = 2105580u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002020ec));
}
#[inline(always)]
pub fn block_0x002020ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2105748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202194));
    } else {
        emu.pc = 2105584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002020f0));
    }
}
#[inline(always)]
pub fn block_0x002020f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2105584u32, 0u32, 2105588u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2105592u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1576u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002020f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2105596u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 560u32, 2105600u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2105604u32);
    emu.adi_no_count(12usize, 19usize, 0u32, 2105608u32);
    emu.apc_no_count(1usize, 2105608u32, 4096u32, 2105612u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2105616u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(936u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202110(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2105884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020221c));
    } else {
        emu.pc = 2105620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202114));
    }
}
#[inline]
pub fn block_0x00202114(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 0u32, 2105624u32);
    emu.lw_no_count(21usize, 18usize, 0u32, 2105628u32)?;
    emu.lw_no_count(23usize, 18usize, 4u32, 2105632u32)?;
    emu.sw_no_count(19usize, 2usize, 4u32, 2105636u32)?;
    emu.sw_no_count(10usize, 2usize, 8u32, 2105640u32)?;
    emu.sw_no_count(0usize, 2usize, 12u32, 2105644u32)?;
    emu.sbr_no_count(22usize, 0usize, 23usize, 2105648u32);
    emu.adi_no_count(23usize, 23usize, 4294967295u32, 2105652u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2105656u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 19usize, 796u32, 2105660u32);
    emu.add_memory_rw_events(11usize);
    let return_addr = 2105664u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2105692u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020215c));
}
#[inline(always)]
pub fn block_0x00202140(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2105668u32)?;
    emu.adr_no_count(10usize, 10usize, 20usize, 2105672u32);
    emu.adi_no_count(20usize, 20usize, 1u32, 2105676u32);
    emu.sb_no_count(24usize, 10usize, 0u32, 2105680u32);
    emu.sw_no_count(20usize, 2usize, 12u32, 2105684u32)?;
    emu.adi_no_count(23usize, 23usize, 4294967295u32, 2105688u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2105764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002021a4));
    } else {
        emu.pc = 2105692u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020215c));
    }
}
#[inline(always)]
pub fn block_0x0020215c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 22usize, 20usize, 2105696u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2105792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002021c0));
    } else {
        emu.pc = 2105700u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202164));
    }
}
#[inline(always)]
pub fn block_0x00202164(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 21usize, 20usize, 2105704u32);
    emu.lw_no_count(11usize, 2usize, 4u32, 2105708u32)?;
    emu.lbu_no_count(24usize, 10usize, 0u32, 2105712u32);
    emu.adi_no_count(10usize, 10usize, 1u32, 2105716u32);
    emu.sw_no_count(10usize, 18usize, 0u32, 2105720u32)?;
    emu.sw_no_count(23usize, 18usize, 4u32, 2105724u32)?;
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a != b {
        emu.pc = 2105664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202140));
    } else {
        emu.pc = 2105728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202180));
    }
}
#[inline(always)]
pub fn block_0x00202180(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 4u32, 2105732u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2105736u32);
    emu.apc_no_count(1usize, 2105736u32, 36864u32, 2105740u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2105744u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(268u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202190(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2105748u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2105664u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202140));
}
#[inline(always)]
pub fn block_0x00202194(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2105752u32);
    emu.sw_no_count(0usize, 2usize, 4u32, 2105756u32)?;
    emu.sw_no_count(10usize, 2usize, 8u32, 2105760u32)?;
    emu.sw_no_count(0usize, 2usize, 12u32, 2105764u32)?;
    emu.add_memory_rw_events(4usize);
    emu.pc = 2105764u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002021a4));
}
#[inline(always)]
pub fn block_0x002021a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 4u32, 2105768u32)?;
    emu.lw_no_count(11usize, 2usize, 8u32, 2105772u32)?;
    emu.lw_no_count(12usize, 2usize, 12u32, 2105776u32)?;
    emu.sw_no_count(10usize, 8usize, 0u32, 2105780u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2105784u32)?;
    emu.sw_no_count(12usize, 8usize, 8u32, 2105788u32)?;
    emu.add_memory_rw_events(7usize);
    let return_addr = 2105792u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2105836u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002021ec));
}
#[inline(always)]
pub fn block_0x002021c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2105796u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2105800u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1281u32, 2105804u32);
    emu.sw_no_count(11usize, 2usize, 16u32, 2105808u32)?;
    emu.sw_no_count(10usize, 2usize, 20u32, 2105812u32)?;
    emu.adi_no_count(10usize, 2usize, 16u32, 2105816u32);
    emu.apc_no_count(1usize, 2105816u32, 4096u32, 2105820u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2105824u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1644u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002021e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2105828u32;
    emu.update_insn_clock();
    emu.sw_no_count(11usize, 8usize, 0u32, 2105832u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2105836u32)?;
    emu.add_memory_rw_events(3usize);
    emu.pc = 2105836u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002021ec));
}
#[inline]
pub fn block_0x002021ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2105840u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2105844u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2105848u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2105852u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2105856u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2105860u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2105864u32)?;
    emu.lw_no_count(22usize, 2usize, 32u32, 2105868u32)?;
    emu.lw_no_count(23usize, 2usize, 28u32, 2105872u32)?;
    emu.lw_no_count(24usize, 2usize, 24u32, 2105876u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2105880u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2105884u32;
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
pub fn block_0x0020221c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2105888u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 780u32, 2105892u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2105896u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2105900u32);
    emu.apc_no_count(1usize, 2105900u32, 36864u32, 2105904u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2105908u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(672u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00202234(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967216u32, 2105912u32);
    emu.sw_no_count(1usize, 2usize, 76u32, 2105916u32)?;
    emu.sw_no_count(8usize, 2usize, 72u32, 2105920u32)?;
    emu.sw_no_count(9usize, 2usize, 68u32, 2105924u32)?;
    emu.sw_no_count(18usize, 2usize, 64u32, 2105928u32)?;
    emu.sw_no_count(19usize, 2usize, 60u32, 2105932u32)?;
    emu.sw_no_count(20usize, 2usize, 56u32, 2105936u32)?;
    emu.sw_no_count(21usize, 2usize, 52u32, 2105940u32)?;
    emu.sw_no_count(22usize, 2usize, 48u32, 2105944u32)?;
    emu.sw_no_count(23usize, 2usize, 44u32, 2105948u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2105952u32);
    let a = 0u32.wrapping_add(53248u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2105956u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966476u32, 2105960u32);
    emu.sw_no_count(11usize, 2usize, 4u32, 2105964u32)?;
    emu.sw_no_count(12usize, 2usize, 8u32, 2105968u32)?;
    emu.adi_no_count(18usize, 12usize, 0u32, 2105972u32);
    emu.add_memory_rw_events(16usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2105980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020227c));
    } else {
        emu.pc = 2105976u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202278));
    }
}
#[inline(always)]
pub fn block_0x00202278(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2105980u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2105980u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020227c));
}
#[inline(always)]
pub fn block_0x0020227c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 20u32, 2105984u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2106052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002022c4));
    } else {
        emu.pc = 2105988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202284));
    }
}
#[inline(always)]
pub fn block_0x00202284(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.mul_no_count(9usize, 18usize, 19usize, 2105992u32);
    emu.apc_no_count(1usize, 2105992u32, 0u32, 2105996u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106000u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1168u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202290(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2106004u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 560u32, 2106008u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2106012u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2106016u32);
    emu.apc_no_count(1usize, 2106016u32, 4096u32, 2106020u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106024u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(528u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002022a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2106060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002022cc));
    } else {
        emu.pc = 2106028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002022ac));
    }
}
#[inline(always)]
pub fn block_0x002022ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2106032u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 780u32, 2106036u32);
    emu.adi_no_count(10usize, 0usize, 4u32, 2106040u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2106044u32);
    emu.apc_no_count(1usize, 2106044u32, 36864u32, 2106048u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106052u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(528u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002022c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 0u32, 2106056u32);
    emu.adi_no_count(10usize, 0usize, 4u32, 2106060u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2106060u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002022cc));
}
#[inline]
pub fn block_0x002022cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 2usize, 12u32, 2106064u32)?;
    emu.sw_no_count(10usize, 2usize, 16u32, 2106068u32)?;
    emu.sw_no_count(0usize, 2usize, 20u32, 2106072u32)?;
    emu.adi_no_count(9usize, 2usize, 26u32, 2106076u32);
    emu.adi_no_count(20usize, 0usize, 5u32, 2106080u32);
    emu.adi_no_count(21usize, 0usize, 6u32, 2106084u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(18usize, a);
    emu.pc = 2106088u32;
    emu.update_insn_clock();
    emu.adi_no_count(18usize, 18usize, 796u32, 2106092u32);
    emu.add_memory_rw_events(9usize);
    let return_addr = 2106096u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2106140u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020231c));
}
#[inline]
pub fn block_0x002022f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 16u32, 2106100u32)?;
    emu.mul_no_count(11usize, 23usize, 19usize, 2106104u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2106108u32);
    emu.sh_no_count(22usize, 10usize, 0u32, 2106112u32)?;
    emu.adi_no_count(10usize, 10usize, 2u32, 2106116u32);
    emu.adi_no_count(12usize, 0usize, 18u32, 2106120u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2106124u32);
    emu.apc_no_count(1usize, 2106124u32, 4096u32, 2106128u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106132u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966652u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202314(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(23usize, 23usize, 1u32, 2106136u32);
    emu.sw_no_count(23usize, 2usize, 20u32, 2106140u32)?;
    emu.add_memory_rw_events(2usize);
    emu.pc = 2106140u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020231c));
}
#[inline(always)]
pub fn block_0x0020231c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 24u32, 2106144u32);
    emu.adi_no_count(11usize, 2usize, 4u32, 2106148u32);
    emu.apc_no_count(1usize, 2106148u32, 0u32, 2106152u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106156u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965340u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020232c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lhu_no_count(22usize, 2usize, 24u32, 2106160u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a == b {
        emu.pc = 2106220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020236c));
    } else {
        emu.pc = 2106164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202334));
    }
}
#[inline(always)]
pub fn block_0x00202334(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a == b {
        emu.pc = 2106200u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202358));
    } else {
        emu.pc = 2106168u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202338));
    }
}
#[inline(always)]
pub fn block_0x00202338(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2106172u32)?;
    emu.lw_no_count(23usize, 2usize, 20u32, 2106176u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a != b {
        emu.pc = 2106096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002022f0));
    } else {
        emu.pc = 2106180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202344));
    }
}
#[inline(always)]
pub fn block_0x00202344(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 12u32, 2106184u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2106188u32);
    emu.apc_no_count(1usize, 2106188u32, 4294963200u32, 2106192u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106196u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1096u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202354(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2106200u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2106096u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002022f0));
}
#[inline(always)]
pub fn block_0x00202358(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 28u32, 2106204u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2106208u32;
    emu.update_insn_clock();
    emu.sw_no_count(11usize, 8usize, 0u32, 2106212u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2106216u32)?;
    emu.add_memory_rw_events(5usize);
    let return_addr = 2106220u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2106244u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202384));
}
#[inline(always)]
pub fn block_0x0020236c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2106224u32)?;
    emu.lw_no_count(11usize, 2usize, 16u32, 2106228u32)?;
    emu.lw_no_count(12usize, 2usize, 20u32, 2106232u32)?;
    emu.sw_no_count(10usize, 8usize, 0u32, 2106236u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2106240u32)?;
    emu.sw_no_count(12usize, 8usize, 8u32, 2106244u32)?;
    emu.add_memory_rw_events(6usize);
    emu.pc = 2106244u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202384));
}
#[inline]
pub fn block_0x00202384(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 76u32, 2106248u32)?;
    emu.lw_no_count(8usize, 2usize, 72u32, 2106252u32)?;
    emu.lw_no_count(9usize, 2usize, 68u32, 2106256u32)?;
    emu.lw_no_count(18usize, 2usize, 64u32, 2106260u32)?;
    emu.lw_no_count(19usize, 2usize, 60u32, 2106264u32)?;
    emu.lw_no_count(20usize, 2usize, 56u32, 2106268u32)?;
    emu.lw_no_count(21usize, 2usize, 52u32, 2106272u32)?;
    emu.lw_no_count(22usize, 2usize, 48u32, 2106276u32)?;
    emu.lw_no_count(23usize, 2usize, 44u32, 2106280u32)?;
    emu.adi_no_count(2usize, 2usize, 80u32, 2106284u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106288u32;
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
pub fn block_0x002023b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2106288u32, 24576u32, 2106292u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2106296u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(72u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002023b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2106300u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 812u32, 2106304u32);
    emu.adi_no_count(12usize, 0usize, 17u32, 2106308u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2106312u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2106316u32);
    emu.apc_no_count(6usize, 2106316u32, 61440u32, 2106320u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2106324u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966224u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002023d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2106328u32)?;
    emu.lw_no_count(11usize, 10usize, 0u32, 2106332u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2106336u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2106376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202408));
    } else {
        emu.pc = 2106340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002023e4));
    }
}
#[inline(always)]
pub fn block_0x002023e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 10usize, 4u32, 2106344u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2106348u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2106376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202408));
    } else {
        emu.pc = 2106352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002023f0));
    }
}
#[inline(always)]
pub fn block_0x002023f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 8u32, 2106356u32)?;
    emu.lw_no_count(11usize, 10usize, 4u32, 2106360u32)?;
    emu.lw_no_count(6usize, 11usize, 0u32, 2106364u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2106376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202408));
    } else {
        emu.pc = 2106368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202400));
    }
}
#[inline(always)]
pub fn block_0x00202400(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2106372u32)?;
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2106376u32;
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
pub fn block_0x00202408(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106380u32;
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
pub fn block_0x0020240c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2106384u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2106388u32)?;
    emu.lw_no_count(12usize, 10usize, 0u32, 2106392u32)?;
    emu.lw_no_count(13usize, 12usize, 0u32, 2106396u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2106400u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2106432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202440));
    } else {
        emu.pc = 2106404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202424));
    }
}
#[inline(always)]
pub fn block_0x00202424(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 8u32, 2106408u32);
    emu.sli_no_count(11usize, 11usize, 2u32, 2106412u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2106416u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 668u32, 2106420u32);
    emu.adr_no_count(11usize, 11usize, 13usize, 2106424u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2106428u32)?;
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2106432u32;
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
pub fn block_0x00202440(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2106436u32;
    emu.update_insn_clock();
    emu.xrr_no_count(11usize, 13usize, 11usize, 2106440u32);
    emu.sli_no_count(11usize, 11usize, 2u32, 2106444u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2106448u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 668u32, 2106452u32);
    emu.adr_no_count(11usize, 11usize, 13usize, 2106456u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2106460u32)?;
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2106464u32;
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
pub fn block_0x00202460(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 4u32, 2106468u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2106472u32)?;
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2106476u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 848u32, 2106480u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2106484u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 832u32, 2106488u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2106492u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2106496u32);
    emu.apc_no_count(1usize, 2106496u32, 61440u32, 2106500u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106504u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966560u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202488(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2106508u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2106512u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106516u32;
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
pub fn block_0x00202494(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 4u32, 2106520u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2106524u32)?;
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2106528u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 960u32, 2106532u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2106536u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 944u32, 2106540u32);
    emu.adi_no_count(12usize, 0usize, 18u32, 2106544u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2106548u32);
    emu.apc_no_count(1usize, 2106548u32, 61440u32, 2106552u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106556u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966508u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002024bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2106560u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2106564u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106568u32;
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
pub fn block_0x002024c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 4u32, 2106572u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2106576u32)?;
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2106580u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 904u32, 2106584u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2106588u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 888u32, 2106592u32);
    emu.adi_no_count(12usize, 0usize, 19u32, 2106596u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2106600u32);
    emu.apc_no_count(1usize, 2106600u32, 61440u32, 2106604u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106608u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966456u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002024f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2106612u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2106616u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106620u32;
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
pub fn block_0x002024fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2106624u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 923u32, 2106628u32);
    emu.adi_no_count(12usize, 0usize, 19u32, 2106632u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2106636u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2106640u32);
    emu.apc_no_count(6usize, 2106640u32, 61440u32, 2106644u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2106648u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965900u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202518(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2106652u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1013u32, 2106656u32);
    emu.adi_no_count(12usize, 0usize, 22u32, 2106660u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2106664u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2106668u32);
    emu.apc_no_count(6usize, 2106668u32, 61440u32, 2106672u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2106676u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965872u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00202534(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 4u32, 2106680u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2106684u32)?;
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2106688u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 868u32, 2106692u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2106696u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 852u32, 2106700u32);
    emu.adi_no_count(12usize, 0usize, 19u32, 2106704u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2106708u32);
    emu.apc_no_count(1usize, 2106708u32, 61440u32, 2106712u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106716u32;
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
pub fn block_0x0020255c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2106720u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2106724u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106728u32;
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
pub fn block_0x00202568(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2106732u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 978u32, 2106736u32);
    emu.adi_no_count(12usize, 0usize, 26u32, 2106740u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2106744u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2106748u32);
    emu.apc_no_count(6usize, 2106748u32, 61440u32, 2106752u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2106756u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x00202584(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2106760u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1004u32, 2106764u32);
    emu.adi_no_count(12usize, 0usize, 9u32, 2106768u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2106772u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2106776u32);
    emu.apc_no_count(6usize, 2106776u32, 61440u32, 2106780u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2106784u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965764u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002025a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(12usize, 2usize, 8u32, 2106788u32)?;
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2106792u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1052u32, 2106796u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2106800u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1036u32, 2106804u32);
    emu.adi_no_count(12usize, 0usize, 6u32, 2106808u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2106812u32);
    emu.apc_no_count(1usize, 2106812u32, 61440u32, 2106816u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106820u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966244u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002025c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2106824u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2106828u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106832u32;
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
pub fn block_0x002025d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2106836u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2106840u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2106844u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2106848u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2106852u32)?;
    emu.adi_no_count(11usize, 10usize, 0u32, 2106856u32);
    emu.lw_no_count(12usize, 10usize, 4u32, 2106860u32)?;
    emu.lw_no_count(10usize, 10usize, 12u32, 2106864u32)?;
    emu.adi_no_count(13usize, 0usize, 1u32, 2106868u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2106924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020262c));
    } else {
        emu.pc = 2106872u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002025f8));
    }
}
#[inline(always)]
pub fn block_0x002025f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2106928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202630));
    } else {
        emu.pc = 2106876u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002025fc));
    }
}
#[inline(always)]
pub fn block_0x002025fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2106928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202630));
    } else {
        emu.pc = 2106880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202600));
    }
}
#[inline(always)]
pub fn block_0x00202600(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 0usize, 0u32, 2106884u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2106888u32);
    emu.adi_no_count(9usize, 0usize, 1u32, 2106892u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2106892u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020260c));
}
#[inline(always)]
pub fn block_0x0020260c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2106896u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2106900u32);
    emu.apc_no_count(1usize, 2106900u32, 4096u32, 2106904u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106908u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965876u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020261c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(8usize, 2usize, 4u32, 2106912u32)?;
    emu.sw_no_count(9usize, 2usize, 8u32, 2106916u32)?;
    emu.sw_no_count(8usize, 2usize, 12u32, 2106920u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2106924u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2106940u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020263c));
}
#[inline(always)]
pub fn block_0x0020262c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2107024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202690));
    } else {
        emu.pc = 2106928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202630));
    }
}
#[inline(always)]
pub fn block_0x00202630(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 4u32, 2106932u32);
    emu.apc_no_count(1usize, 2106932u32, 36864u32, 2106936u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106940u32;
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
pub fn block_0x0020263c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2106940u32, 0u32, 2106944u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106948u32;
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
#[inline(always)]
pub fn block_0x00202644(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2106952u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 560u32, 2106956u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2106960u32);
    emu.adi_no_count(12usize, 0usize, 12u32, 2106964u32);
    emu.apc_no_count(1usize, 2106964u32, 4096u32, 2106968u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106972u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966876u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020265c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2107060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002026b4));
    } else {
        emu.pc = 2106976u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202660));
    }
}
#[inline]
pub fn block_0x00202660(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 4u32, 2106980u32)?;
    emu.lw_no_count(12usize, 2usize, 8u32, 2106984u32)?;
    emu.lw_no_count(13usize, 2usize, 12u32, 2106988u32)?;
    emu.sw_no_count(11usize, 10usize, 0u32, 2106992u32)?;
    emu.sw_no_count(12usize, 10usize, 4u32, 2106996u32)?;
    emu.sw_no_count(13usize, 10usize, 8u32, 2107000u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2107004u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2107008u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2107012u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2107016u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2107020u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107024u32;
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
pub fn block_0x00202690(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 11usize, 0u32, 2107028u32)?;
    emu.lw_no_count(8usize, 10usize, 4u32, 2107032u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(8usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2107076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002026c4));
    } else {
        emu.pc = 2107036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020269c));
    }
}
#[inline(always)]
pub fn block_0x0020269c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2107040u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2107040u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002026a0));
}
#[inline(always)]
pub fn block_0x002026a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2107044u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1060u32, 2107048u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2107052u32);
    emu.apc_no_count(1usize, 2107052u32, 36864u32, 2107056u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107060u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966816u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002026b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2107064u32);
    emu.adi_no_count(11usize, 0usize, 12u32, 2107068u32);
    emu.apc_no_count(1usize, 2107068u32, 36864u32, 2107072u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107076u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966824u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002026c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 10usize, 0u32, 2107080u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2107132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002026fc));
    } else {
        emu.pc = 2107084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002026cc));
    }
}
#[inline(always)]
pub fn block_0x002026cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2107084u32, 0u32, 2107088u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107092u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(76u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002026d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2107096u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 560u32, 2107100u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2107104u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2107108u32);
    emu.apc_no_count(1usize, 2107108u32, 4096u32, 2107112u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107116u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966732u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002026ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2107120u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2107124u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a != b {
        emu.pc = 2107136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202700));
    } else {
        emu.pc = 2107128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002026f8));
    }
}
#[inline(always)]
pub fn block_0x002026f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2107132u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2107040u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002026a0));
}
#[inline(always)]
pub fn block_0x002026fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 1u32, 2107136u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2107136u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202700));
}
#[inline(always)]
pub fn block_0x00202700(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 18usize, 0u32, 2107140u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2107144u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2106892u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020260c));
}
#[inline(always)]
pub fn block_0x00202708(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2107144u32, 28672u32, 2107148u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2107152u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(204u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202710(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2107156u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107160u32;
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
pub fn block_0x00202718(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107164u32;
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
pub fn block_0x0020271c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2107168u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2107172u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2107176u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2107180u32)?;
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2107184u32;
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
pub fn block_0x00202730(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(9usize, 11usize, 12u32, 2107188u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2107192u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2107196u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2107200u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(9usize);
    let return_addr = 2107204u32;
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
pub fn block_0x00202744(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2107208u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2107212u32)?;
    emu.lw_no_count(12usize, 2usize, 0u32, 2107216u32)?;
    emu.lw_no_count(13usize, 2usize, 4u32, 2107220u32)?;
    let a = 0u32.wrapping_add(3112902656u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2107224u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966129u32, 2107228u32);
    emu.xrr_no_count(11usize, 11usize, 14usize, 2107232u32);
    let a = 0u32.wrapping_add(1676365824u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2107236u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 44u32, 2107240u32);
    emu.xrr_no_count(13usize, 13usize, 14usize, 2107244u32);
    let a = 0u32.wrapping_add(1470513152u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2107248u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 376u32, 2107252u32);
    emu.xrr_no_count(10usize, 10usize, 14usize, 2107256u32);
    let a = 0u32.wrapping_add(3603652608u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2107260u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966637u32, 2107264u32);
    emu.xrr_no_count(12usize, 12usize, 14usize, 2107268u32);
    emu.orr_no_count(11usize, 13usize, 11usize, 2107272u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2107276u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2107280u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2107308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002027ac));
    } else {
        emu.pc = 2107284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202794));
    }
}
#[inline(always)]
pub fn block_0x00202794(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 4u32, 2107288u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2107292u32);
    emu.adr_no_count(11usize, 8usize, 11usize, 2107296u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2107300u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2107420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020281c));
    } else {
        emu.pc = 2107304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002027a8));
    }
}
#[inline(always)]
pub fn block_0x002027a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2107308u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2107440u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202830));
}
#[inline(always)]
pub fn block_0x002027ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2107312u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2107316u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(9usize);
    let return_addr = 2107320u32;
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
pub fn block_0x002027b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2107324u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2107328u32)?;
    emu.lw_no_count(12usize, 2usize, 0u32, 2107332u32)?;
    emu.lw_no_count(13usize, 2usize, 4u32, 2107336u32)?;
    let a = 0u32.wrapping_add(2330587136u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2107340u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 428u32, 2107344u32);
    emu.xrr_no_count(11usize, 11usize, 14usize, 2107348u32);
    let a = 0u32.wrapping_add(2965131264u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2107352u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965685u32, 2107356u32);
    emu.xrr_no_count(13usize, 13usize, 14usize, 2107360u32);
    let a = 0u32.wrapping_add(4112453632u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2107364u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1473u32, 2107368u32);
    emu.xrr_no_count(10usize, 10usize, 14usize, 2107372u32);
    let a = 0u32.wrapping_add(2020298752u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2107376u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965580u32, 2107380u32);
    emu.xrr_no_count(12usize, 12usize, 14usize, 2107384u32);
    emu.orr_no_count(11usize, 13usize, 11usize, 2107388u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2107392u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2107396u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2107420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020281c));
    } else {
        emu.pc = 2107400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202808));
    }
}
#[inline(always)]
pub fn block_0x00202808(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 4u32, 2107404u32);
    emu.adi_no_count(11usize, 0usize, 8u32, 2107408u32);
    emu.adr_no_count(11usize, 8usize, 11usize, 2107412u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2107416u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2107440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202830));
    } else {
        emu.pc = 2107420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020281c));
    }
}
#[inline(always)]
pub fn block_0x0020281c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2107424u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2107428u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2107432u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2107436u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107440u32;
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
pub fn block_0x00202830(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2107444u32)?;
    emu.apc_no_count(1usize, 2107444u32, 0u32, 2107448u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107452u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1820u32);
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
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2107456u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2107460u32)?;
    emu.lw_no_count(11usize, 11usize, 20u32, 2107464u32)?;
    emu.apc_no_count(1usize, 2107464u32, 0u32, 2107468u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107472u32;
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
pub fn block_0x00202850(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2107472u32, 32768u32, 2107476u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107480u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(160u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202858(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2107484u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2107488u32)?;
    emu.apc_no_count(1usize, 2107488u32, 4096u32, 2107492u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107496u32;
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
pub fn block_0x00202868(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2107500u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2107504u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107508u32;
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
pub fn block_0x00202874(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967088u32, 2107512u32);
    emu.sw_no_count(1usize, 2usize, 204u32, 2107516u32)?;
    emu.sw_no_count(8usize, 2usize, 200u32, 2107520u32)?;
    emu.sw_no_count(9usize, 2usize, 196u32, 2107524u32)?;
    emu.sw_no_count(18usize, 2usize, 192u32, 2107528u32)?;
    emu.sw_no_count(19usize, 2usize, 188u32, 2107532u32)?;
    emu.sw_no_count(20usize, 2usize, 184u32, 2107536u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2107540u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2107544u32;
    emu.update_insn_clock();
    emu.lw_no_count(11usize, 10usize, 4294966560u32, 2107548u32)?;
    emu.sw_no_count(0usize, 10usize, 4294966564u32, 2107552u32)?;
    emu.ani_no_count(11usize, 11usize, 1u32, 2107556u32);
    emu.sw_no_count(0usize, 10usize, 4294966560u32, 2107560u32)?;
    emu.add_memory_rw_events(13usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2108876u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202dcc));
    } else {
        emu.pc = 2107564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002028ac));
    }
}
#[inline(always)]
pub fn block_0x002028ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2107568u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966568u32, 2107572u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2107576u32);
    emu.adi_no_count(12usize, 0usize, 112u32, 2107580u32);
    emu.apc_no_count(1usize, 2107580u32, 0u32, 2107584u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107588u32;
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
pub fn block_0x002028c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 35u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 2usize, 48u32, 2107592u32);
    emu.lbu_no_count(18usize, 2usize, 112u32, 2107596u32);
    emu.lw_no_count(10usize, 2usize, 40u32, 2107600u32)?;
    emu.lw_no_count(11usize, 2usize, 44u32, 2107604u32)?;
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2107608u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 0usize, 128u32, 2107612u32);
    emu.adi_no_count(12usize, 12usize, 4294967040u32, 2107616u32);
    emu.sli_no_count(14usize, 11usize, 9u32, 2107620u32);
    emu.sli_no_count(15usize, 10usize, 9u32, 2107624u32);
    emu.sli_no_count(16usize, 18usize, 3u32, 2107628u32);
    emu.sli_no_count(17usize, 10usize, 1u32, 2107632u32);
    emu.sli_no_count(11usize, 11usize, 1u32, 2107636u32);
    emu.orr_no_count(16usize, 15usize, 16usize, 2107640u32);
    emu.anr_no_count(17usize, 17usize, 12usize, 2107644u32);
    emu.sri_no_count(15usize, 15usize, 24u32, 2107648u32);
    emu.anr_no_count(11usize, 11usize, 12usize, 2107652u32);
    emu.orr_no_count(15usize, 17usize, 15usize, 2107656u32);
    emu.sri_no_count(17usize, 14usize, 24u32, 2107660u32);
    emu.orr_no_count(11usize, 11usize, 17usize, 2107664u32);
    emu.adi_no_count(17usize, 0usize, 63u32, 2107668u32);
    emu.sri_no_count(5usize, 10usize, 23u32, 2107672u32);
    emu.orr_no_count(10usize, 14usize, 5usize, 2107676u32);
    emu.anr_no_count(14usize, 16usize, 12usize, 2107680u32);
    emu.anr_no_count(12usize, 10usize, 12usize, 2107684u32);
    emu.sli_no_count(10usize, 18usize, 27u32, 2107688u32);
    emu.orr_no_count(15usize, 10usize, 15usize, 2107692u32);
    emu.adr_no_count(10usize, 9usize, 18usize, 2107696u32);
    emu.sli_no_count(5usize, 5usize, 24u32, 2107700u32);
    emu.sli_no_count(14usize, 14usize, 8u32, 2107704u32);
    emu.sli_no_count(12usize, 12usize, 8u32, 2107708u32);
    emu.orr_no_count(11usize, 5usize, 11usize, 2107712u32);
    emu.orr_no_count(20usize, 15usize, 14usize, 2107716u32);
    emu.orr_no_count(19usize, 11usize, 12usize, 2107720u32);
    emu.sb_no_count(13usize, 10usize, 0u32, 2107724u32);
    emu.add_memory_rw_events(34usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2107760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202970));
    } else {
        emu.pc = 2107728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202950));
    }
}
#[inline(always)]
pub fn block_0x00202950(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 10usize, 1u32, 2107732u32);
    emu.xri_no_count(12usize, 18usize, 63u32, 2107736u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2107740u32);
    emu.apc_no_count(1usize, 2107740u32, 0u32, 2107744u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107748u32;
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
pub fn block_0x00202964(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(10usize, 18usize, 56u32, 2107752u32);
    emu.adi_no_count(11usize, 0usize, 7u32, 2107756u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2107872u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002029e0));
    } else {
        emu.pc = 2107760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202970));
    }
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
    emu.adi_no_count(10usize, 2usize, 8u32, 2107764u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2107768u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2107772u32);
    emu.apc_no_count(1usize, 2107772u32, 4096u32, 2107776u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107780u32;
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
pub fn block_0x00202984(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 120u32, 2107784u32);
    emu.adi_no_count(12usize, 0usize, 56u32, 2107788u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2107792u32);
    emu.apc_no_count(1usize, 2107792u32, 0u32, 2107796u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107800u32;
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
pub fn block_0x00202998(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 20usize, 24u32, 2107804u32);
    emu.sri_no_count(11usize, 20usize, 16u32, 2107808u32);
    emu.sri_no_count(12usize, 20usize, 8u32, 2107812u32);
    emu.sri_no_count(13usize, 19usize, 24u32, 2107816u32);
    emu.sri_no_count(14usize, 19usize, 16u32, 2107820u32);
    emu.sb_no_count(20usize, 2usize, 180u32, 2107824u32);
    emu.sb_no_count(12usize, 2usize, 181u32, 2107828u32);
    emu.sb_no_count(11usize, 2usize, 182u32, 2107832u32);
    emu.sb_no_count(10usize, 2usize, 183u32, 2107836u32);
    emu.sri_no_count(10usize, 19usize, 8u32, 2107840u32);
    emu.sb_no_count(19usize, 2usize, 176u32, 2107844u32);
    emu.sb_no_count(10usize, 2usize, 177u32, 2107848u32);
    emu.sb_no_count(14usize, 2usize, 178u32, 2107852u32);
    emu.sb_no_count(13usize, 2usize, 179u32, 2107856u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2107860u32);
    emu.adi_no_count(11usize, 2usize, 120u32, 2107864u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2107868u32);
    emu.add_memory_rw_events(18usize);
    let return_addr = 2107872u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2107892u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002029f4));
}
#[inline(always)]
pub fn block_0x002029e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(19usize, 2usize, 104u32, 2107876u32)?;
    emu.sw_no_count(20usize, 2usize, 108u32, 2107880u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2107884u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2107888u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2107892u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2107892u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002029f4));
}
#[inline(always)]
pub fn block_0x002029f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2107892u32, 4096u32, 2107896u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107900u32;
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
pub fn block_0x002029fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 85u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2107904u32);
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2107908u32;
    emu.update_insn_clock();
    emu.lw_no_count(11usize, 2usize, 8u32, 2107912u32)?;
    emu.lw_no_count(13usize, 2usize, 12u32, 2107916u32)?;
    emu.lw_no_count(15usize, 2usize, 16u32, 2107920u32)?;
    emu.lw_no_count(16usize, 2usize, 20u32, 2107924u32)?;
    emu.lw_no_count(17usize, 2usize, 24u32, 2107928u32)?;
    emu.lw_no_count(5usize, 2usize, 28u32, 2107932u32)?;
    emu.lw_no_count(6usize, 2usize, 32u32, 2107936u32)?;
    emu.lw_no_count(14usize, 2usize, 36u32, 2107940u32)?;
    emu.adi_no_count(12usize, 12usize, 4294967040u32, 2107944u32);
    emu.sri_no_count(7usize, 11usize, 8u32, 2107948u32);
    emu.sri_no_count(28usize, 11usize, 24u32, 2107952u32);
    emu.anr_no_count(29usize, 11usize, 12usize, 2107956u32);
    emu.sli_no_count(11usize, 11usize, 24u32, 2107960u32);
    emu.sri_no_count(30usize, 13usize, 8u32, 2107964u32);
    emu.sri_no_count(31usize, 13usize, 24u32, 2107968u32);
    emu.anr_no_count(9usize, 13usize, 12usize, 2107972u32);
    emu.sli_no_count(13usize, 13usize, 24u32, 2107976u32);
    emu.sri_no_count(18usize, 15usize, 8u32, 2107980u32);
    emu.sri_no_count(19usize, 15usize, 24u32, 2107984u32);
    emu.anr_no_count(20usize, 15usize, 12usize, 2107988u32);
    emu.sli_no_count(15usize, 15usize, 24u32, 2107992u32);
    emu.anr_no_count(7usize, 7usize, 12usize, 2107996u32);
    emu.orr_no_count(7usize, 7usize, 28usize, 2108000u32);
    emu.sri_no_count(28usize, 16usize, 8u32, 2108004u32);
    emu.sli_no_count(29usize, 29usize, 8u32, 2108008u32);
    emu.orr_no_count(11usize, 11usize, 29usize, 2108012u32);
    emu.sri_no_count(29usize, 16usize, 24u32, 2108016u32);
    emu.anr_no_count(30usize, 30usize, 12usize, 2108020u32);
    emu.orr_no_count(30usize, 30usize, 31usize, 2108024u32);
    emu.anr_no_count(31usize, 16usize, 12usize, 2108028u32);
    emu.sli_no_count(16usize, 16usize, 24u32, 2108032u32);
    emu.sli_no_count(9usize, 9usize, 8u32, 2108036u32);
    emu.orr_no_count(13usize, 13usize, 9usize, 2108040u32);
    emu.sri_no_count(9usize, 17usize, 8u32, 2108044u32);
    emu.anr_no_count(18usize, 18usize, 12usize, 2108048u32);
    emu.orr_no_count(18usize, 18usize, 19usize, 2108052u32);
    emu.sri_no_count(19usize, 17usize, 24u32, 2108056u32);
    emu.sli_no_count(20usize, 20usize, 8u32, 2108060u32);
    emu.orr_no_count(15usize, 15usize, 20usize, 2108064u32);
    emu.anr_no_count(20usize, 17usize, 12usize, 2108068u32);
    emu.sli_no_count(17usize, 17usize, 24u32, 2108072u32);
    emu.anr_no_count(28usize, 28usize, 12usize, 2108076u32);
    emu.orr_no_count(28usize, 28usize, 29usize, 2108080u32);
    emu.sri_no_count(29usize, 5usize, 8u32, 2108084u32);
    emu.sli_no_count(31usize, 31usize, 8u32, 2108088u32);
    emu.orr_no_count(16usize, 16usize, 31usize, 2108092u32);
    emu.sri_no_count(31usize, 5usize, 24u32, 2108096u32);
    emu.anr_no_count(9usize, 9usize, 12usize, 2108100u32);
    emu.orr_no_count(9usize, 9usize, 19usize, 2108104u32);
    emu.anr_no_count(19usize, 5usize, 12usize, 2108108u32);
    emu.sli_no_count(5usize, 5usize, 24u32, 2108112u32);
    emu.sli_no_count(20usize, 20usize, 8u32, 2108116u32);
    emu.orr_no_count(17usize, 17usize, 20usize, 2108120u32);
    emu.sri_no_count(20usize, 6usize, 8u32, 2108124u32);
    emu.anr_no_count(29usize, 29usize, 12usize, 2108128u32);
    emu.orr_no_count(29usize, 29usize, 31usize, 2108132u32);
    emu.sri_no_count(31usize, 6usize, 24u32, 2108136u32);
    emu.sli_no_count(19usize, 19usize, 8u32, 2108140u32);
    emu.orr_no_count(5usize, 5usize, 19usize, 2108144u32);
    emu.anr_no_count(19usize, 6usize, 12usize, 2108148u32);
    emu.sli_no_count(6usize, 6usize, 24u32, 2108152u32);
    emu.anr_no_count(20usize, 20usize, 12usize, 2108156u32);
    emu.orr_no_count(31usize, 20usize, 31usize, 2108160u32);
    emu.sri_no_count(20usize, 14usize, 8u32, 2108164u32);
    emu.sli_no_count(19usize, 19usize, 8u32, 2108168u32);
    emu.orr_no_count(6usize, 6usize, 19usize, 2108172u32);
    emu.sri_no_count(19usize, 14usize, 24u32, 2108176u32);
    emu.anr_no_count(20usize, 20usize, 12usize, 2108180u32);
    emu.orr_no_count(19usize, 20usize, 19usize, 2108184u32);
    emu.anr_no_count(12usize, 14usize, 12usize, 2108188u32);
    emu.sli_no_count(14usize, 14usize, 24u32, 2108192u32);
    emu.sli_no_count(12usize, 12usize, 8u32, 2108196u32);
    emu.orr_no_count(20usize, 14usize, 12usize, 2108200u32);
    emu.orr_no_count(11usize, 11usize, 7usize, 2108204u32);
    emu.orr_no_count(12usize, 13usize, 30usize, 2108208u32);
    emu.orr_no_count(13usize, 15usize, 18usize, 2108212u32);
    emu.orr_no_count(14usize, 16usize, 28usize, 2108216u32);
    emu.orr_no_count(15usize, 17usize, 9usize, 2108220u32);
    emu.orr_no_count(16usize, 5usize, 29usize, 2108224u32);
    emu.orr_no_count(17usize, 6usize, 31usize, 2108228u32);
    emu.adi_no_count(5usize, 0usize, 16u32, 2108232u32);
    emu.orr_no_count(6usize, 20usize, 19usize, 2108236u32);
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
pub fn block_0x00202b50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 16u32, 2108244u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2108248u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2108252u32);
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
pub fn block_0x00202b60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 16u32, 2108260u32);
    emu.adi_no_count(10usize, 0usize, 2u32, 2108264u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2108268u32);
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
pub fn block_0x00202b70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 16u32, 2108276u32);
    emu.adi_no_count(10usize, 0usize, 3u32, 2108280u32);
    emu.adi_no_count(11usize, 14usize, 0u32, 2108284u32);
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
pub fn block_0x00202b80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 16u32, 2108292u32);
    emu.adi_no_count(10usize, 0usize, 4u32, 2108296u32);
    emu.adi_no_count(11usize, 15usize, 0u32, 2108300u32);
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
pub fn block_0x00202b90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 16u32, 2108308u32);
    emu.adi_no_count(10usize, 0usize, 5u32, 2108312u32);
    emu.adi_no_count(11usize, 16usize, 0u32, 2108316u32);
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
pub fn block_0x00202ba0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 16u32, 2108324u32);
    emu.adi_no_count(10usize, 0usize, 6u32, 2108328u32);
    emu.adi_no_count(11usize, 17usize, 0u32, 2108332u32);
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
pub fn block_0x00202bb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 16u32, 2108340u32);
    emu.adi_no_count(10usize, 0usize, 7u32, 2108344u32);
    emu.adi_no_count(11usize, 6usize, 0u32, 2108348u32);
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
pub fn block_0x00202bc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2108356u32;
    emu.update_insn_clock();
    emu.lw_no_count(10usize, 10usize, 4294966520u32, 2108360u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2108892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202ddc));
    } else {
        emu.pc = 2108364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202bcc));
    }
}
#[inline]
pub fn block_0x00202bcc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2108368u32);
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2108372u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966520u32, 2108376u32);
    let a = 0u32.wrapping_add(2164260864u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2108380u32;
    emu.update_insn_clock();
    emu.lw_no_count(11usize, 14usize, 4u32, 2108384u32)?;
    let a = 0u32.wrapping_add(2130706432u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2108388u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1u32, 2108392u32);
    emu.adi_no_count(12usize, 12usize, 1u32, 2108396u32);
    emu.mul_no_count(15usize, 11usize, 13usize, 2108400u32);
    emu.mulhu_no_count(16usize, 15usize, 12usize, 2108404u32);
    emu.mul_no_count(15usize, 15usize, 12usize, 2108408u32);
    emu.sltru_no_count(11usize, 11usize, 15usize, 2108412u32);
    emu.sltru_no_count(15usize, 0usize, 16usize, 2108416u32);
    emu.adr_no_count(16usize, 16usize, 11usize, 2108420u32);
    emu.orr_no_count(11usize, 11usize, 15usize, 2108424u32);
    emu.sbr_no_count(11usize, 0usize, 11usize, 2108428u32);
    emu.anr_no_count(11usize, 11usize, 12usize, 2108432u32);
    emu.sbr_no_count(11usize, 11usize, 16usize, 2108436u32);
    emu.adi_no_count(5usize, 0usize, 26u32, 2108440u32);
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
pub fn block_0x00202c1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 14usize, 8u32, 2108448u32)?;
    emu.mul_no_count(11usize, 10usize, 13usize, 2108452u32);
    emu.mulhu_no_count(15usize, 11usize, 12usize, 2108456u32);
    emu.mul_no_count(11usize, 11usize, 12usize, 2108460u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2108464u32);
    emu.sltru_no_count(11usize, 0usize, 15usize, 2108468u32);
    emu.adr_no_count(15usize, 15usize, 10usize, 2108472u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2108476u32);
    emu.adi_no_count(5usize, 0usize, 26u32, 2108480u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2108484u32);
    emu.anr_no_count(10usize, 10usize, 12usize, 2108488u32);
    emu.sbr_no_count(11usize, 10usize, 15usize, 2108492u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2108496u32);
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
pub fn block_0x00202c54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 14usize, 12u32, 2108504u32)?;
    emu.mul_no_count(11usize, 10usize, 13usize, 2108508u32);
    emu.mulhu_no_count(15usize, 11usize, 12usize, 2108512u32);
    emu.mul_no_count(11usize, 11usize, 12usize, 2108516u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2108520u32);
    emu.sltru_no_count(11usize, 0usize, 15usize, 2108524u32);
    emu.adr_no_count(15usize, 15usize, 10usize, 2108528u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2108532u32);
    emu.adi_no_count(5usize, 0usize, 26u32, 2108536u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2108540u32);
    emu.anr_no_count(10usize, 10usize, 12usize, 2108544u32);
    emu.sbr_no_count(11usize, 10usize, 15usize, 2108548u32);
    emu.adi_no_count(10usize, 0usize, 2u32, 2108552u32);
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
pub fn block_0x00202c8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 14usize, 16u32, 2108560u32)?;
    emu.mul_no_count(11usize, 10usize, 13usize, 2108564u32);
    emu.mulhu_no_count(15usize, 11usize, 12usize, 2108568u32);
    emu.mul_no_count(11usize, 11usize, 12usize, 2108572u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2108576u32);
    emu.sltru_no_count(11usize, 0usize, 15usize, 2108580u32);
    emu.adr_no_count(15usize, 15usize, 10usize, 2108584u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2108588u32);
    emu.adi_no_count(5usize, 0usize, 26u32, 2108592u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2108596u32);
    emu.anr_no_count(10usize, 10usize, 12usize, 2108600u32);
    emu.sbr_no_count(11usize, 10usize, 15usize, 2108604u32);
    emu.adi_no_count(10usize, 0usize, 3u32, 2108608u32);
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
pub fn block_0x00202cc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 14usize, 20u32, 2108616u32)?;
    emu.mul_no_count(11usize, 10usize, 13usize, 2108620u32);
    emu.mulhu_no_count(15usize, 11usize, 12usize, 2108624u32);
    emu.mul_no_count(11usize, 11usize, 12usize, 2108628u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2108632u32);
    emu.sltru_no_count(11usize, 0usize, 15usize, 2108636u32);
    emu.adr_no_count(15usize, 15usize, 10usize, 2108640u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2108644u32);
    emu.adi_no_count(5usize, 0usize, 26u32, 2108648u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2108652u32);
    emu.anr_no_count(10usize, 10usize, 12usize, 2108656u32);
    emu.sbr_no_count(11usize, 10usize, 15usize, 2108660u32);
    emu.adi_no_count(10usize, 0usize, 4u32, 2108664u32);
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
pub fn block_0x00202cfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 14usize, 24u32, 2108672u32)?;
    emu.mul_no_count(11usize, 10usize, 13usize, 2108676u32);
    emu.mulhu_no_count(15usize, 11usize, 12usize, 2108680u32);
    emu.mul_no_count(11usize, 11usize, 12usize, 2108684u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2108688u32);
    emu.sltru_no_count(11usize, 0usize, 15usize, 2108692u32);
    emu.adr_no_count(15usize, 15usize, 10usize, 2108696u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2108700u32);
    emu.adi_no_count(5usize, 0usize, 26u32, 2108704u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2108708u32);
    emu.anr_no_count(10usize, 10usize, 12usize, 2108712u32);
    emu.sbr_no_count(11usize, 10usize, 15usize, 2108716u32);
    emu.adi_no_count(10usize, 0usize, 5u32, 2108720u32);
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
pub fn block_0x00202d34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 14usize, 28u32, 2108728u32)?;
    emu.mul_no_count(11usize, 10usize, 13usize, 2108732u32);
    emu.mulhu_no_count(15usize, 11usize, 12usize, 2108736u32);
    emu.mul_no_count(11usize, 11usize, 12usize, 2108740u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2108744u32);
    emu.sltru_no_count(11usize, 0usize, 15usize, 2108748u32);
    emu.adr_no_count(15usize, 15usize, 10usize, 2108752u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2108756u32);
    emu.adi_no_count(5usize, 0usize, 26u32, 2108760u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2108764u32);
    emu.anr_no_count(10usize, 10usize, 12usize, 2108768u32);
    emu.sbr_no_count(11usize, 10usize, 15usize, 2108772u32);
    emu.adi_no_count(10usize, 0usize, 6u32, 2108776u32);
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
pub fn block_0x00202d6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 14usize, 32u32, 2108784u32)?;
    emu.adi_no_count(5usize, 0usize, 26u32, 2108788u32);
    emu.mul_no_count(11usize, 10usize, 13usize, 2108792u32);
    emu.mulhu_no_count(13usize, 11usize, 12usize, 2108796u32);
    emu.mul_no_count(11usize, 11usize, 12usize, 2108800u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2108804u32);
    emu.sltru_no_count(11usize, 0usize, 13usize, 2108808u32);
    emu.adr_no_count(13usize, 13usize, 10usize, 2108812u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2108816u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2108820u32);
    emu.anr_no_count(10usize, 10usize, 12usize, 2108824u32);
    emu.sbr_no_count(11usize, 10usize, 13usize, 2108828u32);
    emu.adi_no_count(10usize, 0usize, 7u32, 2108832u32);
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
pub fn block_0x00202da4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 0u32, 2108840u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2108844u32);
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
pub fn block_0x00202db0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2108852u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1168u32, 2108856u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2108860u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1240u32, 2108864u32);
    emu.adi_no_count(11usize, 0usize, 40u32, 2108868u32);
    emu.apc_no_count(1usize, 2108868u32, 36864u32, 2108872u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108876u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(2012u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202dcc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2108880u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1208u32, 2108884u32);
    emu.apc_no_count(1usize, 2108884u32, 45056u32, 2108888u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108892u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1432u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202ddc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2108896u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1224u32, 2108900u32);
    emu.apc_no_count(1usize, 2108900u32, 45056u32, 2108904u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2108908u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1416u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
