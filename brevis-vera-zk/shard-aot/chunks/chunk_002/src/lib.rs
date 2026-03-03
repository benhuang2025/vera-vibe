pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2105040u32;
pub const PC_MAX: u32 = 2107420u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 109usize] = [
        block_0x00201ed0,
        block_0x00201f34,
        block_0x00201f40,
        block_0x00201fac,
        block_0x00201fb8,
        block_0x0020200c,
        block_0x00202010,
        block_0x00202014,
        block_0x00202024,
        block_0x0020203c,
        block_0x00202040,
        block_0x00202068,
        block_0x002020c8,
        block_0x002020d8,
        block_0x002020e0,
        block_0x002020ec,
        block_0x00202104,
        block_0x0020210c,
        block_0x00202110,
        block_0x00202124,
        block_0x0020212c,
        block_0x00202150,
        block_0x0020215c,
        block_0x00202170,
        block_0x00202174,
        block_0x00202184,
        block_0x002021a0,
        block_0x002021a8,
        block_0x002021ac,
        block_0x002021b8,
        block_0x002021f4,
        block_0x0020220c,
        block_0x00202250,
        block_0x00202254,
        block_0x00202258,
        block_0x00202260,
        block_0x00202278,
        block_0x0020227c,
        block_0x002022a8,
        block_0x002022c4,
        block_0x002022cc,
        block_0x002022e8,
        block_0x002022f8,
        block_0x002022fc,
        block_0x0020230c,
        block_0x00202328,
        block_0x00202348,
        block_0x00202354,
        block_0x00202384,
        block_0x0020239c,
        block_0x002023a4,
        block_0x002023c0,
        block_0x002023d0,
        block_0x002023dc,
        block_0x002023ec,
        block_0x002023f4,
        block_0x002023f8,
        block_0x00202410,
        block_0x0020242c,
        block_0x0020244c,
        block_0x00202474,
        block_0x00202480,
        block_0x002024a8,
        block_0x002024b4,
        block_0x002024dc,
        block_0x002024e8,
        block_0x00202504,
        block_0x00202520,
        block_0x00202548,
        block_0x00202554,
        block_0x00202570,
        block_0x0020258c,
        block_0x002025b0,
        block_0x002025bc,
        block_0x002025e4,
        block_0x002025e8,
        block_0x002025ec,
        block_0x002025f8,
        block_0x00202608,
        block_0x00202618,
        block_0x0020261c,
        block_0x00202628,
        block_0x00202630,
        block_0x00202648,
        block_0x0020264c,
        block_0x0020267c,
        block_0x00202688,
        block_0x0020268c,
        block_0x002026a0,
        block_0x002026b0,
        block_0x002026b8,
        block_0x002026c0,
        block_0x002026d8,
        block_0x002026e4,
        block_0x002026e8,
        block_0x002026ec,
        block_0x002026f4,
        block_0x002026fc,
        block_0x00202704,
        block_0x00202708,
        block_0x0020271c,
        block_0x00202730,
        block_0x00202780,
        block_0x00202794,
        block_0x00202798,
        block_0x002027a4,
        block_0x002027f4,
        block_0x00202808,
        block_0x0020281c,
    ];
    const IDX: [u16; 596usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16,
        0u16, 0u16, 3u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 4u16, 0u16, 0u16, 5u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 6u16, 7u16, 8u16, 0u16, 0u16, 0u16, 9u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        10u16, 11u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 12u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 13u16, 0u16, 0u16, 0u16,
        14u16, 0u16, 15u16, 0u16, 0u16, 16u16, 0u16, 0u16, 0u16, 0u16, 0u16, 17u16, 0u16,
        18u16, 19u16, 0u16, 0u16, 0u16, 0u16, 20u16, 0u16, 21u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 22u16, 0u16, 0u16, 23u16, 0u16, 0u16, 0u16, 0u16, 24u16,
        25u16, 0u16, 0u16, 0u16, 26u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 27u16, 0u16,
        28u16, 29u16, 0u16, 0u16, 30u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 31u16, 0u16, 0u16, 0u16, 0u16, 0u16, 32u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 33u16, 34u16, 35u16, 0u16, 36u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        37u16, 38u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 39u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 40u16, 0u16, 41u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 42u16, 0u16, 0u16, 0u16, 43u16, 44u16, 0u16, 0u16, 0u16, 45u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 46u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        47u16, 0u16, 0u16, 48u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 49u16, 0u16, 0u16, 0u16, 0u16, 0u16, 50u16, 0u16, 51u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 52u16, 0u16, 0u16, 0u16, 53u16, 0u16, 0u16, 54u16, 0u16,
        0u16, 0u16, 55u16, 0u16, 56u16, 57u16, 0u16, 0u16, 0u16, 0u16, 0u16, 58u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 59u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        60u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 61u16, 0u16, 0u16,
        62u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 63u16, 0u16, 0u16,
        64u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 65u16, 0u16, 0u16,
        66u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 67u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 68u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 69u16, 0u16,
        0u16, 70u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 71u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 72u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 73u16, 0u16,
        0u16, 74u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 75u16, 76u16,
        77u16, 0u16, 0u16, 78u16, 0u16, 0u16, 0u16, 79u16, 0u16, 0u16, 0u16, 80u16,
        81u16, 0u16, 0u16, 82u16, 0u16, 83u16, 0u16, 0u16, 0u16, 0u16, 0u16, 84u16,
        85u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 86u16,
        0u16, 0u16, 87u16, 88u16, 0u16, 0u16, 0u16, 0u16, 89u16, 0u16, 0u16, 0u16, 90u16,
        0u16, 91u16, 0u16, 92u16, 0u16, 0u16, 0u16, 0u16, 0u16, 93u16, 0u16, 0u16, 94u16,
        95u16, 96u16, 0u16, 97u16, 0u16, 98u16, 0u16, 99u16, 100u16, 0u16, 0u16, 0u16,
        0u16, 101u16, 0u16, 0u16, 0u16, 0u16, 102u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        103u16, 0u16, 0u16, 0u16, 0u16, 104u16, 105u16, 0u16, 0u16, 106u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 107u16, 0u16, 0u16, 0u16, 0u16, 108u16, 0u16, 0u16, 0u16,
        0u16, 109u16,
    ];
    if pc < 2105040u32 || pc > 2107420u32 {
        return None;
    }
    let word_offset = ((pc - 2105040u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(never)]
pub fn block_0x00201ed0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 25u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2105044u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2105048u32)?;
    emu.sw_no_count(11usize, 2usize, 12u32, 2105052u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2105056u32)?;
    let a = 0u32.wrapping_add(2129920u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2105060u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966516u32, 2105064u32);
    emu.adi_no_count(12usize, 2usize, 12u32, 2105068u32);
    let a = 0u32.wrapping_add(2101248u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2105072u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1124u32, 2105076u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2105080u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 496u32, 2105084u32);
    emu.adi_no_count(15usize, 0usize, 2u32, 2105088u32);
    emu.sw_no_count(0usize, 2usize, 36u32, 2105092u32)?;
    emu.sw_no_count(10usize, 2usize, 44u32, 2105096u32)?;
    emu.sw_no_count(11usize, 2usize, 48u32, 2105100u32)?;
    emu.sw_no_count(12usize, 2usize, 52u32, 2105104u32)?;
    emu.sw_no_count(13usize, 2usize, 56u32, 2105108u32)?;
    emu.adi_no_count(10usize, 2usize, 44u32, 2105112u32);
    emu.sw_no_count(14usize, 2usize, 20u32, 2105116u32)?;
    emu.sw_no_count(15usize, 2usize, 24u32, 2105120u32)?;
    emu.sw_no_count(10usize, 2usize, 28u32, 2105124u32)?;
    emu.sw_no_count(15usize, 2usize, 32u32, 2105128u32)?;
    emu.adi_no_count(10usize, 2usize, 20u32, 2105132u32);
    emu.apc_no_count(1usize, 2105132u32, 0u32, 2105136u32);
    emu.add_memory_rw_events(25usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2105140u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1680u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00201f34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2105144u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2105148u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2105152u32;
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
pub fn block_0x00201f40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2105156u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2105160u32)?;
    emu.sw_no_count(10usize, 2usize, 8u32, 2105164u32)?;
    emu.sw_no_count(11usize, 2usize, 12u32, 2105168u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2105172u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2105176u32);
    let a = 0u32.wrapping_add(2146304u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2105180u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966404u32, 2105184u32);
    emu.adi_no_count(12usize, 2usize, 12u32, 2105188u32);
    let a = 0u32.wrapping_add(2101248u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2105192u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1124u32, 2105196u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2105200u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 528u32, 2105204u32);
    emu.adi_no_count(15usize, 0usize, 2u32, 2105208u32);
    emu.sw_no_count(0usize, 2usize, 36u32, 2105212u32)?;
    emu.sw_no_count(10usize, 2usize, 44u32, 2105216u32)?;
    emu.sw_no_count(11usize, 2usize, 48u32, 2105220u32)?;
    emu.sw_no_count(12usize, 2usize, 52u32, 2105224u32)?;
    emu.sw_no_count(13usize, 2usize, 56u32, 2105228u32)?;
    emu.adi_no_count(10usize, 2usize, 44u32, 2105232u32);
    emu.sw_no_count(14usize, 2usize, 20u32, 2105236u32)?;
    emu.sw_no_count(15usize, 2usize, 24u32, 2105240u32)?;
    emu.sw_no_count(10usize, 2usize, 28u32, 2105244u32)?;
    emu.sw_no_count(15usize, 2usize, 32u32, 2105248u32)?;
    emu.adi_no_count(10usize, 2usize, 20u32, 2105252u32);
    emu.apc_no_count(1usize, 2105252u32, 0u32, 2105256u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2105260u32;
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
pub fn block_0x00201fac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2105264u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2105268u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2105272u32;
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
pub fn block_0x00201fb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967184u32, 2105276u32);
    emu.sw_no_count(1usize, 2usize, 108u32, 2105280u32)?;
    emu.sw_no_count(8usize, 2usize, 104u32, 2105284u32)?;
    emu.sw_no_count(9usize, 2usize, 100u32, 2105288u32)?;
    emu.sw_no_count(18usize, 2usize, 96u32, 2105292u32)?;
    emu.sw_no_count(19usize, 2usize, 92u32, 2105296u32)?;
    emu.sw_no_count(20usize, 2usize, 88u32, 2105300u32)?;
    emu.sw_no_count(21usize, 2usize, 84u32, 2105304u32)?;
    emu.sw_no_count(22usize, 2usize, 80u32, 2105308u32)?;
    emu.sw_no_count(23usize, 2usize, 76u32, 2105312u32)?;
    emu.sw_no_count(24usize, 2usize, 72u32, 2105316u32)?;
    emu.sw_no_count(25usize, 2usize, 68u32, 2105320u32)?;
    emu.sw_no_count(26usize, 2usize, 64u32, 2105324u32)?;
    emu.sw_no_count(27usize, 2usize, 60u32, 2105328u32)?;
    emu.adi_no_count(9usize, 12usize, 0u32, 2105332u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2105336u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2105340u32);
    let a = 0u32.wrapping_add(53248u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2105344u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966476u32, 2105348u32);
    emu.adi_no_count(20usize, 12usize, 0u32, 2105352u32);
    emu.add_memory_rw_events(20usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2105360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202010));
    } else {
        emu.pc = 2105356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020200c));
    }
}
#[inline(always)]
pub fn block_0x0020200c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 10usize, 0u32, 2105360u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2105360u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202010));
}
#[inline(always)]
pub fn block_0x00202010(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2105716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202174));
    } else {
        emu.pc = 2105364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202014));
    }
}
#[inline(always)]
pub fn block_0x00202014(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 20u32, 2105368u32);
    emu.mul_no_count(19usize, 20usize, 10usize, 2105372u32);
    emu.apc_no_count(1usize, 2105372u32, 0u32, 2105376u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2105380u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1768u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202024(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2105384u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294967248u32, 2105388u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2105392u32);
    emu.adi_no_count(12usize, 19usize, 0u32, 2105396u32);
    emu.apc_no_count(1usize, 2105396u32, 4096u32, 2105400u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2105404u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1128u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020203c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2105844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002021f4));
    } else {
        emu.pc = 2105408u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202040));
    }
}
#[inline]
pub fn block_0x00202040(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(20usize, 2usize, 8u32, 2105412u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2105416u32)?;
    emu.sw_no_count(0usize, 2usize, 16u32, 2105420u32)?;
    emu.adi_no_count(22usize, 2usize, 40u32, 2105424u32);
    emu.adi_no_count(23usize, 0usize, 2u32, 2105428u32);
    emu.adi_no_count(24usize, 0usize, 3u32, 2105432u32);
    emu.adi_no_count(25usize, 0usize, 20u32, 2105436u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(21usize, a);
    emu.pc = 2105440u32;
    emu.update_insn_clock();
    emu.adi_no_count(21usize, 21usize, 12u32, 2105444u32);
    emu.add_memory_rw_events(10usize);
    let return_addr = 2105448u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2105544u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002020c8));
}
#[inline]
pub fn block_0x00202068(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2105452u32)?;
    emu.adi_no_count(9usize, 9usize, 4294967295u32, 2105456u32);
    emu.mul_no_count(11usize, 19usize, 25usize, 2105460u32);
    emu.sri_no_count(12usize, 26usize, 16u32, 2105464u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2105468u32);
    emu.lh_no_count(11usize, 2usize, 20u32, 2105472u32)?;
    emu.lh_no_count(13usize, 2usize, 22u32, 2105476u32)?;
    emu.lh_no_count(14usize, 2usize, 24u32, 2105480u32)?;
    emu.lh_no_count(15usize, 2usize, 26u32, 2105484u32)?;
    emu.sh_no_count(27usize, 10usize, 0u32, 2105488u32)?;
    emu.sh_no_count(20usize, 10usize, 2u32, 2105492u32)?;
    emu.sh_no_count(26usize, 10usize, 4u32, 2105496u32)?;
    emu.sh_no_count(12usize, 10usize, 6u32, 2105500u32)?;
    emu.lh_no_count(12usize, 2usize, 28u32, 2105504u32)?;
    emu.lh_no_count(16usize, 2usize, 30u32, 2105508u32)?;
    emu.adi_no_count(19usize, 19usize, 1u32, 2105512u32);
    emu.sh_no_count(11usize, 10usize, 8u32, 2105516u32)?;
    emu.sh_no_count(13usize, 10usize, 10u32, 2105520u32)?;
    emu.sh_no_count(14usize, 10usize, 12u32, 2105524u32)?;
    emu.sh_no_count(15usize, 10usize, 14u32, 2105528u32)?;
    emu.sh_no_count(12usize, 10usize, 16u32, 2105532u32)?;
    emu.sh_no_count(16usize, 10usize, 18u32, 2105536u32)?;
    emu.sw_no_count(19usize, 2usize, 16u32, 2105540u32)?;
    emu.add_memory_rw_events(23usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2105732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202184));
    } else {
        emu.pc = 2105544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002020c8));
    }
}
#[inline(always)]
pub fn block_0x002020c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 52u32, 2105548u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2105552u32);
    emu.apc_no_count(1usize, 2105552u32, 0u32, 2105556u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2105560u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965628u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002020d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 52u32, 2105564u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2105760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002021a0));
    } else {
        emu.pc = 2105568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002020e0));
    }
}
#[inline(always)]
pub fn block_0x002020e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 56u32, 2105572u32)?;
    emu.ani_no_count(10usize, 10usize, 1u32, 2105576u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2105616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202110));
    } else {
        emu.pc = 2105580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002020ec));
    }
}
#[inline(always)]
pub fn block_0x002020ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 32u32, 2105584u32);
    emu.adi_no_count(13usize, 0usize, 1u32, 2105588u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2105592u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 52u32, 2105596u32);
    emu.apc_no_count(1usize, 2105596u32, 0u32, 2105600u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2105604u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965400u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202104(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lhu_no_count(27usize, 2usize, 32u32, 2105608u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(27usize);
    if a != b {
        emu.pc = 2105644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020212c));
    } else {
        emu.pc = 2105612u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020210c));
    }
}
#[inline(always)]
pub fn block_0x0020210c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2105616u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2105768u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002021a8));
}
#[inline(always)]
pub fn block_0x00202110(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 32u32, 2105620u32);
    emu.adi_no_count(13usize, 0usize, 4u32, 2105624u32);
    emu.adi_no_count(12usize, 21usize, 0u32, 2105628u32);
    emu.apc_no_count(1usize, 2105628u32, 4294963200u32, 2105632u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2105636u32;
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
pub fn block_0x00202124(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lhu_no_count(27usize, 2usize, 32u32, 2105640u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(27usize);
    if a == b {
        emu.pc = 2105768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002021a8));
    } else {
        emu.pc = 2105644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020212c));
    }
}
#[inline]
pub fn block_0x0020212c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 22usize, 0u32, 2105648u32)?;
    emu.lw_no_count(11usize, 22usize, 4u32, 2105652u32)?;
    emu.lw_no_count(12usize, 22usize, 8u32, 2105656u32)?;
    emu.lhu_no_count(20usize, 2usize, 34u32, 2105660u32)?;
    emu.lw_no_count(26usize, 2usize, 36u32, 2105664u32)?;
    emu.sw_no_count(10usize, 2usize, 20u32, 2105668u32)?;
    emu.sw_no_count(11usize, 2usize, 24u32, 2105672u32)?;
    emu.sw_no_count(12usize, 2usize, 28u32, 2105676u32)?;
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(27usize);
    if a == b {
        emu.pc = 2105772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002021ac));
    } else {
        emu.pc = 2105680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202150));
    }
}
#[inline(always)]
pub fn block_0x00202150(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2105684u32)?;
    emu.lw_no_count(19usize, 2usize, 16u32, 2105688u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a != b {
        emu.pc = 2105448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202068));
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
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 8u32, 2105696u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2105700u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 560u32, 2105704u32);
    emu.apc_no_count(1usize, 2105704u32, 4294963200u32, 2105708u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2105712u32;
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
pub fn block_0x00202170(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2105716u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2105448u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202068));
}
#[inline(always)]
pub fn block_0x00202174(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2105720u32);
    emu.sw_no_count(0usize, 2usize, 8u32, 2105724u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2105728u32)?;
    emu.sw_no_count(0usize, 2usize, 16u32, 2105732u32)?;
    emu.add_memory_rw_events(4usize);
    emu.pc = 2105732u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202184));
}
#[inline(always)]
pub fn block_0x00202184(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2105736u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2105740u32)?;
    emu.lw_no_count(12usize, 2usize, 16u32, 2105744u32)?;
    emu.sw_no_count(10usize, 8usize, 0u32, 2105748u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2105752u32)?;
    emu.sw_no_count(12usize, 8usize, 8u32, 2105756u32)?;
    emu.add_memory_rw_events(7usize);
    let return_addr = 2105760u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2105784u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002021b8));
}
#[inline(always)]
pub fn block_0x002021a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(26usize, 2usize, 56u32, 2105764u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2105768u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2105772u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002021ac));
}
#[inline(always)]
pub fn block_0x002021a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(26usize, 2usize, 36u32, 2105772u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2105772u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002021ac));
}
#[inline(always)]
pub fn block_0x002021ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2105776u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 8usize, 0u32, 2105780u32)?;
    emu.sw_no_count(26usize, 8usize, 4u32, 2105784u32)?;
    emu.add_memory_rw_events(3usize);
    emu.pc = 2105784u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002021b8));
}
#[inline]
pub fn block_0x002021b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 108u32, 2105788u32)?;
    emu.lw_no_count(8usize, 2usize, 104u32, 2105792u32)?;
    emu.lw_no_count(9usize, 2usize, 100u32, 2105796u32)?;
    emu.lw_no_count(18usize, 2usize, 96u32, 2105800u32)?;
    emu.lw_no_count(19usize, 2usize, 92u32, 2105804u32)?;
    emu.lw_no_count(20usize, 2usize, 88u32, 2105808u32)?;
    emu.lw_no_count(21usize, 2usize, 84u32, 2105812u32)?;
    emu.lw_no_count(22usize, 2usize, 80u32, 2105816u32)?;
    emu.lw_no_count(23usize, 2usize, 76u32, 2105820u32)?;
    emu.lw_no_count(24usize, 2usize, 72u32, 2105824u32)?;
    emu.lw_no_count(25usize, 2usize, 68u32, 2105828u32)?;
    emu.lw_no_count(26usize, 2usize, 64u32, 2105832u32)?;
    emu.lw_no_count(27usize, 2usize, 60u32, 2105836u32)?;
    emu.adi_no_count(2usize, 2usize, 112u32, 2105840u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2105844u32;
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
pub fn block_0x002021f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2105848u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 544u32, 2105852u32);
    emu.adi_no_count(10usize, 0usize, 4u32, 2105856u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2105860u32);
    emu.apc_no_count(1usize, 2105860u32, 36864u32, 2105864u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2105868u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(412u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020220c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2105872u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2105876u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2105880u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2105884u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2105888u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2105892u32)?;
    emu.sw_no_count(20usize, 2usize, 40u32, 2105896u32)?;
    emu.sw_no_count(21usize, 2usize, 36u32, 2105900u32)?;
    emu.sw_no_count(22usize, 2usize, 32u32, 2105904u32)?;
    emu.sw_no_count(23usize, 2usize, 28u32, 2105908u32)?;
    emu.sw_no_count(24usize, 2usize, 24u32, 2105912u32)?;
    emu.adi_no_count(9usize, 12usize, 0u32, 2105916u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2105920u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2105924u32);
    let a = 0u32.wrapping_add(1048576u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2105928u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 12usize, 0u32, 2105932u32);
    emu.add_memory_rw_events(16usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2105940u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202254));
    } else {
        emu.pc = 2105936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202250));
    }
}
#[inline(always)]
pub fn block_0x00202250(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(1048576u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2105940u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(1usize);
    emu.pc = 2105940u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202254));
}
#[inline(always)]
pub fn block_0x00202254(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2106108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002022fc));
    } else {
        emu.pc = 2105944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202258));
    }
}
#[inline(always)]
pub fn block_0x00202258(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2105944u32, 0u32, 2105948u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2105952u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1196u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202260(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2105956u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294967248u32, 2105960u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2105964u32);
    emu.adi_no_count(12usize, 19usize, 0u32, 2105968u32);
    emu.apc_no_count(1usize, 2105968u32, 4096u32, 2105972u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2105976u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(556u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
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
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2106244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202384));
    } else {
        emu.pc = 2105980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020227c));
    }
}
#[inline]
pub fn block_0x0020227c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 0u32, 2105984u32);
    emu.lw_no_count(21usize, 18usize, 0u32, 2105988u32)?;
    emu.lw_no_count(23usize, 18usize, 4u32, 2105992u32)?;
    emu.sw_no_count(19usize, 2usize, 4u32, 2105996u32)?;
    emu.sw_no_count(10usize, 2usize, 8u32, 2106000u32)?;
    emu.sw_no_count(0usize, 2usize, 12u32, 2106004u32)?;
    emu.sbr_no_count(22usize, 0usize, 23usize, 2106008u32);
    emu.adi_no_count(23usize, 23usize, 4294967295u32, 2106012u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2106016u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 19usize, 560u32, 2106020u32);
    emu.add_memory_rw_events(11usize);
    let return_addr = 2106024u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2106052u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002022c4));
}
#[inline(always)]
pub fn block_0x002022a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2106028u32)?;
    emu.adr_no_count(10usize, 10usize, 20usize, 2106032u32);
    emu.adi_no_count(20usize, 20usize, 1u32, 2106036u32);
    emu.sb_no_count(24usize, 10usize, 0u32, 2106040u32);
    emu.sw_no_count(20usize, 2usize, 12u32, 2106044u32)?;
    emu.adi_no_count(23usize, 23usize, 4294967295u32, 2106048u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2106124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020230c));
    } else {
        emu.pc = 2106052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002022c4));
    }
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
    emu.adr_no_count(10usize, 22usize, 20usize, 2106056u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2106152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202328));
    } else {
        emu.pc = 2106060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002022cc));
    }
}
#[inline(always)]
pub fn block_0x002022cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 21usize, 20usize, 2106064u32);
    emu.lw_no_count(11usize, 2usize, 4u32, 2106068u32)?;
    emu.lbu_no_count(24usize, 10usize, 0u32, 2106072u32);
    emu.adi_no_count(10usize, 10usize, 1u32, 2106076u32);
    emu.sw_no_count(10usize, 18usize, 0u32, 2106080u32)?;
    emu.sw_no_count(23usize, 18usize, 4u32, 2106084u32)?;
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a != b {
        emu.pc = 2106024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002022a8));
    } else {
        emu.pc = 2106088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002022e8));
    }
}
#[inline(always)]
pub fn block_0x002022e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 4u32, 2106092u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2106096u32);
    emu.apc_no_count(1usize, 2106096u32, 36864u32, 2106100u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106104u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966904u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002022f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2106108u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2106024u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002022a8));
}
#[inline(always)]
pub fn block_0x002022fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2106112u32);
    emu.sw_no_count(0usize, 2usize, 4u32, 2106116u32)?;
    emu.sw_no_count(10usize, 2usize, 8u32, 2106120u32)?;
    emu.sw_no_count(0usize, 2usize, 12u32, 2106124u32)?;
    emu.add_memory_rw_events(4usize);
    emu.pc = 2106124u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020230c));
}
#[inline(always)]
pub fn block_0x0020230c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 4u32, 2106128u32)?;
    emu.lw_no_count(11usize, 2usize, 8u32, 2106132u32)?;
    emu.lw_no_count(12usize, 2usize, 12u32, 2106136u32)?;
    emu.sw_no_count(10usize, 8usize, 0u32, 2106140u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2106144u32)?;
    emu.sw_no_count(12usize, 8usize, 8u32, 2106148u32)?;
    emu.add_memory_rw_events(7usize);
    let return_addr = 2106152u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2106196u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202354));
}
#[inline(always)]
pub fn block_0x00202328(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2106156u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2106160u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1281u32, 2106164u32);
    emu.sw_no_count(11usize, 2usize, 16u32, 2106168u32)?;
    emu.sw_no_count(10usize, 2usize, 20u32, 2106172u32)?;
    emu.adi_no_count(10usize, 2usize, 16u32, 2106176u32);
    emu.apc_no_count(1usize, 2106176u32, 4096u32, 2106180u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106184u32;
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
pub fn block_0x00202348(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2106188u32;
    emu.update_insn_clock();
    emu.sw_no_count(11usize, 8usize, 0u32, 2106192u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2106196u32)?;
    emu.add_memory_rw_events(3usize);
    emu.pc = 2106196u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202354));
}
#[inline]
pub fn block_0x00202354(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2106200u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2106204u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2106208u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2106212u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2106216u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2106220u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2106224u32)?;
    emu.lw_no_count(22usize, 2usize, 32u32, 2106228u32)?;
    emu.lw_no_count(23usize, 2usize, 28u32, 2106232u32)?;
    emu.lw_no_count(24usize, 2usize, 24u32, 2106236u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2106240u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106244u32;
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
pub fn block_0x00202384(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2106248u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 544u32, 2106252u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2106256u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2106260u32);
    emu.apc_no_count(1usize, 2106260u32, 36864u32, 2106264u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106268u32;
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
pub fn block_0x0020239c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2106268u32, 24576u32, 2106272u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2106276u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x002023a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2106280u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 576u32, 2106284u32);
    emu.adi_no_count(12usize, 0usize, 17u32, 2106288u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2106292u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2106296u32);
    emu.apc_no_count(6usize, 2106296u32, 61440u32, 2106300u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2106304u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965944u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002023c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2106308u32)?;
    emu.lw_no_count(11usize, 10usize, 0u32, 2106312u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2106316u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2106356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002023f4));
    } else {
        emu.pc = 2106320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002023d0));
    }
}
#[inline(always)]
pub fn block_0x002023d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 10usize, 4u32, 2106324u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2106328u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2106356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002023f4));
    } else {
        emu.pc = 2106332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002023dc));
    }
}
#[inline(always)]
pub fn block_0x002023dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 8u32, 2106336u32)?;
    emu.lw_no_count(11usize, 10usize, 4u32, 2106340u32)?;
    emu.lw_no_count(6usize, 11usize, 0u32, 2106344u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2106356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002023f4));
    } else {
        emu.pc = 2106348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002023ec));
    }
}
#[inline(always)]
pub fn block_0x002023ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2106352u32)?;
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2106356u32;
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
pub fn block_0x002023f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106360u32;
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
pub fn block_0x002023f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2106364u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2106368u32)?;
    emu.lw_no_count(12usize, 10usize, 0u32, 2106372u32)?;
    emu.lw_no_count(13usize, 12usize, 0u32, 2106376u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2106380u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2106412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020242c));
    } else {
        emu.pc = 2106384u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202410));
    }
}
#[inline(always)]
pub fn block_0x00202410(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 8u32, 2106388u32);
    emu.sli_no_count(11usize, 11usize, 2u32, 2106392u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2106396u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 432u32, 2106400u32);
    emu.adr_no_count(11usize, 11usize, 13usize, 2106404u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2106408u32)?;
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2106412u32;
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
pub fn block_0x0020242c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2106416u32;
    emu.update_insn_clock();
    emu.xrr_no_count(11usize, 13usize, 11usize, 2106420u32);
    emu.sli_no_count(11usize, 11usize, 2u32, 2106424u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2106428u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 432u32, 2106432u32);
    emu.adr_no_count(11usize, 11usize, 13usize, 2106436u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2106440u32)?;
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2106444u32;
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
pub fn block_0x0020244c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 4u32, 2106448u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2106452u32)?;
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2106456u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 612u32, 2106460u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2106464u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 596u32, 2106468u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2106472u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2106476u32);
    emu.apc_no_count(1usize, 2106476u32, 61440u32, 2106480u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106484u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966280u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202474(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2106488u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2106492u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106496u32;
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
pub fn block_0x00202480(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 4u32, 2106500u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2106504u32)?;
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2106508u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 724u32, 2106512u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2106516u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 708u32, 2106520u32);
    emu.adi_no_count(12usize, 0usize, 18u32, 2106524u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2106528u32);
    emu.apc_no_count(1usize, 2106528u32, 61440u32, 2106532u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106536u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966228u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002024a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2106540u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2106544u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106548u32;
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
pub fn block_0x002024b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 4u32, 2106552u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2106556u32)?;
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2106560u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 668u32, 2106564u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2106568u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 652u32, 2106572u32);
    emu.adi_no_count(12usize, 0usize, 19u32, 2106576u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2106580u32);
    emu.apc_no_count(1usize, 2106580u32, 61440u32, 2106584u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106588u32;
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
pub fn block_0x002024dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2106592u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2106596u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106600u32;
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
pub fn block_0x002024e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2106604u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 687u32, 2106608u32);
    emu.adi_no_count(12usize, 0usize, 19u32, 2106612u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2106616u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2106620u32);
    emu.apc_no_count(6usize, 2106620u32, 61440u32, 2106624u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2106628u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965620u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202504(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2106632u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 777u32, 2106636u32);
    emu.adi_no_count(12usize, 0usize, 22u32, 2106640u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2106644u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2106648u32);
    emu.apc_no_count(6usize, 2106648u32, 61440u32, 2106652u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2106656u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965592u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00202520(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 4u32, 2106660u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2106664u32)?;
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2106668u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 632u32, 2106672u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2106676u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 616u32, 2106680u32);
    emu.adi_no_count(12usize, 0usize, 19u32, 2106684u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2106688u32);
    emu.apc_no_count(1usize, 2106688u32, 61440u32, 2106692u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106696u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966068u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202548(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2106700u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2106704u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106708u32;
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
pub fn block_0x00202554(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2106712u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 742u32, 2106716u32);
    emu.adi_no_count(12usize, 0usize, 26u32, 2106720u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2106724u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2106728u32);
    emu.apc_no_count(6usize, 2106728u32, 61440u32, 2106732u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2106736u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965512u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202570(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2106740u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 768u32, 2106744u32);
    emu.adi_no_count(12usize, 0usize, 9u32, 2106748u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2106752u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2106756u32);
    emu.apc_no_count(6usize, 2106756u32, 61440u32, 2106760u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2106764u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965484u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020258c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(12usize, 2usize, 8u32, 2106768u32)?;
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2106772u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 816u32, 2106776u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2106780u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 800u32, 2106784u32);
    emu.adi_no_count(12usize, 0usize, 6u32, 2106788u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2106792u32);
    emu.apc_no_count(1usize, 2106792u32, 61440u32, 2106796u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106800u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965964u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002025b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2106804u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2106808u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106812u32;
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
pub fn block_0x002025bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2106816u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2106820u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2106824u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2106828u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2106832u32)?;
    emu.adi_no_count(11usize, 10usize, 0u32, 2106836u32);
    emu.lw_no_count(12usize, 10usize, 4u32, 2106840u32)?;
    emu.lw_no_count(10usize, 10usize, 12u32, 2106844u32)?;
    emu.adi_no_count(13usize, 0usize, 1u32, 2106848u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2106904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202618));
    } else {
        emu.pc = 2106852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002025e4));
    }
}
#[inline(always)]
pub fn block_0x002025e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2106908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020261c));
    } else {
        emu.pc = 2106856u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002025e8));
    }
}
#[inline(always)]
pub fn block_0x002025e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2106908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020261c));
    } else {
        emu.pc = 2106860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002025ec));
    }
}
#[inline(always)]
pub fn block_0x002025ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 0usize, 0u32, 2106864u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2106868u32);
    emu.adi_no_count(9usize, 0usize, 1u32, 2106872u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2106872u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002025f8));
}
#[inline(always)]
pub fn block_0x002025f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2106876u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2106880u32);
    emu.apc_no_count(1usize, 2106880u32, 4096u32, 2106884u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106888u32;
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
pub fn block_0x00202608(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(8usize, 2usize, 4u32, 2106892u32)?;
    emu.sw_no_count(9usize, 2usize, 8u32, 2106896u32)?;
    emu.sw_no_count(8usize, 2usize, 12u32, 2106900u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2106904u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2106920u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00202628));
}
#[inline(always)]
pub fn block_0x00202618(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2107004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020267c));
    } else {
        emu.pc = 2106908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020261c));
    }
}
#[inline(always)]
pub fn block_0x0020261c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 4u32, 2106912u32);
    emu.apc_no_count(1usize, 2106912u32, 36864u32, 2106916u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106920u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00202628(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2106920u32, 0u32, 2106924u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106928u32;
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
pub fn block_0x00202630(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2106932u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294967248u32, 2106936u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2106940u32);
    emu.adi_no_count(12usize, 0usize, 12u32, 2106944u32);
    emu.apc_no_count(1usize, 2106944u32, 4096u32, 2106948u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2106952u32;
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
pub fn block_0x00202648(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2107040u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002026a0));
    } else {
        emu.pc = 2106956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020264c));
    }
}
#[inline]
pub fn block_0x0020264c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 4u32, 2106960u32)?;
    emu.lw_no_count(12usize, 2usize, 8u32, 2106964u32)?;
    emu.lw_no_count(13usize, 2usize, 12u32, 2106968u32)?;
    emu.sw_no_count(11usize, 10usize, 0u32, 2106972u32)?;
    emu.sw_no_count(12usize, 10usize, 4u32, 2106976u32)?;
    emu.sw_no_count(13usize, 10usize, 8u32, 2106980u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2106984u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2106988u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2106992u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2106996u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2107000u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107004u32;
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
pub fn block_0x0020267c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 11usize, 0u32, 2107008u32)?;
    emu.lw_no_count(8usize, 10usize, 4u32, 2107012u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(8usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2107056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002026b0));
    } else {
        emu.pc = 2107016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202688));
    }
}
#[inline(always)]
pub fn block_0x00202688(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2107020u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2107020u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020268c));
}
#[inline(always)]
pub fn block_0x0020268c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2107024u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 824u32, 2107028u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2107032u32);
    emu.apc_no_count(1usize, 2107032u32, 36864u32, 2107036u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107040u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966536u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002026a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2107044u32);
    emu.adi_no_count(11usize, 0usize, 12u32, 2107048u32);
    emu.apc_no_count(1usize, 2107048u32, 36864u32, 2107052u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107056u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966544u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002026b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 10usize, 0u32, 2107060u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2107112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002026e8));
    } else {
        emu.pc = 2107064u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002026b8));
    }
}
#[inline(always)]
pub fn block_0x002026b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2107064u32, 0u32, 2107068u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107072u32;
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
pub fn block_0x002026c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2107076u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294967248u32, 2107080u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2107084u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2107088u32);
    emu.apc_no_count(1usize, 2107088u32, 4096u32, 2107092u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107096u32;
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
pub fn block_0x002026d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2107100u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2107104u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a != b {
        emu.pc = 2107116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002026ec));
    } else {
        emu.pc = 2107108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002026e4));
    }
}
#[inline(always)]
pub fn block_0x002026e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2107112u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2107020u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020268c));
}
#[inline(always)]
pub fn block_0x002026e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 1u32, 2107116u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2107116u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002026ec));
}
#[inline(always)]
pub fn block_0x002026ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 18usize, 0u32, 2107120u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2107124u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2106872u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002025f8));
}
#[inline(always)]
pub fn block_0x002026f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2107124u32, 28672u32, 2107128u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2107132u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294967220u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002026fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2107136u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107140u32;
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
pub fn block_0x00202704(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107144u32;
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
pub fn block_0x00202708(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2107148u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2107152u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2107156u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2107160u32)?;
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2107164u32;
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
pub fn block_0x0020271c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(9usize, 11usize, 12u32, 2107168u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2107172u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2107176u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2107180u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(9usize);
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
#[inline]
pub fn block_0x00202730(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2107188u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2107192u32)?;
    emu.lw_no_count(12usize, 2usize, 0u32, 2107196u32)?;
    emu.lw_no_count(13usize, 2usize, 4u32, 2107200u32)?;
    let a = 0u32.wrapping_add(3112902656u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2107204u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966129u32, 2107208u32);
    emu.xrr_no_count(11usize, 11usize, 14usize, 2107212u32);
    let a = 0u32.wrapping_add(1676365824u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2107216u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 44u32, 2107220u32);
    emu.xrr_no_count(13usize, 13usize, 14usize, 2107224u32);
    let a = 0u32.wrapping_add(1470513152u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2107228u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 376u32, 2107232u32);
    emu.xrr_no_count(10usize, 10usize, 14usize, 2107236u32);
    let a = 0u32.wrapping_add(3603652608u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2107240u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966637u32, 2107244u32);
    emu.xrr_no_count(12usize, 12usize, 14usize, 2107248u32);
    emu.orr_no_count(11usize, 13usize, 11usize, 2107252u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2107256u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2107260u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2107288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202798));
    } else {
        emu.pc = 2107264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202780));
    }
}
#[inline(always)]
pub fn block_0x00202780(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 4u32, 2107268u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2107272u32);
    emu.adr_no_count(11usize, 8usize, 11usize, 2107276u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2107280u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2107400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202808));
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
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2107288u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2107420u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020281c));
}
#[inline(always)]
pub fn block_0x00202798(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2107292u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2107296u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(9usize);
    let return_addr = 2107300u32;
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
pub fn block_0x002027a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2107304u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2107308u32)?;
    emu.lw_no_count(12usize, 2usize, 0u32, 2107312u32)?;
    emu.lw_no_count(13usize, 2usize, 4u32, 2107316u32)?;
    let a = 0u32.wrapping_add(2330587136u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2107320u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 428u32, 2107324u32);
    emu.xrr_no_count(11usize, 11usize, 14usize, 2107328u32);
    let a = 0u32.wrapping_add(2965131264u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2107332u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965685u32, 2107336u32);
    emu.xrr_no_count(13usize, 13usize, 14usize, 2107340u32);
    let a = 0u32.wrapping_add(4112453632u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2107344u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1473u32, 2107348u32);
    emu.xrr_no_count(10usize, 10usize, 14usize, 2107352u32);
    let a = 0u32.wrapping_add(2020298752u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2107356u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965580u32, 2107360u32);
    emu.xrr_no_count(12usize, 12usize, 14usize, 2107364u32);
    emu.orr_no_count(11usize, 13usize, 11usize, 2107368u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2107372u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2107376u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2107400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00202808));
    } else {
        emu.pc = 2107380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002027f4));
    }
}
#[inline(always)]
pub fn block_0x002027f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 4u32, 2107384u32);
    emu.adi_no_count(11usize, 0usize, 8u32, 2107388u32);
    emu.adr_no_count(11usize, 8usize, 11usize, 2107392u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2107396u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
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
    emu.lw_no_count(1usize, 2usize, 28u32, 2107404u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2107408u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2107412u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2107416u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107420u32;
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
pub fn block_0x0020281c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2107424u32)?;
    emu.apc_no_count(1usize, 2107424u32, 0u32, 2107428u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2107432u32;
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
