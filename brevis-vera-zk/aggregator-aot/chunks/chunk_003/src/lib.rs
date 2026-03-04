pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2109932u32;
pub const PC_MAX: u32 = 2112284u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 107usize] = [
        block_0x002031ec,
        block_0x00203200,
        block_0x00203214,
        block_0x00203228,
        block_0x0020323c,
        block_0x00203250,
        block_0x00203264,
        block_0x00203278,
        block_0x0020328c,
        block_0x002032a0,
        block_0x002032b4,
        block_0x002032c8,
        block_0x002032dc,
        block_0x002032f0,
        block_0x00203308,
        block_0x0020331c,
        block_0x00203328,
        block_0x00203344,
        block_0x0020335c,
        block_0x00203364,
        block_0x0020336c,
        block_0x00203380,
        block_0x00203394,
        block_0x00203398,
        block_0x002033b0,
        block_0x002033d8,
        block_0x00203408,
        block_0x00203410,
        block_0x00203430,
        block_0x0020344c,
        block_0x00203464,
        block_0x0020347c,
        block_0x00203490,
        block_0x0020349c,
        block_0x002034b0,
        block_0x002034c4,
        block_0x002034d8,
        block_0x002034dc,
        block_0x002034ec,
        block_0x002034fc,
        block_0x0020350c,
        block_0x0020351c,
        block_0x00203530,
        block_0x0020353c,
        block_0x00203550,
        block_0x00203590,
        block_0x002035b0,
        block_0x002035bc,
        block_0x002035cc,
        block_0x002035e4,
        block_0x0020361c,
        block_0x00203630,
        block_0x00203670,
        block_0x00203694,
        block_0x002036a4,
        block_0x002036bc,
        block_0x002036d8,
        block_0x002036f0,
        block_0x00203708,
        block_0x0020370c,
        block_0x00203718,
        block_0x00203728,
        block_0x00203738,
        block_0x00203748,
        block_0x0020376c,
        block_0x0020377c,
        block_0x00203780,
        block_0x002037a4,
        block_0x002037d4,
        block_0x00203808,
        block_0x00203830,
        block_0x0020383c,
        block_0x00203840,
        block_0x00203858,
        block_0x00203860,
        block_0x00203868,
        block_0x0020386c,
        block_0x00203878,
        block_0x00203880,
        block_0x00203884,
        block_0x00203890,
        block_0x0020389c,
        block_0x002038a4,
        block_0x002038a8,
        block_0x002038ac,
        block_0x002038b4,
        block_0x002038b8,
        block_0x002038c4,
        block_0x002038c8,
        block_0x002038d8,
        block_0x00203908,
        block_0x0020391c,
        block_0x00203984,
        block_0x00203990,
        block_0x002039cc,
        block_0x00203a04,
        block_0x00203a10,
        block_0x00203a1c,
        block_0x00203a28,
        block_0x00203a5c,
        block_0x00203a68,
        block_0x00203a94,
        block_0x00203ab0,
        block_0x00203acc,
        block_0x00203af4,
        block_0x00203b00,
        block_0x00203b1c,
    ];
    const IDX: [u16; 589usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 0u16, 0u16, 0u16, 3u16, 0u16, 0u16,
        0u16, 0u16, 4u16, 0u16, 0u16, 0u16, 0u16, 5u16, 0u16, 0u16, 0u16, 0u16, 6u16,
        0u16, 0u16, 0u16, 0u16, 7u16, 0u16, 0u16, 0u16, 0u16, 8u16, 0u16, 0u16, 0u16,
        0u16, 9u16, 0u16, 0u16, 0u16, 0u16, 10u16, 0u16, 0u16, 0u16, 0u16, 11u16, 0u16,
        0u16, 0u16, 0u16, 12u16, 0u16, 0u16, 0u16, 0u16, 13u16, 0u16, 0u16, 0u16, 0u16,
        14u16, 0u16, 0u16, 0u16, 0u16, 0u16, 15u16, 0u16, 0u16, 0u16, 0u16, 16u16, 0u16,
        0u16, 17u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 18u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 19u16, 0u16, 20u16, 0u16, 21u16, 0u16, 0u16, 0u16, 0u16, 22u16, 0u16, 0u16,
        0u16, 0u16, 23u16, 24u16, 0u16, 0u16, 0u16, 0u16, 0u16, 25u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 26u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 27u16, 0u16, 28u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 29u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 30u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 31u16, 0u16, 0u16, 0u16, 0u16, 0u16, 32u16, 0u16, 0u16, 0u16, 0u16,
        33u16, 0u16, 0u16, 34u16, 0u16, 0u16, 0u16, 0u16, 35u16, 0u16, 0u16, 0u16, 0u16,
        36u16, 0u16, 0u16, 0u16, 0u16, 37u16, 38u16, 0u16, 0u16, 0u16, 39u16, 0u16, 0u16,
        0u16, 40u16, 0u16, 0u16, 0u16, 41u16, 0u16, 0u16, 0u16, 42u16, 0u16, 0u16, 0u16,
        0u16, 43u16, 0u16, 0u16, 44u16, 0u16, 0u16, 0u16, 0u16, 45u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 46u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 47u16, 0u16, 0u16, 48u16, 0u16, 0u16,
        0u16, 49u16, 0u16, 0u16, 0u16, 0u16, 0u16, 50u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 51u16, 0u16, 0u16, 0u16, 0u16,
        52u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 53u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 54u16,
        0u16, 0u16, 0u16, 55u16, 0u16, 0u16, 0u16, 0u16, 0u16, 56u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 57u16, 0u16, 0u16, 0u16, 0u16, 0u16, 58u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 59u16, 60u16, 0u16, 0u16, 61u16, 0u16, 0u16, 0u16, 62u16, 0u16, 0u16,
        0u16, 63u16, 0u16, 0u16, 0u16, 64u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 65u16, 0u16, 0u16, 0u16, 66u16, 67u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 68u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 69u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 70u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 71u16, 0u16,
        0u16, 72u16, 73u16, 0u16, 0u16, 0u16, 0u16, 0u16, 74u16, 0u16, 75u16, 0u16,
        76u16, 77u16, 0u16, 0u16, 78u16, 0u16, 79u16, 80u16, 0u16, 0u16, 81u16, 0u16,
        0u16, 82u16, 0u16, 83u16, 84u16, 85u16, 0u16, 86u16, 87u16, 0u16, 0u16, 88u16,
        89u16, 0u16, 0u16, 0u16, 90u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 91u16, 0u16, 0u16, 0u16, 0u16, 92u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 93u16, 0u16, 0u16, 94u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        95u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 96u16, 0u16, 0u16, 97u16, 0u16, 0u16, 98u16, 0u16, 0u16, 99u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 100u16, 0u16, 0u16,
        101u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 102u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 103u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 104u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 105u16, 0u16, 0u16, 106u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 107u16,
    ];
    if pc < 2109932u32 || pc > 2112284u32 {
        return None;
    }
    let word_offset = ((pc - 2109932u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x002031ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 300u32, 2109936u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2109940u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2109944u32);
    emu.apc_no_count(1usize, 2109944u32, 24576u32, 2109948u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109952u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966076u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203200(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 396u32, 2109956u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2109960u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2109964u32);
    emu.apc_no_count(1usize, 2109964u32, 24576u32, 2109968u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109972u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966056u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203214(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 492u32, 2109976u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2109980u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2109984u32);
    emu.apc_no_count(1usize, 2109984u32, 24576u32, 2109988u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2109992u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966036u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203228(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 588u32, 2109996u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2110000u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2110004u32);
    emu.apc_no_count(1usize, 2110004u32, 24576u32, 2110008u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110012u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966016u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020323c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 684u32, 2110016u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2110020u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2110024u32);
    emu.apc_no_count(1usize, 2110024u32, 24576u32, 2110028u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110032u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965996u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203250(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 780u32, 2110036u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2110040u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2110044u32);
    emu.apc_no_count(1usize, 2110044u32, 24576u32, 2110048u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110052u32;
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
pub fn block_0x00203264(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 876u32, 2110056u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2110060u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2110064u32);
    emu.apc_no_count(1usize, 2110064u32, 24576u32, 2110068u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110072u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965956u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203278(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 972u32, 2110076u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2110080u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2110084u32);
    emu.apc_no_count(1usize, 2110084u32, 24576u32, 2110088u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110092u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965936u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020328c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1068u32, 2110096u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2110100u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2110104u32);
    emu.apc_no_count(1usize, 2110104u32, 24576u32, 2110108u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110112u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965916u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002032a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1164u32, 2110116u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2110120u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2110124u32);
    emu.apc_no_count(1usize, 2110124u32, 24576u32, 2110128u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110132u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965896u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002032b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1260u32, 2110136u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2110140u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2110144u32);
    emu.apc_no_count(1usize, 2110144u32, 24576u32, 2110148u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110152u32;
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
pub fn block_0x002032c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1356u32, 2110156u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2110160u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2110164u32);
    emu.apc_no_count(1usize, 2110164u32, 24576u32, 2110168u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110172u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965856u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002032dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1452u32, 2110176u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2110180u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2110184u32);
    emu.apc_no_count(1usize, 2110184u32, 24576u32, 2110188u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110192u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965836u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002032f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 12u32, 2110196u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2110200u32);
    emu.adi_no_count(8usize, 2usize, 12u32, 2110204u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2110208u32);
    emu.apc_no_count(1usize, 2110208u32, 24576u32, 2110212u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110216u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965812u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203308(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 96u32, 2110220u32);
    emu.adi_no_count(10usize, 20usize, 0u32, 2110224u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2110228u32);
    emu.apc_no_count(1usize, 2110228u32, 24576u32, 2110232u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110236u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x0020331c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 2u32, 2110240u32);
    emu.adi_no_count(20usize, 0usize, 16u32, 2110244u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2110248u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2110308u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203364));
}
#[inline(always)]
pub fn block_0x00203328(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 9usize, 4u32, 2110252u32);
    emu.sli_no_count(11usize, 9usize, 6u32, 2110256u32);
    emu.sbr_no_count(11usize, 11usize, 10usize, 2110260u32);
    emu.adr_no_count(11usize, 8usize, 11usize, 2110264u32);
    emu.adi_no_count(10usize, 2usize, 1932u32, 2110268u32);
    emu.apc_no_count(1usize, 2110268u32, 8192u32, 2110272u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110276u32;
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
pub fn block_0x00203344(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 9usize, 1u32, 2110280u32);
    emu.adi_no_count(11usize, 2usize, 1932u32, 2110284u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2110288u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2110292u32);
    emu.apc_no_count(1usize, 2110292u32, 24576u32, 2110296u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110300u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965728u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020335c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 19usize, 96u32, 2110304u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2110360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203398));
    } else {
        emu.pc = 2110308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203364));
    }
}
#[inline(always)]
pub fn block_0x00203364(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 9usize, 1u32, 2110312u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2110248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203328));
    } else {
        emu.pc = 2110316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020336c));
    }
}
#[inline(always)]
pub fn block_0x0020336c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 19usize, 4294967200u32, 2110320u32);
    emu.adi_no_count(10usize, 2usize, 2028u32, 2110324u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2110328u32);
    emu.apc_no_count(1usize, 2110328u32, 24576u32, 2110332u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110336u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965692u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203380(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1932u32, 2110340u32);
    emu.adi_no_count(11usize, 2usize, 2028u32, 2110344u32);
    emu.adi_no_count(12usize, 18usize, 0u32, 2110348u32);
    emu.apc_no_count(1usize, 2110348u32, 4096u32, 2110352u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110356u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967212u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203394(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2110360u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2110276u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203344));
}
#[inline(always)]
pub fn block_0x00203398(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2110364u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965360u32, 2110368u32);
    emu.adi_no_count(10usize, 2usize, 1548u32, 2110372u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2110376u32);
    emu.apc_no_count(1usize, 2110376u32, 24576u32, 2110380u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110384u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965644u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002033b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 2usize, 1676u32, 2110388u32);
    emu.adi_no_count(20usize, 2usize, 1708u32, 2110392u32);
    emu.adi_no_count(21usize, 2usize, 2047u32, 2110396u32);
    emu.adi_no_count(21usize, 21usize, 13u32, 2110400u32);
    emu.adi_no_count(22usize, 2usize, 2047u32, 2110404u32);
    emu.adi_no_count(22usize, 22usize, 45u32, 2110408u32);
    emu.adi_no_count(26usize, 0usize, 252u32, 2110412u32);
    emu.adi_no_count(10usize, 0usize, 31u32, 2110416u32);
    emu.adi_no_count(27usize, 2usize, 172u32, 2110420u32);
    emu.adi_no_count(8usize, 0usize, 16u32, 2110424u32);
    emu.add_memory_rw_events(10usize);
    emu.pc = 2110424u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002033d8));
}
#[inline]
pub fn block_0x002033d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 8u32, 2110428u32)?;
    emu.adr_no_count(10usize, 11usize, 10usize, 2110432u32);
    emu.lbu_no_count(10usize, 10usize, 0u32, 2110436u32);
    emu.ani_no_count(11usize, 26usize, 4u32, 2110440u32);
    emu.srr_no_count(9usize, 10usize, 11usize, 2110444u32);
    emu.ani_no_count(9usize, 9usize, 15u32, 2110448u32);
    emu.adi_no_count(10usize, 2usize, 1644u32, 2110452u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2110456u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2110460u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965360u32, 2110464u32);
    emu.apc_no_count(1usize, 2110464u32, 24576u32, 2110468u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110472u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965556u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203408(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2110476u32);
    emu.adi_no_count(23usize, 27usize, 0u32, 2110480u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2110480u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203410));
}
#[inline(always)]
pub fn block_0x00203410(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 1u32, 2110484u32);
    emu.adi_no_count(24usize, 23usize, 4294967232u32, 2110488u32);
    emu.xrr_no_count(10usize, 10usize, 9usize, 2110492u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2110496u32);
    emu.sli_no_count(10usize, 10usize, 23u32, 2110500u32);
    emu.sri_no_count(10usize, 10usize, 31u32, 2110504u32);
    emu.apc_no_count(1usize, 2110504u32, 77824u32, 2110508u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110512u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967292u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203430(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(25usize, 10usize, 255u32, 2110516u32);
    emu.adi_no_count(10usize, 2usize, 2028u32, 2110520u32);
    emu.adi_no_count(11usize, 2usize, 1644u32, 2110524u32);
    emu.adi_no_count(12usize, 24usize, 0u32, 2110528u32);
    emu.adi_no_count(13usize, 25usize, 0u32, 2110532u32);
    emu.apc_no_count(1usize, 2110532u32, 57344u32, 2110536u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110540u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965576u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020344c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 23usize, 4294967264u32, 2110544u32);
    emu.adi_no_count(10usize, 21usize, 0u32, 2110548u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2110552u32);
    emu.adi_no_count(13usize, 25usize, 0u32, 2110556u32);
    emu.apc_no_count(1usize, 2110556u32, 57344u32, 2110560u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110564u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965552u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203464(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 22usize, 0u32, 2110568u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2110572u32);
    emu.adi_no_count(12usize, 23usize, 0u32, 2110576u32);
    emu.adi_no_count(13usize, 25usize, 0u32, 2110580u32);
    emu.apc_no_count(1usize, 2110580u32, 57344u32, 2110584u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110588u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965528u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020347c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1644u32, 2110592u32);
    emu.adi_no_count(11usize, 2usize, 2028u32, 2110596u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2110600u32);
    emu.apc_no_count(1usize, 2110600u32, 24576u32, 2110604u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110608u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965420u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203490(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(23usize, 23usize, 96u32, 2110612u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2110616u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a != b {
        emu.pc = 2110480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203410));
    } else {
        emu.pc = 2110620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020349c));
    }
}
#[inline(always)]
pub fn block_0x0020349c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 2028u32, 2110624u32);
    emu.adi_no_count(11usize, 2usize, 1548u32, 2110628u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2110632u32);
    emu.apc_no_count(1usize, 2110632u32, 24576u32, 2110636u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110640u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965388u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002034b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1932u32, 2110644u32);
    emu.adi_no_count(11usize, 2usize, 2028u32, 2110648u32);
    emu.adi_no_count(12usize, 2usize, 1644u32, 2110652u32);
    emu.apc_no_count(1usize, 2110652u32, 4096u32, 2110656u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110660u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966908u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002034c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1548u32, 2110664u32);
    emu.adi_no_count(11usize, 2usize, 1932u32, 2110668u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2110672u32);
    emu.apc_no_count(1usize, 2110672u32, 24576u32, 2110676u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110680u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965348u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002034d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a == b {
        emu.pc = 2110780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020353c));
    } else {
        emu.pc = 2110684u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002034dc));
    }
}
#[inline(always)]
pub fn block_0x002034dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 2028u32, 2110688u32);
    emu.adi_no_count(11usize, 2usize, 1548u32, 2110692u32);
    emu.apc_no_count(1usize, 2110692u32, 8192u32, 2110696u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110700u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966408u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002034ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1932u32, 2110704u32);
    emu.adi_no_count(11usize, 2usize, 2028u32, 2110708u32);
    emu.apc_no_count(1usize, 2110708u32, 8192u32, 2110712u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110716u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966392u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002034fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1836u32, 2110720u32);
    emu.adi_no_count(11usize, 2usize, 1932u32, 2110724u32);
    emu.apc_no_count(1usize, 2110724u32, 8192u32, 2110728u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110732u32;
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
#[inline(always)]
pub fn block_0x0020350c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1740u32, 2110736u32);
    emu.adi_no_count(11usize, 2usize, 1836u32, 2110740u32);
    emu.apc_no_count(1usize, 2110740u32, 8192u32, 2110744u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110748u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966360u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020351c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1548u32, 2110752u32);
    emu.adi_no_count(11usize, 2usize, 1740u32, 2110756u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2110760u32);
    emu.apc_no_count(1usize, 2110760u32, 24576u32, 2110764u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110768u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965260u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203530(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 26usize, 4294967292u32, 2110772u32);
    emu.sri_no_count(10usize, 26usize, 3u32, 2110776u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2110780u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2110424u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002033d8));
}
#[inline(always)]
pub fn block_0x0020353c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 2usize, 1548u32, 2110784u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2110788u32);
    emu.lw_no_count(10usize, 2usize, 4u32, 2110792u32)?;
    emu.apc_no_count(1usize, 2110792u32, 20480u32, 2110796u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110800u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(2028u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00203550(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 144u32, 2110804u32);
    emu.lw_no_count(1usize, 2usize, 2028u32, 2110808u32)?;
    emu.lw_no_count(8usize, 2usize, 2024u32, 2110812u32)?;
    emu.lw_no_count(9usize, 2usize, 2020u32, 2110816u32)?;
    emu.lw_no_count(18usize, 2usize, 2016u32, 2110820u32)?;
    emu.lw_no_count(19usize, 2usize, 2012u32, 2110824u32)?;
    emu.lw_no_count(20usize, 2usize, 2008u32, 2110828u32)?;
    emu.lw_no_count(21usize, 2usize, 2004u32, 2110832u32)?;
    emu.lw_no_count(22usize, 2usize, 2000u32, 2110836u32)?;
    emu.lw_no_count(23usize, 2usize, 1996u32, 2110840u32)?;
    emu.lw_no_count(24usize, 2usize, 1992u32, 2110844u32)?;
    emu.lw_no_count(25usize, 2usize, 1988u32, 2110848u32)?;
    emu.lw_no_count(26usize, 2usize, 1984u32, 2110852u32)?;
    emu.lw_no_count(27usize, 2usize, 1980u32, 2110856u32)?;
    emu.adi_no_count(2usize, 2usize, 2032u32, 2110860u32);
    emu.add_memory_rw_events(16usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2110864u32;
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
pub fn block_0x00203590(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294966944u32, 2110868u32);
    emu.sw_no_count(1usize, 2usize, 348u32, 2110872u32)?;
    emu.sw_no_count(8usize, 2usize, 344u32, 2110876u32)?;
    emu.sw_no_count(9usize, 2usize, 340u32, 2110880u32)?;
    emu.sw_no_count(18usize, 2usize, 336u32, 2110884u32)?;
    emu.sw_no_count(19usize, 2usize, 332u32, 2110888u32)?;
    emu.sw_no_count(20usize, 2usize, 328u32, 2110892u32)?;
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2111356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020377c));
    } else {
        emu.pc = 2110896u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002035b0));
    }
}
#[inline(always)]
pub fn block_0x002035b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(13usize, 11usize, 0u32, 2110900u32);
    emu.adi_no_count(14usize, 0usize, 5u32, 2110904u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2111356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020377c));
    } else {
        emu.pc = 2110908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002035bc));
    }
}
#[inline(always)]
pub fn block_0x002035bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 61u32, 2110912u32);
    emu.srr_no_count(14usize, 14usize, 13usize, 2110916u32);
    emu.ani_no_count(14usize, 14usize, 1u32, 2110920u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2111356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020377c));
    } else {
        emu.pc = 2110924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002035cc));
    }
}
#[inline(always)]
pub fn block_0x002035cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(13usize, 13usize, 2u32, 2110928u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2110932u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294967168u32, 2110936u32);
    emu.adr_no_count(13usize, 14usize, 13usize, 2110940u32);
    emu.lw_no_count(13usize, 13usize, 0u32, 2110944u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2111356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020377c));
    } else {
        emu.pc = 2110948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002035e4));
    }
}
#[inline]
pub fn block_0x002035e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2110952u32);
    emu.sltiu_no_count(10usize, 12usize, 65u32, 2110956u32);
    emu.adi_no_count(13usize, 0usize, 65u32, 2110960u32);
    emu.sbr_no_count(13usize, 13usize, 12usize, 2110964u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2110968u32);
    emu.anr_no_count(13usize, 10usize, 13usize, 2110972u32);
    emu.adi_no_count(10usize, 2usize, 68u32, 2110976u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2110980u32);
    emu.adi_no_count(8usize, 11usize, 0u32, 2110984u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2110988u32);
    emu.adi_no_count(9usize, 12usize, 0u32, 2110992u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2110996u32);
    emu.apc_no_count(1usize, 2110996u32, 20480u32, 2111000u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111004u32;
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
pub fn block_0x0020361c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 68u32, 2111008u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2111012u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2111016u32);
    emu.apc_no_count(1usize, 2111016u32, 20480u32, 2111020u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111024u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1804u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00203630(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 69u32, 2111028u32);
    emu.lbu_no_count(11usize, 2usize, 68u32, 2111032u32);
    emu.lbu_no_count(12usize, 2usize, 70u32, 2111036u32);
    emu.lbu_no_count(8usize, 2usize, 71u32, 2111040u32);
    emu.sli_no_count(10usize, 10usize, 8u32, 2111044u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2111048u32);
    emu.lbu_no_count(9usize, 2usize, 72u32, 2111052u32);
    emu.lbu_no_count(19usize, 2usize, 73u32, 2111056u32);
    emu.lbu_no_count(20usize, 2usize, 74u32, 2111060u32);
    emu.sh_no_count(10usize, 2usize, 2u32, 2111064u32)?;
    emu.sb_no_count(12usize, 2usize, 4u32, 2111068u32);
    emu.adi_no_count(11usize, 2usize, 75u32, 2111072u32);
    emu.adi_no_count(10usize, 2usize, 204u32, 2111076u32);
    emu.adi_no_count(12usize, 0usize, 58u32, 2111080u32);
    emu.apc_no_count(1usize, 2111080u32, 20480u32, 2111084u32);
    emu.add_memory_rw_events(16usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111088u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1740u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00203670(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(8usize, 2usize, 5u32, 2111092u32);
    emu.sb_no_count(9usize, 2usize, 6u32, 2111096u32);
    emu.sb_no_count(19usize, 2usize, 7u32, 2111100u32);
    emu.sb_no_count(20usize, 2usize, 8u32, 2111104u32);
    emu.adi_no_count(10usize, 2usize, 9u32, 2111108u32);
    emu.adi_no_count(11usize, 2usize, 204u32, 2111112u32);
    emu.adi_no_count(12usize, 0usize, 58u32, 2111116u32);
    emu.apc_no_count(1usize, 2111116u32, 20480u32, 2111120u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111124u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1704u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203694(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 68u32, 2111128u32);
    emu.adi_no_count(11usize, 2usize, 2u32, 2111132u32);
    emu.apc_no_count(1usize, 2111132u32, 4294963200u32, 2111136u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111140u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966420u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002036a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2111144u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967100u32, 2111148u32);
    emu.adi_no_count(10usize, 2usize, 204u32, 2111152u32);
    emu.adi_no_count(12usize, 0usize, 68u32, 2111156u32);
    emu.apc_no_count(1usize, 2111156u32, 20480u32, 2111160u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111164u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1664u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002036bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(8usize, 2usize, 136u32, 2111168u32);
    emu.adi_no_count(10usize, 2usize, 140u32, 2111172u32);
    emu.adi_no_count(11usize, 2usize, 204u32, 2111176u32);
    emu.adi_no_count(12usize, 2usize, 68u32, 2111180u32);
    emu.adi_no_count(13usize, 8usize, 0u32, 2111184u32);
    emu.apc_no_count(1usize, 2111184u32, 53248u32, 2111188u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111192u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1724u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002036d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 2usize, 236u32, 2111196u32);
    emu.adi_no_count(12usize, 2usize, 100u32, 2111200u32);
    emu.adi_no_count(10usize, 2usize, 172u32, 2111204u32);
    emu.adi_no_count(13usize, 8usize, 0u32, 2111208u32);
    emu.apc_no_count(1usize, 2111208u32, 53248u32, 2111212u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111216u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1700u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002036f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 2u32, 2111220u32);
    emu.lbu_no_count(9usize, 2usize, 268u32, 2111224u32);
    emu.lbu_no_count(19usize, 2usize, 132u32, 2111228u32);
    emu.adi_no_count(11usize, 10usize, 4294967294u32, 2111232u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2111236u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2111244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020370c));
    } else {
        emu.pc = 2111240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203708));
    }
}
#[inline(always)]
pub fn block_0x00203708(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2111396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002037a4));
    } else {
        emu.pc = 2111244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020370c));
    }
}
#[inline(always)]
pub fn block_0x0020370c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(10usize, 10usize, 1u32, 2111248u32);
    emu.apc_no_count(1usize, 2111248u32, 77824u32, 2111252u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111256u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966548u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203718(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(10usize, 10usize, 4294967295u32, 2111260u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2111264u32);
    emu.apc_no_count(1usize, 2111264u32, 77824u32, 2111268u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111272u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966532u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203728(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 2usize, 136u32, 2111276u32);
    emu.anr_no_count(10usize, 10usize, 11usize, 2111280u32);
    emu.apc_no_count(1usize, 2111280u32, 77824u32, 2111284u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111288u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966516u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203738(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(12usize, 10usize, 255u32, 2111292u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2111296u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2111300u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2111360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203780));
    } else {
        emu.pc = 2111304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203748));
    }
}
#[inline]
pub fn block_0x00203748(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xrr_no_count(11usize, 19usize, 9usize, 2111308u32);
    emu.sbr_no_count(12usize, 0usize, 8usize, 2111312u32);
    emu.anr_no_count(11usize, 11usize, 12usize, 2111316u32);
    emu.xrr_no_count(9usize, 11usize, 9usize, 2111320u32);
    emu.adi_no_count(10usize, 10usize, 4u32, 2111324u32);
    emu.adi_no_count(11usize, 2usize, 140u32, 2111328u32);
    emu.adi_no_count(12usize, 0usize, 64u32, 2111332u32);
    emu.apc_no_count(1usize, 2111332u32, 20480u32, 2111336u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111340u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1488u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020376c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2111344u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2111348u32);
    emu.sb_no_count(9usize, 18usize, 68u32, 2111352u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2111356u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2111360u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203780));
}
#[inline(always)]
pub fn block_0x0020377c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2111360u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2111360u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203780));
}
#[inline]
pub fn block_0x00203780(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 10usize, 0u32, 2111364u32)?;
    emu.lw_no_count(1usize, 2usize, 348u32, 2111368u32)?;
    emu.lw_no_count(8usize, 2usize, 344u32, 2111372u32)?;
    emu.lw_no_count(9usize, 2usize, 340u32, 2111376u32)?;
    emu.lw_no_count(18usize, 2usize, 336u32, 2111380u32)?;
    emu.lw_no_count(19usize, 2usize, 332u32, 2111384u32)?;
    emu.lw_no_count(20usize, 2usize, 328u32, 2111388u32)?;
    emu.adi_no_count(2usize, 2usize, 352u32, 2111392u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111396u32;
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
pub fn block_0x002037a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 7u32, 2111400u32);
    emu.sw_no_count(10usize, 2usize, 272u32, 2111404u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2111408u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 420u32, 2111412u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2111416u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 404u32, 2111420u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2111424u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 432u32, 2111428u32);
    emu.adi_no_count(11usize, 0usize, 11u32, 2111432u32);
    emu.adi_no_count(12usize, 2usize, 272u32, 2111436u32);
    emu.apc_no_count(1usize, 2111436u32, 102400u32, 2111440u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111444u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(260u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002037d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2111448u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2111452u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2111456u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2111460u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2111464u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2111468u32)?;
    emu.sw_no_count(20usize, 2usize, 24u32, 2111472u32)?;
    emu.sw_no_count(21usize, 2usize, 20u32, 2111476u32)?;
    emu.sw_no_count(22usize, 2usize, 16u32, 2111480u32)?;
    emu.sw_no_count(23usize, 2usize, 12u32, 2111484u32)?;
    emu.sw_no_count(24usize, 2usize, 8u32, 2111488u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2111492u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2111672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002038b8));
    } else {
        emu.pc = 2111496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203808));
    }
}
#[inline]
pub fn block_0x00203808(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 13usize, 0u32, 2111500u32);
    emu.adi_no_count(18usize, 12usize, 0u32, 2111504u32);
    emu.adi_no_count(19usize, 11usize, 0u32, 2111508u32);
    emu.adi_no_count(21usize, 0usize, 4u32, 2111512u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(20usize, a);
    emu.pc = 2111516u32;
    emu.update_insn_clock();
    emu.adi_no_count(20usize, 20usize, 4294966016u32, 2111520u32);
    emu.adi_no_count(22usize, 0usize, 1u32, 2111524u32);
    emu.adi_no_count(23usize, 0usize, 35u32, 2111528u32);
    emu.adi_no_count(24usize, 0usize, 2u32, 2111532u32);
    emu.add_memory_rw_events(10usize);
    let return_addr = 2111536u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2111552u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00203840));
}
#[inline(always)]
pub fn block_0x00203830(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 4u32, 2111540u32)?;
    emu.lbu_no_count(11usize, 11usize, 8u32, 2111544u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2111688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002038c8));
    } else {
        emu.pc = 2111548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020383c));
    }
}
#[inline(always)]
pub fn block_0x0020383c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2111672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002038b8));
    } else {
        emu.pc = 2111552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203840));
    }
}
#[inline(always)]
pub fn block_0x00203840(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2111556u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2111560u32);
    emu.adi_no_count(12usize, 18usize, 0u32, 2111564u32);
    emu.adi_no_count(13usize, 9usize, 0u32, 2111568u32);
    emu.apc_no_count(1usize, 2111568u32, 24576u32, 2111572u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111576u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965912u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203858(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 2usize, 0u32, 2111580u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2111608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203878));
    } else {
        emu.pc = 2111584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203860));
    }
}
#[inline(always)]
pub fn block_0x00203860(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 4u32, 2111588u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2111684u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002038c4));
    } else {
        emu.pc = 2111592u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203868));
    }
}
#[inline(always)]
pub fn block_0x00203868(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a < b {
        emu.pc = 2111752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203908));
    } else {
        emu.pc = 2111596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020386c));
    }
}
#[inline(always)]
pub fn block_0x0020386c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(9usize, 9usize, 10usize, 2111600u32);
    emu.adr_no_count(18usize, 18usize, 10usize, 2111604u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2111608u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2111548u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020383c));
}
#[inline(always)]
pub fn block_0x00203878(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2111612u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(22usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2111656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002038a8));
    } else {
        emu.pc = 2111616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203880));
    }
}
#[inline(always)]
pub fn block_0x00203880(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2111536u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203830));
    } else {
        emu.pc = 2111620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203884));
    }
}
#[inline(always)]
pub fn block_0x00203884(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 4u32, 2111624u32)?;
    emu.lbu_no_count(12usize, 11usize, 8u32, 2111628u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2111688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002038c8));
    } else {
        emu.pc = 2111632u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00203890));
    }
}
#[inline(always)]
pub fn block_0x00203890(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 11usize, 4u32, 2111636u32)?;
    emu.lw_no_count(12usize, 10usize, 0u32, 2111640u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2111548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020383c));
    } else {
        emu.pc = 2111644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020389c));
    }
}
#[inline(always)]
pub fn block_0x0020389c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 11usize, 0u32, 2111648u32)?;
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2111652u32;
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
pub fn block_0x002038a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2111656u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2111548u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020383c));
}
#[inline(always)]
pub fn block_0x002038a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2111688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002038c8));
    } else {
        emu.pc = 2111660u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002038ac));
    }
}
#[inline(always)]
pub fn block_0x002038ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 2usize, 1u32, 2111664u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2111548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020383c));
    } else {
        emu.pc = 2111668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002038b4));
    }
}
#[inline(always)]
pub fn block_0x002038b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2111672u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2111688u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002038c8));
}
#[inline(always)]
pub fn block_0x002038b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2111676u32);
    emu.sb_no_count(10usize, 8usize, 0u32, 2111680u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2111684u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2111704u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002038d8));
}
#[inline(always)]
pub fn block_0x002038c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 20usize, 0u32, 2111688u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2111688u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002038c8));
}
#[inline(always)]
pub fn block_0x002038c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2111692u32)?;
    emu.lw_no_count(10usize, 10usize, 4u32, 2111696u32)?;
    emu.sw_no_count(11usize, 8usize, 0u32, 2111700u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2111704u32)?;
    emu.add_memory_rw_events(4usize);
    emu.pc = 2111704u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002038d8));
}
#[inline]
pub fn block_0x002038d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2111708u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2111712u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2111716u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2111720u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2111724u32)?;
    emu.lw_no_count(20usize, 2usize, 24u32, 2111728u32)?;
    emu.lw_no_count(21usize, 2usize, 20u32, 2111732u32)?;
    emu.lw_no_count(22usize, 2usize, 16u32, 2111736u32)?;
    emu.lw_no_count(23usize, 2usize, 12u32, 2111740u32)?;
    emu.lw_no_count(24usize, 2usize, 8u32, 2111744u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2111748u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111752u32;
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
pub fn block_0x00203908(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2111756u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966024u32, 2111760u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2111764u32);
    emu.apc_no_count(1usize, 2111764u32, 110592u32, 2111768u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111772u32;
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
#[inline(never)]
pub fn block_0x0020391c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 26u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2111776u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2111780u32)?;
    emu.adi_no_count(5usize, 11usize, 0u32, 2111784u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2111788u32)?;
    emu.adi_no_count(15usize, 10usize, 8u32, 2111792u32);
    emu.sw_no_count(10usize, 2usize, 24u32, 2111796u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2111800u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966056u32, 2111804u32);
    emu.adi_no_count(6usize, 2usize, 24u32, 2111808u32);
    emu.adi_no_count(7usize, 0usize, 8u32, 2111812u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2111816u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966072u32, 2111820u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2111824u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966096u32, 2111828u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2111832u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 4294966040u32, 2111836u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2111840u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 17usize, 4294966196u32, 2111844u32);
    emu.adi_no_count(12usize, 0usize, 5u32, 2111848u32);
    emu.adi_no_count(14usize, 0usize, 4u32, 2111852u32);
    emu.sw_no_count(7usize, 2usize, 0u32, 2111856u32)?;
    emu.sw_no_count(6usize, 2usize, 4u32, 2111860u32)?;
    emu.sw_no_count(10usize, 2usize, 8u32, 2111864u32)?;
    emu.adi_no_count(10usize, 5usize, 0u32, 2111868u32);
    emu.apc_no_count(1usize, 2111868u32, 106496u32, 2111872u32);
    emu.add_memory_rw_events(26usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111876u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966772u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203984(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2111880u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2111884u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2111888u32;
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
pub fn block_0x00203990(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2111892u32)?;
    emu.lbu_no_count(10usize, 10usize, 0u32, 2111896u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2111900u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967192u32, 2111904u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2111908u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 64u32, 2111912u32);
    emu.sli_no_count(10usize, 10usize, 2u32, 2111916u32);
    emu.adr_no_count(12usize, 12usize, 10usize, 2111920u32);
    emu.adr_no_count(10usize, 13usize, 10usize, 2111924u32);
    emu.lw_no_count(12usize, 12usize, 0u32, 2111928u32)?;
    emu.lw_no_count(13usize, 10usize, 0u32, 2111932u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2111936u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2111940u32);
    emu.apc_no_count(6usize, 2111940u32, 106496u32, 2111944u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2111948u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966364u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002039cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2111952u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2111956u32)?;
    emu.adi_no_count(15usize, 11usize, 0u32, 2111960u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2111964u32)?;
    emu.sw_no_count(10usize, 2usize, 8u32, 2111968u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2111972u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966244u32, 2111976u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2111980u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966228u32, 2111984u32);
    emu.adi_no_count(12usize, 0usize, 6u32, 2111988u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2111992u32);
    emu.adi_no_count(10usize, 15usize, 0u32, 2111996u32);
    emu.apc_no_count(1usize, 2111996u32, 106496u32, 2112000u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112004u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967060u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203a04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2112008u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2112012u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112016u32;
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
pub fn block_0x00203a10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2112020u32)?;
    emu.apc_no_count(6usize, 2112020u32, 77824u32, 2112024u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2112028u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965264u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203a1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2112032u32)?;
    emu.apc_no_count(6usize, 2112032u32, 90112u32, 2112036u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2112040u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(980u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00203a28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2112044u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2112048u32)?;
    emu.adi_no_count(15usize, 11usize, 0u32, 2112052u32);
    emu.sw_no_count(10usize, 2usize, 8u32, 2112056u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2112060u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966244u32, 2112064u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2112068u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966228u32, 2112072u32);
    emu.adi_no_count(12usize, 0usize, 6u32, 2112076u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2112080u32);
    emu.adi_no_count(10usize, 15usize, 0u32, 2112084u32);
    emu.apc_no_count(1usize, 2112084u32, 106496u32, 2112088u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112092u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966972u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203a5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2112096u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2112100u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112104u32;
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
pub fn block_0x00203a68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2112108u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2112112u32)?;
    emu.adi_no_count(12usize, 10usize, 0u32, 2112116u32);
    emu.lbu_no_count(10usize, 10usize, 0u32, 2112120u32);
    emu.sli_no_count(10usize, 10usize, 2u32, 2112124u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2112128u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294965456u32, 2112132u32);
    emu.adr_no_count(10usize, 10usize, 13usize, 2112136u32);
    emu.lw_no_count(13usize, 10usize, 0u32, 2112140u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2112144u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2112148u32;
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
pub fn block_0x00203a94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2112152u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966204u32, 2112156u32);
    emu.adi_no_count(12usize, 0usize, 8u32, 2112160u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2112164u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2112168u32);
    emu.apc_no_count(6usize, 2112168u32, 106496u32, 2112172u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2112176u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966136u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203ab0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2112180u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966479u32, 2112184u32);
    emu.adi_no_count(12usize, 0usize, 6u32, 2112188u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2112192u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2112196u32);
    emu.apc_no_count(6usize, 2112196u32, 106496u32, 2112200u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2112204u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966108u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00203acc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 4u32, 2112208u32);
    emu.sw_no_count(12usize, 2usize, 24u32, 2112212u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2112216u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966476u32, 2112220u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2112224u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966460u32, 2112228u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2112232u32);
    emu.adi_no_count(13usize, 2usize, 24u32, 2112236u32);
    emu.apc_no_count(1usize, 2112236u32, 106496u32, 2112240u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112244u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966820u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00203af4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2112248u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2112252u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112256u32;
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
pub fn block_0x00203b00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2112260u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966437u32, 2112264u32);
    emu.adi_no_count(12usize, 0usize, 11u32, 2112268u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2112272u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2112276u32);
    emu.apc_no_count(6usize, 2112276u32, 106496u32, 2112280u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2112284u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966028u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00203b1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 1u32, 2112288u32);
    emu.sw_no_count(12usize, 2usize, 24u32, 2112292u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2112296u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966348u32, 2112300u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2112304u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966332u32, 2112308u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2112312u32);
    emu.adi_no_count(13usize, 2usize, 24u32, 2112316u32);
    emu.apc_no_count(1usize, 2112316u32, 106496u32, 2112320u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2112324u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966740u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
