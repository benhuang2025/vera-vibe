pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2165720u32;
pub const PC_MAX: u32 = 2168352u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 151usize] = [
        block_0x00210bd8,
        block_0x00210c24,
        block_0x00210c34,
        block_0x00210c38,
        block_0x00210c40,
        block_0x00210c48,
        block_0x00210cb4,
        block_0x00210cc8,
        block_0x00210ccc,
        block_0x00210cd0,
        block_0x00210ce0,
        block_0x00210ce4,
        block_0x00210cf8,
        block_0x00210cfc,
        block_0x00210d04,
        block_0x00210d3c,
        block_0x00210d44,
        block_0x00210d48,
        block_0x00210d58,
        block_0x00210d5c,
        block_0x00210d64,
        block_0x00210d68,
        block_0x00210d6c,
        block_0x00210d70,
        block_0x00210d80,
        block_0x00210d88,
        block_0x00210d8c,
        block_0x00210d94,
        block_0x00210d98,
        block_0x00210da0,
        block_0x00210da4,
        block_0x00210db0,
        block_0x00210ddc,
        block_0x00210dec,
        block_0x00210df0,
        block_0x00210e08,
        block_0x00210e0c,
        block_0x00210e14,
        block_0x00210e18,
        block_0x00210e54,
        block_0x00210e70,
        block_0x00210e80,
        block_0x00210eb4,
        block_0x00210ed4,
        block_0x00210f34,
        block_0x00210f5c,
        block_0x00210f78,
        block_0x00210f88,
        block_0x00210f90,
        block_0x00210fa0,
        block_0x00210fbc,
        block_0x00210fd4,
        block_0x00210fd8,
        block_0x0021100c,
        block_0x00211048,
        block_0x00211074,
        block_0x002110c0,
        block_0x002110c8,
        block_0x002110f8,
        block_0x00211104,
        block_0x0021111c,
        block_0x00211120,
        block_0x00211130,
        block_0x00211134,
        block_0x00211138,
        block_0x0021114c,
        block_0x00211150,
        block_0x0021119c,
        block_0x002111a0,
        block_0x002111bc,
        block_0x002111c0,
        block_0x002111c4,
        block_0x002111d0,
        block_0x002111f0,
        block_0x002111f4,
        block_0x00211210,
        block_0x00211218,
        block_0x00211228,
        block_0x00211234,
        block_0x00211248,
        block_0x0021125c,
        block_0x002112b4,
        block_0x002112bc,
        block_0x002112c4,
        block_0x00211304,
        block_0x00211308,
        block_0x00211318,
        block_0x00211324,
        block_0x00211334,
        block_0x00211338,
        block_0x0021133c,
        block_0x00211348,
        block_0x0021134c,
        block_0x0021135c,
        block_0x00211374,
        block_0x00211390,
        block_0x002113b4,
        block_0x002113c0,
        block_0x002113cc,
        block_0x002113e0,
        block_0x002113f4,
        block_0x002113fc,
        block_0x00211404,
        block_0x00211408,
        block_0x00211414,
        block_0x00211418,
        block_0x0021141c,
        block_0x00211420,
        block_0x00211424,
        block_0x00211438,
        block_0x0021143c,
        block_0x00211448,
        block_0x00211468,
        block_0x0021146c,
        block_0x00211478,
        block_0x00211488,
        block_0x0021148c,
        block_0x00211490,
        block_0x002114a8,
        block_0x002114ac,
        block_0x002114b4,
        block_0x002114c0,
        block_0x002114cc,
        block_0x002114d4,
        block_0x002114e0,
        block_0x002114ec,
        block_0x002114f4,
        block_0x002114fc,
        block_0x00211504,
        block_0x00211514,
        block_0x00211528,
        block_0x0021152c,
        block_0x00211530,
        block_0x00211540,
        block_0x00211544,
        block_0x0021154c,
        block_0x00211550,
        block_0x00211554,
        block_0x00211558,
        block_0x00211568,
        block_0x00211570,
        block_0x00211578,
        block_0x0021157c,
        block_0x00211594,
        block_0x00211598,
        block_0x002115a4,
        block_0x002115a8,
        block_0x002115e8,
        block_0x002115f0,
        block_0x00211604,
        block_0x00211620,
    ];
    const IDX: [u16; 659usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 0u16, 0u16, 3u16, 4u16, 0u16,
        5u16, 0u16, 6u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 7u16, 0u16, 0u16, 0u16, 0u16, 8u16, 9u16, 10u16, 0u16, 0u16,
        0u16, 11u16, 12u16, 0u16, 0u16, 0u16, 0u16, 13u16, 14u16, 0u16, 15u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 16u16,
        0u16, 17u16, 18u16, 0u16, 0u16, 0u16, 19u16, 20u16, 0u16, 21u16, 22u16, 23u16,
        24u16, 0u16, 0u16, 0u16, 25u16, 0u16, 26u16, 27u16, 0u16, 28u16, 29u16, 0u16,
        30u16, 31u16, 0u16, 0u16, 32u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 33u16, 0u16, 0u16, 0u16, 34u16, 35u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        36u16, 37u16, 0u16, 38u16, 39u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 40u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        41u16, 0u16, 0u16, 0u16, 42u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 43u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 44u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 45u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 46u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 47u16, 0u16, 0u16, 0u16, 48u16, 0u16, 49u16, 0u16, 0u16, 0u16, 50u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 51u16, 0u16, 0u16, 0u16, 0u16, 0u16, 52u16, 53u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 54u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 55u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 56u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 57u16, 0u16, 58u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 59u16, 0u16, 0u16, 60u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 61u16, 62u16, 0u16, 0u16, 0u16, 63u16, 64u16, 65u16, 0u16, 0u16,
        0u16, 0u16, 66u16, 67u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 68u16, 69u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 70u16, 71u16, 72u16, 0u16, 0u16, 73u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 74u16, 75u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 76u16,
        0u16, 77u16, 0u16, 0u16, 0u16, 78u16, 0u16, 0u16, 79u16, 0u16, 0u16, 0u16, 0u16,
        80u16, 0u16, 0u16, 0u16, 0u16, 81u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 82u16, 0u16, 83u16, 0u16, 84u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 85u16, 86u16, 0u16, 0u16, 0u16,
        87u16, 0u16, 0u16, 88u16, 0u16, 0u16, 0u16, 89u16, 90u16, 91u16, 0u16, 0u16,
        92u16, 93u16, 0u16, 0u16, 0u16, 94u16, 0u16, 0u16, 0u16, 0u16, 0u16, 95u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 96u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 97u16, 0u16, 0u16, 98u16, 0u16, 0u16, 99u16, 0u16, 0u16, 0u16, 0u16,
        100u16, 0u16, 0u16, 0u16, 0u16, 101u16, 0u16, 102u16, 0u16, 103u16, 104u16, 0u16,
        0u16, 105u16, 106u16, 107u16, 108u16, 109u16, 0u16, 0u16, 0u16, 0u16, 110u16,
        111u16, 0u16, 0u16, 112u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 113u16,
        114u16, 0u16, 0u16, 115u16, 0u16, 0u16, 0u16, 116u16, 117u16, 118u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 119u16, 120u16, 0u16, 121u16, 0u16, 0u16, 122u16, 0u16, 0u16,
        123u16, 0u16, 124u16, 0u16, 0u16, 125u16, 0u16, 0u16, 126u16, 0u16, 127u16, 0u16,
        128u16, 0u16, 129u16, 0u16, 0u16, 0u16, 130u16, 0u16, 0u16, 0u16, 0u16, 131u16,
        132u16, 133u16, 0u16, 0u16, 0u16, 134u16, 135u16, 0u16, 136u16, 137u16, 138u16,
        139u16, 0u16, 0u16, 0u16, 140u16, 0u16, 141u16, 0u16, 142u16, 143u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 144u16, 145u16, 0u16, 0u16, 146u16, 147u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 148u16,
        0u16, 149u16, 0u16, 0u16, 0u16, 0u16, 150u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        151u16,
    ];
    if pc < 2165720u32 || pc > 2168352u32 {
        return None;
    }
    let word_offset = ((pc - 2165720u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline]
pub fn block_0x00210bd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967216u32, 2165724u32);
    emu.sw_no_count(1usize, 2usize, 76u32, 2165728u32)?;
    emu.sw_no_count(8usize, 2usize, 72u32, 2165732u32)?;
    emu.sw_no_count(9usize, 2usize, 68u32, 2165736u32)?;
    emu.sw_no_count(18usize, 2usize, 64u32, 2165740u32)?;
    emu.sw_no_count(19usize, 2usize, 60u32, 2165744u32)?;
    emu.sw_no_count(20usize, 2usize, 56u32, 2165748u32)?;
    emu.sw_no_count(21usize, 2usize, 52u32, 2165752u32)?;
    emu.sw_no_count(22usize, 2usize, 48u32, 2165756u32)?;
    emu.sw_no_count(23usize, 2usize, 44u32, 2165760u32)?;
    emu.sw_no_count(24usize, 2usize, 40u32, 2165764u32)?;
    emu.sw_no_count(25usize, 2usize, 36u32, 2165768u32)?;
    emu.sw_no_count(26usize, 2usize, 32u32, 2165772u32)?;
    emu.sw_no_count(27usize, 2usize, 28u32, 2165776u32)?;
    emu.adi_no_count(18usize, 12usize, 0u32, 2165780u32);
    emu.adi_no_count(8usize, 11usize, 0u32, 2165784u32);
    emu.lw_no_count(12usize, 12usize, 4u32, 2165788u32)?;
    emu.adi_no_count(9usize, 10usize, 0u32, 2165792u32);
    emu.add_memory_rw_events(18usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2165824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210c40));
    } else {
        emu.pc = 2165796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210c24));
    }
}
#[inline(always)]
pub fn block_0x00210c24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 18usize, 0u32, 2165800u32)?;
    emu.lw_no_count(13usize, 8usize, 12u32, 2165804u32)?;
    emu.adi_no_count(10usize, 9usize, 0u32, 2165808u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2165812u32;
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
pub fn block_0x00210c34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2165824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210c40));
    } else {
        emu.pc = 2165816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210c38));
    }
}
#[inline(always)]
pub fn block_0x00210c38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2165820u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2165824u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2166296u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210e18));
}
#[inline(always)]
pub fn block_0x00210c40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 12u32, 2165828u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2166296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210e18));
    } else {
        emu.pc = 2165832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210c48));
    }
}
#[inline(never)]
pub fn block_0x00210c48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(20usize, 18usize, 8u32, 2165836u32)?;
    emu.sli_no_count(11usize, 10usize, 2u32, 2165840u32);
    emu.sli_no_count(10usize, 10usize, 4u32, 2165844u32);
    emu.adi_no_count(22usize, 0usize, 65u32, 2165848u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(18usize, a);
    emu.pc = 2165852u32;
    emu.update_insn_clock();
    emu.adi_no_count(18usize, 18usize, 480u32, 2165856u32);
    emu.adi_no_count(23usize, 0usize, 64u32, 2165860u32);
    emu.adi_no_count(24usize, 0usize, 1u32, 2165864u32);
    emu.adi_no_count(25usize, 0usize, 2u32, 2165868u32);
    let a = 0u32.wrapping_add(3435986944u32);
    emu.write_reg_no_count(27usize, a);
    emu.pc = 2165872u32;
    emu.update_insn_clock();
    emu.adi_no_count(21usize, 0usize, 10u32, 2165876u32);
    let a = 0u32.wrapping_add(393216u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2165880u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(524288u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2165884u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(917504u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2165888u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(516096u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2165892u32;
    emu.update_insn_clock();
    emu.sbr_no_count(10usize, 10usize, 11usize, 2165896u32);
    emu.adi_no_count(12usize, 12usize, 4294967286u32, 2165900u32);
    emu.sw_no_count(12usize, 2usize, 16u32, 2165904u32)?;
    emu.adi_no_count(11usize, 13usize, 4294967196u32, 2165908u32);
    emu.sw_no_count(11usize, 2usize, 12u32, 2165912u32)?;
    emu.adi_no_count(11usize, 14usize, 4294966296u32, 2165916u32);
    emu.sw_no_count(11usize, 2usize, 8u32, 2165920u32)?;
    emu.adr_no_count(26usize, 20usize, 10usize, 2165924u32);
    emu.adi_no_count(10usize, 20usize, 12u32, 2165928u32);
    emu.adi_no_count(11usize, 15usize, 4294965488u32, 2165932u32);
    emu.sw_no_count(11usize, 2usize, 4u32, 2165936u32)?;
    emu.add_memory_rw_events(27usize);
    let return_addr = 2165940u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2165968u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210cd0));
}
#[inline(always)]
pub fn block_0x00210cb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 12usize, 4u32, 2165944u32)?;
    emu.lw_no_count(12usize, 12usize, 8u32, 2165948u32)?;
    emu.lw_no_count(13usize, 8usize, 12u32, 2165952u32)?;
    emu.adi_no_count(10usize, 9usize, 0u32, 2165956u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2165960u32;
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
pub fn block_0x00210cc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2166256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210df0));
    } else {
        emu.pc = 2165964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210ccc));
    }
}
#[inline(always)]
pub fn block_0x00210ccc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2165968u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2165816u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210c38));
}
#[inline(always)]
pub fn block_0x00210cd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 20usize, 0u32, 2165972u32);
    emu.lhu_no_count(11usize, 20usize, 0u32, 2165976u32)?;
    emu.adi_no_count(20usize, 10usize, 0u32, 2165980u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2166076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210d3c));
    } else {
        emu.pc = 2165984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210ce0));
    }
}
#[inline(always)]
pub fn block_0x00210ce0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2165940u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210cb4));
    } else {
        emu.pc = 2165988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210ce4));
    }
}
#[inline(always)]
pub fn block_0x00210ce4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lhu_no_count(10usize, 12usize, 2u32, 2165992u32)?;
    emu.sw_no_count(0usize, 2usize, 20u32, 2165996u32)?;
    emu.sb_no_count(0usize, 2usize, 24u32, 2166000u32);
    emu.lhu_no_count(11usize, 12usize, 0u32, 2166004u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(25usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2166156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210d8c));
    } else {
        emu.pc = 2166008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210cf8));
    }
}
#[inline(always)]
pub fn block_0x00210cf8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2166164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210d94));
    } else {
        emu.pc = 2166012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210cfc));
    }
}
#[inline(always)]
pub fn block_0x00210cfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lhu_no_count(11usize, 12usize, 2u32, 2166016u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2166284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210e0c));
    } else {
        emu.pc = 2166020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210d04));
    }
}
#[inline]
pub fn block_0x00210d04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 2usize, 16u32, 2166024u32)?;
    emu.adr_no_count(12usize, 11usize, 12usize, 2166028u32);
    emu.lw_no_count(13usize, 2usize, 12u32, 2166032u32)?;
    emu.adr_no_count(13usize, 11usize, 13usize, 2166036u32);
    emu.anr_no_count(12usize, 12usize, 13usize, 2166040u32);
    emu.lw_no_count(13usize, 2usize, 8u32, 2166044u32)?;
    emu.adr_no_count(13usize, 11usize, 13usize, 2166048u32);
    emu.lw_no_count(14usize, 2usize, 4u32, 2166052u32)?;
    emu.adr_no_count(11usize, 11usize, 14usize, 2166056u32);
    emu.anr_no_count(11usize, 13usize, 11usize, 2166060u32);
    emu.xrr_no_count(11usize, 12usize, 11usize, 2166064u32);
    emu.sri_no_count(11usize, 11usize, 17u32, 2166068u32);
    emu.adi_no_count(12usize, 11usize, 1u32, 2166072u32);
    emu.add_memory_rw_events(14usize);
    let return_addr = 2166076u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2166168u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210d98));
}
#[inline(always)]
pub fn block_0x00210d3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(19usize, 12usize, 4u32, 2166080u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a < b {
        emu.pc = 2166120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210d68));
    } else {
        emu.pc = 2166084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210d44));
    }
}
#[inline(always)]
pub fn block_0x00210d44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(22usize, 8usize, 12u32, 2166088u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2166088u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210d48));
}
#[inline(always)]
pub fn block_0x00210d48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 64u32, 2166092u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2166096u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2166100u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(22usize);
    let return_addr = 2166104u32;
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
pub fn block_0x00210d58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2165816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210c38));
    } else {
        emu.pc = 2166108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210d5c));
    }
}
#[inline(always)]
pub fn block_0x00210d5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 19usize, 4294967232u32, 2166112u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a < b {
        emu.pc = 2166088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210d48));
    } else {
        emu.pc = 2166116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210d64));
    }
}
#[inline(always)]
pub fn block_0x00210d64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2166120u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2166128u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210d70));
}
#[inline(always)]
pub fn block_0x00210d68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2166256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210df0));
    } else {
        emu.pc = 2166124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210d6c));
    }
}
#[inline(always)]
pub fn block_0x00210d6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(22usize, 8usize, 12u32, 2166128u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2166128u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210d70));
}
#[inline(always)]
pub fn block_0x00210d70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2166132u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2166136u32);
    emu.adi_no_count(12usize, 19usize, 0u32, 2166140u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(22usize);
    let return_addr = 2166144u32;
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
pub fn block_0x00210d80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(22usize, 0usize, 65u32, 2166148u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2166256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210df0));
    } else {
        emu.pc = 2166152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210d88));
    }
}
#[inline(always)]
pub fn block_0x00210d88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2166156u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2165816u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210c38));
}
#[inline(always)]
pub fn block_0x00210d8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 12usize, 8u32, 2166160u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2166164u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2166168u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210d98));
}
#[inline(always)]
pub fn block_0x00210d94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 12usize, 4u32, 2166168u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2166168u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210d98));
}
#[inline(always)]
pub fn block_0x00210d98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 6u32, 2166172u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a >= b {
        emu.pc = 2166356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210e54));
    } else {
        emu.pc = 2166176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210da0));
    }
}
#[inline(always)]
pub fn block_0x00210da0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2166236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210ddc));
    } else {
        emu.pc = 2166180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210da4));
    }
}
#[inline(always)]
pub fn block_0x00210da4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 0usize, 12usize, 2166184u32);
    emu.adi_no_count(13usize, 2usize, 19u32, 2166188u32);
    emu.adr_no_count(13usize, 13usize, 12usize, 2166192u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2166192u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210db0));
}
#[inline]
pub fn block_0x00210db0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(14usize, 10usize, 16u32, 2166196u32);
    emu.adi_no_count(11usize, 11usize, 1u32, 2166200u32);
    emu.mulhu_no_count(14usize, 14usize, 27usize, 2166204u32);
    emu.sri_no_count(14usize, 14usize, 19u32, 2166208u32);
    emu.mul_no_count(15usize, 14usize, 21usize, 2166212u32);
    emu.sbr_no_count(10usize, 10usize, 15usize, 2166216u32);
    emu.ori_no_count(10usize, 10usize, 48u32, 2166220u32);
    emu.sb_no_count(10usize, 13usize, 0u32, 2166224u32);
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2166228u32);
    emu.adi_no_count(10usize, 14usize, 0u32, 2166232u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2166192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210db0));
    } else {
        emu.pc = 2166236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210ddc));
    }
}
#[inline(always)]
pub fn block_0x00210ddc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 8usize, 12u32, 2166240u32)?;
    emu.adi_no_count(11usize, 2usize, 20u32, 2166244u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2166248u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2166252u32;
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
pub fn block_0x00210dec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2165816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210c38));
    } else {
        emu.pc = 2166256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210df0));
    }
}
#[inline(always)]
pub fn block_0x00210df0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xrr_no_count(10usize, 20usize, 26usize, 2166260u32);
    emu.sltiu_no_count(10usize, 10usize, 1u32, 2166264u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2166268u32);
    emu.ani_no_count(10usize, 10usize, 12u32, 2166272u32);
    emu.adr_no_count(10usize, 20usize, 10usize, 2166276u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(26usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a != b {
        emu.pc = 2165968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210cd0));
    } else {
        emu.pc = 2166280u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210e08));
    }
}
#[inline(always)]
pub fn block_0x00210e08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2166284u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2166292u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210e14));
}
#[inline(always)]
pub fn block_0x00210e0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 1u32, 2166288u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2166292u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2166180u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210da4));
}
#[inline(always)]
pub fn block_0x00210e14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2166296u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2166296u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210e18));
}
#[inline]
pub fn block_0x00210e18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 76u32, 2166300u32)?;
    emu.lw_no_count(8usize, 2usize, 72u32, 2166304u32)?;
    emu.lw_no_count(9usize, 2usize, 68u32, 2166308u32)?;
    emu.lw_no_count(18usize, 2usize, 64u32, 2166312u32)?;
    emu.lw_no_count(19usize, 2usize, 60u32, 2166316u32)?;
    emu.lw_no_count(20usize, 2usize, 56u32, 2166320u32)?;
    emu.lw_no_count(21usize, 2usize, 52u32, 2166324u32)?;
    emu.lw_no_count(22usize, 2usize, 48u32, 2166328u32)?;
    emu.lw_no_count(23usize, 2usize, 44u32, 2166332u32)?;
    emu.lw_no_count(24usize, 2usize, 40u32, 2166336u32)?;
    emu.lw_no_count(25usize, 2usize, 36u32, 2166340u32)?;
    emu.lw_no_count(26usize, 2usize, 32u32, 2166344u32)?;
    emu.lw_no_count(27usize, 2usize, 28u32, 2166348u32)?;
    emu.adi_no_count(2usize, 2usize, 80u32, 2166352u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2166356u32;
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
pub fn block_0x00210e54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2166360u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 544u32, 2166364u32);
    emu.adi_no_count(11usize, 0usize, 5u32, 2166368u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2166372u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2166376u32);
    emu.apc_no_count(1usize, 2166376u32, 12288u32, 2166380u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2166384u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966792u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00210e70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 10usize, 4u32, 2166388u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2166392u32)?;
    emu.lw_no_count(6usize, 13usize, 12u32, 2166396u32)?;
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2166400u32;
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
pub fn block_0x00210e80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2166404u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2166408u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2166412u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2166416u32)?;
    emu.adi_no_count(8usize, 11usize, 0u32, 2166420u32);
    emu.lw_no_count(14usize, 11usize, 4u32, 2166424u32)?;
    emu.lw_no_count(11usize, 11usize, 0u32, 2166428u32)?;
    emu.lw_no_count(14usize, 14usize, 12u32, 2166432u32)?;
    emu.adi_no_count(9usize, 10usize, 0u32, 2166436u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2166440u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2166444u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2166448u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2166452u32;
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
pub fn block_0x00210eb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(8usize, 9usize, 0u32, 2166456u32)?;
    emu.sb_no_count(10usize, 9usize, 4u32, 2166460u32);
    emu.sb_no_count(0usize, 9usize, 5u32, 2166464u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2166468u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2166472u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2166476u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2166480u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2166484u32;
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
pub fn block_0x00210ed4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2166488u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2166492u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2166496u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2166500u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2166504u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2166508u32)?;
    emu.sw_no_count(20usize, 2usize, 24u32, 2166512u32)?;
    emu.sw_no_count(21usize, 2usize, 20u32, 2166516u32)?;
    emu.sw_no_count(22usize, 2usize, 16u32, 2166520u32)?;
    emu.sw_no_count(23usize, 2usize, 12u32, 2166524u32)?;
    emu.sw_no_count(24usize, 2usize, 8u32, 2166528u32)?;
    emu.adi_no_count(8usize, 17usize, 0u32, 2166532u32);
    emu.adi_no_count(9usize, 16usize, 0u32, 2166536u32);
    emu.adi_no_count(18usize, 15usize, 0u32, 2166540u32);
    emu.adi_no_count(19usize, 14usize, 0u32, 2166544u32);
    emu.adi_no_count(20usize, 13usize, 0u32, 2166548u32);
    emu.adi_no_count(21usize, 10usize, 0u32, 2166552u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2166556u32)?;
    emu.lw_no_count(13usize, 21usize, 4u32, 2166560u32)?;
    emu.lw_no_count(22usize, 2usize, 48u32, 2166564u32)?;
    emu.lw_no_count(23usize, 2usize, 52u32, 2166568u32)?;
    emu.lw_no_count(24usize, 2usize, 56u32, 2166572u32)?;
    emu.lw_no_count(13usize, 13usize, 12u32, 2166576u32)?;
    emu.add_memory_rw_events(24usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2166580u32;
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
pub fn block_0x00210f34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(21usize, 2usize, 0u32, 2166584u32)?;
    emu.sb_no_count(10usize, 2usize, 4u32, 2166588u32);
    emu.sb_no_count(0usize, 2usize, 5u32, 2166592u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2166596u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2166600u32);
    emu.adi_no_count(12usize, 19usize, 0u32, 2166604u32);
    emu.adi_no_count(13usize, 18usize, 0u32, 2166608u32);
    emu.adi_no_count(14usize, 9usize, 0u32, 2166612u32);
    emu.apc_no_count(1usize, 2166612u32, 4294950912u32, 2166616u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2166620u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(276u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00210f5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2166624u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2166628u32);
    emu.adi_no_count(12usize, 22usize, 0u32, 2166632u32);
    emu.adi_no_count(13usize, 23usize, 0u32, 2166636u32);
    emu.adi_no_count(14usize, 24usize, 0u32, 2166640u32);
    emu.apc_no_count(1usize, 2166640u32, 4294950912u32, 2166644u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2166648u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(248u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00210f78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 2usize, 4u32, 2166652u32);
    emu.lbu_no_count(12usize, 2usize, 5u32, 2166656u32);
    emu.orr_no_count(10usize, 12usize, 11usize, 2166660u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2166744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210fd8));
    } else {
        emu.pc = 2166664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210f88));
    }
}
#[inline(always)]
pub fn block_0x00210f88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 11usize, 1u32, 2166668u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2166744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210fd8));
    } else {
        emu.pc = 2166672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210f90));
    }
}
#[inline(always)]
pub fn block_0x00210f90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 0u32, 2166676u32)?;
    emu.lbu_no_count(11usize, 10usize, 10u32, 2166680u32);
    emu.ani_no_count(11usize, 11usize, 128u32, 2166684u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2166716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210fbc));
    } else {
        emu.pc = 2166688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210fa0));
    }
}
#[inline(always)]
pub fn block_0x00210fa0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 4u32, 2166692u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2166696u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2166700u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2166704u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1057u32, 2166708u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2166712u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2166716u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2166740u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210fd4));
}
#[inline(always)]
pub fn block_0x00210fbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 4u32, 2166720u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2166724u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2166728u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2166732u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1056u32, 2166736u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2166740u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2166740u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210fd4));
}
#[inline(always)]
pub fn block_0x00210fd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2166744u32;
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
pub fn block_0x00210fd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 1u32, 2166748u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2166752u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2166756u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2166760u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2166764u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2166768u32)?;
    emu.lw_no_count(20usize, 2usize, 24u32, 2166772u32)?;
    emu.lw_no_count(21usize, 2usize, 20u32, 2166776u32)?;
    emu.lw_no_count(22usize, 2usize, 16u32, 2166780u32)?;
    emu.lw_no_count(23usize, 2usize, 12u32, 2166784u32)?;
    emu.lw_no_count(24usize, 2usize, 8u32, 2166788u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2166792u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2166796u32;
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
pub fn block_0x0021100c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2166800u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2166804u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2166808u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2166812u32)?;
    emu.sw_no_count(18usize, 2usize, 0u32, 2166816u32)?;
    emu.adi_no_count(8usize, 13usize, 0u32, 2166820u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2166824u32);
    emu.lw_no_count(13usize, 11usize, 4u32, 2166828u32)?;
    emu.lw_no_count(11usize, 11usize, 0u32, 2166832u32)?;
    emu.lw_no_count(13usize, 13usize, 12u32, 2166836u32)?;
    emu.adi_no_count(18usize, 10usize, 0u32, 2166840u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2166844u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2166848u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2166852u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2166856u32;
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
pub fn block_0x00211048(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(11usize, 8usize, 1u32, 2166860u32);
    emu.sw_no_count(0usize, 18usize, 0u32, 2166864u32)?;
    emu.sw_no_count(9usize, 18usize, 4u32, 2166868u32)?;
    emu.sb_no_count(10usize, 18usize, 8u32, 2166872u32);
    emu.sb_no_count(11usize, 18usize, 9u32, 2166876u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2166880u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2166884u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2166888u32)?;
    emu.lw_no_count(18usize, 2usize, 0u32, 2166892u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2166896u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2166900u32;
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
pub fn block_0x00211074(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967216u32, 2166904u32);
    emu.sw_no_count(1usize, 2usize, 76u32, 2166908u32)?;
    emu.sw_no_count(8usize, 2usize, 72u32, 2166912u32)?;
    emu.sw_no_count(9usize, 2usize, 68u32, 2166916u32)?;
    emu.sw_no_count(18usize, 2usize, 64u32, 2166920u32)?;
    emu.sw_no_count(19usize, 2usize, 60u32, 2166924u32)?;
    emu.sw_no_count(20usize, 2usize, 56u32, 2166928u32)?;
    emu.sw_no_count(21usize, 2usize, 52u32, 2166932u32)?;
    emu.sw_no_count(22usize, 2usize, 48u32, 2166936u32)?;
    emu.sw_no_count(23usize, 2usize, 44u32, 2166940u32)?;
    emu.adi_no_count(20usize, 14usize, 0u32, 2166944u32);
    emu.adi_no_count(19usize, 13usize, 0u32, 2166948u32);
    emu.adi_no_count(9usize, 12usize, 0u32, 2166952u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2166956u32);
    emu.lw_no_count(22usize, 10usize, 4u32, 2166960u32)?;
    emu.lw_no_count(21usize, 10usize, 0u32, 2166964u32)?;
    emu.lw_no_count(23usize, 22usize, 12u32, 2166968u32)?;
    emu.adi_no_count(10usize, 21usize, 0u32, 2166972u32);
    emu.add_memory_rw_events(19usize);
    let base = emu.read_reg_b_tracked(23usize);
    let return_addr = 2166976u32;
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
pub fn block_0x002110c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 1u32, 2166980u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2167032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002110f8));
    } else {
        emu.pc = 2166984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002110c8));
    }
}
#[inline]
pub fn block_0x002110c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2166988u32);
    emu.lw_no_count(1usize, 2usize, 76u32, 2166992u32)?;
    emu.lw_no_count(8usize, 2usize, 72u32, 2166996u32)?;
    emu.lw_no_count(9usize, 2usize, 68u32, 2167000u32)?;
    emu.lw_no_count(18usize, 2usize, 64u32, 2167004u32)?;
    emu.lw_no_count(19usize, 2usize, 60u32, 2167008u32)?;
    emu.lw_no_count(20usize, 2usize, 56u32, 2167012u32)?;
    emu.lw_no_count(21usize, 2usize, 52u32, 2167016u32)?;
    emu.lw_no_count(22usize, 2usize, 48u32, 2167020u32)?;
    emu.lw_no_count(23usize, 2usize, 44u32, 2167024u32)?;
    emu.adi_no_count(2usize, 2usize, 80u32, 2167028u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2167032u32;
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
pub fn block_0x002110f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 8usize, 10u32, 2167036u32);
    emu.ani_no_count(10usize, 10usize, 128u32, 2167040u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2167096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211138));
    } else {
        emu.pc = 2167044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211104));
    }
}
#[inline(always)]
pub fn block_0x00211104(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2167048u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1059u32, 2167052u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2167056u32);
    emu.adi_no_count(18usize, 0usize, 1u32, 2167060u32);
    emu.adi_no_count(10usize, 21usize, 0u32, 2167064u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(23usize);
    let return_addr = 2167068u32;
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
pub fn block_0x0021111c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2166984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002110c8));
    } else {
        emu.pc = 2167072u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211120));
    }
}
#[inline(always)]
pub fn block_0x00211120(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 20usize, 12u32, 2167076u32)?;
    emu.adi_no_count(10usize, 19usize, 0u32, 2167080u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2167084u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2167088u32;
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
pub fn block_0x00211130(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2166984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002110c8));
    } else {
        emu.pc = 2167092u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211134));
    }
}
#[inline(always)]
pub fn block_0x00211134(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2167096u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2167232u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002111c0));
}
#[inline(always)]
pub fn block_0x00211138(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2167100u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1060u32, 2167104u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2167108u32);
    emu.adi_no_count(10usize, 21usize, 0u32, 2167112u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(23usize);
    let return_addr = 2167116u32;
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
pub fn block_0x0021114c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2166984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002110c8));
    } else {
        emu.pc = 2167120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211150));
    }
}
#[inline]
pub fn block_0x00211150(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 1u32, 2167124u32);
    emu.adi_no_count(10usize, 2usize, 27u32, 2167128u32);
    emu.lw_no_count(11usize, 8usize, 8u32, 2167132u32)?;
    emu.lw_no_count(12usize, 8usize, 12u32, 2167136u32)?;
    emu.adi_no_count(13usize, 2usize, 12u32, 2167140u32);
    emu.sw_no_count(21usize, 2usize, 12u32, 2167144u32)?;
    emu.sw_no_count(22usize, 2usize, 16u32, 2167148u32)?;
    emu.sw_no_count(10usize, 2usize, 20u32, 2167152u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2167156u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1020u32, 2167160u32);
    emu.lw_no_count(14usize, 20usize, 12u32, 2167164u32)?;
    emu.sb_no_count(18usize, 2usize, 27u32, 2167168u32);
    emu.sw_no_count(13usize, 2usize, 28u32, 2167172u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2167176u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2167180u32)?;
    emu.sw_no_count(12usize, 2usize, 40u32, 2167184u32)?;
    emu.adi_no_count(11usize, 2usize, 28u32, 2167188u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2167192u32);
    emu.add_memory_rw_events(19usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2167196u32;
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
pub fn block_0x0021119c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2166984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002110c8));
    } else {
        emu.pc = 2167200u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002111a0));
    }
}
#[inline(always)]
pub fn block_0x002111a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 32u32, 2167204u32)?;
    emu.lw_no_count(10usize, 2usize, 28u32, 2167208u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2167212u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2167216u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1054u32, 2167220u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2167224u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2167228u32;
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
pub fn block_0x002111bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2166984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002110c8));
    } else {
        emu.pc = 2167232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002111c0));
    }
}
#[inline(always)]
pub fn block_0x002111c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a != b {
        emu.pc = 2167284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002111f4));
    } else {
        emu.pc = 2167236u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002111c4));
    }
}
#[inline(always)]
pub fn block_0x002111c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 8usize, 10u32, 2167240u32);
    emu.ani_no_count(10usize, 10usize, 128u32, 2167244u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2167284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002111f4));
    } else {
        emu.pc = 2167248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002111d0));
    }
}
#[inline(always)]
pub fn block_0x002111d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 8usize, 4u32, 2167252u32)?;
    emu.lw_no_count(10usize, 8usize, 0u32, 2167256u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2167260u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2167264u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1063u32, 2167268u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2167272u32);
    emu.adi_no_count(18usize, 0usize, 1u32, 2167276u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2167280u32;
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
pub fn block_0x002111f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2166984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002110c8));
    } else {
        emu.pc = 2167284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002111f4));
    }
}
#[inline(always)]
pub fn block_0x002111f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 8usize, 4u32, 2167288u32)?;
    emu.lw_no_count(10usize, 8usize, 0u32, 2167292u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2167296u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2167300u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1062u32, 2167304u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2167308u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2167312u32;
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
pub fn block_0x00211210(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2167316u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2167320u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2166984u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002110c8));
}
#[inline(always)]
pub fn block_0x00211218(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 4u32, 2167324u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2167328u32)?;
    emu.lw_no_count(6usize, 12usize, 16u32, 2167332u32)?;
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2167336u32;
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
pub fn block_0x00211228(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(12usize, 10usize, 0u32, 2167340u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2167344u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2167368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211248));
    } else {
        emu.pc = 2167348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211234));
    }
}
#[inline(always)]
pub fn block_0x00211234(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2167352u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967272u32, 2167356u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2167360u32);
    emu.apc_no_count(6usize, 2167360u32, 4294963200u32, 2167364u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2167368u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x00211248(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2167372u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 560u32, 2167376u32);
    emu.adi_no_count(12usize, 0usize, 5u32, 2167380u32);
    emu.apc_no_count(6usize, 2167380u32, 4294963200u32, 2167384u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2167388u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1004u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0021125c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 22u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967200u32, 2167392u32);
    emu.sw_no_count(1usize, 2usize, 92u32, 2167396u32)?;
    emu.sw_no_count(8usize, 2usize, 88u32, 2167400u32)?;
    emu.sw_no_count(9usize, 2usize, 84u32, 2167404u32)?;
    emu.sw_no_count(18usize, 2usize, 80u32, 2167408u32)?;
    emu.sw_no_count(19usize, 2usize, 76u32, 2167412u32)?;
    emu.sw_no_count(20usize, 2usize, 72u32, 2167416u32)?;
    emu.sw_no_count(21usize, 2usize, 68u32, 2167420u32)?;
    emu.sw_no_count(22usize, 2usize, 64u32, 2167424u32)?;
    emu.sw_no_count(23usize, 2usize, 60u32, 2167428u32)?;
    emu.sw_no_count(24usize, 2usize, 56u32, 2167432u32)?;
    emu.sw_no_count(25usize, 2usize, 52u32, 2167436u32)?;
    emu.sw_no_count(26usize, 2usize, 48u32, 2167440u32)?;
    emu.sw_no_count(27usize, 2usize, 44u32, 2167444u32)?;
    emu.adi_no_count(19usize, 11usize, 0u32, 2167448u32);
    emu.lw_no_count(21usize, 12usize, 4u32, 2167452u32)?;
    emu.lw_no_count(8usize, 12usize, 0u32, 2167456u32)?;
    emu.lw_no_count(9usize, 21usize, 16u32, 2167460u32)?;
    emu.adi_no_count(23usize, 10usize, 0u32, 2167464u32);
    emu.adi_no_count(11usize, 0usize, 34u32, 2167468u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2167472u32);
    emu.add_memory_rw_events(22usize);
    let base = emu.read_reg_b_tracked(9usize);
    let return_addr = 2167476u32;
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
pub fn block_0x002112b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 1u32, 2167480u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2168232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002115a8));
    } else {
        emu.pc = 2167484u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002112bc));
    }
}
#[inline(always)]
pub fn block_0x002112bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(8usize, 2usize, 24u32, 2167488u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2168132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211544));
    } else {
        emu.pc = 2167492u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002112c4));
    }
}
#[inline]
pub fn block_0x002112c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(21usize, 2usize, 16u32, 2167496u32)?;
    emu.sw_no_count(9usize, 2usize, 12u32, 2167500u32)?;
    emu.adi_no_count(20usize, 0usize, 0u32, 2167504u32);
    emu.adi_no_count(9usize, 0usize, 0u32, 2167508u32);
    emu.sbr_no_count(10usize, 0usize, 19usize, 2167512u32);
    emu.sw_no_count(10usize, 2usize, 8u32, 2167516u32)?;
    emu.adi_no_count(27usize, 0usize, 4294967201u32, 2167520u32);
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2167524u32;
    emu.update_insn_clock();
    emu.adi_no_count(24usize, 0usize, 1u32, 2167528u32);
    emu.adi_no_count(26usize, 0usize, 34u32, 2167532u32);
    emu.adi_no_count(10usize, 10usize, 1u32, 2167536u32);
    emu.sw_no_count(10usize, 2usize, 20u32, 2167540u32)?;
    emu.adi_no_count(21usize, 0usize, 92u32, 2167544u32);
    emu.adi_no_count(18usize, 23usize, 0u32, 2167548u32);
    emu.adi_no_count(13usize, 19usize, 0u32, 2167552u32);
    emu.add_memory_rw_events(16usize);
    let return_addr = 2167556u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2167576u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211318));
}
#[inline(always)]
pub fn block_0x00211304(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2167560u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2167560u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211308));
}
#[inline(always)]
pub fn block_0x00211308(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 10usize, 9usize, 2167564u32);
    emu.sbr_no_count(13usize, 8usize, 18usize, 2167568u32);
    emu.adr_no_count(9usize, 10usize, 25usize, 2167572u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2168304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002115f0));
    } else {
        emu.pc = 2167576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211318));
    }
}
#[inline(always)]
pub fn block_0x00211318(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 0usize, 0u32, 2167580u32);
    emu.adr_no_count(8usize, 18usize, 13usize, 2167584u32);
    emu.sbr_no_count(11usize, 0usize, 13usize, 2167588u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2167588u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211324));
}
#[inline(always)]
pub fn block_0x00211324(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 18usize, 25usize, 2167592u32);
    emu.lbu_no_count(12usize, 10usize, 0u32, 2167596u32);
    emu.adi_no_count(14usize, 12usize, 4294967169u32, 2167600u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(27usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2167628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021134c));
    } else {
        emu.pc = 2167604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211334));
    }
}
#[inline(always)]
pub fn block_0x00211334(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(26usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2167628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021134c));
    } else {
        emu.pc = 2167608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211338));
    }
}
#[inline(always)]
pub fn block_0x00211338(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2167628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021134c));
    } else {
        emu.pc = 2167612u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021133c));
    }
}
#[inline(always)]
pub fn block_0x0021133c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 25usize, 1u32, 2167616u32);
    emu.adr_no_count(10usize, 11usize, 25usize, 2167620u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2167588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211324));
    } else {
        emu.pc = 2167624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211348));
    }
}
#[inline(always)]
pub fn block_0x00211348(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2167628u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2168084u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211514));
}
#[inline(always)]
pub fn block_0x0021134c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(11usize, 10usize, 0u32, 2167632u32);
    emu.adi_no_count(18usize, 10usize, 1u32, 2167636u32);
    emu.ani_no_count(22usize, 11usize, 255u32, 2167640u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2167756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002113cc));
    } else {
        emu.pc = 2167644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021135c));
    }
}
#[inline(always)]
pub fn block_0x0021135c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(12usize, 18usize, 0u32, 2167648u32);
    emu.ani_no_count(11usize, 22usize, 31u32, 2167652u32);
    emu.adi_no_count(18usize, 10usize, 2u32, 2167656u32);
    emu.ani_no_count(13usize, 12usize, 63u32, 2167660u32);
    emu.adi_no_count(12usize, 0usize, 224u32, 2167664u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a < b {
        emu.pc = 2167732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002113b4));
    } else {
        emu.pc = 2167668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211374));
    }
}
#[inline(always)]
pub fn block_0x00211374(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(14usize, 18usize, 0u32, 2167672u32);
    emu.adi_no_count(12usize, 10usize, 3u32, 2167676u32);
    emu.sli_no_count(13usize, 13usize, 6u32, 2167680u32);
    emu.ani_no_count(14usize, 14usize, 63u32, 2167684u32);
    emu.orr_no_count(13usize, 13usize, 14usize, 2167688u32);
    emu.adi_no_count(14usize, 0usize, 240u32, 2167692u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a < b {
        emu.pc = 2167744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002113c0));
    } else {
        emu.pc = 2167696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211390));
    }
}
#[inline]
pub fn block_0x00211390(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 4u32, 2167700u32);
    emu.lbu_no_count(10usize, 12usize, 0u32, 2167704u32);
    emu.sli_no_count(11usize, 11usize, 29u32, 2167708u32);
    emu.sri_no_count(11usize, 11usize, 11u32, 2167712u32);
    emu.sli_no_count(13usize, 13usize, 6u32, 2167716u32);
    emu.ani_no_count(10usize, 10usize, 63u32, 2167720u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2167724u32);
    emu.orr_no_count(22usize, 13usize, 10usize, 2167728u32);
    emu.add_memory_rw_events(9usize);
    let return_addr = 2167732u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2167756u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002113cc));
}
#[inline(always)]
pub fn block_0x002113b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(11usize, 11usize, 6u32, 2167736u32);
    emu.orr_no_count(22usize, 11usize, 13usize, 2167740u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2167744u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2167756u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002113cc));
}
#[inline(always)]
pub fn block_0x002113c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(11usize, 11usize, 12u32, 2167748u32);
    emu.orr_no_count(22usize, 13usize, 11usize, 2167752u32);
    emu.adi_no_count(18usize, 12usize, 0u32, 2167756u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2167756u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002113cc));
}
#[inline(always)]
pub fn block_0x002113cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 28u32, 2167760u32);
    emu.adi_no_count(11usize, 22usize, 0u32, 2167764u32);
    emu.lw_no_count(12usize, 2usize, 20u32, 2167768u32)?;
    emu.apc_no_count(1usize, 2167768u32, 4294963200u32, 2167772u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2167776u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965268u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002113e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 40u32, 2167780u32);
    emu.lbu_no_count(11usize, 2usize, 41u32, 2167784u32);
    emu.sbr_no_count(11usize, 11usize, 10usize, 2167788u32);
    emu.ani_no_count(10usize, 11usize, 255u32, 2167792u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2168044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002114ec));
    } else {
        emu.pc = 2167796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002113f4));
    }
}
#[inline(always)]
pub fn block_0x002113f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(13usize, 9usize, 25usize, 2167800u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2168352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211620));
    } else {
        emu.pc = 2167804u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002113fc));
    }
}
#[inline(always)]
pub fn block_0x002113fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(11usize, 23usize, 20usize, 2167808u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2167836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021141c));
    } else {
        emu.pc = 2167812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211404));
    }
}
#[inline(always)]
pub fn block_0x00211404(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a >= b {
        emu.pc = 2167832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211418));
    } else {
        emu.pc = 2167816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211408));
    }
}
#[inline(always)]
pub fn block_0x00211408(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(10usize, 11usize, 0u32, 2167820u32);
    emu.adi_no_count(12usize, 0usize, 4294967231u32, 2167824u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2167836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021141c));
    } else {
        emu.pc = 2167828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211414));
    }
}
#[inline(always)]
pub fn block_0x00211414(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2167832u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2168352u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211620));
}
#[inline(always)]
pub fn block_0x00211418(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a != b {
        emu.pc = 2168352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211620));
    } else {
        emu.pc = 2167836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021141c));
    }
}
#[inline(always)]
pub fn block_0x0021141c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2167880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211448));
    } else {
        emu.pc = 2167840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211420));
    }
}
#[inline(always)]
pub fn block_0x00211420(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2167868u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021143c));
    } else {
        emu.pc = 2167844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211424));
    }
}
#[inline(always)]
pub fn block_0x00211424(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 23usize, 9usize, 2167848u32);
    emu.adr_no_count(10usize, 10usize, 25usize, 2167852u32);
    emu.lb_no_count(10usize, 10usize, 0u32, 2167856u32);
    emu.adi_no_count(12usize, 0usize, 4294967231u32, 2167860u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2167880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211448));
    } else {
        emu.pc = 2167864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211438));
    }
}
#[inline(always)]
pub fn block_0x00211438(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2167868u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2168352u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211620));
}
#[inline(always)]
pub fn block_0x0021143c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2167872u32)?;
    emu.adr_no_count(10usize, 13usize, 10usize, 2167876u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2168352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211620));
    } else {
        emu.pc = 2167880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211448));
    }
}
#[inline(always)]
pub fn block_0x00211448(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(24usize, 23usize, 0u32, 2167884u32);
    emu.lw_no_count(10usize, 2usize, 16u32, 2167888u32)?;
    emu.lw_no_count(23usize, 10usize, 12u32, 2167892u32)?;
    emu.sbr_no_count(12usize, 9usize, 20usize, 2167896u32);
    emu.adr_no_count(12usize, 12usize, 25usize, 2167900u32);
    emu.lw_no_count(20usize, 2usize, 24u32, 2167904u32)?;
    emu.adi_no_count(10usize, 20usize, 0u32, 2167908u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(23usize);
    let return_addr = 2167912u32;
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
pub fn block_0x00211468(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2168296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002115e8));
    } else {
        emu.pc = 2167916u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021146c));
    }
}
#[inline(always)]
pub fn block_0x0021146c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 41u32, 2167920u32);
    emu.adi_no_count(11usize, 0usize, 129u32, 2167924u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2167952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211490));
    } else {
        emu.pc = 2167928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211478));
    }
}
#[inline(always)]
pub fn block_0x00211478(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 28u32, 2167932u32)?;
    emu.adi_no_count(10usize, 20usize, 0u32, 2167936u32);
    emu.lw_no_count(12usize, 2usize, 12u32, 2167940u32)?;
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2167944u32;
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
pub fn block_0x00211488(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2167980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002114ac));
    } else {
        emu.pc = 2167948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021148c));
    }
}
#[inline(always)]
pub fn block_0x0021148c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2167952u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2168296u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002115e8));
}
#[inline(always)]
pub fn block_0x00211490(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 2usize, 40u32, 2167956u32);
    emu.sbr_no_count(12usize, 10usize, 11usize, 2167960u32);
    emu.adi_no_count(10usize, 2usize, 28u32, 2167964u32);
    emu.adr_no_count(11usize, 10usize, 11usize, 2167968u32);
    emu.adi_no_count(10usize, 20usize, 0u32, 2167972u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(23usize);
    let return_addr = 2167976u32;
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
pub fn block_0x002114a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2168296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002115e8));
    } else {
        emu.pc = 2167980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002114ac));
    }
}
#[inline(always)]
pub fn block_0x002114ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 128u32, 2167984u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a >= b {
        emu.pc = 2168000u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002114c0));
    } else {
        emu.pc = 2167988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002114b4));
    }
}
#[inline(always)]
pub fn block_0x002114b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 1u32, 2167992u32);
    emu.adi_no_count(23usize, 24usize, 0u32, 2167996u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2168000u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2168032u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002114e0));
}
#[inline(always)]
pub fn block_0x002114c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 22usize, 11u32, 2168004u32);
    emu.adi_no_count(23usize, 24usize, 0u32, 2168008u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2168020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002114d4));
    } else {
        emu.pc = 2168012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002114cc));
    }
}
#[inline(always)]
pub fn block_0x002114cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 2u32, 2168016u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2168020u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2168032u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002114e0));
}
#[inline(always)]
pub fn block_0x002114d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 22usize, 16u32, 2168024u32);
    emu.sltru_no_count(20usize, 0usize, 10usize, 2168028u32);
    emu.adi_no_count(20usize, 20usize, 3u32, 2168032u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2168032u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002114e0));
}
#[inline(always)]
pub fn block_0x002114e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(24usize, 0usize, 1u32, 2168036u32);
    emu.adr_no_count(10usize, 9usize, 25usize, 2168040u32);
    emu.adr_no_count(20usize, 20usize, 10usize, 2168044u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2168044u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002114ec));
}
#[inline(always)]
pub fn block_0x002114ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 128u32, 2168048u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a < b {
        emu.pc = 2167556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211304));
    } else {
        emu.pc = 2168052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002114f4));
    }
}
#[inline(always)]
pub fn block_0x002114f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 22usize, 11u32, 2168056u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2168068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211504));
    } else {
        emu.pc = 2168060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002114fc));
    }
}
#[inline(always)]
pub fn block_0x002114fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2168064u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2168068u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2167560u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211308));
}
#[inline(always)]
pub fn block_0x00211504(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 22usize, 16u32, 2168072u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2168076u32);
    emu.adi_no_count(10usize, 10usize, 3u32, 2168080u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2168084u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2167560u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211308));
}
#[inline(always)]
pub fn block_0x00211514(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(13usize, 13usize, 9usize, 2168088u32);
    emu.lw_no_count(21usize, 2usize, 16u32, 2168092u32)?;
    emu.lw_no_count(9usize, 2usize, 12u32, 2168096u32)?;
    emu.adi_no_count(18usize, 0usize, 1u32, 2168100u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2168324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211604));
    } else {
        emu.pc = 2168104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211528));
    }
}
#[inline(always)]
pub fn block_0x00211528(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2168144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211550));
    } else {
        emu.pc = 2168108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021152c));
    }
}
#[inline(always)]
pub fn block_0x0021152c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a >= b {
        emu.pc = 2168140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021154c));
    } else {
        emu.pc = 2168112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211530));
    }
}
#[inline(always)]
pub fn block_0x00211530(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 23usize, 20usize, 2168116u32);
    emu.lb_no_count(10usize, 10usize, 0u32, 2168120u32);
    emu.adi_no_count(11usize, 0usize, 4294967231u32, 2168124u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2168144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211550));
    } else {
        emu.pc = 2168128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211540));
    }
}
#[inline(always)]
pub fn block_0x00211540(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2168132u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2168324u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211604));
}
#[inline(always)]
pub fn block_0x00211544(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 0u32, 2168136u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2168140u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2168188u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021157c));
}
#[inline(always)]
pub fn block_0x0021154c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a != b {
        emu.pc = 2168324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211604));
    } else {
        emu.pc = 2168144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211550));
    }
}
#[inline(always)]
pub fn block_0x00211550(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2168176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211570));
    } else {
        emu.pc = 2168148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211554));
    }
}
#[inline(always)]
pub fn block_0x00211554(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2168184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211578));
    } else {
        emu.pc = 2168152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211558));
    }
}
#[inline(always)]
pub fn block_0x00211558(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 23usize, 13usize, 2168156u32);
    emu.lb_no_count(10usize, 10usize, 0u32, 2168160u32);
    emu.adi_no_count(11usize, 0usize, 4294967231u32, 2168164u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2168324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211604));
    } else {
        emu.pc = 2168168u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211568));
    }
}
#[inline(always)]
pub fn block_0x00211568(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 13usize, 0u32, 2168172u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2168176u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2168188u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021157c));
}
#[inline(always)]
pub fn block_0x00211570(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 0u32, 2168180u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2168184u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2168188u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021157c));
}
#[inline(always)]
pub fn block_0x00211578(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2168324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211604));
    } else {
        emu.pc = 2168188u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021157c));
    }
}
#[inline(always)]
pub fn block_0x0021157c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 21usize, 12u32, 2168192u32)?;
    emu.sbr_no_count(12usize, 19usize, 20usize, 2168196u32);
    emu.adr_no_count(11usize, 23usize, 20usize, 2168200u32);
    emu.lw_no_count(8usize, 2usize, 24u32, 2168204u32)?;
    emu.adi_no_count(10usize, 8usize, 0u32, 2168208u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2168212u32;
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
pub fn block_0x00211594(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2168232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002115a8));
    } else {
        emu.pc = 2168216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211598));
    }
}
#[inline(always)]
pub fn block_0x00211598(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 34u32, 2168220u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2168224u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(9usize);
    let return_addr = 2168228u32;
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
pub fn block_0x002115a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2168232u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2168232u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002115a8));
}
#[inline]
pub fn block_0x002115a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2168236u32);
    emu.lw_no_count(1usize, 2usize, 92u32, 2168240u32)?;
    emu.lw_no_count(8usize, 2usize, 88u32, 2168244u32)?;
    emu.lw_no_count(9usize, 2usize, 84u32, 2168248u32)?;
    emu.lw_no_count(18usize, 2usize, 80u32, 2168252u32)?;
    emu.lw_no_count(19usize, 2usize, 76u32, 2168256u32)?;
    emu.lw_no_count(20usize, 2usize, 72u32, 2168260u32)?;
    emu.lw_no_count(21usize, 2usize, 68u32, 2168264u32)?;
    emu.lw_no_count(22usize, 2usize, 64u32, 2168268u32)?;
    emu.lw_no_count(23usize, 2usize, 60u32, 2168272u32)?;
    emu.lw_no_count(24usize, 2usize, 56u32, 2168276u32)?;
    emu.lw_no_count(25usize, 2usize, 52u32, 2168280u32)?;
    emu.lw_no_count(26usize, 2usize, 48u32, 2168284u32)?;
    emu.lw_no_count(27usize, 2usize, 44u32, 2168288u32)?;
    emu.adi_no_count(2usize, 2usize, 96u32, 2168292u32);
    emu.add_memory_rw_events(16usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2168296u32;
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
pub fn block_0x002115e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 1u32, 2168300u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2168304u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2168232u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002115a8));
}
#[inline(always)]
pub fn block_0x002115f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(13usize, 10usize, 25usize, 2168308u32);
    emu.lw_no_count(21usize, 2usize, 16u32, 2168312u32)?;
    emu.lw_no_count(9usize, 2usize, 12u32, 2168316u32)?;
    emu.adi_no_count(18usize, 0usize, 1u32, 2168320u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2168104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211528));
    } else {
        emu.pc = 2168324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211604));
    }
}
#[inline(always)]
pub fn block_0x00211604(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2168328u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 584u32, 2168332u32);
    emu.adi_no_count(10usize, 23usize, 0u32, 2168336u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2168340u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2168344u32);
    emu.apc_no_count(1usize, 2168344u32, 4294946816u32, 2168348u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2168352u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(108u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00211620(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2168356u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 568u32, 2168360u32);
    emu.adi_no_count(10usize, 23usize, 0u32, 2168364u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2168368u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2168372u32);
    emu.apc_no_count(1usize, 2168372u32, 4294946816u32, 2168376u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2168380u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(80u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
