pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2137428u32;
pub const PC_MAX: u32 = 2140096u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 112usize] = [
        block_0x00209d54,
        block_0x00209d78,
        block_0x00209d94,
        block_0x00209df4,
        block_0x00209e18,
        block_0x00209e48,
        block_0x00209e58,
        block_0x00209e5c,
        block_0x00209e8c,
        block_0x00209e9c,
        block_0x00209ea8,
        block_0x00209f18,
        block_0x00209f4c,
        block_0x00209f58,
        block_0x00209f64,
        block_0x00209fb8,
        block_0x00209fc4,
        block_0x00209fdc,
        block_0x00209ffc,
        block_0x0020a00c,
        block_0x0020a010,
        block_0x0020a034,
        block_0x0020a044,
        block_0x0020a050,
        block_0x0020a060,
        block_0x0020a078,
        block_0x0020a090,
        block_0x0020a094,
        block_0x0020a098,
        block_0x0020a0a4,
        block_0x0020a0a8,
        block_0x0020a0b4,
        block_0x0020a0e0,
        block_0x0020a110,
        block_0x0020a134,
        block_0x0020a184,
        block_0x0020a190,
        block_0x0020a19c,
        block_0x0020a1ec,
        block_0x0020a1f4,
        block_0x0020a214,
        block_0x0020a234,
        block_0x0020a27c,
        block_0x0020a288,
        block_0x0020a294,
        block_0x0020a2b0,
        block_0x0020a2b4,
        block_0x0020a2d4,
        block_0x0020a2d8,
        block_0x0020a2e0,
        block_0x0020a2ec,
        block_0x0020a354,
        block_0x0020a3a0,
        block_0x0020a3b8,
        block_0x0020a3c4,
        block_0x0020a3c8,
        block_0x0020a3fc,
        block_0x0020a414,
        block_0x0020a418,
        block_0x0020a428,
        block_0x0020a430,
        block_0x0020a460,
        block_0x0020a468,
        block_0x0020a470,
        block_0x0020a474,
        block_0x0020a480,
        block_0x0020a490,
        block_0x0020a4e4,
        block_0x0020a4f4,
        block_0x0020a4f8,
        block_0x0020a504,
        block_0x0020a520,
        block_0x0020a53c,
        block_0x0020a540,
        block_0x0020a55c,
        block_0x0020a560,
        block_0x0020a570,
        block_0x0020a578,
        block_0x0020a580,
        block_0x0020a590,
        block_0x0020a5b0,
        block_0x0020a5c4,
        block_0x0020a5d8,
        block_0x0020a5f0,
        block_0x0020a618,
        block_0x0020a638,
        block_0x0020a640,
        block_0x0020a650,
        block_0x0020a654,
        block_0x0020a658,
        block_0x0020a65c,
        block_0x0020a668,
        block_0x0020a674,
        block_0x0020a680,
        block_0x0020a684,
        block_0x0020a694,
        block_0x0020a6a8,
        block_0x0020a6bc,
        block_0x0020a6c4,
        block_0x0020a6d8,
        block_0x0020a6dc,
        block_0x0020a6e0,
        block_0x0020a6e4,
        block_0x0020a6f4,
        block_0x0020a710,
        block_0x0020a724,
        block_0x0020a738,
        block_0x0020a75c,
        block_0x0020a760,
        block_0x0020a798,
        block_0x0020a7ac,
        block_0x0020a7c0,
    ];
    const IDX: [u16; 668usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 3u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 4u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 5u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 6u16, 0u16, 0u16, 0u16,
        7u16, 8u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        9u16, 0u16, 0u16, 0u16, 10u16, 0u16, 0u16, 11u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 12u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 13u16, 0u16, 0u16, 14u16,
        0u16, 0u16, 15u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 16u16, 0u16, 0u16,
        17u16, 0u16, 0u16, 0u16, 0u16, 0u16, 18u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 19u16, 0u16, 0u16, 0u16, 20u16, 21u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 22u16, 0u16, 0u16, 0u16, 23u16, 0u16, 0u16, 24u16, 0u16, 0u16, 0u16,
        25u16, 0u16, 0u16, 0u16, 0u16, 0u16, 26u16, 0u16, 0u16, 0u16, 0u16, 0u16, 27u16,
        28u16, 29u16, 0u16, 0u16, 30u16, 31u16, 0u16, 0u16, 32u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 33u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 34u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 35u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 36u16, 0u16, 0u16, 37u16,
        0u16, 0u16, 38u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 39u16, 0u16, 40u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 41u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 42u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 43u16, 0u16, 0u16, 44u16, 0u16, 0u16, 45u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 46u16, 47u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 48u16, 49u16, 0u16, 50u16, 0u16, 0u16, 51u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 52u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 53u16, 0u16, 0u16, 0u16, 0u16, 0u16, 54u16, 0u16, 0u16, 55u16, 56u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 57u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 58u16, 59u16, 0u16, 0u16, 0u16, 60u16, 0u16, 61u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 62u16, 0u16, 63u16,
        0u16, 64u16, 65u16, 0u16, 0u16, 66u16, 0u16, 0u16, 0u16, 67u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 68u16, 0u16, 0u16, 0u16, 69u16, 70u16, 0u16, 0u16, 71u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 72u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        73u16, 74u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 75u16, 76u16, 0u16, 0u16, 0u16,
        77u16, 0u16, 78u16, 0u16, 79u16, 0u16, 0u16, 0u16, 80u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 81u16, 0u16, 0u16, 0u16, 0u16, 82u16, 0u16, 0u16, 0u16, 0u16,
        83u16, 0u16, 0u16, 0u16, 0u16, 0u16, 84u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 85u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 86u16, 0u16,
        87u16, 0u16, 0u16, 0u16, 88u16, 89u16, 90u16, 91u16, 0u16, 0u16, 92u16, 0u16,
        0u16, 93u16, 0u16, 0u16, 94u16, 95u16, 0u16, 0u16, 0u16, 96u16, 0u16, 0u16, 0u16,
        0u16, 97u16, 0u16, 0u16, 0u16, 0u16, 98u16, 0u16, 99u16, 0u16, 0u16, 0u16, 0u16,
        100u16, 101u16, 102u16, 103u16, 0u16, 0u16, 0u16, 104u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 105u16, 0u16, 0u16, 0u16, 0u16, 106u16, 0u16, 0u16, 0u16, 0u16,
        107u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 108u16, 109u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 110u16,
        0u16, 0u16, 0u16, 0u16, 111u16, 0u16, 0u16, 0u16, 0u16, 112u16,
    ];
    if pc < 2137428u32 || pc > 2140096u32 {
        return None;
    }
    let word_offset = ((pc - 2137428u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline]
pub fn block_0x00209d54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2137432u32);
    emu.lw_no_count(11usize, 10usize, 0u32, 2137436u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2137440u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2137444u32)?;
    emu.sw_no_count(12usize, 2usize, 8u32, 2137448u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2137452u32)?;
    emu.adi_no_count(10usize, 2usize, 4u32, 2137456u32);
    emu.apc_no_count(1usize, 2137456u32, 0u32, 2137460u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137464u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(776u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209d78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2137468u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2137472u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2137476u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2137480u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2137484u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2137488u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2137624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209e18));
    } else {
        emu.pc = 2137492u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209d94));
    }
}
#[inline]
pub fn block_0x00209d94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 12u32, 2137496u32)?;
    emu.adi_no_count(11usize, 0usize, 1u32, 2137500u32);
    emu.sw_no_count(0usize, 2usize, 20u32, 2137504u32)?;
    emu.sw_no_count(11usize, 2usize, 24u32, 2137508u32)?;
    emu.sw_no_count(0usize, 2usize, 28u32, 2137512u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2137516u32)?;
    emu.lw_no_count(11usize, 10usize, 0u32, 2137520u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2137524u32)?;
    emu.lw_no_count(13usize, 10usize, 8u32, 2137528u32)?;
    emu.lw_no_count(14usize, 10usize, 12u32, 2137532u32)?;
    emu.lw_no_count(15usize, 10usize, 16u32, 2137536u32)?;
    emu.lw_no_count(10usize, 10usize, 20u32, 2137540u32)?;
    emu.sw_no_count(11usize, 2usize, 32u32, 2137544u32)?;
    emu.sw_no_count(12usize, 2usize, 36u32, 2137548u32)?;
    emu.sw_no_count(13usize, 2usize, 40u32, 2137552u32)?;
    emu.sw_no_count(14usize, 2usize, 44u32, 2137556u32)?;
    emu.sw_no_count(15usize, 2usize, 48u32, 2137560u32)?;
    emu.sw_no_count(10usize, 2usize, 52u32, 2137564u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2137568u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965680u32, 2137572u32);
    emu.adi_no_count(10usize, 2usize, 20u32, 2137576u32);
    emu.adi_no_count(12usize, 2usize, 32u32, 2137580u32);
    emu.apc_no_count(1usize, 2137580u32, 24576u32, 2137584u32);
    emu.add_memory_rw_events(24usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137588u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967124u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00209df4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 20u32, 2137592u32)?;
    emu.lw_no_count(11usize, 2usize, 24u32, 2137596u32)?;
    emu.lw_no_count(12usize, 2usize, 28u32, 2137600u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2137604u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2137608u32)?;
    emu.sw_no_count(12usize, 2usize, 40u32, 2137612u32)?;
    emu.sw_no_count(10usize, 8usize, 0u32, 2137616u32)?;
    emu.sw_no_count(11usize, 8usize, 4u32, 2137620u32)?;
    emu.sw_no_count(12usize, 8usize, 8u32, 2137624u32)?;
    emu.add_memory_rw_events(9usize);
    emu.pc = 2137624u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209e18));
}
#[inline]
pub fn block_0x00209e18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 0u32, 2137628u32)?;
    emu.lw_no_count(11usize, 8usize, 4u32, 2137632u32)?;
    emu.lw_no_count(12usize, 8usize, 8u32, 2137636u32)?;
    emu.sw_no_count(10usize, 2usize, 8u32, 2137640u32)?;
    emu.sw_no_count(11usize, 2usize, 12u32, 2137644u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2137648u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2137652u32);
    emu.sw_no_count(0usize, 8usize, 0u32, 2137656u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2137660u32)?;
    emu.sw_no_count(0usize, 8usize, 8u32, 2137664u32)?;
    emu.apc_no_count(1usize, 2137664u32, 4294934528u32, 2137668u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137672u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1868u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209e48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 12u32, 2137676u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2137680u32);
    emu.apc_no_count(1usize, 2137680u32, 4294934528u32, 2137684u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137688u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965424u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209e58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2137740u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209e8c));
    } else {
        emu.pc = 2137692u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209e5c));
    }
}
#[inline]
pub fn block_0x00209e5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 2usize, 8u32, 2137696u32)?;
    emu.lw_no_count(13usize, 2usize, 12u32, 2137700u32)?;
    emu.lw_no_count(14usize, 2usize, 16u32, 2137704u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2137708u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965704u32, 2137712u32);
    emu.sw_no_count(12usize, 10usize, 0u32, 2137716u32)?;
    emu.sw_no_count(13usize, 10usize, 4u32, 2137720u32)?;
    emu.sw_no_count(14usize, 10usize, 8u32, 2137724u32)?;
    emu.lw_no_count(1usize, 2usize, 60u32, 2137728u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2137732u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2137736u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137740u32;
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
pub fn block_0x00209e8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2137744u32);
    emu.adi_no_count(11usize, 0usize, 12u32, 2137748u32);
    emu.apc_no_count(1usize, 2137748u32, 4096u32, 2137752u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137756u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(668u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209e9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2137760u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2137764u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2137932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209f4c));
    } else {
        emu.pc = 2137768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209ea8));
    }
}
#[inline(never)]
pub fn block_0x00209ea8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2137772u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2137776u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2137780u32)?;
    emu.lw_no_count(11usize, 10usize, 12u32, 2137784u32)?;
    emu.adi_no_count(12usize, 0usize, 1u32, 2137788u32);
    emu.sw_no_count(0usize, 2usize, 4u32, 2137792u32)?;
    emu.sw_no_count(12usize, 2usize, 8u32, 2137796u32)?;
    emu.sw_no_count(0usize, 2usize, 12u32, 2137800u32)?;
    emu.lw_no_count(11usize, 11usize, 0u32, 2137804u32)?;
    emu.lw_no_count(12usize, 11usize, 0u32, 2137808u32)?;
    emu.lw_no_count(13usize, 11usize, 4u32, 2137812u32)?;
    emu.lw_no_count(14usize, 11usize, 8u32, 2137816u32)?;
    emu.lw_no_count(15usize, 11usize, 12u32, 2137820u32)?;
    emu.lw_no_count(16usize, 11usize, 16u32, 2137824u32)?;
    emu.lw_no_count(11usize, 11usize, 20u32, 2137828u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2137832u32)?;
    emu.sw_no_count(13usize, 2usize, 20u32, 2137836u32)?;
    emu.sw_no_count(14usize, 2usize, 24u32, 2137840u32)?;
    emu.sw_no_count(15usize, 2usize, 28u32, 2137844u32)?;
    emu.sw_no_count(16usize, 2usize, 32u32, 2137848u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2137852u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2137856u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965680u32, 2137860u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2137864u32);
    emu.adi_no_count(10usize, 2usize, 4u32, 2137868u32);
    emu.adi_no_count(12usize, 2usize, 16u32, 2137872u32);
    emu.apc_no_count(1usize, 2137872u32, 24576u32, 2137876u32);
    emu.add_memory_rw_events(28usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137880u32;
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
#[inline]
pub fn block_0x00209f18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 0u32, 2137884u32);
    emu.lw_no_count(11usize, 2usize, 4u32, 2137888u32)?;
    emu.lw_no_count(12usize, 2usize, 8u32, 2137892u32)?;
    emu.lw_no_count(13usize, 2usize, 12u32, 2137896u32)?;
    emu.sw_no_count(11usize, 2usize, 16u32, 2137900u32)?;
    emu.sw_no_count(12usize, 2usize, 20u32, 2137904u32)?;
    emu.sw_no_count(13usize, 2usize, 24u32, 2137908u32)?;
    emu.sw_no_count(11usize, 8usize, 0u32, 2137912u32)?;
    emu.sw_no_count(12usize, 8usize, 4u32, 2137916u32)?;
    emu.sw_no_count(13usize, 8usize, 8u32, 2137920u32)?;
    emu.lw_no_count(1usize, 2usize, 44u32, 2137924u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2137928u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2137932u32);
    emu.add_memory_rw_events(13usize);
    emu.pc = 2137932u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209f4c));
}
#[inline(always)]
pub fn block_0x00209f4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2137936u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965704u32, 2137940u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2137944u32;
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
pub fn block_0x00209f58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2137948u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2137952u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2138052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209fc4));
    } else {
        emu.pc = 2137956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209f64));
    }
}
#[inline]
pub fn block_0x00209f64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2137960u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2137964u32)?;
    emu.lw_no_count(10usize, 10usize, 12u32, 2137968u32)?;
    emu.lw_no_count(12usize, 10usize, 0u32, 2137972u32)?;
    emu.lw_no_count(10usize, 11usize, 0u32, 2137976u32)?;
    emu.lw_no_count(11usize, 11usize, 4u32, 2137980u32)?;
    emu.lw_no_count(13usize, 12usize, 0u32, 2137984u32)?;
    emu.lw_no_count(14usize, 12usize, 4u32, 2137988u32)?;
    emu.lw_no_count(15usize, 12usize, 8u32, 2137992u32)?;
    emu.lw_no_count(16usize, 12usize, 12u32, 2137996u32)?;
    emu.lw_no_count(17usize, 12usize, 16u32, 2138000u32)?;
    emu.lw_no_count(12usize, 12usize, 20u32, 2138004u32)?;
    emu.sw_no_count(13usize, 2usize, 4u32, 2138008u32)?;
    emu.sw_no_count(14usize, 2usize, 8u32, 2138012u32)?;
    emu.sw_no_count(15usize, 2usize, 12u32, 2138016u32)?;
    emu.sw_no_count(16usize, 2usize, 16u32, 2138020u32)?;
    emu.sw_no_count(17usize, 2usize, 20u32, 2138024u32)?;
    emu.sw_no_count(12usize, 2usize, 24u32, 2138028u32)?;
    emu.adi_no_count(12usize, 2usize, 4u32, 2138032u32);
    emu.apc_no_count(1usize, 2138032u32, 24576u32, 2138036u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138040u32;
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
#[inline(always)]
pub fn block_0x00209fb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2138044u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2138048u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138052u32;
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
pub fn block_0x00209fc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 10usize, 4u32, 2138056u32)?;
    emu.lw_no_count(12usize, 10usize, 8u32, 2138060u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2138064u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2138068u32);
    emu.apc_no_count(6usize, 2138068u32, 28672u32, 2138072u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2138076u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966292u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00209fdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2138080u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2138084u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2138088u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2138092u32)?;
    emu.lw_no_count(8usize, 10usize, 0u32, 2138096u32)?;
    emu.lw_no_count(9usize, 10usize, 4u32, 2138100u32)?;
    emu.apc_no_count(1usize, 2138100u32, 4294934528u32, 2138104u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138108u32;
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
pub fn block_0x00209ffc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 8u32, 2138112u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2138116u32);
    emu.apc_no_count(1usize, 2138116u32, 4294930432u32, 2138120u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138124u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1788u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a00c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2138164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a034));
    } else {
        emu.pc = 2138128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a010));
    }
}
#[inline]
pub fn block_0x0020a010(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2138132u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965720u32, 2138136u32);
    emu.sw_no_count(8usize, 10usize, 0u32, 2138140u32)?;
    emu.sw_no_count(9usize, 10usize, 4u32, 2138144u32)?;
    emu.lw_no_count(1usize, 2usize, 12u32, 2138148u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2138152u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2138156u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2138160u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138164u32;
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
pub fn block_0x0020a034(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2138168u32);
    emu.adi_no_count(11usize, 0usize, 8u32, 2138172u32);
    emu.apc_no_count(1usize, 2138172u32, 4096u32, 2138176u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138180u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(244u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a044(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2138184u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965720u32, 2138188u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138192u32;
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
pub fn block_0x0020a050(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2138196u32)?;
    emu.lw_no_count(11usize, 10usize, 4u32, 2138200u32)?;
    emu.adi_no_count(10usize, 12usize, 0u32, 2138204u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138208u32;
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
pub fn block_0x0020a060(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 10usize, 0u32, 2138212u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2138216u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2138220u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2138224u32);
    emu.apc_no_count(6usize, 2138224u32, 28672u32, 2138228u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2138232u32;
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
pub fn block_0x0020a078(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2138236u32);
    emu.lw_no_count(11usize, 10usize, 0u32, 2138240u32)?;
    emu.lw_no_count(13usize, 11usize, 4u32, 2138244u32)?;
    emu.lw_no_count(12usize, 11usize, 12u32, 2138248u32)?;
    emu.adi_no_count(14usize, 0usize, 1u32, 2138252u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2138276u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a0a4));
    } else {
        emu.pc = 2138256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a090));
    }
}
#[inline(always)]
pub fn block_0x0020a090(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2138336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a0e0));
    } else {
        emu.pc = 2138260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a094));
    }
}
#[inline(always)]
pub fn block_0x0020a094(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2138336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a0e0));
    } else {
        emu.pc = 2138264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a098));
    }
}
#[inline(always)]
pub fn block_0x0020a098(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2138268u32);
    emu.adi_no_count(15usize, 0usize, 1u32, 2138272u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2138276u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2138292u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a0b4));
}
#[inline(always)]
pub fn block_0x0020a0a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2138336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a0e0));
    } else {
        emu.pc = 2138280u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a0a8));
    }
}
#[inline(always)]
pub fn block_0x0020a0a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 11usize, 0u32, 2138284u32)?;
    emu.lw_no_count(15usize, 11usize, 0u32, 2138288u32)?;
    emu.lw_no_count(11usize, 11usize, 4u32, 2138292u32)?;
    emu.add_memory_rw_events(3usize);
    emu.pc = 2138292u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a0b4));
}
#[inline]
pub fn block_0x0020a0b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(14usize, 10usize, 8u32, 2138296u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2138300u32)?;
    emu.lbu_no_count(13usize, 14usize, 8u32, 2138304u32);
    emu.lbu_no_count(14usize, 14usize, 9u32, 2138308u32);
    emu.sw_no_count(15usize, 2usize, 0u32, 2138312u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2138316u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2138320u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965736u32, 2138324u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2138328u32);
    emu.apc_no_count(1usize, 2138328u32, 0u32, 2138332u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138336u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(348u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020a0e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 8u32, 2138340u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2138344u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2138348u32)?;
    emu.lbu_no_count(13usize, 11usize, 8u32, 2138352u32);
    emu.lbu_no_count(14usize, 11usize, 9u32, 2138356u32);
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2138360u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 2usize, 0u32, 2138364u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2138368u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965764u32, 2138372u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2138376u32);
    emu.apc_no_count(1usize, 2138376u32, 0u32, 2138380u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138384u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(300u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020a110(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2138388u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2138392u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2138396u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2138400u32)?;
    emu.lw_no_count(9usize, 11usize, 12u32, 2138404u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2138408u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2138412u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2138416u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(9usize);
    let return_addr = 2138420u32;
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
pub fn block_0x0020a134(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2138424u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2138428u32)?;
    emu.lw_no_count(12usize, 2usize, 0u32, 2138432u32)?;
    emu.lw_no_count(13usize, 2usize, 4u32, 2138436u32)?;
    let a = 0u32.wrapping_add(3112902656u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2138440u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966129u32, 2138444u32);
    emu.xrr_no_count(11usize, 11usize, 14usize, 2138448u32);
    let a = 0u32.wrapping_add(1676365824u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2138452u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 44u32, 2138456u32);
    emu.xrr_no_count(13usize, 13usize, 14usize, 2138460u32);
    let a = 0u32.wrapping_add(1470513152u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2138464u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 376u32, 2138468u32);
    emu.xrr_no_count(10usize, 10usize, 14usize, 2138472u32);
    let a = 0u32.wrapping_add(3603652608u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2138476u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966637u32, 2138480u32);
    emu.xrr_no_count(12usize, 12usize, 14usize, 2138484u32);
    emu.orr_no_count(11usize, 13usize, 11usize, 2138488u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2138492u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2138496u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2138512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a190));
    } else {
        emu.pc = 2138500u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a184));
    }
}
#[inline(always)]
pub fn block_0x0020a184(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 4u32, 2138504u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2138508u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2138512u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2138612u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a1f4));
}
#[inline(always)]
pub fn block_0x0020a190(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2138516u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2138520u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(9usize);
    let return_addr = 2138524u32;
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
pub fn block_0x0020a19c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2138528u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2138532u32)?;
    emu.lw_no_count(12usize, 2usize, 0u32, 2138536u32)?;
    emu.lw_no_count(13usize, 2usize, 4u32, 2138540u32)?;
    let a = 0u32.wrapping_add(2330587136u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2138544u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 428u32, 2138548u32);
    emu.xrr_no_count(11usize, 11usize, 14usize, 2138552u32);
    let a = 0u32.wrapping_add(2965131264u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2138556u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965685u32, 2138560u32);
    emu.xrr_no_count(13usize, 13usize, 14usize, 2138564u32);
    let a = 0u32.wrapping_add(4112453632u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2138568u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1473u32, 2138572u32);
    emu.xrr_no_count(10usize, 10usize, 14usize, 2138576u32);
    let a = 0u32.wrapping_add(2020298752u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2138580u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965580u32, 2138584u32);
    emu.xrr_no_count(12usize, 12usize, 14usize, 2138588u32);
    emu.orr_no_count(11usize, 13usize, 11usize, 2138592u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2138596u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2138600u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2138644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a214));
    } else {
        emu.pc = 2138604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a1ec));
    }
}
#[inline(always)]
pub fn block_0x0020a1ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 4u32, 2138608u32);
    emu.adi_no_count(11usize, 0usize, 8u32, 2138612u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2138612u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a1f4));
}
#[inline(always)]
pub fn block_0x0020a1f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2138616u32)?;
    emu.adr_no_count(11usize, 8usize, 11usize, 2138620u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2138624u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2138628u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2138632u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2138636u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2138640u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138644u32;
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
pub fn block_0x0020a214(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2138648u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965792u32, 2138652u32);
    emu.adi_no_count(11usize, 0usize, 12u32, 2138656u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2138660u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2138664u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2138668u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2138672u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138676u32;
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
pub fn block_0x0020a234(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967200u32, 2138680u32);
    emu.sw_no_count(1usize, 2usize, 92u32, 2138684u32)?;
    emu.sw_no_count(8usize, 2usize, 88u32, 2138688u32)?;
    emu.sw_no_count(9usize, 2usize, 84u32, 2138692u32)?;
    emu.sw_no_count(18usize, 2usize, 80u32, 2138696u32)?;
    emu.sw_no_count(19usize, 2usize, 76u32, 2138700u32)?;
    emu.sw_no_count(20usize, 2usize, 72u32, 2138704u32)?;
    emu.adi_no_count(19usize, 14usize, 0u32, 2138708u32);
    emu.adi_no_count(18usize, 13usize, 0u32, 2138712u32);
    emu.adi_no_count(20usize, 12usize, 0u32, 2138716u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2138720u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2138724u32);
    emu.sw_no_count(10usize, 2usize, 4u32, 2138728u32)?;
    emu.sw_no_count(11usize, 2usize, 8u32, 2138732u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2138736u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2138740u32);
    emu.apc_no_count(1usize, 2138740u32, 4096u32, 2138744u32);
    emu.add_memory_rw_events(18usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138748u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x0020a27c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 255u32, 2138752u32);
    emu.adi_no_count(11usize, 0usize, 2u32, 2138756u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2138840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a2d8));
    } else {
        emu.pc = 2138760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a288));
    }
}
#[inline(always)]
pub fn block_0x0020a288(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2138764u32;
    emu.update_insn_clock();
    emu.lw_no_count(11usize, 10usize, 4294965652u32, 2138768u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2139252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a474));
    } else {
        emu.pc = 2138772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a294));
    }
}
#[inline(always)]
pub fn block_0x0020a294(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 4294965652u32, 2138776u32);
    emu.lw_no_count(12usize, 12usize, 4u32, 2138780u32)?;
    emu.adi_no_count(11usize, 11usize, 1u32, 2138784u32);
    emu.sw_no_count(11usize, 10usize, 4294965652u32, 2138788u32)?;
    emu.lw_no_count(11usize, 9usize, 20u32, 2138792u32)?;
    emu.adi_no_count(10usize, 8usize, 0u32, 2138796u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2139076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a3c4));
    } else {
        emu.pc = 2138800u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a2b0));
    }
}
#[inline(always)]
pub fn block_0x0020a2b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2138804u32;
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
pub fn block_0x0020a2b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 48u32, 2138808u32)?;
    emu.sw_no_count(11usize, 2usize, 52u32, 2138812u32)?;
    emu.sw_no_count(20usize, 2usize, 56u32, 2138816u32)?;
    emu.sb_no_count(18usize, 2usize, 60u32, 2138820u32);
    emu.sb_no_count(19usize, 2usize, 61u32, 2138824u32);
    emu.adi_no_count(10usize, 2usize, 48u32, 2138828u32);
    emu.apc_no_count(1usize, 2138828u32, 0u32, 2138832u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138836u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965372u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a2d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2138840u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2139132u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a3fc));
}
#[inline(always)]
pub fn block_0x0020a2d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 1u32, 2138844u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2138964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a354));
    } else {
        emu.pc = 2138848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a2e0));
    }
}
#[inline(always)]
pub fn block_0x0020a2e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 9usize, 24u32, 2138852u32)?;
    emu.adi_no_count(10usize, 8usize, 0u32, 2138856u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2138860u32;
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
#[inline(never)]
pub fn block_0x0020a2ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 26u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(12usize, 10usize, 1u32, 2138864u32);
    emu.adi_no_count(13usize, 2usize, 12u32, 2138868u32);
    let a = 0u32.wrapping_add(2134016u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2138872u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966420u32, 2138876u32);
    emu.adi_no_count(15usize, 2usize, 16u32, 2138880u32);
    let a = 0u32.wrapping_add(2142208u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2138884u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 4294965708u32, 2138888u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2138892u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 17usize, 4294965920u32, 2138896u32);
    emu.sw_no_count(13usize, 2usize, 32u32, 2138900u32)?;
    emu.sw_no_count(14usize, 2usize, 36u32, 2138904u32)?;
    emu.sw_no_count(15usize, 2usize, 40u32, 2138908u32)?;
    emu.sw_no_count(16usize, 2usize, 44u32, 2138912u32)?;
    emu.adi_no_count(13usize, 0usize, 3u32, 2138916u32);
    emu.sw_no_count(0usize, 2usize, 64u32, 2138920u32)?;
    emu.adr_no_count(10usize, 10usize, 12usize, 2138924u32);
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2138928u32);
    emu.anr_no_count(11usize, 12usize, 11usize, 2138932u32);
    emu.adi_no_count(12usize, 2usize, 32u32, 2138936u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2138940u32)?;
    emu.sw_no_count(11usize, 2usize, 20u32, 2138944u32)?;
    emu.adi_no_count(10usize, 0usize, 2u32, 2138948u32);
    emu.sw_no_count(17usize, 2usize, 48u32, 2138952u32)?;
    emu.sw_no_count(13usize, 2usize, 52u32, 2138956u32)?;
    emu.sw_no_count(12usize, 2usize, 56u32, 2138960u32)?;
    emu.add_memory_rw_events(26usize);
    let return_addr = 2138964u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2139040u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a3a0));
}
#[inline]
pub fn block_0x0020a354(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 12u32, 2138968u32);
    let a = 0u32.wrapping_add(2134016u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2138972u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966420u32, 2138976u32);
    emu.adi_no_count(12usize, 2usize, 4u32, 2138980u32);
    let a = 0u32.wrapping_add(2142208u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2138984u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966036u32, 2138988u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2138992u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965832u32, 2138996u32);
    emu.adi_no_count(15usize, 0usize, 3u32, 2139000u32);
    emu.sw_no_count(0usize, 2usize, 64u32, 2139004u32)?;
    emu.adi_no_count(16usize, 2usize, 32u32, 2139008u32);
    emu.sw_no_count(10usize, 2usize, 32u32, 2139012u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2139016u32)?;
    emu.sw_no_count(12usize, 2usize, 40u32, 2139020u32)?;
    emu.sw_no_count(13usize, 2usize, 44u32, 2139024u32)?;
    emu.adi_no_count(10usize, 0usize, 2u32, 2139028u32);
    emu.sw_no_count(14usize, 2usize, 48u32, 2139032u32)?;
    emu.sw_no_count(15usize, 2usize, 52u32, 2139036u32)?;
    emu.sw_no_count(16usize, 2usize, 56u32, 2139040u32)?;
    emu.add_memory_rw_events(19usize);
    emu.pc = 2139040u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a3a0));
}
#[inline(always)]
pub fn block_0x0020a3a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 60u32, 2139044u32)?;
    emu.adi_no_count(10usize, 2usize, 24u32, 2139048u32);
    emu.adi_no_count(11usize, 2usize, 24u32, 2139052u32);
    emu.adi_no_count(12usize, 2usize, 48u32, 2139056u32);
    emu.apc_no_count(1usize, 2139056u32, 0u32, 2139060u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139064u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1060u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a3b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 24u32, 2139068u32);
    emu.lw_no_count(11usize, 2usize, 28u32, 2139072u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2139076u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2139240u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a468));
}
#[inline(always)]
pub fn block_0x0020a3c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2139080u32;
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
pub fn block_0x0020a3c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2139084u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965652u32, 2139088u32);
    emu.lw_no_count(13usize, 12usize, 8u32, 2139092u32)?;
    emu.lw_no_count(12usize, 12usize, 4u32, 2139096u32)?;
    emu.lw_no_count(13usize, 13usize, 20u32, 2139100u32)?;
    emu.sw_no_count(10usize, 2usize, 48u32, 2139104u32)?;
    emu.sw_no_count(11usize, 2usize, 52u32, 2139108u32)?;
    emu.sw_no_count(20usize, 2usize, 56u32, 2139112u32)?;
    emu.sb_no_count(18usize, 2usize, 60u32, 2139116u32);
    emu.sb_no_count(19usize, 2usize, 61u32, 2139120u32);
    emu.adi_no_count(11usize, 2usize, 48u32, 2139124u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2139128u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2139132u32;
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
pub fn block_0x0020a3fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2139136u32;
    emu.update_insn_clock();
    emu.lw_no_count(11usize, 10usize, 4294965652u32, 2139140u32)?;
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2139144u32);
    emu.sw_no_count(11usize, 10usize, 4294965652u32, 2139148u32)?;
    emu.apc_no_count(1usize, 2139148u32, 4096u32, 2139152u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139156u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965892u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a414(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2139176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a428));
    } else {
        emu.pc = 2139160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a418));
    }
}
#[inline(always)]
pub fn block_0x0020a418(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 0u32, 2139164u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2139168u32);
    emu.apc_no_count(1usize, 2139168u32, 0u32, 2139172u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139176u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(96u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a428(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2139180u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965992u32, 2139184u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2139184u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a430));
}
#[inline]
pub fn block_0x0020a430(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2139188u32);
    emu.sw_no_count(0usize, 2usize, 64u32, 2139192u32)?;
    emu.adi_no_count(12usize, 0usize, 4u32, 2139196u32);
    emu.sw_no_count(10usize, 2usize, 48u32, 2139200u32)?;
    emu.sw_no_count(11usize, 2usize, 52u32, 2139204u32)?;
    emu.sw_no_count(12usize, 2usize, 56u32, 2139208u32)?;
    emu.sw_no_count(0usize, 2usize, 60u32, 2139212u32)?;
    emu.adi_no_count(10usize, 2usize, 32u32, 2139216u32);
    emu.adi_no_count(11usize, 2usize, 24u32, 2139220u32);
    emu.adi_no_count(12usize, 2usize, 48u32, 2139224u32);
    emu.apc_no_count(1usize, 2139224u32, 0u32, 2139228u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139232u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(892u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a460(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 32u32, 2139236u32);
    emu.lw_no_count(11usize, 2usize, 36u32, 2139240u32)?;
    emu.add_memory_rw_events(2usize);
    emu.pc = 2139240u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a468));
}
#[inline(always)]
pub fn block_0x0020a468(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2139240u32, 4294963200u32, 2139244u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139248u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(144u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a470(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.update_insn_clock();
    return Err(format!("Unimplemented instruction (UNIMP) at PC {:#x}", 2139248u32));
}
#[inline(always)]
pub fn block_0x0020a474(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2139256u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966140u32, 2139260u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2139264u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2139184u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a430));
}
#[inline(always)]
pub fn block_0x0020a480(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2139268u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2139272u32)?;
    emu.apc_no_count(1usize, 2139272u32, 4294934528u32, 2139276u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139280u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(552u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020a490(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 12u32, 2139284u32)?;
    emu.adi_no_count(10usize, 2usize, 12u32, 2139288u32);
    let a = 0u32.wrapping_add(2146304u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2139292u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965756u32, 2139296u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2139300u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966064u32, 2139304u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2139308u32);
    emu.sw_no_count(0usize, 2usize, 40u32, 2139312u32)?;
    emu.adi_no_count(14usize, 2usize, 48u32, 2139316u32);
    emu.sw_no_count(10usize, 2usize, 48u32, 2139320u32)?;
    emu.sw_no_count(11usize, 2usize, 52u32, 2139324u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2139328u32);
    emu.sw_no_count(12usize, 2usize, 24u32, 2139332u32)?;
    emu.sw_no_count(13usize, 2usize, 28u32, 2139336u32)?;
    emu.sw_no_count(14usize, 2usize, 32u32, 2139340u32)?;
    emu.sw_no_count(10usize, 2usize, 36u32, 2139344u32)?;
    emu.adi_no_count(10usize, 2usize, 16u32, 2139348u32);
    emu.adi_no_count(11usize, 2usize, 59u32, 2139352u32);
    emu.adi_no_count(12usize, 2usize, 24u32, 2139356u32);
    emu.apc_no_count(1usize, 2139356u32, 0u32, 2139360u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139364u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(760u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a4e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 16u32, 2139368u32);
    emu.lw_no_count(11usize, 2usize, 20u32, 2139372u32)?;
    emu.apc_no_count(1usize, 2139372u32, 4294963200u32, 2139376u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139380u32;
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
pub fn block_0x0020a4f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.update_insn_clock();
    return Err(format!("Unimplemented instruction (UNIMP) at PC {:#x}", 2139380u32));
}
#[inline(always)]
pub fn block_0x0020a4f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 4u32, 2139388u32);
    emu.sb_no_count(11usize, 10usize, 0u32, 2139392u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139396u32;
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
pub fn block_0x0020a504(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2139400u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2139404u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2139408u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2139412u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2139416u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2139420u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139424u32;
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
pub fn block_0x0020a520(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2139428u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2139432u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2139436u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2139440u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2139444u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2139448u32)?;
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(5usize);
    let return_addr = 2139452u32;
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
pub fn block_0x0020a53c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.update_insn_clock();
    return Err(format!("Unimplemented instruction (UNIMP) at PC {:#x}", 2139452u32));
}
#[inline(always)]
pub fn block_0x0020a540(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2139460u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2139464u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2139468u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2139472u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2139476u32);
    emu.adi_no_count(12usize, 12usize, 4294967288u32, 2139480u32);
    emu.sli_no_count(13usize, 13usize, 3u32, 2139484u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2139484u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a55c));
}
#[inline(always)]
pub fn block_0x0020a55c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2139512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a578));
    } else {
        emu.pc = 2139488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a560));
    }
}
#[inline(always)]
pub fn block_0x0020a560(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(9usize, 12usize, 12u32, 2139492u32)?;
    emu.adi_no_count(12usize, 12usize, 8u32, 2139496u32);
    emu.adi_no_count(13usize, 13usize, 4294967288u32, 2139500u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2139484u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a55c));
    } else {
        emu.pc = 2139504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a570));
    }
}
#[inline(always)]
pub fn block_0x0020a570(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 12usize, 0u32, 2139508u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2139512u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2139520u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a580));
}
#[inline(always)]
pub fn block_0x0020a578(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2139516u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2139520u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2139520u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a580));
}
#[inline(always)]
pub fn block_0x0020a580(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2139524u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2139528u32);
    emu.apc_no_count(1usize, 2139528u32, 4294938624u32, 2139532u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139536u32;
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
pub fn block_0x0020a590(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2139540u32);
    emu.sb_no_count(10usize, 8usize, 0u32, 2139544u32);
    emu.sw_no_count(9usize, 8usize, 4u32, 2139548u32)?;
    emu.lw_no_count(1usize, 2usize, 12u32, 2139552u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2139556u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2139560u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2139564u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139568u32;
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
pub fn block_0x0020a5b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2139572u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2139576u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2139580u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2139584u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2139608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a5d8));
    } else {
        emu.pc = 2139588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a5c4));
    }
}
#[inline(always)]
pub fn block_0x0020a5c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2139592u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2139596u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2139600u32);
    emu.apc_no_count(1usize, 2139600u32, 4294938624u32, 2139604u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139608u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965276u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a5d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2139612u32);
    emu.sb_no_count(10usize, 8usize, 0u32, 2139616u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2139620u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2139624u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2139628u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139632u32;
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
pub fn block_0x0020a5f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2139636u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2139640u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2139644u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2139648u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2139652u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2139656u32)?;
    emu.sw_no_count(20usize, 2usize, 40u32, 2139660u32)?;
    emu.sw_no_count(21usize, 2usize, 36u32, 2139664u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2139668u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2139740u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a65c));
    } else {
        emu.pc = 2139672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a618));
    }
}
#[inline(always)]
pub fn block_0x0020a618(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2139676u32);
    let a = 0u32.wrapping_add(536870912u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2139680u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 13usize, 4294967295u32, 2139684u32);
    emu.sli_no_count(11usize, 13usize, 3u32, 2139688u32);
    emu.adi_no_count(19usize, 19usize, 4294967295u32, 2139692u32);
    emu.anr_no_count(14usize, 14usize, 19usize, 2139696u32);
    emu.adi_no_count(14usize, 14usize, 1u32, 2139700u32);
    emu.adi_no_count(15usize, 12usize, 4u32, 2139704u32);
    emu.add_memory_rw_events(8usize);
    emu.pc = 2139704u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a638));
}
#[inline(always)]
pub fn block_0x0020a638(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 15usize, 0u32, 2139708u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a != b {
        emu.pc = 2139732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a654));
    } else {
        emu.pc = 2139712u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a640));
    }
}
#[inline(always)]
pub fn block_0x0020a640(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 10usize, 1u32, 2139716u32);
    emu.adi_no_count(11usize, 11usize, 4294967288u32, 2139720u32);
    emu.adi_no_count(15usize, 15usize, 8u32, 2139724u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2139704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a638));
    } else {
        emu.pc = 2139728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a650));
    }
}
#[inline(always)]
pub fn block_0x0020a650(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 14usize, 0u32, 2139732u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2139732u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a654));
}
#[inline(always)]
pub fn block_0x0020a654(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2140056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a798));
    } else {
        emu.pc = 2139736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a658));
    }
}
#[inline(always)]
pub fn block_0x0020a658(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2139752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a668));
    } else {
        emu.pc = 2139740u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a65c));
    }
}
#[inline(always)]
pub fn block_0x0020a65c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2139744u32);
    emu.sb_no_count(10usize, 8usize, 0u32, 2139748u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2139752u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2139960u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a738));
}
#[inline(always)]
pub fn block_0x0020a668(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(20usize, 10usize, 3u32, 2139756u32);
    emu.adr_no_count(20usize, 12usize, 20usize, 2139760u32);
    emu.sbr_no_count(9usize, 13usize, 10usize, 2139764u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2139764u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a674));
}
#[inline(always)]
pub fn block_0x0020a674(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(21usize, 9usize, 3u32, 2139768u32);
    emu.adi_no_count(10usize, 20usize, 4294967288u32, 2139772u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2139776u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2139776u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a680));
}
#[inline(always)]
pub fn block_0x0020a680(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2139920u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a710));
    } else {
        emu.pc = 2139780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a684));
    }
}
#[inline(always)]
pub fn block_0x0020a684(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 10usize, 12u32, 2139784u32)?;
    emu.adi_no_count(10usize, 10usize, 8u32, 2139788u32);
    emu.adi_no_count(11usize, 11usize, 4294967288u32, 2139792u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2139776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a680));
    } else {
        emu.pc = 2139796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a694));
    }
}
#[inline(always)]
pub fn block_0x0020a694(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2139800u32)?;
    emu.adi_no_count(10usize, 0usize, 2u32, 2139804u32);
    emu.adi_no_count(12usize, 18usize, 0u32, 2139808u32);
    emu.apc_no_count(1usize, 2139808u32, 4294934528u32, 2139812u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139816u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1868u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a6a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2139820u32);
    emu.adr_no_count(11usize, 9usize, 19usize, 2139824u32);
    emu.anr_no_count(11usize, 11usize, 19usize, 2139828u32);
    emu.adi_no_count(11usize, 11usize, 1u32, 2139832u32);
    emu.adi_no_count(12usize, 20usize, 4u32, 2139836u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2139836u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a6bc));
}
#[inline(always)]
pub fn block_0x0020a6bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 12usize, 0u32, 2139840u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a < b {
        emu.pc = 2139868u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a6dc));
    } else {
        emu.pc = 2139844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a6c4));
    }
}
#[inline(always)]
pub fn block_0x0020a6c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(18usize, 18usize, 13usize, 2139848u32);
    emu.adi_no_count(10usize, 10usize, 1u32, 2139852u32);
    emu.adi_no_count(21usize, 21usize, 4294967288u32, 2139856u32);
    emu.adi_no_count(12usize, 12usize, 8u32, 2139860u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a != b {
        emu.pc = 2139836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a6bc));
    } else {
        emu.pc = 2139864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a6d8));
    }
}
#[inline(always)]
pub fn block_0x0020a6d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 11usize, 0u32, 2139868u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2139868u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a6dc));
}
#[inline(always)]
pub fn block_0x0020a6dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2140076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a7ac));
    } else {
        emu.pc = 2139872u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a6e0));
    }
}
#[inline(always)]
pub fn block_0x0020a6e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2139996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a75c));
    } else {
        emu.pc = 2139876u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a6e4));
    }
}
#[inline(always)]
pub fn block_0x0020a6e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(11usize, 10usize, 3u32, 2139880u32);
    emu.adr_no_count(20usize, 20usize, 11usize, 2139884u32);
    emu.lw_no_count(11usize, 20usize, 4u32, 2139888u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2140096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a7c0));
    } else {
        emu.pc = 2139892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a6f4));
    }
}
#[inline(always)]
pub fn block_0x0020a6f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 20usize, 0u32, 2139896u32)?;
    emu.sbr_no_count(9usize, 9usize, 10usize, 2139900u32);
    emu.sbr_no_count(10usize, 11usize, 18usize, 2139904u32);
    emu.adr_no_count(12usize, 12usize, 18usize, 2139908u32);
    emu.sw_no_count(12usize, 20usize, 0u32, 2139912u32)?;
    emu.sw_no_count(10usize, 20usize, 4u32, 2139916u32)?;
    emu.add_memory_rw_events(7usize);
    let return_addr = 2139920u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2139764u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a674));
}
#[inline(always)]
pub fn block_0x0020a710(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2139924u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2139928u32);
    emu.adi_no_count(12usize, 0usize, 0u32, 2139932u32);
    emu.apc_no_count(1usize, 2139932u32, 4294934528u32, 2139936u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139940u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1744u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a724(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2139944u32;
    emu.update_insn_clock();
    emu.lw_no_count(11usize, 10usize, 4294966288u32, 2139948u32)?;
    emu.lw_no_count(10usize, 10usize, 4294966292u32, 2139952u32)?;
    emu.sw_no_count(11usize, 8usize, 0u32, 2139956u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2139960u32)?;
    emu.add_memory_rw_events(5usize);
    emu.pc = 2139960u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a738));
}
#[inline]
pub fn block_0x0020a738(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2139964u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2139968u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2139972u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2139976u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2139980u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2139984u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2139988u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2139992u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139996u32;
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
pub fn block_0x0020a75c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2139740u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a65c));
    } else {
        emu.pc = 2140000u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a760));
    }
}
#[inline]
pub fn block_0x0020a760(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2140004u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966188u32, 2140008u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2140012u32);
    emu.sw_no_count(0usize, 2usize, 28u32, 2140016u32)?;
    emu.adi_no_count(12usize, 0usize, 4u32, 2140020u32);
    emu.sw_no_count(10usize, 2usize, 12u32, 2140024u32)?;
    emu.sw_no_count(11usize, 2usize, 16u32, 2140028u32)?;
    emu.sw_no_count(12usize, 2usize, 20u32, 2140032u32)?;
    emu.sw_no_count(0usize, 2usize, 24u32, 2140036u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2140040u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966196u32, 2140044u32);
    emu.adi_no_count(10usize, 2usize, 12u32, 2140048u32);
    emu.apc_no_count(1usize, 2140048u32, 8192u32, 2140052u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140056u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965820u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a798(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2140060u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966228u32, 2140064u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2140068u32);
    emu.apc_no_count(1usize, 2140068u32, 36864u32, 2140072u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140076u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(572u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a7ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2140080u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966228u32, 2140084u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2140088u32);
    emu.apc_no_count(1usize, 2140088u32, 36864u32, 2140092u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140096u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(552u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a7c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2140100u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966212u32, 2140104u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2140108u32);
    emu.apc_no_count(1usize, 2140108u32, 36864u32, 2140112u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140116u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(532u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
