pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2179008u32;
pub const PC_MAX: u32 = 2180464u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 69usize] = [
        block_0x00213fc0,
        block_0x00214020,
        block_0x00214080,
        block_0x002140e0,
        block_0x00214104,
        block_0x00214118,
        block_0x0021411c,
        block_0x00214120,
        block_0x00214124,
        block_0x00214128,
        block_0x0021413c,
        block_0x00214150,
        block_0x00214168,
        block_0x00214180,
        block_0x00214184,
        block_0x002141a0,
        block_0x002141b0,
        block_0x002141c8,
        block_0x002141cc,
        block_0x002141d8,
        block_0x002141dc,
        block_0x002141e8,
        block_0x002141ec,
        block_0x00214200,
        block_0x00214214,
        block_0x0021422c,
        block_0x00214244,
        block_0x00214248,
        block_0x00214288,
        block_0x0021429c,
        block_0x002142ac,
        block_0x002142b4,
        block_0x002142c0,
        block_0x002142c4,
        block_0x002142d0,
        block_0x002142d8,
        block_0x002142ec,
        block_0x00214300,
        block_0x00214320,
        block_0x00214324,
        block_0x00214358,
        block_0x00214370,
        block_0x0021438c,
        block_0x002143a8,
        block_0x002143ac,
        block_0x002143c4,
        block_0x002143e0,
        block_0x002143fc,
        block_0x00214400,
        block_0x00214414,
        block_0x00214418,
        block_0x00214430,
        block_0x00214444,
        block_0x00214454,
        block_0x00214460,
        block_0x00214468,
        block_0x0021448c,
        block_0x002144a0,
        block_0x002144f8,
        block_0x002144fc,
        block_0x00214504,
        block_0x00214508,
        block_0x0021450c,
        block_0x00214520,
        block_0x00214538,
        block_0x0021454c,
        block_0x0021455c,
        block_0x00214568,
        block_0x00214570,
    ];
    const IDX: [u16; 365usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 3u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 4u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 5u16, 0u16, 0u16, 0u16, 0u16, 6u16, 7u16, 8u16, 9u16, 10u16,
        0u16, 0u16, 0u16, 0u16, 11u16, 0u16, 0u16, 0u16, 0u16, 12u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 13u16, 0u16, 0u16, 0u16, 0u16, 0u16, 14u16, 15u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 16u16, 0u16, 0u16, 0u16, 17u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        18u16, 19u16, 0u16, 0u16, 20u16, 21u16, 0u16, 0u16, 22u16, 23u16, 0u16, 0u16,
        0u16, 0u16, 24u16, 0u16, 0u16, 0u16, 0u16, 25u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        26u16, 0u16, 0u16, 0u16, 0u16, 0u16, 27u16, 28u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 29u16, 0u16, 0u16,
        0u16, 0u16, 30u16, 0u16, 0u16, 0u16, 31u16, 0u16, 32u16, 0u16, 0u16, 33u16,
        34u16, 0u16, 0u16, 35u16, 0u16, 36u16, 0u16, 0u16, 0u16, 0u16, 37u16, 0u16, 0u16,
        0u16, 0u16, 38u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 39u16, 40u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 41u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 42u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 43u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 44u16, 45u16, 0u16, 0u16, 0u16, 0u16, 0u16, 46u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 47u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        48u16, 49u16, 0u16, 0u16, 0u16, 0u16, 50u16, 51u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        52u16, 0u16, 0u16, 0u16, 0u16, 53u16, 0u16, 0u16, 0u16, 54u16, 0u16, 0u16, 55u16,
        0u16, 56u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 57u16, 0u16, 0u16,
        0u16, 0u16, 58u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 59u16, 60u16,
        0u16, 61u16, 62u16, 63u16, 0u16, 0u16, 0u16, 0u16, 64u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 65u16, 0u16, 0u16, 0u16, 0u16, 66u16, 0u16, 0u16, 0u16, 67u16, 0u16, 0u16,
        68u16, 0u16, 69u16,
    ];
    if pc < 2179008u32 || pc > 2180464u32 {
        return None;
    }
    let word_offset = ((pc - 2179008u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline]
pub fn block_0x00213fc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2179012u32);
    emu.sw_no_count(10usize, 2usize, 0u32, 2179016u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2179020u32)?;
    emu.adi_no_count(10usize, 2usize, 0u32, 2179024u32);
    let a = 0u32.wrapping_add(2146304u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2179028u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966704u32, 2179032u32);
    emu.adi_no_count(13usize, 2usize, 4u32, 2179036u32);
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2179040u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966324u32, 2179044u32);
    emu.adi_no_count(15usize, 0usize, 2u32, 2179048u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2179052u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2179056u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2179060u32)?;
    emu.sw_no_count(13usize, 2usize, 40u32, 2179064u32)?;
    emu.sw_no_count(11usize, 2usize, 44u32, 2179068u32)?;
    emu.adi_no_count(10usize, 2usize, 32u32, 2179072u32);
    emu.sw_no_count(14usize, 2usize, 8u32, 2179076u32)?;
    emu.sw_no_count(15usize, 2usize, 12u32, 2179080u32)?;
    emu.sw_no_count(10usize, 2usize, 16u32, 2179084u32)?;
    emu.sw_no_count(15usize, 2usize, 20u32, 2179088u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2179092u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2179096u32);
    emu.apc_no_count(1usize, 2179096u32, 4294934528u32, 2179100u32);
    emu.add_memory_rw_events(24usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2179104u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1384u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00214020(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2179108u32);
    emu.sw_no_count(10usize, 2usize, 0u32, 2179112u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2179116u32)?;
    emu.adi_no_count(10usize, 2usize, 0u32, 2179120u32);
    let a = 0u32.wrapping_add(2146304u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2179124u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966704u32, 2179128u32);
    emu.adi_no_count(13usize, 2usize, 4u32, 2179132u32);
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2179136u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966340u32, 2179140u32);
    emu.adi_no_count(15usize, 0usize, 2u32, 2179144u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2179148u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2179152u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2179156u32)?;
    emu.sw_no_count(13usize, 2usize, 40u32, 2179160u32)?;
    emu.sw_no_count(11usize, 2usize, 44u32, 2179164u32)?;
    emu.adi_no_count(10usize, 2usize, 32u32, 2179168u32);
    emu.sw_no_count(14usize, 2usize, 8u32, 2179172u32)?;
    emu.sw_no_count(15usize, 2usize, 12u32, 2179176u32)?;
    emu.sw_no_count(10usize, 2usize, 16u32, 2179180u32)?;
    emu.sw_no_count(15usize, 2usize, 20u32, 2179184u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2179188u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2179192u32);
    emu.apc_no_count(1usize, 2179192u32, 4294934528u32, 2179196u32);
    emu.add_memory_rw_events(24usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2179200u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1288u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00214080(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2179204u32);
    emu.sw_no_count(10usize, 2usize, 0u32, 2179208u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2179212u32)?;
    emu.adi_no_count(10usize, 2usize, 0u32, 2179216u32);
    let a = 0u32.wrapping_add(2146304u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2179220u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966704u32, 2179224u32);
    emu.adi_no_count(13usize, 2usize, 4u32, 2179228u32);
    let a = 0u32.wrapping_add(2199552u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2179232u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966392u32, 2179236u32);
    emu.adi_no_count(15usize, 0usize, 2u32, 2179240u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2179244u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2179248u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2179252u32)?;
    emu.sw_no_count(13usize, 2usize, 40u32, 2179256u32)?;
    emu.sw_no_count(11usize, 2usize, 44u32, 2179260u32)?;
    emu.adi_no_count(10usize, 2usize, 32u32, 2179264u32);
    emu.sw_no_count(14usize, 2usize, 8u32, 2179268u32)?;
    emu.sw_no_count(15usize, 2usize, 12u32, 2179272u32)?;
    emu.sw_no_count(10usize, 2usize, 16u32, 2179276u32)?;
    emu.sw_no_count(15usize, 2usize, 20u32, 2179280u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2179284u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2179288u32);
    emu.apc_no_count(1usize, 2179288u32, 4294934528u32, 2179292u32);
    emu.add_memory_rw_events(24usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2179296u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1192u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002140e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2179300u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2179304u32)?;
    emu.adi_no_count(14usize, 13usize, 0u32, 2179308u32);
    emu.adi_no_count(13usize, 12usize, 0u32, 2179312u32);
    emu.adi_no_count(12usize, 11usize, 0u32, 2179316u32);
    emu.adi_no_count(11usize, 10usize, 0u32, 2179320u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2179324u32);
    emu.apc_no_count(1usize, 2179324u32, 0u32, 2179328u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2179332u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(28u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00214104(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2179336u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2179340u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2179344u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2179348u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2179352u32;
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
pub fn block_0x00214118(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2179528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002141c8));
    } else {
        emu.pc = 2179356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021411c));
    }
}
#[inline(always)]
pub fn block_0x0021411c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2179528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002141c8));
    } else {
        emu.pc = 2179360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00214120));
    }
}
#[inline(always)]
pub fn block_0x00214120(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2179840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00214300));
    } else {
        emu.pc = 2179364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00214124));
    }
}
#[inline(always)]
pub fn block_0x00214124(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a >= b {
        emu.pc = 2179872u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00214320));
    } else {
        emu.pc = 2179368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00214128));
    }
}
#[inline(always)]
pub fn block_0x00214128(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(17usize, 13usize, 16u32, 2179372u32);
    emu.sltru_no_count(15usize, 17usize, 12usize, 2179376u32);
    emu.xri_no_count(16usize, 15usize, 1u32, 2179380u32);
    emu.adi_no_count(15usize, 13usize, 0u32, 2179384u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a >= b {
        emu.pc = 2180012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002143ac));
    } else {
        emu.pc = 2179388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021413c));
    }
}
#[inline(always)]
pub fn block_0x0021413c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(16usize, 16usize, 4u32, 2179392u32);
    emu.sri_no_count(5usize, 15usize, 8u32, 2179396u32);
    emu.sltru_no_count(17usize, 5usize, 12usize, 2179400u32);
    emu.xri_no_count(17usize, 17usize, 1u32, 2179404u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a >= b {
        emu.pc = 2180036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002143c4));
    } else {
        emu.pc = 2179408u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00214150));
    }
}
#[inline(always)]
pub fn block_0x00214150(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(5usize, 17usize, 3u32, 2179412u32);
    emu.sri_no_count(17usize, 15usize, 4u32, 2179416u32);
    emu.orr_no_count(16usize, 5usize, 16usize, 2179420u32);
    emu.sltru_no_count(5usize, 17usize, 12usize, 2179424u32);
    emu.xri_no_count(5usize, 5usize, 1u32, 2179428u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a >= b {
        emu.pc = 2180064u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002143e0));
    } else {
        emu.pc = 2179432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00214168));
    }
}
#[inline(always)]
pub fn block_0x00214168(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(5usize, 5usize, 2u32, 2179436u32);
    emu.sri_no_count(17usize, 15usize, 2u32, 2179440u32);
    emu.orr_no_count(16usize, 16usize, 5usize, 2179444u32);
    emu.sltru_no_count(5usize, 17usize, 12usize, 2179448u32);
    emu.xri_no_count(5usize, 5usize, 1u32, 2179452u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a < b {
        emu.pc = 2179460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00214184));
    } else {
        emu.pc = 2179456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00214180));
    }
}
#[inline(always)]
pub fn block_0x00214180(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 17usize, 0u32, 2179460u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2179460u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00214184));
}
#[inline(always)]
pub fn block_0x00214184(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(5usize, 5usize, 1u32, 2179464u32);
    emu.sri_no_count(15usize, 15usize, 1u32, 2179468u32);
    emu.sltru_no_count(15usize, 15usize, 12usize, 2179472u32);
    emu.xri_no_count(15usize, 15usize, 1u32, 2179476u32);
    emu.orr_no_count(15usize, 5usize, 15usize, 2179480u32);
    emu.orr_no_count(15usize, 16usize, 15usize, 2179484u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a != b {
        emu.pc = 2180096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00214400));
    } else {
        emu.pc = 2179488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002141a0));
    }
}
#[inline(always)]
pub fn block_0x002141a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 31u32, 2179492u32);
    emu.adi_no_count(5usize, 16usize, 4294967264u32, 2179496u32);
    emu.slr_no_count(17usize, 13usize, 16usize, 2179500u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2180116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00214414));
    } else {
        emu.pc = 2179504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002141b0));
    }
}
#[inline(always)]
pub fn block_0x002141b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.slr_no_count(14usize, 14usize, 16usize, 2179508u32);
    emu.xri_no_count(15usize, 16usize, 4294967295u32, 2179512u32);
    emu.sri_no_count(6usize, 13usize, 1u32, 2179516u32);
    emu.srr_no_count(15usize, 6usize, 15usize, 2179520u32);
    emu.orr_no_count(15usize, 14usize, 15usize, 2179524u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2179528u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2180120u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00214418));
}
#[inline(always)]
pub fn block_0x002141c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2179548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002141dc));
    } else {
        emu.pc = 2179532u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002141cc));
    }
}
#[inline(always)]
pub fn block_0x002141cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(15usize, 12usize, 14usize, 2179536u32);
    emu.adi_no_count(17usize, 0usize, 0u32, 2179540u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2179560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002141e8));
    } else {
        emu.pc = 2179544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002141d8));
    }
}
#[inline(always)]
pub fn block_0x002141d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2179548u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2179800u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002142d8));
}
#[inline(always)]
pub fn block_0x002141dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(15usize, 11usize, 13usize, 2179552u32);
    emu.adi_no_count(17usize, 0usize, 0u32, 2179556u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a != b {
        emu.pc = 2179800u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002142d8));
    } else {
        emu.pc = 2179560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002141e8));
    }
}
#[inline(always)]
pub fn block_0x002141e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2179800u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002142d8));
    } else {
        emu.pc = 2179564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002141ec));
    }
}
#[inline(always)]
pub fn block_0x002141ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(17usize, 12usize, 16u32, 2179568u32);
    emu.sltru_no_count(15usize, 17usize, 14usize, 2179572u32);
    emu.xri_no_count(16usize, 15usize, 1u32, 2179576u32);
    emu.adi_no_count(15usize, 12usize, 0u32, 2179580u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a >= b {
        emu.pc = 2179928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00214358));
    } else {
        emu.pc = 2179584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00214200));
    }
}
#[inline(always)]
pub fn block_0x00214200(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(16usize, 16usize, 4u32, 2179588u32);
    emu.sri_no_count(5usize, 15usize, 8u32, 2179592u32);
    emu.sltru_no_count(17usize, 5usize, 14usize, 2179596u32);
    emu.xri_no_count(17usize, 17usize, 1u32, 2179600u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a >= b {
        emu.pc = 2179952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00214370));
    } else {
        emu.pc = 2179604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00214214));
    }
}
#[inline(always)]
pub fn block_0x00214214(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(5usize, 17usize, 3u32, 2179608u32);
    emu.sri_no_count(17usize, 15usize, 4u32, 2179612u32);
    emu.orr_no_count(16usize, 5usize, 16usize, 2179616u32);
    emu.sltru_no_count(5usize, 17usize, 14usize, 2179620u32);
    emu.xri_no_count(5usize, 5usize, 1u32, 2179624u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a >= b {
        emu.pc = 2179980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021438c));
    } else {
        emu.pc = 2179628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021422c));
    }
}
#[inline(always)]
pub fn block_0x0021422c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(5usize, 5usize, 2u32, 2179632u32);
    emu.sri_no_count(17usize, 15usize, 2u32, 2179636u32);
    emu.orr_no_count(16usize, 16usize, 5usize, 2179640u32);
    emu.sltru_no_count(5usize, 17usize, 14usize, 2179644u32);
    emu.xri_no_count(5usize, 5usize, 1u32, 2179648u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a < b {
        emu.pc = 2179656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00214248));
    } else {
        emu.pc = 2179652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00214244));
    }
}
#[inline(always)]
pub fn block_0x00214244(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 17usize, 0u32, 2179656u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2179656u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00214248));
}
#[inline]
pub fn block_0x00214248(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 0usize, 0u32, 2179660u32);
    emu.sli_no_count(5usize, 5usize, 1u32, 2179664u32);
    emu.sri_no_count(15usize, 15usize, 1u32, 2179668u32);
    emu.sltru_no_count(15usize, 15usize, 14usize, 2179672u32);
    emu.xri_no_count(15usize, 15usize, 1u32, 2179676u32);
    emu.orr_no_count(15usize, 5usize, 15usize, 2179680u32);
    emu.sri_no_count(5usize, 13usize, 1u32, 2179684u32);
    emu.orr_no_count(16usize, 16usize, 15usize, 2179688u32);
    emu.xri_no_count(15usize, 16usize, 31u32, 2179692u32);
    emu.srr_no_count(15usize, 5usize, 15usize, 2179696u32);
    emu.slr_no_count(5usize, 14usize, 16usize, 2179700u32);
    emu.orr_no_count(15usize, 5usize, 15usize, 2179704u32);
    emu.adi_no_count(5usize, 0usize, 1u32, 2179708u32);
    emu.slr_no_count(5usize, 5usize, 16usize, 2179712u32);
    emu.slr_no_count(6usize, 13usize, 16usize, 2179716u32);
    emu.add_memory_rw_events(16usize);
    let return_addr = 2179720u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2179740u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021429c));
}
#[inline(always)]
pub fn block_0x00214288(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(16usize, 6usize, 1u32, 2179724u32);
    emu.sli_no_count(6usize, 15usize, 31u32, 2179728u32);
    emu.sri_no_count(15usize, 15usize, 1u32, 2179732u32);
    emu.orr_no_count(6usize, 16usize, 6usize, 2179736u32);
    emu.sri_no_count(5usize, 5usize, 1u32, 2179740u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2179740u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021429c));
}
#[inline(always)]
pub fn block_0x0021429c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 11usize, 6usize, 2179744u32);
    emu.sbr_no_count(7usize, 12usize, 15usize, 2179748u32);
    emu.sbr_no_count(16usize, 7usize, 16usize, 2179752u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2179720u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00214288));
    } else {
        emu.pc = 2179756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002142ac));
    }
}
#[inline(always)]
pub fn block_0x002142ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 11usize, 6usize, 2179760u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2179780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002142c4));
    } else {
        emu.pc = 2179764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002142b4));
    }
}
#[inline(always)]
pub fn block_0x002142b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 16usize, 14usize, 2179768u32);
    emu.orr_no_count(17usize, 5usize, 17usize, 2179772u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2179792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002142d0));
    } else {
        emu.pc = 2179776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002142c0));
    }
}
#[inline(always)]
pub fn block_0x002142c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2179780u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2179820u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002142ec));
}
#[inline(always)]
pub fn block_0x002142c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 11usize, 13usize, 2179784u32);
    emu.orr_no_count(17usize, 5usize, 17usize, 2179788u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2179820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002142ec));
    } else {
        emu.pc = 2179792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002142d0));
    }
}
#[inline(always)]
pub fn block_0x002142d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 16usize, 0u32, 2179796u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2179800u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2179720u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00214288));
}
#[inline(always)]
pub fn block_0x002142d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(17usize, 10usize, 0u32, 2179804u32)?;
    emu.sw_no_count(17usize, 10usize, 4u32, 2179808u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2179812u32)?;
    emu.sw_no_count(12usize, 10usize, 12u32, 2179816u32)?;
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2179820u32;
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
pub fn block_0x002142ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(17usize, 10usize, 0u32, 2179824u32)?;
    emu.sw_no_count(0usize, 10usize, 4u32, 2179828u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2179832u32)?;
    emu.sw_no_count(16usize, 10usize, 12u32, 2179836u32)?;
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2179840u32;
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
pub fn block_0x00214300(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.divu_no_count(17usize, 11usize, 13usize, 2179844u32);
    emu.mul_no_count(12usize, 17usize, 13usize, 2179848u32);
    emu.sbr_no_count(11usize, 11usize, 12usize, 2179852u32);
    emu.sw_no_count(17usize, 10usize, 0u32, 2179856u32)?;
    emu.sw_no_count(0usize, 10usize, 4u32, 2179860u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2179864u32)?;
    emu.sw_no_count(0usize, 10usize, 12u32, 2179868u32)?;
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2179872u32;
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
pub fn block_0x00214320(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2180236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021448c));
    } else {
        emu.pc = 2179876u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00214324));
    }
}
#[inline]
pub fn block_0x00214324(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.divu_no_count(13usize, 11usize, 12usize, 2179880u32);
    emu.mul_no_count(12usize, 13usize, 12usize, 2179884u32);
    emu.sltru_no_count(15usize, 0usize, 13usize, 2179888u32);
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2179892u32);
    emu.sbr_no_count(11usize, 11usize, 12usize, 2179896u32);
    emu.adi_no_count(17usize, 13usize, 1u32, 2179900u32);
    emu.sltiu_no_count(12usize, 17usize, 1u32, 2179904u32);
    emu.adr_no_count(15usize, 15usize, 12usize, 2179908u32);
    emu.sw_no_count(17usize, 10usize, 0u32, 2179912u32)?;
    emu.sw_no_count(15usize, 10usize, 4u32, 2179916u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2179920u32)?;
    emu.sw_no_count(0usize, 10usize, 12u32, 2179924u32)?;
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2179928u32;
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
pub fn block_0x00214358(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 17usize, 0u32, 2179932u32);
    emu.sli_no_count(16usize, 16usize, 4u32, 2179936u32);
    emu.sri_no_count(5usize, 17usize, 8u32, 2179940u32);
    emu.sltru_no_count(17usize, 5usize, 14usize, 2179944u32);
    emu.xri_no_count(17usize, 17usize, 1u32, 2179948u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a < b {
        emu.pc = 2179604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00214214));
    } else {
        emu.pc = 2179952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00214370));
    }
}
#[inline(always)]
pub fn block_0x00214370(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 5usize, 0u32, 2179956u32);
    emu.sli_no_count(5usize, 17usize, 3u32, 2179960u32);
    emu.sri_no_count(17usize, 15usize, 4u32, 2179964u32);
    emu.orr_no_count(16usize, 5usize, 16usize, 2179968u32);
    emu.sltru_no_count(5usize, 17usize, 14usize, 2179972u32);
    emu.xri_no_count(5usize, 5usize, 1u32, 2179976u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a < b {
        emu.pc = 2179628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021422c));
    } else {
        emu.pc = 2179980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021438c));
    }
}
#[inline(always)]
pub fn block_0x0021438c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 17usize, 0u32, 2179984u32);
    emu.sli_no_count(5usize, 5usize, 2u32, 2179988u32);
    emu.sri_no_count(17usize, 17usize, 2u32, 2179992u32);
    emu.orr_no_count(16usize, 16usize, 5usize, 2179996u32);
    emu.sltru_no_count(5usize, 17usize, 14usize, 2180000u32);
    emu.xri_no_count(5usize, 5usize, 1u32, 2180004u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a >= b {
        emu.pc = 2179652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00214244));
    } else {
        emu.pc = 2180008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002143a8));
    }
}
#[inline(always)]
pub fn block_0x002143a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2180012u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2179656u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00214248));
}
#[inline(always)]
pub fn block_0x002143ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 17usize, 0u32, 2180016u32);
    emu.sli_no_count(16usize, 16usize, 4u32, 2180020u32);
    emu.sri_no_count(5usize, 17usize, 8u32, 2180024u32);
    emu.sltru_no_count(17usize, 5usize, 12usize, 2180028u32);
    emu.xri_no_count(17usize, 17usize, 1u32, 2180032u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a < b {
        emu.pc = 2179408u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00214150));
    } else {
        emu.pc = 2180036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002143c4));
    }
}
#[inline(always)]
pub fn block_0x002143c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 5usize, 0u32, 2180040u32);
    emu.sli_no_count(5usize, 17usize, 3u32, 2180044u32);
    emu.sri_no_count(17usize, 15usize, 4u32, 2180048u32);
    emu.orr_no_count(16usize, 5usize, 16usize, 2180052u32);
    emu.sltru_no_count(5usize, 17usize, 12usize, 2180056u32);
    emu.xri_no_count(5usize, 5usize, 1u32, 2180060u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a < b {
        emu.pc = 2179432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00214168));
    } else {
        emu.pc = 2180064u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002143e0));
    }
}
#[inline(always)]
pub fn block_0x002143e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 17usize, 0u32, 2180068u32);
    emu.sli_no_count(5usize, 5usize, 2u32, 2180072u32);
    emu.sri_no_count(17usize, 17usize, 2u32, 2180076u32);
    emu.orr_no_count(16usize, 16usize, 5usize, 2180080u32);
    emu.sltru_no_count(5usize, 17usize, 12usize, 2180084u32);
    emu.xri_no_count(5usize, 5usize, 1u32, 2180088u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a >= b {
        emu.pc = 2179456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00214180));
    } else {
        emu.pc = 2180092u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002143fc));
    }
}
#[inline(always)]
pub fn block_0x002143fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2180096u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2179460u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00214184));
}
#[inline(always)]
pub fn block_0x00214400(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 32u32, 2180100u32);
    emu.sbr_no_count(16usize, 16usize, 15usize, 2180104u32);
    emu.adi_no_count(5usize, 16usize, 4294967264u32, 2180108u32);
    emu.slr_no_count(17usize, 13usize, 16usize, 2180112u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2179504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002141b0));
    } else {
        emu.pc = 2180116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00214414));
    }
}
#[inline(always)]
pub fn block_0x00214414(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 17usize, 0u32, 2180120u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2180120u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00214418));
}
#[inline(always)]
pub fn block_0x00214418(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 0u32, 2180124u32);
    emu.sai_no_count(5usize, 5usize, 1055u32, 2180128u32);
    emu.anr_no_count(17usize, 5usize, 17usize, 2180132u32);
    emu.adi_no_count(5usize, 0usize, 1u32, 2180136u32);
    emu.slr_no_count(16usize, 5usize, 16usize, 2180140u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2180144u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2180164u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00214444));
}
#[inline(always)]
pub fn block_0x00214430(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(17usize, 17usize, 1u32, 2180148u32);
    emu.sli_no_count(5usize, 15usize, 31u32, 2180152u32);
    emu.sri_no_count(15usize, 15usize, 1u32, 2180156u32);
    emu.orr_no_count(17usize, 17usize, 5usize, 2180160u32);
    emu.sri_no_count(16usize, 16usize, 1u32, 2180164u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2180164u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00214444));
}
#[inline(always)]
pub fn block_0x00214444(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 11usize, 17usize, 2180168u32);
    emu.sbr_no_count(6usize, 12usize, 15usize, 2180172u32);
    emu.sbr_no_count(5usize, 6usize, 5usize, 2180176u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2180144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00214430));
    } else {
        emu.pc = 2180180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00214454));
    }
}
#[inline(always)]
pub fn block_0x00214454(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 11usize, 17usize, 2180184u32);
    emu.orr_no_count(14usize, 16usize, 14usize, 2180188u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2180200u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00214468));
    } else {
        emu.pc = 2180192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00214460));
    }
}
#[inline(always)]
pub fn block_0x00214460(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 5usize, 0u32, 2180196u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2180200u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2180144u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00214430));
}
#[inline]
pub fn block_0x00214468(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.divu_no_count(12usize, 11usize, 13usize, 2180204u32);
    emu.mul_no_count(13usize, 12usize, 13usize, 2180208u32);
    emu.sbr_no_count(11usize, 11usize, 13usize, 2180212u32);
    emu.orr_no_count(17usize, 12usize, 14usize, 2180216u32);
    emu.sw_no_count(17usize, 10usize, 0u32, 2180220u32)?;
    emu.sw_no_count(0usize, 10usize, 4u32, 2180224u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2180228u32)?;
    emu.sw_no_count(0usize, 10usize, 12u32, 2180232u32)?;
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2180236u32;
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
pub fn block_0x0021448c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(17usize, 13usize, 16u32, 2180240u32);
    emu.divu_no_count(15usize, 12usize, 13usize, 2180244u32);
    emu.mul_no_count(16usize, 15usize, 13usize, 2180248u32);
    emu.sbr_no_count(16usize, 12usize, 16usize, 2180252u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a != b {
        emu.pc = 2180344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002144f8));
    } else {
        emu.pc = 2180256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002144a0));
    }
}
#[inline]
pub fn block_0x002144a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 22u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(12usize, 11usize, 16u32, 2180260u32);
    emu.sli_no_count(16usize, 16usize, 16u32, 2180264u32);
    emu.sli_no_count(11usize, 11usize, 16u32, 2180268u32);
    emu.orr_no_count(14usize, 16usize, 12usize, 2180272u32);
    emu.sri_no_count(11usize, 11usize, 16u32, 2180276u32);
    emu.divu_no_count(14usize, 14usize, 13usize, 2180280u32);
    emu.mul_no_count(16usize, 14usize, 13usize, 2180284u32);
    emu.sbr_no_count(12usize, 12usize, 16usize, 2180288u32);
    emu.sli_no_count(16usize, 14usize, 16u32, 2180292u32);
    emu.sri_no_count(14usize, 14usize, 16u32, 2180296u32);
    emu.orr_no_count(15usize, 14usize, 15usize, 2180300u32);
    emu.sli_no_count(12usize, 12usize, 16u32, 2180304u32);
    emu.orr_no_count(11usize, 12usize, 11usize, 2180308u32);
    emu.divu_no_count(12usize, 11usize, 13usize, 2180312u32);
    emu.mul_no_count(13usize, 12usize, 13usize, 2180316u32);
    emu.sbr_no_count(11usize, 11usize, 13usize, 2180320u32);
    emu.orr_no_count(17usize, 16usize, 12usize, 2180324u32);
    emu.sw_no_count(17usize, 10usize, 0u32, 2180328u32)?;
    emu.sw_no_count(15usize, 10usize, 4u32, 2180332u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2180336u32)?;
    emu.sw_no_count(0usize, 10usize, 12u32, 2180340u32)?;
    emu.add_memory_rw_events(22usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2180344u32;
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
pub fn block_0x002144f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2180356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00214504));
    } else {
        emu.pc = 2180348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002144fc));
    }
}
#[inline(always)]
pub fn block_0x002144fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 16usize, 14usize, 2180352u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2180356u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2180360u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00214508));
}
#[inline(always)]
pub fn block_0x00214504(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 11usize, 13usize, 2180360u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2180360u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00214508));
}
#[inline(always)]
pub fn block_0x00214508(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2180384u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00214520));
    } else {
        emu.pc = 2180364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021450c));
    }
}
#[inline(always)]
pub fn block_0x0021450c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 10usize, 0u32, 2180368u32)?;
    emu.sw_no_count(15usize, 10usize, 4u32, 2180372u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2180376u32)?;
    emu.sw_no_count(16usize, 10usize, 12u32, 2180380u32)?;
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2180384u32;
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
pub fn block_0x00214520(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(17usize, 13usize, 1u32, 2180388u32);
    emu.sli_no_count(14usize, 14usize, 31u32, 2180392u32);
    emu.sli_no_count(5usize, 13usize, 31u32, 2180396u32);
    emu.orr_no_count(14usize, 14usize, 17usize, 2180400u32);
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2180404u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(6usize);
    let return_addr = 2180408u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2180428u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021454c));
}
#[inline(always)]
pub fn block_0x00214538(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(5usize, 5usize, 1u32, 2180412u32);
    emu.sli_no_count(6usize, 14usize, 31u32, 2180416u32);
    emu.sri_no_count(14usize, 14usize, 1u32, 2180420u32);
    emu.orr_no_count(5usize, 5usize, 6usize, 2180424u32);
    emu.sri_no_count(17usize, 17usize, 1u32, 2180428u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2180428u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021454c));
}
#[inline(always)]
pub fn block_0x0021454c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(6usize, 11usize, 5usize, 2180432u32);
    emu.sbr_no_count(7usize, 16usize, 14usize, 2180436u32);
    emu.sbr_no_count(6usize, 7usize, 6usize, 2180440u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(6usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2180408u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00214538));
    } else {
        emu.pc = 2180444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021455c));
    }
}
#[inline(always)]
pub fn block_0x0021455c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 11usize, 5usize, 2180448u32);
    emu.orr_no_count(12usize, 17usize, 12usize, 2180452u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2180464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00214570));
    } else {
        emu.pc = 2180456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00214568));
    }
}
#[inline(always)]
pub fn block_0x00214568(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 6usize, 0u32, 2180460u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2180464u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2180408u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00214538));
}
#[inline]
pub fn block_0x00214570(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.divu_no_count(14usize, 11usize, 13usize, 2180468u32);
    emu.mul_no_count(13usize, 14usize, 13usize, 2180472u32);
    emu.sbr_no_count(11usize, 11usize, 13usize, 2180476u32);
    emu.orr_no_count(17usize, 14usize, 12usize, 2180480u32);
    emu.sw_no_count(17usize, 10usize, 0u32, 2180484u32)?;
    emu.sw_no_count(15usize, 10usize, 4u32, 2180488u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2180492u32)?;
    emu.sw_no_count(0usize, 10usize, 12u32, 2180496u32)?;
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2180500u32;
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
