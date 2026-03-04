pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2161776u32;
pub const PC_MAX: u32 = 2164428u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 114usize] = [
        block_0x0020fc70,
        block_0x0020fce0,
        block_0x0020fd18,
        block_0x0020fd38,
        block_0x0020fd50,
        block_0x0020fd60,
        block_0x0020fd68,
        block_0x0020fd78,
        block_0x0020fd80,
        block_0x0020fd88,
        block_0x0020fd98,
        block_0x0020fda0,
        block_0x0020fed8,
        block_0x0020fee8,
        block_0x0020fef0,
        block_0x0020ff00,
        block_0x0020ff0c,
        block_0x0020ff1c,
        block_0x0020ff28,
        block_0x0020ff38,
        block_0x0020ff3c,
        block_0x0020ff4c,
        block_0x00210080,
        block_0x00210094,
        block_0x00210098,
        block_0x002100a0,
        block_0x002100a4,
        block_0x002100b8,
        block_0x002100d8,
        block_0x002100f4,
        block_0x00210148,
        block_0x00210150,
        block_0x0021018c,
        block_0x00210198,
        block_0x002101ac,
        block_0x002101b0,
        block_0x002101b8,
        block_0x002101bc,
        block_0x002101d4,
        block_0x002101d8,
        block_0x002101ec,
        block_0x002101f8,
        block_0x00210200,
        block_0x0021020c,
        block_0x00210210,
        block_0x00210214,
        block_0x00210240,
        block_0x00210244,
        block_0x00210268,
        block_0x0021026c,
        block_0x00210274,
        block_0x002102a0,
        block_0x002102ac,
        block_0x002102c0,
        block_0x002102c4,
        block_0x002102d4,
        block_0x002102d8,
        block_0x002102f8,
        block_0x00210300,
        block_0x00210304,
        block_0x00210310,
        block_0x00210334,
        block_0x00210338,
        block_0x00210340,
        block_0x00210344,
        block_0x00210378,
        block_0x002103c8,
        block_0x002103dc,
        block_0x002103e0,
        block_0x002103f0,
        block_0x002103f8,
        block_0x00210400,
        block_0x00210408,
        block_0x00210420,
        block_0x0021042c,
        block_0x00210434,
        block_0x00210450,
        block_0x00210454,
        block_0x0021045c,
        block_0x00210470,
        block_0x0021047c,
        block_0x002104a0,
        block_0x002104a4,
        block_0x002104f0,
        block_0x00210500,
        block_0x0021050c,
        block_0x00210510,
        block_0x00210554,
        block_0x0021055c,
        block_0x00210570,
        block_0x00210578,
        block_0x0021058c,
        block_0x00210590,
        block_0x00210594,
        block_0x002105a0,
        block_0x002105ac,
        block_0x002105c0,
        block_0x002105c4,
        block_0x002105d4,
        block_0x002105d8,
        block_0x002105f4,
        block_0x002105fc,
        block_0x00210610,
        block_0x00210614,
        block_0x00210618,
        block_0x00210658,
        block_0x00210674,
        block_0x0021067c,
        block_0x00210690,
        block_0x00210694,
        block_0x002106a8,
        block_0x002106b4,
        block_0x002106c8,
        block_0x002106cc,
    ];
    const IDX: [u16; 664usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 2u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 3u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 4u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 5u16, 0u16, 0u16, 0u16, 6u16, 0u16, 7u16, 0u16, 0u16,
        0u16, 8u16, 0u16, 9u16, 0u16, 10u16, 0u16, 0u16, 0u16, 11u16, 0u16, 12u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 13u16, 0u16,
        0u16, 0u16, 14u16, 0u16, 15u16, 0u16, 0u16, 0u16, 16u16, 0u16, 0u16, 17u16, 0u16,
        0u16, 0u16, 18u16, 0u16, 0u16, 19u16, 0u16, 0u16, 0u16, 20u16, 21u16, 0u16, 0u16,
        0u16, 22u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        23u16, 0u16, 0u16, 0u16, 0u16, 24u16, 25u16, 0u16, 26u16, 27u16, 0u16, 0u16,
        0u16, 0u16, 28u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 29u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 30u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 31u16,
        0u16, 32u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 33u16, 0u16, 0u16, 34u16, 0u16, 0u16, 0u16, 0u16, 35u16, 36u16,
        0u16, 37u16, 38u16, 0u16, 0u16, 0u16, 0u16, 0u16, 39u16, 40u16, 0u16, 0u16, 0u16,
        0u16, 41u16, 0u16, 0u16, 42u16, 0u16, 43u16, 0u16, 0u16, 44u16, 45u16, 46u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 47u16, 48u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 49u16, 50u16, 0u16, 51u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 52u16, 0u16, 0u16, 53u16, 0u16,
        0u16, 0u16, 0u16, 54u16, 55u16, 0u16, 0u16, 0u16, 56u16, 57u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 58u16, 0u16, 59u16, 60u16, 0u16, 0u16, 61u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 62u16, 63u16, 0u16, 64u16, 65u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 66u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 67u16, 0u16, 0u16, 0u16, 0u16, 68u16, 69u16, 0u16, 0u16,
        0u16, 70u16, 0u16, 71u16, 0u16, 72u16, 0u16, 73u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        74u16, 0u16, 0u16, 75u16, 0u16, 76u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 77u16,
        78u16, 0u16, 79u16, 0u16, 0u16, 0u16, 0u16, 80u16, 0u16, 0u16, 81u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 82u16, 83u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        84u16, 0u16, 0u16, 0u16, 85u16, 0u16, 0u16, 86u16, 87u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 88u16,
        0u16, 89u16, 0u16, 0u16, 0u16, 0u16, 90u16, 0u16, 91u16, 0u16, 0u16, 0u16, 0u16,
        92u16, 93u16, 94u16, 0u16, 0u16, 95u16, 0u16, 0u16, 96u16, 0u16, 0u16, 0u16,
        0u16, 97u16, 98u16, 0u16, 0u16, 0u16, 99u16, 100u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 101u16, 0u16, 102u16, 0u16, 0u16, 0u16, 0u16, 103u16, 104u16, 105u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 106u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 107u16, 0u16, 108u16,
        0u16, 0u16, 0u16, 0u16, 109u16, 110u16, 0u16, 0u16, 0u16, 0u16, 111u16, 0u16,
        0u16, 112u16, 0u16, 0u16, 0u16, 0u16, 113u16, 114u16,
    ];
    if pc < 2161776u32 || pc > 2164428u32 {
        return None;
    }
    let word_offset = ((pc - 2161776u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(never)]
pub fn block_0x0020fc70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2161780u32);
    emu.sw_no_count(10usize, 2usize, 8u32, 2161784u32)?;
    emu.sw_no_count(11usize, 2usize, 12u32, 2161788u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2161792u32)?;
    emu.sw_no_count(13usize, 2usize, 20u32, 2161796u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2161800u32);
    let a = 0u32.wrapping_add(2170880u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2161804u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965980u32, 2161808u32);
    emu.adi_no_count(12usize, 2usize, 16u32, 2161812u32);
    let a = 0u32.wrapping_add(2170880u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2161816u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294965964u32, 2161820u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2161824u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 508u32, 2161828u32);
    emu.adi_no_count(16usize, 0usize, 2u32, 2161832u32);
    emu.sw_no_count(0usize, 2usize, 40u32, 2161836u32)?;
    emu.adi_no_count(17usize, 2usize, 48u32, 2161840u32);
    emu.sw_no_count(10usize, 2usize, 48u32, 2161844u32)?;
    emu.sw_no_count(11usize, 2usize, 52u32, 2161848u32)?;
    emu.sw_no_count(12usize, 2usize, 56u32, 2161852u32)?;
    emu.sw_no_count(13usize, 2usize, 60u32, 2161856u32)?;
    emu.sw_no_count(15usize, 2usize, 24u32, 2161860u32)?;
    emu.sw_no_count(16usize, 2usize, 28u32, 2161864u32)?;
    emu.sw_no_count(17usize, 2usize, 32u32, 2161868u32)?;
    emu.sw_no_count(16usize, 2usize, 36u32, 2161872u32)?;
    emu.adi_no_count(10usize, 2usize, 24u32, 2161876u32);
    emu.adi_no_count(11usize, 14usize, 0u32, 2161880u32);
    emu.apc_no_count(1usize, 2161880u32, 4294955008u32, 2161884u32);
    emu.add_memory_rw_events(28usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161888u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965416u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020fce0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2161892u32);
    emu.adi_no_count(11usize, 10usize, 0u32, 2161896u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2161900u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 552u32, 2161904u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2161908u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2161912u32)?;
    emu.adi_no_count(13usize, 0usize, 4u32, 2161916u32);
    emu.sw_no_count(10usize, 2usize, 8u32, 2161920u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2161924u32)?;
    emu.sw_no_count(13usize, 2usize, 16u32, 2161928u32)?;
    emu.sw_no_count(0usize, 2usize, 20u32, 2161932u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2161936u32);
    emu.apc_no_count(1usize, 2161936u32, 4294955008u32, 2161940u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161944u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965360u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020fd18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2161948u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2161952u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2161956u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2161960u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2161964u32)?;
    emu.adi_no_count(13usize, 0usize, 39u32, 2161968u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2161972u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2162016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fd60));
    } else {
        emu.pc = 2161976u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fd38));
    }
}
#[inline(always)]
pub fn block_0x0020fd38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 11usize, 2u32, 2161980u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2161984u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 560u32, 2161988u32);
    emu.adr_no_count(10usize, 10usize, 13usize, 2161992u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2161996u32)?;
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(10usize);
    let return_addr = 2162000u32;
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
pub fn block_0x0020fd50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2162004u32);
    let a = 0u32.wrapping_add(12288u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2162008u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 92u32, 2162012u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2162016u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2162852u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002100a4));
}
#[inline(always)]
pub fn block_0x0020fd60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 92u32, 2162020u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2162040u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fd78));
    } else {
        emu.pc = 2162024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fd68));
    }
}
#[inline(always)]
pub fn block_0x0020fd68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2162028u32);
    let a = 0u32.wrapping_add(24576u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2162032u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966364u32, 2162036u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2162040u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2162852u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002100a4));
}
#[inline(always)]
pub fn block_0x0020fd78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(12usize, 12usize, 1u32, 2162044u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2162472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ff28));
    } else {
        emu.pc = 2162048u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fd80));
    }
}
#[inline(always)]
pub fn block_0x0020fd80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 767u32, 2162052u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2162472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ff28));
    } else {
        emu.pc = 2162056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fd88));
    }
}
#[inline(always)]
pub fn block_0x0020fd88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 11usize, 0u32, 2162060u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2162064u32);
    emu.apc_no_count(1usize, 2162064u32, 4294959104u32, 2162068u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2162072u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1524u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020fd98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 9usize, 0u32, 2162076u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2162472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ff28));
    } else {
        emu.pc = 2162080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fda0));
    }
}
#[inline(never)]
pub fn block_0x0020fda0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 78u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(14usize, 11usize, 1u32, 2162084u32);
    let a = 0u32.wrapping_add(1431654400u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2162088u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(858992640u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2162092u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(252645376u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2162096u32;
    emu.update_insn_clock();
    emu.sri_no_count(15usize, 11usize, 20u32, 2162100u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2162104u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 1492u32, 2162108u32);
    emu.sli_no_count(17usize, 11usize, 12u32, 2162112u32);
    emu.sli_no_count(5usize, 11usize, 16u32, 2162116u32);
    emu.sli_no_count(6usize, 11usize, 20u32, 2162120u32);
    emu.sli_no_count(7usize, 11usize, 24u32, 2162124u32);
    emu.orr_no_count(14usize, 11usize, 14usize, 2162128u32);
    emu.ani_no_count(11usize, 11usize, 15u32, 2162132u32);
    emu.adr_no_count(15usize, 16usize, 15usize, 2162136u32);
    emu.sri_no_count(17usize, 17usize, 28u32, 2162140u32);
    emu.sri_no_count(5usize, 5usize, 28u32, 2162144u32);
    emu.sri_no_count(6usize, 6usize, 28u32, 2162148u32);
    emu.sri_no_count(7usize, 7usize, 28u32, 2162152u32);
    emu.adr_no_count(11usize, 16usize, 11usize, 2162156u32);
    emu.adr_no_count(17usize, 16usize, 17usize, 2162160u32);
    emu.adr_no_count(5usize, 16usize, 5usize, 2162164u32);
    emu.adr_no_count(6usize, 16usize, 6usize, 2162168u32);
    emu.adr_no_count(16usize, 16usize, 7usize, 2162172u32);
    emu.sri_no_count(7usize, 14usize, 2u32, 2162176u32);
    emu.orr_no_count(14usize, 14usize, 7usize, 2162180u32);
    emu.sri_no_count(7usize, 14usize, 4u32, 2162184u32);
    emu.orr_no_count(14usize, 14usize, 7usize, 2162188u32);
    emu.sri_no_count(7usize, 14usize, 8u32, 2162192u32);
    emu.orr_no_count(14usize, 14usize, 7usize, 2162196u32);
    let a = 0u32.wrapping_add(16842752u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2162200u32;
    emu.update_insn_clock();
    emu.lbu_no_count(15usize, 15usize, 0u32, 2162204u32);
    emu.lbu_no_count(17usize, 17usize, 0u32, 2162208u32);
    emu.lbu_no_count(5usize, 5usize, 0u32, 2162212u32);
    emu.lbu_no_count(6usize, 6usize, 0u32, 2162216u32);
    emu.sh_no_count(0usize, 2usize, 12u32, 2162220u32)?;
    emu.sb_no_count(0usize, 2usize, 14u32, 2162224u32);
    emu.sb_no_count(15usize, 2usize, 15u32, 2162228u32);
    emu.sb_no_count(17usize, 2usize, 16u32, 2162232u32);
    emu.adi_no_count(15usize, 0usize, 125u32, 2162236u32);
    emu.adi_no_count(13usize, 13usize, 1365u32, 2162240u32);
    emu.lbu_no_count(16usize, 16usize, 0u32, 2162244u32);
    emu.lbu_no_count(11usize, 11usize, 0u32, 2162248u32);
    emu.sb_no_count(5usize, 2usize, 17u32, 2162252u32);
    emu.sb_no_count(6usize, 2usize, 18u32, 2162256u32);
    emu.sb_no_count(16usize, 2usize, 19u32, 2162260u32);
    emu.sri_no_count(16usize, 14usize, 16u32, 2162264u32);
    emu.orr_no_count(14usize, 14usize, 16usize, 2162268u32);
    emu.xri_no_count(14usize, 14usize, 4294967295u32, 2162272u32);
    emu.sri_no_count(16usize, 14usize, 1u32, 2162276u32);
    emu.anr_no_count(13usize, 16usize, 13usize, 2162280u32);
    emu.adi_no_count(16usize, 2usize, 12u32, 2162284u32);
    emu.adi_no_count(12usize, 12usize, 819u32, 2162288u32);
    emu.ani_no_count(14usize, 14usize, 4294967294u32, 2162292u32);
    emu.sbr_no_count(14usize, 14usize, 13usize, 2162296u32);
    emu.anr_no_count(13usize, 14usize, 12usize, 2162300u32);
    emu.sri_no_count(14usize, 14usize, 2u32, 2162304u32);
    emu.anr_no_count(12usize, 14usize, 12usize, 2162308u32);
    emu.adi_no_count(14usize, 0usize, 92u32, 2162312u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2162316u32);
    emu.sri_no_count(13usize, 12usize, 4u32, 2162320u32);
    emu.adr_no_count(12usize, 12usize, 13usize, 2162324u32);
    emu.adi_no_count(13usize, 0usize, 117u32, 2162328u32);
    emu.adi_no_count(10usize, 10usize, 4294967055u32, 2162332u32);
    emu.anr_no_count(10usize, 12usize, 10usize, 2162336u32);
    emu.adi_no_count(12usize, 0usize, 123u32, 2162340u32);
    emu.adi_no_count(17usize, 7usize, 257u32, 2162344u32);
    emu.mul_no_count(10usize, 10usize, 17usize, 2162348u32);
    emu.sri_no_count(10usize, 10usize, 26u32, 2162352u32);
    emu.adi_no_count(9usize, 10usize, 4294967294u32, 2162356u32);
    emu.adr_no_count(10usize, 16usize, 10usize, 2162360u32);
    emu.adr_no_count(16usize, 16usize, 9usize, 2162364u32);
    emu.sb_no_count(14usize, 16usize, 0u32, 2162368u32);
    emu.sb_no_count(13usize, 10usize, 4294967295u32, 2162372u32);
    emu.sb_no_count(12usize, 10usize, 0u32, 2162376u32);
    emu.sb_no_count(11usize, 2usize, 20u32, 2162380u32);
    emu.sb_no_count(15usize, 2usize, 21u32, 2162384u32);
    emu.adi_no_count(11usize, 2usize, 12u32, 2162388u32);
    emu.add_memory_rw_events(78usize);
    let return_addr = 2162392u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2162816u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210080));
}
#[inline(always)]
pub fn block_0x0020fed8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2162396u32);
    let a = 0u32.wrapping_add(28672u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2162400u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966876u32, 2162404u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2162408u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2162852u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002100a4));
}
#[inline(always)]
pub fn block_0x0020fee8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 12usize, 256u32, 2162412u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2162472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ff28));
    } else {
        emu.pc = 2162416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fef0));
    }
}
#[inline(always)]
pub fn block_0x0020fef0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2162420u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2162424u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1884u32, 2162428u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2162432u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2162852u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002100a4));
}
#[inline(always)]
pub fn block_0x0020ff00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2162436u32);
    let a = 0u32.wrapping_add(28672u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2162440u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(3usize);
    let return_addr = 2162444u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2162848u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002100a0));
}
#[inline(always)]
pub fn block_0x0020ff0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2162448u32);
    let a = 0u32.wrapping_add(28672u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2162452u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1116u32, 2162456u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2162460u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2162852u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002100a4));
}
#[inline(always)]
pub fn block_0x0020ff1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 8u32, 2162464u32);
    emu.sri_no_count(12usize, 12usize, 24u32, 2162468u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2162840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210098));
    } else {
        emu.pc = 2162472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ff28));
    }
}
#[inline(always)]
pub fn block_0x0020ff28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 11usize, 0u32, 2162476u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2162480u32);
    emu.apc_no_count(1usize, 2162480u32, 8192u32, 2162484u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2162488u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966460u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020ff38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2162508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ff4c));
    } else {
        emu.pc = 2162492u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ff3c));
    }
}
#[inline(always)]
pub fn block_0x0020ff3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(9usize, 8usize, 0u32, 2162496u32)?;
    emu.adi_no_count(18usize, 0usize, 129u32, 2162500u32);
    emu.adi_no_count(9usize, 0usize, 128u32, 2162504u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2162508u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2162872u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002100b8));
}
#[inline(never)]
pub fn block_0x0020ff4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 77u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(14usize, 9usize, 1u32, 2162512u32);
    let a = 0u32.wrapping_add(1431654400u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2162516u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(858992640u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2162520u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(252645376u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2162524u32;
    emu.update_insn_clock();
    emu.sri_no_count(15usize, 9usize, 20u32, 2162528u32);
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2162532u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 1492u32, 2162536u32);
    emu.sli_no_count(17usize, 9usize, 12u32, 2162540u32);
    emu.sli_no_count(5usize, 9usize, 16u32, 2162544u32);
    emu.sli_no_count(6usize, 9usize, 20u32, 2162548u32);
    emu.sli_no_count(7usize, 9usize, 24u32, 2162552u32);
    emu.orr_no_count(14usize, 9usize, 14usize, 2162556u32);
    emu.ani_no_count(13usize, 9usize, 15u32, 2162560u32);
    emu.adr_no_count(15usize, 16usize, 15usize, 2162564u32);
    emu.sri_no_count(17usize, 17usize, 28u32, 2162568u32);
    emu.sri_no_count(5usize, 5usize, 28u32, 2162572u32);
    emu.sri_no_count(6usize, 6usize, 28u32, 2162576u32);
    emu.sri_no_count(7usize, 7usize, 28u32, 2162580u32);
    emu.adr_no_count(13usize, 16usize, 13usize, 2162584u32);
    emu.adr_no_count(17usize, 16usize, 17usize, 2162588u32);
    emu.adr_no_count(5usize, 16usize, 5usize, 2162592u32);
    emu.adr_no_count(6usize, 16usize, 6usize, 2162596u32);
    emu.adr_no_count(16usize, 16usize, 7usize, 2162600u32);
    emu.sri_no_count(7usize, 14usize, 2u32, 2162604u32);
    emu.orr_no_count(14usize, 14usize, 7usize, 2162608u32);
    emu.sri_no_count(7usize, 14usize, 4u32, 2162612u32);
    emu.orr_no_count(14usize, 14usize, 7usize, 2162616u32);
    emu.sri_no_count(7usize, 14usize, 8u32, 2162620u32);
    emu.orr_no_count(14usize, 14usize, 7usize, 2162624u32);
    let a = 0u32.wrapping_add(16842752u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2162628u32;
    emu.update_insn_clock();
    emu.lbu_no_count(15usize, 15usize, 0u32, 2162632u32);
    emu.lbu_no_count(17usize, 17usize, 0u32, 2162636u32);
    emu.lbu_no_count(5usize, 5usize, 0u32, 2162640u32);
    emu.lbu_no_count(6usize, 6usize, 0u32, 2162644u32);
    emu.sh_no_count(0usize, 2usize, 22u32, 2162648u32)?;
    emu.sb_no_count(0usize, 2usize, 24u32, 2162652u32);
    emu.sb_no_count(15usize, 2usize, 25u32, 2162656u32);
    emu.sb_no_count(17usize, 2usize, 26u32, 2162660u32);
    emu.adi_no_count(15usize, 0usize, 125u32, 2162664u32);
    emu.adi_no_count(12usize, 12usize, 1365u32, 2162668u32);
    emu.lbu_no_count(16usize, 16usize, 0u32, 2162672u32);
    emu.lbu_no_count(13usize, 13usize, 0u32, 2162676u32);
    emu.sb_no_count(5usize, 2usize, 27u32, 2162680u32);
    emu.sb_no_count(6usize, 2usize, 28u32, 2162684u32);
    emu.sb_no_count(16usize, 2usize, 29u32, 2162688u32);
    emu.sri_no_count(16usize, 14usize, 16u32, 2162692u32);
    emu.orr_no_count(14usize, 14usize, 16usize, 2162696u32);
    emu.xri_no_count(14usize, 14usize, 4294967295u32, 2162700u32);
    emu.sri_no_count(16usize, 14usize, 1u32, 2162704u32);
    emu.anr_no_count(12usize, 16usize, 12usize, 2162708u32);
    emu.adi_no_count(16usize, 2usize, 22u32, 2162712u32);
    emu.adi_no_count(11usize, 11usize, 819u32, 2162716u32);
    emu.ani_no_count(14usize, 14usize, 4294967294u32, 2162720u32);
    emu.sbr_no_count(14usize, 14usize, 12usize, 2162724u32);
    emu.anr_no_count(12usize, 14usize, 11usize, 2162728u32);
    emu.sri_no_count(14usize, 14usize, 2u32, 2162732u32);
    emu.anr_no_count(11usize, 14usize, 11usize, 2162736u32);
    emu.adi_no_count(14usize, 0usize, 92u32, 2162740u32);
    emu.adr_no_count(11usize, 12usize, 11usize, 2162744u32);
    emu.sri_no_count(12usize, 11usize, 4u32, 2162748u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2162752u32);
    emu.adi_no_count(12usize, 0usize, 117u32, 2162756u32);
    emu.adi_no_count(10usize, 10usize, 4294967055u32, 2162760u32);
    emu.anr_no_count(10usize, 11usize, 10usize, 2162764u32);
    emu.adi_no_count(11usize, 0usize, 123u32, 2162768u32);
    emu.adi_no_count(17usize, 7usize, 257u32, 2162772u32);
    emu.mul_no_count(10usize, 10usize, 17usize, 2162776u32);
    emu.sri_no_count(10usize, 10usize, 26u32, 2162780u32);
    emu.adi_no_count(9usize, 10usize, 4294967294u32, 2162784u32);
    emu.adr_no_count(10usize, 16usize, 10usize, 2162788u32);
    emu.adr_no_count(16usize, 16usize, 9usize, 2162792u32);
    emu.sb_no_count(14usize, 16usize, 0u32, 2162796u32);
    emu.sb_no_count(12usize, 10usize, 4294967295u32, 2162800u32);
    emu.sb_no_count(11usize, 10usize, 0u32, 2162804u32);
    emu.sb_no_count(13usize, 2usize, 30u32, 2162808u32);
    emu.sb_no_count(15usize, 2usize, 31u32, 2162812u32);
    emu.adi_no_count(11usize, 2usize, 22u32, 2162816u32);
    emu.add_memory_rw_events(77usize);
    emu.pc = 2162816u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210080));
}
#[inline(always)]
pub fn block_0x00210080(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 10u32, 2162820u32);
    emu.adi_no_count(18usize, 0usize, 10u32, 2162824u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2162828u32);
    emu.apc_no_count(1usize, 2162828u32, 4294914048u32, 2162832u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2162836u32;
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
pub fn block_0x00210094(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2162840u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2162872u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002100b8));
}
#[inline(always)]
pub fn block_0x00210098(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2162844u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2162848u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(2usize);
    emu.pc = 2162848u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002100a0));
}
#[inline(always)]
pub fn block_0x002100a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 10usize, 604u32, 2162852u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2162852u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002100a4));
}
#[inline(always)]
pub fn block_0x002100a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 8usize, 0u32, 2162856u32)?;
    emu.sh_no_count(0usize, 8usize, 4u32, 2162860u32)?;
    emu.sh_no_count(0usize, 8usize, 6u32, 2162864u32)?;
    emu.sh_no_count(0usize, 8usize, 8u32, 2162868u32)?;
    emu.adi_no_count(18usize, 0usize, 2u32, 2162872u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2162872u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002100b8));
}
#[inline(always)]
pub fn block_0x002100b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(9usize, 8usize, 12u32, 2162876u32);
    emu.sb_no_count(18usize, 8usize, 13u32, 2162880u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2162884u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2162888u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2162892u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2162896u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2162900u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2162904u32;
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
pub fn block_0x002100d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 11usize, 0u32, 2162908u32)?;
    emu.lw_no_count(11usize, 11usize, 4u32, 2162912u32)?;
    emu.adi_no_count(13usize, 10usize, 0u32, 2162916u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2162920u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2162924u32);
    emu.apc_no_count(6usize, 2162924u32, 0u32, 2162928u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2162932u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(8u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002100f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2162936u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2162940u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2162944u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2162948u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2162952u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2162956u32)?;
    emu.sw_no_count(20usize, 2usize, 40u32, 2162960u32)?;
    emu.sw_no_count(21usize, 2usize, 36u32, 2162964u32)?;
    emu.sw_no_count(22usize, 2usize, 32u32, 2162968u32)?;
    emu.sw_no_count(23usize, 2usize, 28u32, 2162972u32)?;
    emu.sw_no_count(24usize, 2usize, 24u32, 2162976u32)?;
    emu.sw_no_count(25usize, 2usize, 20u32, 2162980u32)?;
    emu.adi_no_count(8usize, 12usize, 0u32, 2162984u32);
    let a = 0u32.wrapping_add(3758096384u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2162988u32;
    emu.update_insn_clock();
    emu.lw_no_count(21usize, 8usize, 16u32, 2162992u32)?;
    emu.adi_no_count(12usize, 12usize, 32u32, 2162996u32);
    emu.sw_no_count(10usize, 2usize, 4u32, 2163000u32)?;
    emu.sw_no_count(11usize, 2usize, 8u32, 2163004u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2163008u32)?;
    emu.sw_no_count(0usize, 2usize, 16u32, 2163012u32)?;
    emu.add_memory_rw_events(20usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a == b {
        emu.pc = 2163308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021026c));
    } else {
        emu.pc = 2163016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210148));
    }
}
#[inline(always)]
pub fn block_0x00210148(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 8usize, 20u32, 2163020u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2163460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210304));
    } else {
        emu.pc = 2163024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210150));
    }
}
#[inline]
pub fn block_0x00210150(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 0u32, 2163028u32);
    emu.sli_no_count(12usize, 11usize, 3u32, 2163032u32);
    emu.sli_no_count(13usize, 11usize, 5u32, 2163036u32);
    emu.adi_no_count(10usize, 21usize, 24u32, 2163040u32);
    emu.lw_no_count(23usize, 8usize, 0u32, 2163044u32)?;
    emu.lw_no_count(19usize, 8usize, 8u32, 2163048u32)?;
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2163052u32);
    emu.adi_no_count(20usize, 0usize, 2u32, 2163056u32);
    emu.sbr_no_count(13usize, 13usize, 12usize, 2163060u32);
    emu.sli_no_count(11usize, 11usize, 3u32, 2163064u32);
    emu.adr_no_count(22usize, 21usize, 13usize, 2163068u32);
    emu.sri_no_count(11usize, 11usize, 3u32, 2163072u32);
    emu.adi_no_count(9usize, 11usize, 1u32, 2163076u32);
    emu.adi_no_count(23usize, 23usize, 4u32, 2163080u32);
    emu.adi_no_count(24usize, 0usize, 1u32, 2163084u32);
    emu.add_memory_rw_events(15usize);
    emu.pc = 2163084u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021018c));
}
#[inline(always)]
pub fn block_0x0021018c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 23usize, 0u32, 2163088u32)?;
    emu.adi_no_count(25usize, 10usize, 0u32, 2163092u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2163120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002101b0));
    } else {
        emu.pc = 2163096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210198));
    }
}
#[inline(always)]
pub fn block_0x00210198(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 2usize, 8u32, 2163100u32)?;
    emu.lw_no_count(10usize, 2usize, 4u32, 2163104u32)?;
    emu.lw_no_count(11usize, 23usize, 4294967292u32, 2163108u32)?;
    emu.lw_no_count(13usize, 13usize, 12u32, 2163112u32)?;
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2163116u32;
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
pub fn block_0x002101ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2163512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210338));
    } else {
        emu.pc = 2163120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002101b0));
    }
}
#[inline(always)]
pub fn block_0x002101b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lhu_no_count(10usize, 21usize, 8u32, 2163124u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2163180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002101ec));
    } else {
        emu.pc = 2163128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002101b8));
    }
}
#[inline(always)]
pub fn block_0x002101b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2163200u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210200));
    } else {
        emu.pc = 2163132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002101bc));
    }
}
#[inline(always)]
pub fn block_0x002101bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 21usize, 12u32, 2163136u32)?;
    emu.sli_no_count(10usize, 10usize, 3u32, 2163140u32);
    emu.adr_no_count(10usize, 19usize, 10usize, 2163144u32);
    emu.lhu_no_count(11usize, 10usize, 4u32, 2163148u32)?;
    emu.lhu_no_count(10usize, 21usize, 0u32, 2163152u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2163192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002101f8));
    } else {
        emu.pc = 2163156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002101d4));
    }
}
#[inline(always)]
pub fn block_0x002101d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2163216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210210));
    } else {
        emu.pc = 2163160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002101d8));
    }
}
#[inline(always)]
pub fn block_0x002101d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 21usize, 4u32, 2163164u32)?;
    emu.sli_no_count(10usize, 10usize, 3u32, 2163168u32);
    emu.adr_no_count(10usize, 19usize, 10usize, 2163172u32);
    emu.lhu_no_count(12usize, 10usize, 4u32, 2163176u32)?;
    emu.add_memory_rw_events(5usize);
    let return_addr = 2163180u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2163220u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210214));
}
#[inline(always)]
pub fn block_0x002101ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lhu_no_count(11usize, 21usize, 10u32, 2163184u32)?;
    emu.lhu_no_count(10usize, 21usize, 0u32, 2163188u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2163156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002101d4));
    } else {
        emu.pc = 2163192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002101f8));
    }
}
#[inline(always)]
pub fn block_0x002101f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 0u32, 2163196u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2163200u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2163220u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210214));
}
#[inline(always)]
pub fn block_0x00210200(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2163204u32);
    emu.lhu_no_count(10usize, 21usize, 0u32, 2163208u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2163156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002101d4));
    } else {
        emu.pc = 2163212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021020c));
    }
}
#[inline(always)]
pub fn block_0x0021020c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2163216u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2163192u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002101f8));
}
#[inline(always)]
pub fn block_0x00210210(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lhu_no_count(12usize, 21usize, 2u32, 2163220u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2163220u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210214));
}
#[inline]
pub fn block_0x00210214(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 21usize, 16u32, 2163224u32)?;
    emu.lw_no_count(13usize, 21usize, 20u32, 2163228u32)?;
    emu.sli_no_count(10usize, 10usize, 3u32, 2163232u32);
    emu.adr_no_count(14usize, 19usize, 10usize, 2163236u32);
    emu.lw_no_count(10usize, 14usize, 0u32, 2163240u32)?;
    emu.lw_no_count(14usize, 14usize, 4u32, 2163244u32)?;
    emu.sw_no_count(13usize, 2usize, 12u32, 2163248u32)?;
    emu.sh_no_count(11usize, 2usize, 16u32, 2163252u32)?;
    emu.sh_no_count(12usize, 2usize, 18u32, 2163256u32)?;
    emu.adi_no_count(11usize, 2usize, 4u32, 2163260u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2163264u32;
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
pub fn block_0x00210240(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2163512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210338));
    } else {
        emu.pc = 2163268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210244));
    }
}
#[inline]
pub fn block_0x00210244(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 18usize, 1u32, 2163272u32);
    emu.xrr_no_count(10usize, 25usize, 22usize, 2163276u32);
    emu.sltiu_no_count(10usize, 10usize, 1u32, 2163280u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2163284u32);
    emu.ani_no_count(10usize, 10usize, 24u32, 2163288u32);
    emu.adr_no_count(10usize, 25usize, 10usize, 2163292u32);
    emu.adi_no_count(23usize, 23usize, 8u32, 2163296u32);
    emu.adi_no_count(21usize, 25usize, 0u32, 2163300u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a != b {
        emu.pc = 2163084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021018c));
    } else {
        emu.pc = 2163304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210268));
    }
}
#[inline(always)]
pub fn block_0x00210268(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2163308u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2163448u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002102f8));
}
#[inline(always)]
pub fn block_0x0021026c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 12u32, 2163312u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2163460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210304));
    } else {
        emu.pc = 2163316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210274));
    }
}
#[inline]
pub fn block_0x00210274(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 0u32, 2163320u32);
    emu.lw_no_count(20usize, 8usize, 0u32, 2163324u32)?;
    emu.lw_no_count(21usize, 8usize, 8u32, 2163328u32)?;
    emu.sli_no_count(19usize, 10usize, 3u32, 2163332u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2163336u32);
    emu.sli_no_count(10usize, 10usize, 3u32, 2163340u32);
    emu.sri_no_count(10usize, 10usize, 3u32, 2163344u32);
    emu.adi_no_count(9usize, 10usize, 1u32, 2163348u32);
    emu.adr_no_count(19usize, 21usize, 19usize, 2163352u32);
    emu.adi_no_count(10usize, 21usize, 8u32, 2163356u32);
    emu.adi_no_count(20usize, 20usize, 4u32, 2163360u32);
    emu.add_memory_rw_events(11usize);
    emu.pc = 2163360u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002102a0));
}
#[inline(always)]
pub fn block_0x002102a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 20usize, 0u32, 2163364u32)?;
    emu.adi_no_count(22usize, 10usize, 0u32, 2163368u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2163396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002102c4));
    } else {
        emu.pc = 2163372u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002102ac));
    }
}
#[inline(always)]
pub fn block_0x002102ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 2usize, 8u32, 2163376u32)?;
    emu.lw_no_count(10usize, 2usize, 4u32, 2163380u32)?;
    emu.lw_no_count(11usize, 20usize, 4294967292u32, 2163384u32)?;
    emu.lw_no_count(13usize, 13usize, 12u32, 2163388u32)?;
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2163392u32;
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
pub fn block_0x002102c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2163512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210338));
    } else {
        emu.pc = 2163396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002102c4));
    }
}
#[inline(always)]
pub fn block_0x002102c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 21usize, 0u32, 2163400u32)?;
    emu.lw_no_count(12usize, 21usize, 4u32, 2163404u32)?;
    emu.adi_no_count(11usize, 2usize, 4u32, 2163408u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2163412u32;
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
pub fn block_0x002102d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2163512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210338));
    } else {
        emu.pc = 2163416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002102d8));
    }
}
#[inline(always)]
pub fn block_0x002102d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 18usize, 1u32, 2163420u32);
    emu.xrr_no_count(10usize, 22usize, 19usize, 2163424u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2163428u32);
    emu.sli_no_count(10usize, 10usize, 3u32, 2163432u32);
    emu.adr_no_count(10usize, 22usize, 10usize, 2163436u32);
    emu.adi_no_count(20usize, 20usize, 8u32, 2163440u32);
    emu.adi_no_count(21usize, 22usize, 0u32, 2163444u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a != b {
        emu.pc = 2163360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002102a0));
    } else {
        emu.pc = 2163448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002102f8));
    }
}
#[inline(always)]
pub fn block_0x002102f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 4u32, 2163452u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a < b {
        emu.pc = 2163472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210310));
    } else {
        emu.pc = 2163456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210300));
    }
}
#[inline(always)]
pub fn block_0x00210300(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2163460u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2163520u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210340));
}
#[inline(always)]
pub fn block_0x00210304(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2163464u32);
    emu.lw_no_count(10usize, 8usize, 4u32, 2163468u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(0usize);
    if a >= b {
        emu.pc = 2163520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210340));
    } else {
        emu.pc = 2163472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210310));
    }
}
#[inline]
pub fn block_0x00210310(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 8usize, 0u32, 2163476u32)?;
    emu.sli_no_count(9usize, 9usize, 3u32, 2163480u32);
    emu.lw_no_count(10usize, 2usize, 4u32, 2163484u32)?;
    emu.lw_no_count(13usize, 2usize, 8u32, 2163488u32)?;
    emu.adr_no_count(9usize, 11usize, 9usize, 2163492u32);
    emu.lw_no_count(11usize, 9usize, 0u32, 2163496u32)?;
    emu.lw_no_count(12usize, 9usize, 4u32, 2163500u32)?;
    emu.lw_no_count(13usize, 13usize, 12u32, 2163504u32)?;
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2163508u32;
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
pub fn block_0x00210334(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2163520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210340));
    } else {
        emu.pc = 2163512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210338));
    }
}
#[inline(always)]
pub fn block_0x00210338(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2163516u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2163520u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2163524u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210344));
}
#[inline(always)]
pub fn block_0x00210340(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2163524u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2163524u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210344));
}
#[inline]
pub fn block_0x00210344(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2163528u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2163532u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2163536u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2163540u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2163544u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2163548u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2163552u32)?;
    emu.lw_no_count(22usize, 2usize, 32u32, 2163556u32)?;
    emu.lw_no_count(23usize, 2usize, 28u32, 2163560u32)?;
    emu.lw_no_count(24usize, 2usize, 24u32, 2163564u32)?;
    emu.lw_no_count(25usize, 2usize, 20u32, 2163568u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2163572u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2163576u32;
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
pub fn block_0x00210378(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2163580u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2163584u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2163588u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2163592u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2163596u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2163600u32)?;
    emu.sw_no_count(20usize, 2usize, 40u32, 2163604u32)?;
    emu.sw_no_count(21usize, 2usize, 36u32, 2163608u32)?;
    emu.sw_no_count(22usize, 2usize, 32u32, 2163612u32)?;
    emu.sw_no_count(23usize, 2usize, 28u32, 2163616u32)?;
    emu.sw_no_count(24usize, 2usize, 24u32, 2163620u32)?;
    emu.sw_no_count(25usize, 2usize, 20u32, 2163624u32)?;
    emu.sw_no_count(26usize, 2usize, 16u32, 2163628u32)?;
    emu.sw_no_count(27usize, 2usize, 12u32, 2163632u32)?;
    emu.adi_no_count(8usize, 15usize, 0u32, 2163636u32);
    emu.adi_no_count(9usize, 14usize, 0u32, 2163640u32);
    emu.adi_no_count(19usize, 13usize, 0u32, 2163644u32);
    emu.adi_no_count(20usize, 12usize, 0u32, 2163648u32);
    emu.adi_no_count(18usize, 10usize, 0u32, 2163652u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2163804u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021045c));
    } else {
        emu.pc = 2163656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002103c8));
    }
}
#[inline(always)]
pub fn block_0x002103c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(22usize, 18usize, 8u32, 2163660u32)?;
    let a = 0u32.wrapping_add(2097152u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2163664u32;
    emu.update_insn_clock();
    emu.anr_no_count(10usize, 22usize, 10usize, 2163668u32);
    let a = 0u32.wrapping_add(1114112u32);
    emu.write_reg_no_count(21usize, a);
    emu.pc = 2163672u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2163680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002103e0));
    } else {
        emu.pc = 2163676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002103dc));
    }
}
#[inline(always)]
pub fn block_0x002103dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(21usize, 0usize, 43u32, 2163680u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2163680u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002103e0));
}
#[inline(always)]
pub fn block_0x002103e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 10usize, 21u32, 2163684u32);
    emu.adr_no_count(24usize, 10usize, 8usize, 2163688u32);
    emu.sli_no_count(10usize, 22usize, 8u32, 2163692u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2163824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210470));
    } else {
        emu.pc = 2163696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002103f0));
    }
}
#[inline(always)]
pub fn block_0x002103f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 16u32, 2163700u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a >= b {
        emu.pc = 2163952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002104f0));
    } else {
        emu.pc = 2163704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002103f8));
    }
}
#[inline(always)]
pub fn block_0x002103f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2163708u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2163744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210420));
    } else {
        emu.pc = 2163712u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210400));
    }
}
#[inline(always)]
pub fn block_0x00210400(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(11usize, 20usize, 19usize, 2163716u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2163720u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2163720u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210408));
}
#[inline(always)]
pub fn block_0x00210408(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(13usize, 12usize, 0u32, 2163724u32);
    emu.adi_no_count(12usize, 12usize, 1u32, 2163728u32);
    emu.slti_no_count(13usize, 13usize, 4294967232u32, 2163732u32);
    emu.xri_no_count(13usize, 13usize, 1u32, 2163736u32);
    emu.adr_no_count(10usize, 10usize, 13usize, 2163740u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2163720u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210408));
    } else {
        emu.pc = 2163744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210420));
    }
}
#[inline(always)]
pub fn block_0x00210420(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(24usize, 10usize, 24usize, 2163748u32);
    emu.lhu_no_count(27usize, 18usize, 12u32, 2163752u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(27usize);
    let a = emu.read_reg_a_tracked(24usize);
    if a >= b {
        emu.pc = 2163836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021047c));
    } else {
        emu.pc = 2163756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021042c));
    }
}
#[inline(always)]
pub fn block_0x0021042c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 22usize, 7u32, 2163760u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2163984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210510));
    } else {
        emu.pc = 2163764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210434));
    }
}
#[inline(always)]
pub fn block_0x00210434(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(24usize, 27usize, 24usize, 2163768u32);
    emu.sli_no_count(10usize, 22usize, 1u32, 2163772u32);
    emu.sri_no_count(10usize, 10usize, 30u32, 2163776u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2163780u32);
    emu.sli_no_count(22usize, 22usize, 11u32, 2163784u32);
    emu.sw_no_count(9usize, 2usize, 8u32, 2163788u32)?;
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2164116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210594));
    } else {
        emu.pc = 2163792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210450));
    }
}
#[inline(always)]
pub fn block_0x00210450(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2164180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002105d4));
    } else {
        emu.pc = 2163796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210454));
    }
}
#[inline(always)]
pub fn block_0x00210454(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 0usize, 0u32, 2163800u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2163804u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2164184u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002105d8));
}
#[inline(always)]
pub fn block_0x0021045c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(22usize, 18usize, 8u32, 2163808u32)?;
    emu.adi_no_count(24usize, 8usize, 1u32, 2163812u32);
    emu.adi_no_count(21usize, 0usize, 45u32, 2163816u32);
    emu.sli_no_count(10usize, 22usize, 8u32, 2163820u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2163696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002103f0));
    } else {
        emu.pc = 2163824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210470));
    }
}
#[inline(always)]
pub fn block_0x00210470(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 0u32, 2163828u32);
    emu.lhu_no_count(27usize, 18usize, 12u32, 2163832u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(27usize);
    let a = emu.read_reg_a_tracked(24usize);
    if a < b {
        emu.pc = 2163756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021042c));
    } else {
        emu.pc = 2163836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021047c));
    }
}
#[inline]
pub fn block_0x0021047c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(22usize, 18usize, 0u32, 2163840u32)?;
    emu.lw_no_count(18usize, 18usize, 4u32, 2163844u32)?;
    emu.adi_no_count(10usize, 22usize, 0u32, 2163848u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2163852u32);
    emu.adi_no_count(12usize, 21usize, 0u32, 2163856u32);
    emu.adi_no_count(13usize, 20usize, 0u32, 2163860u32);
    emu.adi_no_count(14usize, 19usize, 0u32, 2163864u32);
    emu.apc_no_count(1usize, 2163864u32, 0u32, 2163868u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2163872u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(568u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002104a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2164244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210614));
    } else {
        emu.pc = 2163876u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002104a4));
    }
}
#[inline]
pub fn block_0x002104a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(6usize, 18usize, 12u32, 2163880u32)?;
    emu.adi_no_count(10usize, 22usize, 0u32, 2163884u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2163888u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2163892u32);
    emu.lw_no_count(1usize, 2usize, 60u32, 2163896u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2163900u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2163904u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2163908u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2163912u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2163916u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2163920u32)?;
    emu.lw_no_count(22usize, 2usize, 32u32, 2163924u32)?;
    emu.lw_no_count(23usize, 2usize, 28u32, 2163928u32)?;
    emu.lw_no_count(24usize, 2usize, 24u32, 2163932u32)?;
    emu.lw_no_count(25usize, 2usize, 20u32, 2163936u32)?;
    emu.lw_no_count(26usize, 2usize, 16u32, 2163940u32)?;
    emu.lw_no_count(27usize, 2usize, 12u32, 2163944u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2163948u32);
    emu.add_memory_rw_events(19usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2163952u32;
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
pub fn block_0x002104f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 20usize, 0u32, 2163956u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2163960u32);
    emu.apc_no_count(1usize, 2163960u32, 16384u32, 2163964u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2163968u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965428u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00210500(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(24usize, 10usize, 24usize, 2163972u32);
    emu.lhu_no_count(27usize, 18usize, 12u32, 2163976u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(27usize);
    let a = emu.read_reg_a_tracked(24usize);
    if a >= b {
        emu.pc = 2163836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021047c));
    } else {
        emu.pc = 2163980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021050c));
    }
}
#[inline(always)]
pub fn block_0x0021050c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2163984u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2163756u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021042c));
}
#[inline]
pub fn block_0x00210510(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(22usize, 18usize, 0u32, 2163988u32)?;
    emu.lw_no_count(23usize, 18usize, 4u32, 2163992u32)?;
    emu.lw_no_count(25usize, 18usize, 8u32, 2163996u32)?;
    emu.lw_no_count(26usize, 18usize, 12u32, 2164000u32)?;
    let a = 0u32.wrapping_add(2682257408u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2164004u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(536870912u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2164008u32;
    emu.update_insn_clock();
    emu.anr_no_count(10usize, 25usize, 10usize, 2164012u32);
    emu.adi_no_count(11usize, 11usize, 48u32, 2164016u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2164020u32);
    emu.sw_no_count(10usize, 18usize, 8u32, 2164024u32)?;
    emu.adi_no_count(10usize, 22usize, 0u32, 2164028u32);
    emu.adi_no_count(11usize, 23usize, 0u32, 2164032u32);
    emu.adi_no_count(12usize, 21usize, 0u32, 2164036u32);
    emu.adi_no_count(13usize, 20usize, 0u32, 2164040u32);
    emu.adi_no_count(14usize, 19usize, 0u32, 2164044u32);
    emu.apc_no_count(1usize, 2164044u32, 0u32, 2164048u32);
    emu.add_memory_rw_events(17usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2164052u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(388u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00210554(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 1u32, 2164056u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2164248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210618));
    } else {
        emu.pc = 2164060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021055c));
    }
}
#[inline(always)]
pub fn block_0x0021055c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 0u32, 2164064u32);
    emu.sbr_no_count(10usize, 27usize, 24usize, 2164068u32);
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(21usize, a);
    emu.pc = 2164072u32;
    emu.update_insn_clock();
    emu.adi_no_count(21usize, 21usize, 4294967295u32, 2164076u32);
    emu.anr_no_count(24usize, 10usize, 21usize, 2164080u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2164080u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210570));
}
#[inline(always)]
pub fn block_0x00210570(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(10usize, 20usize, 21usize, 2164084u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2164140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002105ac));
    } else {
        emu.pc = 2164088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210578));
    }
}
#[inline(always)]
pub fn block_0x00210578(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 23usize, 16u32, 2164092u32)?;
    emu.adi_no_count(20usize, 20usize, 1u32, 2164096u32);
    emu.adi_no_count(11usize, 0usize, 48u32, 2164100u32);
    emu.adi_no_count(10usize, 22usize, 0u32, 2164104u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2164108u32;
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
pub fn block_0x0021058c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2164080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210570));
    } else {
        emu.pc = 2164112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210590));
    }
}
#[inline(always)]
pub fn block_0x00210590(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2164116u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2164248u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210618));
}
#[inline(always)]
pub fn block_0x00210594(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 2u32, 2164120u32);
    emu.adi_no_count(25usize, 24usize, 0u32, 2164124u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2164184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002105d8));
    } else {
        emu.pc = 2164128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002105a0));
    }
}
#[inline(always)]
pub fn block_0x002105a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 24usize, 16u32, 2164132u32);
    emu.sri_no_count(25usize, 10usize, 17u32, 2164136u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2164140u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2164184u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002105d8));
}
#[inline(always)]
pub fn block_0x002105ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 23usize, 12u32, 2164144u32)?;
    emu.adi_no_count(10usize, 22usize, 0u32, 2164148u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2164152u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2164156u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2164160u32;
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
pub fn block_0x002105c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2164248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210618));
    } else {
        emu.pc = 2164164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002105c4));
    }
}
#[inline(always)]
pub fn block_0x002105c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 0u32, 2164168u32);
    emu.sw_no_count(25usize, 18usize, 8u32, 2164172u32)?;
    emu.sw_no_count(26usize, 18usize, 12u32, 2164176u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2164180u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2164248u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210618));
}
#[inline(always)]
pub fn block_0x002105d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 24usize, 0u32, 2164184u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2164184u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002105d8));
}
#[inline(always)]
pub fn block_0x002105d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 0usize, 0u32, 2164188u32);
    emu.sri_no_count(22usize, 22usize, 11u32, 2164192u32);
    emu.lw_no_count(23usize, 18usize, 0u32, 2164196u32)?;
    emu.lw_no_count(18usize, 18usize, 4u32, 2164200u32)?;
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(27usize, a);
    emu.pc = 2164204u32;
    emu.update_insn_clock();
    emu.adi_no_count(27usize, 27usize, 4294967295u32, 2164208u32);
    emu.anr_no_count(9usize, 25usize, 27usize, 2164212u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2164212u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002105f4));
}
#[inline(always)]
pub fn block_0x002105f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(10usize, 26usize, 27usize, 2164216u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2164312u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210658));
    } else {
        emu.pc = 2164220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002105fc));
    }
}
#[inline(always)]
pub fn block_0x002105fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 16u32, 2164224u32)?;
    emu.adi_no_count(26usize, 26usize, 1u32, 2164228u32);
    emu.adi_no_count(10usize, 23usize, 0u32, 2164232u32);
    emu.adi_no_count(11usize, 22usize, 0u32, 2164236u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2164240u32;
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
pub fn block_0x00210610(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2164212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002105f4));
    } else {
        emu.pc = 2164244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210614));
    }
}
#[inline(always)]
pub fn block_0x00210614(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 1u32, 2164248u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2164248u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210618));
}
#[inline]
pub fn block_0x00210618(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 19usize, 0u32, 2164252u32);
    emu.lw_no_count(1usize, 2usize, 60u32, 2164256u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2164260u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2164264u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2164268u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2164272u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2164276u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2164280u32)?;
    emu.lw_no_count(22usize, 2usize, 32u32, 2164284u32)?;
    emu.lw_no_count(23usize, 2usize, 28u32, 2164288u32)?;
    emu.lw_no_count(24usize, 2usize, 24u32, 2164292u32)?;
    emu.lw_no_count(25usize, 2usize, 20u32, 2164296u32)?;
    emu.lw_no_count(26usize, 2usize, 16u32, 2164300u32)?;
    emu.lw_no_count(27usize, 2usize, 12u32, 2164304u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2164308u32);
    emu.add_memory_rw_events(16usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2164312u32;
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
pub fn block_0x00210658(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 23usize, 0u32, 2164316u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2164320u32);
    emu.adi_no_count(12usize, 21usize, 0u32, 2164324u32);
    emu.adi_no_count(13usize, 20usize, 0u32, 2164328u32);
    emu.adi_no_count(14usize, 19usize, 0u32, 2164332u32);
    emu.apc_no_count(1usize, 2164332u32, 0u32, 2164336u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2164340u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(100u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00210674(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 1u32, 2164344u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2164248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210618));
    } else {
        emu.pc = 2164348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021067c));
    }
}
#[inline(always)]
pub fn block_0x0021067c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 18usize, 12u32, 2164352u32)?;
    emu.adi_no_count(10usize, 23usize, 0u32, 2164356u32);
    emu.lw_no_count(11usize, 2usize, 8u32, 2164360u32)?;
    emu.adi_no_count(12usize, 8usize, 0u32, 2164364u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2164368u32;
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
pub fn block_0x00210690(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2164248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210618));
    } else {
        emu.pc = 2164372u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210694));
    }
}
#[inline(always)]
pub fn block_0x00210694(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 0usize, 0u32, 2164376u32);
    emu.sbr_no_count(10usize, 24usize, 25usize, 2164380u32);
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(9usize, a);
    emu.pc = 2164384u32;
    emu.update_insn_clock();
    emu.adi_no_count(9usize, 9usize, 4294967295u32, 2164388u32);
    emu.anr_no_count(20usize, 10usize, 9usize, 2164392u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2164392u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002106a8));
}
#[inline(always)]
pub fn block_0x002106a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(10usize, 8usize, 9usize, 2164396u32);
    emu.sltru_no_count(19usize, 10usize, 20usize, 2164400u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2164248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210618));
    } else {
        emu.pc = 2164404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002106b4));
    }
}
#[inline(always)]
pub fn block_0x002106b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 16u32, 2164408u32)?;
    emu.adi_no_count(8usize, 8usize, 1u32, 2164412u32);
    emu.adi_no_count(10usize, 23usize, 0u32, 2164416u32);
    emu.adi_no_count(11usize, 22usize, 0u32, 2164420u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2164424u32;
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
pub fn block_0x002106c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2164392u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002106a8));
    } else {
        emu.pc = 2164428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002106cc));
    }
}
#[inline(always)]
pub fn block_0x002106cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2164432u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2164248u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210618));
}
