pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2163276u32;
pub const PC_MAX: u32 = 2165716u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 142usize] = [
        block_0x0021024c,
        block_0x0021029c,
        block_0x002102b0,
        block_0x002102b4,
        block_0x002102c4,
        block_0x002102cc,
        block_0x002102d4,
        block_0x002102dc,
        block_0x002102f4,
        block_0x00210300,
        block_0x00210308,
        block_0x00210324,
        block_0x00210328,
        block_0x00210330,
        block_0x00210344,
        block_0x00210350,
        block_0x00210374,
        block_0x00210378,
        block_0x002103c4,
        block_0x002103d4,
        block_0x002103e0,
        block_0x002103e4,
        block_0x00210428,
        block_0x00210430,
        block_0x00210444,
        block_0x0021044c,
        block_0x00210460,
        block_0x00210464,
        block_0x00210468,
        block_0x00210474,
        block_0x00210480,
        block_0x00210494,
        block_0x00210498,
        block_0x002104a8,
        block_0x002104ac,
        block_0x002104c8,
        block_0x002104d0,
        block_0x002104e4,
        block_0x002104e8,
        block_0x002104ec,
        block_0x0021052c,
        block_0x00210548,
        block_0x00210550,
        block_0x00210564,
        block_0x00210568,
        block_0x0021057c,
        block_0x00210588,
        block_0x0021059c,
        block_0x002105a0,
        block_0x002105a4,
        block_0x002105d0,
        block_0x002105e0,
        block_0x002105ec,
        block_0x002105f4,
        block_0x002105f8,
        block_0x00210620,
        block_0x00210624,
        block_0x00210640,
        block_0x00210688,
        block_0x00210690,
        block_0x00210698,
        block_0x002106a0,
        block_0x002106a8,
        block_0x002106c0,
        block_0x002106c8,
        block_0x002106e4,
        block_0x002106e8,
        block_0x002106f0,
        block_0x002106f8,
        block_0x00210714,
        block_0x00210718,
        block_0x0021072c,
        block_0x00210730,
        block_0x00210738,
        block_0x00210740,
        block_0x00210744,
        block_0x0021074c,
        block_0x00210754,
        block_0x0021075c,
        block_0x00210768,
        block_0x002107b0,
        block_0x002107c4,
        block_0x002107d4,
        block_0x002107d8,
        block_0x002107e0,
        block_0x002107e8,
        block_0x00210804,
        block_0x0021080c,
        block_0x00210820,
        block_0x00210824,
        block_0x00210828,
        block_0x0021083c,
        block_0x00210840,
        block_0x00210844,
        block_0x0021087c,
        block_0x0021088c,
        block_0x00210890,
        block_0x002108a4,
        block_0x002108b0,
        block_0x002108c4,
        block_0x002108c8,
        block_0x002108cc,
        block_0x00210910,
        block_0x00210940,
        block_0x00210950,
        block_0x00210984,
        block_0x00210988,
        block_0x00210998,
        block_0x002109a0,
        block_0x002109a4,
        block_0x002109ac,
        block_0x002109d4,
        block_0x002109dc,
        block_0x002109e4,
        block_0x00210a28,
        block_0x00210a44,
        block_0x00210a48,
        block_0x00210a8c,
        block_0x00210a90,
        block_0x00210aa0,
        block_0x00210ab8,
        block_0x00210abc,
        block_0x00210ac4,
        block_0x00210ad8,
        block_0x00210adc,
        block_0x00210ae8,
        block_0x00210af4,
        block_0x00210b00,
        block_0x00210b04,
        block_0x00210b20,
        block_0x00210b28,
        block_0x00210b3c,
        block_0x00210b40,
        block_0x00210b44,
        block_0x00210b58,
        block_0x00210b5c,
        block_0x00210b60,
        block_0x00210b9c,
        block_0x00210bb0,
        block_0x00210bbc,
        block_0x00210bd0,
        block_0x00210bd4,
    ];
    const IDX: [u16; 611usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 0u16, 0u16, 0u16, 3u16,
        4u16, 0u16, 0u16, 0u16, 5u16, 0u16, 6u16, 0u16, 7u16, 0u16, 8u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 9u16, 0u16, 0u16, 10u16, 0u16, 11u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 12u16, 13u16, 0u16, 14u16, 0u16, 0u16, 0u16, 0u16, 15u16, 0u16, 0u16,
        16u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 17u16, 18u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 19u16, 0u16, 0u16, 0u16, 20u16, 0u16, 0u16, 21u16, 22u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 23u16, 0u16, 24u16, 0u16, 0u16, 0u16, 0u16, 25u16, 0u16, 26u16, 0u16,
        0u16, 0u16, 0u16, 27u16, 28u16, 29u16, 0u16, 0u16, 30u16, 0u16, 0u16, 31u16,
        0u16, 0u16, 0u16, 0u16, 32u16, 33u16, 0u16, 0u16, 0u16, 34u16, 35u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 36u16, 0u16, 37u16, 0u16, 0u16, 0u16, 0u16, 38u16, 39u16,
        40u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 41u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 42u16, 0u16, 43u16,
        0u16, 0u16, 0u16, 0u16, 44u16, 45u16, 0u16, 0u16, 0u16, 0u16, 46u16, 0u16, 0u16,
        47u16, 0u16, 0u16, 0u16, 0u16, 48u16, 49u16, 50u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 51u16, 0u16, 0u16, 0u16, 52u16, 0u16, 0u16, 53u16,
        0u16, 54u16, 55u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 56u16,
        57u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 58u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 59u16,
        0u16, 60u16, 0u16, 61u16, 0u16, 62u16, 0u16, 63u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        64u16, 0u16, 65u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 66u16, 67u16, 0u16,
        68u16, 0u16, 69u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 70u16, 71u16, 0u16, 0u16,
        0u16, 0u16, 72u16, 73u16, 0u16, 74u16, 0u16, 75u16, 76u16, 0u16, 77u16, 0u16,
        78u16, 0u16, 79u16, 0u16, 0u16, 80u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 81u16, 0u16, 0u16,
        0u16, 0u16, 82u16, 0u16, 0u16, 0u16, 83u16, 84u16, 0u16, 85u16, 0u16, 86u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 87u16, 0u16, 88u16, 0u16, 0u16, 0u16, 0u16,
        89u16, 90u16, 91u16, 0u16, 0u16, 0u16, 0u16, 92u16, 93u16, 94u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 95u16, 0u16,
        0u16, 0u16, 96u16, 97u16, 0u16, 0u16, 0u16, 0u16, 98u16, 0u16, 0u16, 99u16, 0u16,
        0u16, 0u16, 0u16, 100u16, 101u16, 102u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 103u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 104u16, 0u16, 0u16, 0u16,
        105u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        106u16, 107u16, 0u16, 0u16, 0u16, 108u16, 0u16, 109u16, 110u16, 0u16, 111u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 112u16, 0u16, 113u16, 0u16,
        114u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 115u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 116u16,
        117u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 118u16, 119u16, 0u16, 0u16, 0u16, 120u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 121u16, 122u16, 0u16, 123u16, 0u16, 0u16, 0u16, 0u16, 124u16,
        125u16, 0u16, 0u16, 126u16, 0u16, 0u16, 127u16, 0u16, 0u16, 128u16, 129u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 130u16, 0u16, 131u16, 0u16, 0u16, 0u16, 0u16,
        132u16, 133u16, 134u16, 0u16, 0u16, 0u16, 0u16, 135u16, 136u16, 137u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        138u16, 0u16, 0u16, 0u16, 0u16, 139u16, 0u16, 0u16, 140u16, 0u16, 0u16, 0u16,
        0u16, 141u16, 142u16,
    ];
    if pc < 2163276u32 || pc > 2165716u32 {
        return None;
    }
    let word_offset = ((pc - 2163276u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline]
pub fn block_0x0021024c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2163280u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2163284u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2163288u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2163292u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2163296u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2163300u32)?;
    emu.sw_no_count(20usize, 2usize, 40u32, 2163304u32)?;
    emu.sw_no_count(21usize, 2usize, 36u32, 2163308u32)?;
    emu.sw_no_count(22usize, 2usize, 32u32, 2163312u32)?;
    emu.sw_no_count(23usize, 2usize, 28u32, 2163316u32)?;
    emu.sw_no_count(24usize, 2usize, 24u32, 2163320u32)?;
    emu.sw_no_count(25usize, 2usize, 20u32, 2163324u32)?;
    emu.sw_no_count(26usize, 2usize, 16u32, 2163328u32)?;
    emu.sw_no_count(27usize, 2usize, 12u32, 2163332u32)?;
    emu.adi_no_count(8usize, 15usize, 0u32, 2163336u32);
    emu.adi_no_count(9usize, 14usize, 0u32, 2163340u32);
    emu.adi_no_count(19usize, 13usize, 0u32, 2163344u32);
    emu.adi_no_count(20usize, 12usize, 0u32, 2163348u32);
    emu.adi_no_count(18usize, 10usize, 0u32, 2163352u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2163504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210330));
    } else {
        emu.pc = 2163356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021029c));
    }
}
#[inline(always)]
pub fn block_0x0021029c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(22usize, 18usize, 8u32, 2163360u32)?;
    let a = 0u32.wrapping_add(2097152u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2163364u32;
    emu.update_insn_clock();
    emu.anr_no_count(10usize, 22usize, 10usize, 2163368u32);
    let a = 0u32.wrapping_add(1114112u32);
    emu.write_reg_no_count(21usize, a);
    emu.pc = 2163372u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2163380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002102b4));
    } else {
        emu.pc = 2163376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002102b0));
    }
}
#[inline(always)]
pub fn block_0x002102b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(21usize, 0usize, 43u32, 2163380u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2163380u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002102b4));
}
#[inline(always)]
pub fn block_0x002102b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 10usize, 21u32, 2163384u32);
    emu.adr_no_count(24usize, 10usize, 8usize, 2163388u32);
    emu.sli_no_count(10usize, 22usize, 8u32, 2163392u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2163524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210344));
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
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 16u32, 2163400u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a >= b {
        emu.pc = 2163652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002103c4));
    } else {
        emu.pc = 2163404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002102cc));
    }
}
#[inline(always)]
pub fn block_0x002102cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2163408u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2163444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002102f4));
    } else {
        emu.pc = 2163412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002102d4));
    }
}
#[inline(always)]
pub fn block_0x002102d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(11usize, 20usize, 19usize, 2163416u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2163420u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2163420u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002102dc));
}
#[inline(always)]
pub fn block_0x002102dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(13usize, 12usize, 0u32, 2163424u32);
    emu.adi_no_count(12usize, 12usize, 1u32, 2163428u32);
    emu.slti_no_count(13usize, 13usize, 4294967232u32, 2163432u32);
    emu.xri_no_count(13usize, 13usize, 1u32, 2163436u32);
    emu.adr_no_count(10usize, 10usize, 13usize, 2163440u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2163420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002102dc));
    } else {
        emu.pc = 2163444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002102f4));
    }
}
#[inline(always)]
pub fn block_0x002102f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(24usize, 10usize, 24usize, 2163448u32);
    emu.lhu_no_count(27usize, 18usize, 12u32, 2163452u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(27usize);
    let a = emu.read_reg_a_tracked(24usize);
    if a >= b {
        emu.pc = 2163536u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210350));
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
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 22usize, 7u32, 2163460u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2163684u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002103e4));
    } else {
        emu.pc = 2163464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210308));
    }
}
#[inline(always)]
pub fn block_0x00210308(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(24usize, 27usize, 24usize, 2163468u32);
    emu.sli_no_count(10usize, 22usize, 1u32, 2163472u32);
    emu.sri_no_count(10usize, 10usize, 30u32, 2163476u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2163480u32);
    emu.sli_no_count(22usize, 22usize, 11u32, 2163484u32);
    emu.sw_no_count(9usize, 2usize, 8u32, 2163488u32)?;
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2163816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210468));
    } else {
        emu.pc = 2163492u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210324));
    }
}
#[inline(always)]
pub fn block_0x00210324(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2163880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002104a8));
    } else {
        emu.pc = 2163496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210328));
    }
}
#[inline(always)]
pub fn block_0x00210328(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 0usize, 0u32, 2163500u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2163504u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2163884u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002104ac));
}
#[inline(always)]
pub fn block_0x00210330(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(22usize, 18usize, 8u32, 2163508u32)?;
    emu.adi_no_count(24usize, 8usize, 1u32, 2163512u32);
    emu.adi_no_count(21usize, 0usize, 45u32, 2163516u32);
    emu.sli_no_count(10usize, 22usize, 8u32, 2163520u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2163396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002102c4));
    } else {
        emu.pc = 2163524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210344));
    }
}
#[inline(always)]
pub fn block_0x00210344(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 0u32, 2163528u32);
    emu.lhu_no_count(27usize, 18usize, 12u32, 2163532u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(27usize);
    let a = emu.read_reg_a_tracked(24usize);
    if a < b {
        emu.pc = 2163456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210300));
    } else {
        emu.pc = 2163536u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210350));
    }
}
#[inline]
pub fn block_0x00210350(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(22usize, 18usize, 0u32, 2163540u32)?;
    emu.lw_no_count(18usize, 18usize, 4u32, 2163544u32)?;
    emu.adi_no_count(10usize, 22usize, 0u32, 2163548u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2163552u32);
    emu.adi_no_count(12usize, 21usize, 0u32, 2163556u32);
    emu.adi_no_count(13usize, 20usize, 0u32, 2163560u32);
    emu.adi_no_count(14usize, 19usize, 0u32, 2163564u32);
    emu.apc_no_count(1usize, 2163564u32, 0u32, 2163568u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2163572u32;
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
pub fn block_0x00210374(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2163944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002104e8));
    } else {
        emu.pc = 2163576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210378));
    }
}
#[inline]
pub fn block_0x00210378(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(6usize, 18usize, 12u32, 2163580u32)?;
    emu.adi_no_count(10usize, 22usize, 0u32, 2163584u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2163588u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2163592u32);
    emu.lw_no_count(1usize, 2usize, 60u32, 2163596u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2163600u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2163604u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2163608u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2163612u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2163616u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2163620u32)?;
    emu.lw_no_count(22usize, 2usize, 32u32, 2163624u32)?;
    emu.lw_no_count(23usize, 2usize, 28u32, 2163628u32)?;
    emu.lw_no_count(24usize, 2usize, 24u32, 2163632u32)?;
    emu.lw_no_count(25usize, 2usize, 20u32, 2163636u32)?;
    emu.lw_no_count(26usize, 2usize, 16u32, 2163640u32)?;
    emu.lw_no_count(27usize, 2usize, 12u32, 2163644u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2163648u32);
    emu.add_memory_rw_events(19usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2163652u32;
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
pub fn block_0x002103c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 20usize, 0u32, 2163656u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2163660u32);
    emu.apc_no_count(1usize, 2163660u32, 16384u32, 2163664u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2163668u32;
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
pub fn block_0x002103d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(24usize, 10usize, 24usize, 2163672u32);
    emu.lhu_no_count(27usize, 18usize, 12u32, 2163676u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(27usize);
    let a = emu.read_reg_a_tracked(24usize);
    if a >= b {
        emu.pc = 2163536u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210350));
    } else {
        emu.pc = 2163680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002103e0));
    }
}
#[inline(always)]
pub fn block_0x002103e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2163684u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2163456u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210300));
}
#[inline]
pub fn block_0x002103e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(22usize, 18usize, 0u32, 2163688u32)?;
    emu.lw_no_count(23usize, 18usize, 4u32, 2163692u32)?;
    emu.lw_no_count(25usize, 18usize, 8u32, 2163696u32)?;
    emu.lw_no_count(26usize, 18usize, 12u32, 2163700u32)?;
    let a = 0u32.wrapping_add(2682257408u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2163704u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(536870912u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2163708u32;
    emu.update_insn_clock();
    emu.anr_no_count(10usize, 25usize, 10usize, 2163712u32);
    emu.adi_no_count(11usize, 11usize, 48u32, 2163716u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2163720u32);
    emu.sw_no_count(10usize, 18usize, 8u32, 2163724u32)?;
    emu.adi_no_count(10usize, 22usize, 0u32, 2163728u32);
    emu.adi_no_count(11usize, 23usize, 0u32, 2163732u32);
    emu.adi_no_count(12usize, 21usize, 0u32, 2163736u32);
    emu.adi_no_count(13usize, 20usize, 0u32, 2163740u32);
    emu.adi_no_count(14usize, 19usize, 0u32, 2163744u32);
    emu.apc_no_count(1usize, 2163744u32, 0u32, 2163748u32);
    emu.add_memory_rw_events(17usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2163752u32;
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
pub fn block_0x00210428(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 1u32, 2163756u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2163948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002104ec));
    } else {
        emu.pc = 2163760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210430));
    }
}
#[inline(always)]
pub fn block_0x00210430(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 0u32, 2163764u32);
    emu.sbr_no_count(10usize, 27usize, 24usize, 2163768u32);
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(21usize, a);
    emu.pc = 2163772u32;
    emu.update_insn_clock();
    emu.adi_no_count(21usize, 21usize, 4294967295u32, 2163776u32);
    emu.anr_no_count(24usize, 10usize, 21usize, 2163780u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2163780u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210444));
}
#[inline(always)]
pub fn block_0x00210444(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(10usize, 20usize, 21usize, 2163784u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2163840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210480));
    } else {
        emu.pc = 2163788u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021044c));
    }
}
#[inline(always)]
pub fn block_0x0021044c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 23usize, 16u32, 2163792u32)?;
    emu.adi_no_count(20usize, 20usize, 1u32, 2163796u32);
    emu.adi_no_count(11usize, 0usize, 48u32, 2163800u32);
    emu.adi_no_count(10usize, 22usize, 0u32, 2163804u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2163808u32;
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
pub fn block_0x00210460(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2163780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210444));
    } else {
        emu.pc = 2163812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210464));
    }
}
#[inline(always)]
pub fn block_0x00210464(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2163816u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2163948u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002104ec));
}
#[inline(always)]
pub fn block_0x00210468(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 2u32, 2163820u32);
    emu.adi_no_count(25usize, 24usize, 0u32, 2163824u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2163884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002104ac));
    } else {
        emu.pc = 2163828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210474));
    }
}
#[inline(always)]
pub fn block_0x00210474(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 24usize, 16u32, 2163832u32);
    emu.sri_no_count(25usize, 10usize, 17u32, 2163836u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2163840u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2163884u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002104ac));
}
#[inline(always)]
pub fn block_0x00210480(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 23usize, 12u32, 2163844u32)?;
    emu.adi_no_count(10usize, 22usize, 0u32, 2163848u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2163852u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2163856u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2163860u32;
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
pub fn block_0x00210494(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2163948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002104ec));
    } else {
        emu.pc = 2163864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210498));
    }
}
#[inline(always)]
pub fn block_0x00210498(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 0u32, 2163868u32);
    emu.sw_no_count(25usize, 18usize, 8u32, 2163872u32)?;
    emu.sw_no_count(26usize, 18usize, 12u32, 2163876u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2163880u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2163948u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002104ec));
}
#[inline(always)]
pub fn block_0x002104a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 24usize, 0u32, 2163884u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2163884u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002104ac));
}
#[inline(always)]
pub fn block_0x002104ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 0usize, 0u32, 2163888u32);
    emu.sri_no_count(22usize, 22usize, 11u32, 2163892u32);
    emu.lw_no_count(23usize, 18usize, 0u32, 2163896u32)?;
    emu.lw_no_count(18usize, 18usize, 4u32, 2163900u32)?;
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(27usize, a);
    emu.pc = 2163904u32;
    emu.update_insn_clock();
    emu.adi_no_count(27usize, 27usize, 4294967295u32, 2163908u32);
    emu.anr_no_count(9usize, 25usize, 27usize, 2163912u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2163912u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002104c8));
}
#[inline(always)]
pub fn block_0x002104c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(10usize, 26usize, 27usize, 2163916u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2164012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021052c));
    } else {
        emu.pc = 2163920u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002104d0));
    }
}
#[inline(always)]
pub fn block_0x002104d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 16u32, 2163924u32)?;
    emu.adi_no_count(26usize, 26usize, 1u32, 2163928u32);
    emu.adi_no_count(10usize, 23usize, 0u32, 2163932u32);
    emu.adi_no_count(11usize, 22usize, 0u32, 2163936u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2163940u32;
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
pub fn block_0x002104e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2163912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002104c8));
    } else {
        emu.pc = 2163944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002104e8));
    }
}
#[inline(always)]
pub fn block_0x002104e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 1u32, 2163948u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2163948u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002104ec));
}
#[inline]
pub fn block_0x002104ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 19usize, 0u32, 2163952u32);
    emu.lw_no_count(1usize, 2usize, 60u32, 2163956u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2163960u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2163964u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2163968u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2163972u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2163976u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2163980u32)?;
    emu.lw_no_count(22usize, 2usize, 32u32, 2163984u32)?;
    emu.lw_no_count(23usize, 2usize, 28u32, 2163988u32)?;
    emu.lw_no_count(24usize, 2usize, 24u32, 2163992u32)?;
    emu.lw_no_count(25usize, 2usize, 20u32, 2163996u32)?;
    emu.lw_no_count(26usize, 2usize, 16u32, 2164000u32)?;
    emu.lw_no_count(27usize, 2usize, 12u32, 2164004u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2164008u32);
    emu.add_memory_rw_events(16usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2164012u32;
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
pub fn block_0x0021052c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 23usize, 0u32, 2164016u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2164020u32);
    emu.adi_no_count(12usize, 21usize, 0u32, 2164024u32);
    emu.adi_no_count(13usize, 20usize, 0u32, 2164028u32);
    emu.adi_no_count(14usize, 19usize, 0u32, 2164032u32);
    emu.apc_no_count(1usize, 2164032u32, 0u32, 2164036u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2164040u32;
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
pub fn block_0x00210548(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 1u32, 2164044u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2163948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002104ec));
    } else {
        emu.pc = 2164048u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210550));
    }
}
#[inline(always)]
pub fn block_0x00210550(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 18usize, 12u32, 2164052u32)?;
    emu.adi_no_count(10usize, 23usize, 0u32, 2164056u32);
    emu.lw_no_count(11usize, 2usize, 8u32, 2164060u32)?;
    emu.adi_no_count(12usize, 8usize, 0u32, 2164064u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2164068u32;
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
pub fn block_0x00210564(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2163948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002104ec));
    } else {
        emu.pc = 2164072u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210568));
    }
}
#[inline(always)]
pub fn block_0x00210568(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 0usize, 0u32, 2164076u32);
    emu.sbr_no_count(10usize, 24usize, 25usize, 2164080u32);
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(9usize, a);
    emu.pc = 2164084u32;
    emu.update_insn_clock();
    emu.adi_no_count(9usize, 9usize, 4294967295u32, 2164088u32);
    emu.anr_no_count(20usize, 10usize, 9usize, 2164092u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2164092u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021057c));
}
#[inline(always)]
pub fn block_0x0021057c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(10usize, 8usize, 9usize, 2164096u32);
    emu.sltru_no_count(19usize, 10usize, 20usize, 2164100u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2163948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002104ec));
    } else {
        emu.pc = 2164104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210588));
    }
}
#[inline(always)]
pub fn block_0x00210588(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 16u32, 2164108u32)?;
    emu.adi_no_count(8usize, 8usize, 1u32, 2164112u32);
    emu.adi_no_count(10usize, 23usize, 0u32, 2164116u32);
    emu.adi_no_count(11usize, 22usize, 0u32, 2164120u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2164124u32;
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
pub fn block_0x0021059c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2164092u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021057c));
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
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2164132u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2163948u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002104ec));
}
#[inline]
pub fn block_0x002105a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2164136u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2164140u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2164144u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2164148u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2164152u32)?;
    emu.sw_no_count(19usize, 2usize, 12u32, 2164156u32)?;
    emu.adi_no_count(8usize, 14usize, 0u32, 2164160u32);
    emu.adi_no_count(9usize, 13usize, 0u32, 2164164u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2164168u32);
    let a = 0u32.wrapping_add(1114112u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2164172u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2164212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002105f4));
    } else {
        emu.pc = 2164176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002105d0));
    }
}
#[inline(always)]
pub fn block_0x002105d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 18usize, 16u32, 2164180u32)?;
    emu.adi_no_count(19usize, 10usize, 0u32, 2164184u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2164188u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2164192u32;
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
pub fn block_0x002105e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 10usize, 0u32, 2164196u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2164200u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2164212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002105f4));
    } else {
        emu.pc = 2164204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002105ec));
    }
}
#[inline(always)]
pub fn block_0x002105ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2164208u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2164212u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2164260u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210624));
}
#[inline(always)]
pub fn block_0x002105f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2164256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210620));
    } else {
        emu.pc = 2164216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002105f8));
    }
}
#[inline]
pub fn block_0x002105f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(6usize, 18usize, 12u32, 2164220u32)?;
    emu.adi_no_count(11usize, 9usize, 0u32, 2164224u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2164228u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2164232u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2164236u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2164240u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2164244u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2164248u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2164252u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2164256u32;
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
pub fn block_0x00210620(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2164260u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2164260u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210624));
}
#[inline(always)]
pub fn block_0x00210624(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2164264u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2164268u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2164272u32)?;
    emu.lw_no_count(18usize, 2usize, 16u32, 2164276u32)?;
    emu.lw_no_count(19usize, 2usize, 12u32, 2164280u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2164284u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2164288u32;
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
pub fn block_0x00210640(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2164292u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2164296u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2164300u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2164304u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2164308u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2164312u32)?;
    emu.sw_no_count(20usize, 2usize, 24u32, 2164316u32)?;
    emu.sw_no_count(21usize, 2usize, 20u32, 2164320u32)?;
    emu.sw_no_count(22usize, 2usize, 16u32, 2164324u32)?;
    emu.sw_no_count(23usize, 2usize, 12u32, 2164328u32)?;
    emu.sw_no_count(24usize, 2usize, 8u32, 2164332u32)?;
    emu.sw_no_count(25usize, 2usize, 4u32, 2164336u32)?;
    emu.adi_no_count(9usize, 12usize, 0u32, 2164340u32);
    emu.lw_no_count(18usize, 10usize, 8u32, 2164344u32)?;
    let a = 0u32.wrapping_add(402653184u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2164348u32;
    emu.update_insn_clock();
    emu.anr_no_count(12usize, 18usize, 12usize, 2164352u32);
    emu.adi_no_count(8usize, 11usize, 0u32, 2164356u32);
    emu.add_memory_rw_events(17usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2164584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210768));
    } else {
        emu.pc = 2164360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210688));
    }
}
#[inline(always)]
pub fn block_0x00210688(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(11usize, 18usize, 3u32, 2164364u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2164464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002106f0));
    } else {
        emu.pc = 2164368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210690));
    }
}
#[inline(always)]
pub fn block_0x00210690(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 16u32, 2164372u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a >= b {
        emu.pc = 2164656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002107b0));
    } else {
        emu.pc = 2164376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210698));
    }
}
#[inline(always)]
pub fn block_0x00210698(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2164380u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2164416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002106c0));
    } else {
        emu.pc = 2164384u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002106a0));
    }
}
#[inline(always)]
pub fn block_0x002106a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(12usize, 8usize, 9usize, 2164388u32);
    emu.adi_no_count(13usize, 8usize, 0u32, 2164392u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2164392u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002106a8));
}
#[inline(always)]
pub fn block_0x002106a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(14usize, 13usize, 0u32, 2164396u32);
    emu.adi_no_count(13usize, 13usize, 1u32, 2164400u32);
    emu.slti_no_count(14usize, 14usize, 4294967232u32, 2164404u32);
    emu.xri_no_count(14usize, 14usize, 1u32, 2164408u32);
    emu.adr_no_count(11usize, 11usize, 14usize, 2164412u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2164392u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002106a8));
    } else {
        emu.pc = 2164416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002106c0));
    }
}
#[inline(always)]
pub fn block_0x002106c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lhu_no_count(12usize, 10usize, 12u32, 2164420u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2164584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210768));
    } else {
        emu.pc = 2164424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002106c8));
    }
}
#[inline(always)]
pub fn block_0x002106c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(21usize, 0usize, 0u32, 2164428u32);
    emu.sbr_no_count(20usize, 12usize, 11usize, 2164432u32);
    emu.sli_no_count(11usize, 18usize, 1u32, 2164436u32);
    emu.sri_no_count(11usize, 11usize, 30u32, 2164440u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2164444u32);
    emu.sli_no_count(18usize, 18usize, 11u32, 2164448u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2164696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002107d8));
    } else {
        emu.pc = 2164452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002106e4));
    }
}
#[inline(always)]
pub fn block_0x002106e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2164712u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002107e8));
    } else {
        emu.pc = 2164456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002106e8));
    }
}
#[inline(always)]
pub fn block_0x002106e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(21usize, 20usize, 0u32, 2164460u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2164464u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2164712u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002107e8));
}
#[inline(always)]
pub fn block_0x002106f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lhu_no_count(11usize, 10usize, 14u32, 2164468u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2164860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021087c));
    } else {
        emu.pc = 2164472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002106f8));
    }
}
#[inline(always)]
pub fn block_0x002106f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(13usize, 8usize, 9usize, 2164476u32);
    emu.adi_no_count(14usize, 0usize, 224u32, 2164480u32);
    emu.adi_no_count(15usize, 0usize, 240u32, 2164484u32);
    emu.adi_no_count(16usize, 8usize, 0u32, 2164488u32);
    emu.adi_no_count(12usize, 11usize, 0u32, 2164492u32);
    emu.adi_no_count(9usize, 0usize, 0u32, 2164496u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2164500u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2164524u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021072c));
}
#[inline(always)]
pub fn block_0x00210714(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 16usize, 1u32, 2164504u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2164504u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210718));
}
#[inline(always)]
pub fn block_0x00210718(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(16usize, 16usize, 9usize, 2164508u32);
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2164512u32);
    emu.sbr_no_count(9usize, 17usize, 16usize, 2164516u32);
    emu.adi_no_count(16usize, 17usize, 0u32, 2164520u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2164572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021075c));
    } else {
        emu.pc = 2164524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021072c));
    }
}
#[inline(always)]
pub fn block_0x0021072c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2164572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021075c));
    } else {
        emu.pc = 2164528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210730));
    }
}
#[inline(always)]
pub fn block_0x00210730(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(17usize, 16usize, 0u32, 2164532u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(17usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2164500u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210714));
    } else {
        emu.pc = 2164536u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210738));
    }
}
#[inline(always)]
pub fn block_0x00210738(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(17usize, 17usize, 255u32, 2164540u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a < b {
        emu.pc = 2164556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021074c));
    } else {
        emu.pc = 2164544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210740));
    }
}
#[inline(always)]
pub fn block_0x00210740(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a < b {
        emu.pc = 2164564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210754));
    } else {
        emu.pc = 2164548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210744));
    }
}
#[inline(always)]
pub fn block_0x00210744(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 16usize, 4u32, 2164552u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2164556u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2164504u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210718));
}
#[inline(always)]
pub fn block_0x0021074c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 16usize, 2u32, 2164560u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2164564u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2164504u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210718));
}
#[inline(always)]
pub fn block_0x00210754(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 16usize, 3u32, 2164568u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2164572u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2164504u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210718));
}
#[inline(always)]
pub fn block_0x0021075c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 11usize, 12usize, 2164576u32);
    emu.lhu_no_count(12usize, 10usize, 12u32, 2164580u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2164424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002106c8));
    } else {
        emu.pc = 2164584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210768));
    }
}
#[inline]
pub fn block_0x00210768(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 4u32, 2164588u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2164592u32)?;
    emu.lw_no_count(6usize, 11usize, 12u32, 2164596u32)?;
    emu.adi_no_count(11usize, 8usize, 0u32, 2164600u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2164604u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2164608u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2164612u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2164616u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2164620u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2164624u32)?;
    emu.lw_no_count(20usize, 2usize, 24u32, 2164628u32)?;
    emu.lw_no_count(21usize, 2usize, 20u32, 2164632u32)?;
    emu.lw_no_count(22usize, 2usize, 16u32, 2164636u32)?;
    emu.lw_no_count(23usize, 2usize, 12u32, 2164640u32)?;
    emu.lw_no_count(24usize, 2usize, 8u32, 2164644u32)?;
    emu.lw_no_count(25usize, 2usize, 4u32, 2164648u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2164652u32);
    emu.add_memory_rw_events(18usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2164656u32;
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
pub fn block_0x002107b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 10usize, 0u32, 2164660u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2164664u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2164668u32);
    emu.apc_no_count(1usize, 2164668u32, 12288u32, 2164672u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2164676u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1220u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002107c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 10usize, 0u32, 2164680u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2164684u32);
    emu.lhu_no_count(12usize, 19usize, 12u32, 2164688u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2164584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210768));
    } else {
        emu.pc = 2164692u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002107d4));
    }
}
#[inline(always)]
pub fn block_0x002107d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2164696u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2164424u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002106c8));
}
#[inline(always)]
pub fn block_0x002107d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 2u32, 2164700u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2164712u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002107e8));
    } else {
        emu.pc = 2164704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002107e0));
    }
}
#[inline(always)]
pub fn block_0x002107e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(11usize, 20usize, 16u32, 2164708u32);
    emu.sri_no_count(21usize, 11usize, 17u32, 2164712u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2164712u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002107e8));
}
#[inline(always)]
pub fn block_0x002107e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(23usize, 0usize, 0u32, 2164716u32);
    emu.sri_no_count(18usize, 18usize, 11u32, 2164720u32);
    emu.lw_no_count(19usize, 10usize, 0u32, 2164724u32)?;
    emu.lw_no_count(22usize, 10usize, 4u32, 2164728u32)?;
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(24usize, a);
    emu.pc = 2164732u32;
    emu.update_insn_clock();
    emu.adi_no_count(24usize, 24usize, 4294967295u32, 2164736u32);
    emu.anr_no_count(25usize, 21usize, 24usize, 2164740u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2164740u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210804));
}
#[inline(always)]
pub fn block_0x00210804(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(10usize, 23usize, 24usize, 2164744u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(25usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2164776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210828));
    } else {
        emu.pc = 2164748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021080c));
    }
}
#[inline(always)]
pub fn block_0x0021080c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 22usize, 16u32, 2164752u32)?;
    emu.adi_no_count(23usize, 23usize, 1u32, 2164756u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2164760u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2164764u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2164768u32;
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
pub fn block_0x00210820(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2164740u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210804));
    } else {
        emu.pc = 2164772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210824));
    }
}
#[inline(always)]
pub fn block_0x00210824(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2164776u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2164800u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210840));
}
#[inline(always)]
pub fn block_0x00210828(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 22usize, 12u32, 2164780u32)?;
    emu.adi_no_count(10usize, 19usize, 0u32, 2164784u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2164788u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2164792u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2164796u32;
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
pub fn block_0x0021083c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2164880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210890));
    } else {
        emu.pc = 2164800u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210840));
    }
}
#[inline(always)]
pub fn block_0x00210840(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 0usize, 1u32, 2164804u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2164804u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210844));
}
#[inline]
pub fn block_0x00210844(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 0u32, 2164808u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2164812u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2164816u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2164820u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2164824u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2164828u32)?;
    emu.lw_no_count(20usize, 2usize, 24u32, 2164832u32)?;
    emu.lw_no_count(21usize, 2usize, 20u32, 2164836u32)?;
    emu.lw_no_count(22usize, 2usize, 16u32, 2164840u32)?;
    emu.lw_no_count(23usize, 2usize, 12u32, 2164844u32)?;
    emu.lw_no_count(24usize, 2usize, 8u32, 2164848u32)?;
    emu.lw_no_count(25usize, 2usize, 4u32, 2164852u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2164856u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2164860u32;
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
pub fn block_0x0021087c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2164864u32);
    emu.sbr_no_count(11usize, 11usize, 0usize, 2164868u32);
    emu.lhu_no_count(12usize, 10usize, 12u32, 2164872u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2164424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002106c8));
    } else {
        emu.pc = 2164876u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021088c));
    }
}
#[inline(always)]
pub fn block_0x0021088c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2164880u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2164584u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210768));
}
#[inline(always)]
pub fn block_0x00210890(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2164884u32);
    emu.sbr_no_count(10usize, 20usize, 21usize, 2164888u32);
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(20usize, a);
    emu.pc = 2164892u32;
    emu.update_insn_clock();
    emu.adi_no_count(20usize, 20usize, 4294967295u32, 2164896u32);
    emu.anr_no_count(21usize, 10usize, 20usize, 2164900u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2164900u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002108a4));
}
#[inline(always)]
pub fn block_0x002108a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(10usize, 9usize, 20usize, 2164904u32);
    emu.sltru_no_count(8usize, 10usize, 21usize, 2164908u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2164804u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210844));
    } else {
        emu.pc = 2164912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002108b0));
    }
}
#[inline(always)]
pub fn block_0x002108b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 22usize, 16u32, 2164916u32)?;
    emu.adi_no_count(9usize, 9usize, 1u32, 2164920u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2164924u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2164928u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2164932u32;
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
pub fn block_0x002108c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2164900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002108a4));
    } else {
        emu.pc = 2164936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002108c8));
    }
}
#[inline(always)]
pub fn block_0x002108c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2164940u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2164804u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210844));
}
#[inline]
pub fn block_0x002108cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2164944u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2164948u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2164952u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2164956u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2164960u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2164964u32)?;
    emu.sw_no_count(20usize, 2usize, 40u32, 2164968u32)?;
    emu.sw_no_count(21usize, 2usize, 36u32, 2164972u32)?;
    emu.sw_no_count(22usize, 2usize, 32u32, 2164976u32)?;
    emu.sw_no_count(23usize, 2usize, 28u32, 2164980u32)?;
    emu.sw_no_count(24usize, 2usize, 24u32, 2164984u32)?;
    emu.sw_no_count(25usize, 2usize, 20u32, 2164988u32)?;
    emu.sw_no_count(26usize, 2usize, 16u32, 2164992u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2164996u32);
    emu.lhu_no_count(18usize, 10usize, 12u32, 2165000u32)?;
    emu.adi_no_count(12usize, 11usize, 0u32, 2165004u32);
    emu.add_memory_rw_events(16usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2165220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002109e4));
    } else {
        emu.pc = 2165008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210910));
    }
}
#[inline]
pub fn block_0x00210910(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(21usize, 8usize, 8u32, 2165012u32)?;
    emu.lw_no_count(10usize, 12usize, 0u32, 2165016u32)?;
    emu.lw_no_count(11usize, 12usize, 4u32, 2165020u32)?;
    emu.lw_no_count(13usize, 12usize, 8u32, 2165024u32)?;
    emu.lw_no_count(12usize, 12usize, 12u32, 2165028u32)?;
    emu.lw_no_count(22usize, 8usize, 12u32, 2165032u32)?;
    emu.sli_no_count(14usize, 21usize, 7u32, 2165036u32);
    emu.sw_no_count(10usize, 2usize, 0u32, 2165040u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2165044u32)?;
    emu.sw_no_count(13usize, 2usize, 8u32, 2165048u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2165052u32)?;
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2165288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210a28));
    } else {
        emu.pc = 2165056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210940));
    }
}
#[inline(always)]
pub fn block_0x00210940(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 4u32, 2165060u32)?;
    emu.adi_no_count(11usize, 21usize, 0u32, 2165064u32);
    emu.lw_no_count(13usize, 2usize, 12u32, 2165068u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2165388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210a8c));
    } else {
        emu.pc = 2165072u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210950));
    }
}
#[inline]
pub fn block_0x00210950(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 0u32, 2165076u32);
    emu.lw_no_count(5usize, 2usize, 8u32, 2165080u32)?;
    emu.adi_no_count(14usize, 0usize, 1u32, 2165084u32);
    let a = 0u32.wrapping_add(393216u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2165088u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(524288u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2165092u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(917504u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2165096u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(516096u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2165100u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 4294967286u32, 2165104u32);
    emu.adi_no_count(16usize, 16usize, 4294967196u32, 2165108u32);
    emu.adi_no_count(17usize, 17usize, 4294966296u32, 2165112u32);
    emu.adi_no_count(5usize, 5usize, 4u32, 2165116u32);
    emu.adi_no_count(6usize, 6usize, 4294965488u32, 2165120u32);
    emu.add_memory_rw_events(13usize);
    let return_addr = 2165124u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2165144u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210998));
}
#[inline(always)]
pub fn block_0x00210984(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(7usize, 5usize, 0u32, 2165128u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2165128u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210988));
}
#[inline(always)]
pub fn block_0x00210988(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(12usize, 7usize, 12usize, 2165132u32);
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2165136u32);
    emu.adi_no_count(5usize, 5usize, 12u32, 2165140u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2165392u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210a90));
    } else {
        emu.pc = 2165144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210998));
    }
}
#[inline(always)]
pub fn block_0x00210998(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lhu_no_count(7usize, 5usize, 4294967292u32, 2165148u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a == b {
        emu.pc = 2165124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210984));
    } else {
        emu.pc = 2165152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002109a0));
    }
}
#[inline(always)]
pub fn block_0x002109a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a != b {
        emu.pc = 2165204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002109d4));
    } else {
        emu.pc = 2165156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002109a4));
    }
}
#[inline(always)]
pub fn block_0x002109a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lhu_no_count(7usize, 5usize, 4294967294u32, 2165160u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a == b {
        emu.pc = 2165212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002109dc));
    } else {
        emu.pc = 2165164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002109ac));
    }
}
#[inline]
pub fn block_0x002109ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(28usize, 7usize, 15usize, 2165168u32);
    emu.adr_no_count(29usize, 7usize, 16usize, 2165172u32);
    emu.anr_no_count(28usize, 28usize, 29usize, 2165176u32);
    emu.adr_no_count(29usize, 7usize, 17usize, 2165180u32);
    emu.adr_no_count(7usize, 7usize, 6usize, 2165184u32);
    emu.anr_no_count(7usize, 29usize, 7usize, 2165188u32);
    emu.xrr_no_count(7usize, 28usize, 7usize, 2165192u32);
    emu.sri_no_count(7usize, 7usize, 17u32, 2165196u32);
    emu.adi_no_count(7usize, 7usize, 1u32, 2165200u32);
    emu.add_memory_rw_events(10usize);
    let return_addr = 2165204u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2165128u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210988));
}
#[inline(always)]
pub fn block_0x002109d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(7usize, 5usize, 4u32, 2165208u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2165212u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2165128u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210988));
}
#[inline(always)]
pub fn block_0x002109dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(7usize, 0usize, 1u32, 2165216u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2165220u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2165128u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210988));
}
#[inline]
pub fn block_0x002109e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 0u32, 2165224u32)?;
    emu.lw_no_count(11usize, 8usize, 4u32, 2165228u32)?;
    emu.lw_no_count(1usize, 2usize, 60u32, 2165232u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2165236u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2165240u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2165244u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2165248u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2165252u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2165256u32)?;
    emu.lw_no_count(22usize, 2usize, 32u32, 2165260u32)?;
    emu.lw_no_count(23usize, 2usize, 28u32, 2165264u32)?;
    emu.lw_no_count(24usize, 2usize, 24u32, 2165268u32)?;
    emu.lw_no_count(25usize, 2usize, 20u32, 2165272u32)?;
    emu.lw_no_count(26usize, 2usize, 16u32, 2165276u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2165280u32);
    emu.apc_no_count(6usize, 2165280u32, 0u32, 2165284u32);
    emu.add_memory_rw_events(17usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2165288u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(440u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00210a28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 8usize, 4u32, 2165292u32)?;
    emu.lw_no_count(10usize, 8usize, 0u32, 2165296u32)?;
    emu.lw_no_count(11usize, 2usize, 0u32, 2165300u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2165304u32)?;
    emu.lw_no_count(13usize, 12usize, 12u32, 2165308u32)?;
    emu.adi_no_count(12usize, 9usize, 0u32, 2165312u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2165316u32;
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
pub fn block_0x00210a44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2165596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210b5c));
    } else {
        emu.pc = 2165320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210a48));
    }
}
#[inline]
pub fn block_0x00210a48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2682257408u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2165324u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(536870912u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2165328u32;
    emu.update_insn_clock();
    emu.anr_no_count(11usize, 21usize, 11usize, 2165332u32);
    emu.adi_no_count(12usize, 12usize, 48u32, 2165336u32);
    emu.orr_no_count(11usize, 11usize, 12usize, 2165340u32);
    emu.sli_no_count(9usize, 9usize, 16u32, 2165344u32);
    emu.sri_no_count(9usize, 9usize, 16u32, 2165348u32);
    emu.sbr_no_count(12usize, 18usize, 9usize, 2165352u32);
    emu.sltru_no_count(13usize, 18usize, 12usize, 2165356u32);
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2165360u32);
    emu.anr_no_count(18usize, 13usize, 12usize, 2165364u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2165368u32);
    emu.sw_no_count(12usize, 2usize, 0u32, 2165372u32)?;
    emu.sw_no_count(0usize, 2usize, 4u32, 2165376u32)?;
    emu.sw_no_count(11usize, 8usize, 8u32, 2165380u32)?;
    emu.lw_no_count(13usize, 2usize, 12u32, 2165384u32)?;
    emu.add_memory_rw_events(16usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2165072u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210950));
    } else {
        emu.pc = 2165388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210a8c));
    }
}
#[inline(always)]
pub fn block_0x00210a8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 0u32, 2165392u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2165392u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210a90));
}
#[inline(always)]
pub fn block_0x00210a90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 12usize, 10usize, 2165396u32);
    emu.sli_no_count(12usize, 18usize, 16u32, 2165400u32);
    emu.sri_no_count(12usize, 12usize, 16u32, 2165404u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2165444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210ac4));
    } else {
        emu.pc = 2165408u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210aa0));
    }
}
#[inline(always)]
pub fn block_0x00210aa0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(20usize, 18usize, 10usize, 2165412u32);
    emu.sli_no_count(10usize, 11usize, 1u32, 2165416u32);
    emu.sri_no_count(10usize, 10usize, 30u32, 2165420u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2165424u32);
    emu.sli_no_count(11usize, 11usize, 11u32, 2165428u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2165480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210ae8));
    } else {
        emu.pc = 2165432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210ab8));
    }
}
#[inline(always)]
pub fn block_0x00210ab8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2165504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210b00));
    } else {
        emu.pc = 2165436u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210abc));
    }
}
#[inline(always)]
pub fn block_0x00210abc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(23usize, 0usize, 0u32, 2165440u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2165444u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2165508u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210b04));
}
#[inline(always)]
pub fn block_0x00210ac4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 0u32, 2165448u32)?;
    emu.lw_no_count(11usize, 8usize, 4u32, 2165452u32)?;
    emu.adi_no_count(12usize, 2usize, 0u32, 2165456u32);
    emu.apc_no_count(1usize, 2165456u32, 0u32, 2165460u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2165464u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(264u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00210ad8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 10usize, 0u32, 2165468u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2165468u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210adc));
}
#[inline(always)]
pub fn block_0x00210adc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(21usize, 8usize, 8u32, 2165472u32)?;
    emu.sw_no_count(22usize, 8usize, 12u32, 2165476u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2165480u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2165600u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210b60));
}
#[inline(always)]
pub fn block_0x00210ae8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 2u32, 2165484u32);
    emu.adi_no_count(23usize, 20usize, 0u32, 2165488u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2165508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210b04));
    } else {
        emu.pc = 2165492u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210af4));
    }
}
#[inline(always)]
pub fn block_0x00210af4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 20usize, 16u32, 2165496u32);
    emu.sri_no_count(23usize, 10usize, 17u32, 2165500u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2165504u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2165508u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210b04));
}
#[inline(always)]
pub fn block_0x00210b00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(23usize, 20usize, 0u32, 2165508u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2165508u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210b04));
}
#[inline(always)]
pub fn block_0x00210b04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(24usize, 0usize, 0u32, 2165512u32);
    emu.sri_no_count(9usize, 11usize, 11u32, 2165516u32);
    emu.lw_no_count(18usize, 8usize, 0u32, 2165520u32)?;
    emu.lw_no_count(19usize, 8usize, 4u32, 2165524u32)?;
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(25usize, a);
    emu.pc = 2165528u32;
    emu.update_insn_clock();
    emu.adi_no_count(25usize, 25usize, 4294967295u32, 2165532u32);
    emu.anr_no_count(26usize, 23usize, 25usize, 2165536u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2165536u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210b20));
}
#[inline(always)]
pub fn block_0x00210b20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(10usize, 24usize, 25usize, 2165540u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(26usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2165572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210b44));
    } else {
        emu.pc = 2165544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210b28));
    }
}
#[inline(always)]
pub fn block_0x00210b28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 19usize, 16u32, 2165548u32)?;
    emu.adi_no_count(24usize, 24usize, 1u32, 2165552u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2165556u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2165560u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2165564u32;
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
pub fn block_0x00210b3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2165536u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210b20));
    } else {
        emu.pc = 2165568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210b40));
    }
}
#[inline(always)]
pub fn block_0x00210b40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2165572u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2165596u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210b5c));
}
#[inline(always)]
pub fn block_0x00210b44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 2usize, 0u32, 2165576u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2165580u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2165584u32);
    emu.apc_no_count(1usize, 2165584u32, 0u32, 2165588u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2165592u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(136u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00210b58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2165660u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210b9c));
    } else {
        emu.pc = 2165596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210b5c));
    }
}
#[inline(always)]
pub fn block_0x00210b5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 1u32, 2165600u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2165600u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210b60));
}
#[inline]
pub fn block_0x00210b60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 20usize, 0u32, 2165604u32);
    emu.lw_no_count(1usize, 2usize, 60u32, 2165608u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2165612u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2165616u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2165620u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2165624u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2165628u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2165632u32)?;
    emu.lw_no_count(22usize, 2usize, 32u32, 2165636u32)?;
    emu.lw_no_count(23usize, 2usize, 28u32, 2165640u32)?;
    emu.lw_no_count(24usize, 2usize, 24u32, 2165644u32)?;
    emu.lw_no_count(25usize, 2usize, 20u32, 2165648u32)?;
    emu.lw_no_count(26usize, 2usize, 16u32, 2165652u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2165656u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2165660u32;
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
pub fn block_0x00210b9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(24usize, 0usize, 0u32, 2165664u32);
    emu.sbr_no_count(10usize, 20usize, 23usize, 2165668u32);
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(23usize, a);
    emu.pc = 2165672u32;
    emu.update_insn_clock();
    emu.adi_no_count(23usize, 23usize, 4294967295u32, 2165676u32);
    emu.anr_no_count(25usize, 10usize, 23usize, 2165680u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2165680u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210bb0));
}
#[inline(always)]
pub fn block_0x00210bb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(10usize, 24usize, 23usize, 2165684u32);
    emu.sltru_no_count(20usize, 10usize, 25usize, 2165688u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(25usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2165468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210adc));
    } else {
        emu.pc = 2165692u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210bbc));
    }
}
#[inline(always)]
pub fn block_0x00210bbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 19usize, 16u32, 2165696u32)?;
    emu.adi_no_count(24usize, 24usize, 1u32, 2165700u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2165704u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2165708u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2165712u32;
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
pub fn block_0x00210bd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2165680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210bb0));
    } else {
        emu.pc = 2165716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210bd4));
    }
}
#[inline(always)]
pub fn block_0x00210bd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2165720u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2165468u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210adc));
}
