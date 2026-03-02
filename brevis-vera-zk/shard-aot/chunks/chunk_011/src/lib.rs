pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2144752u32;
pub const PC_MAX: u32 = 2147996u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 107usize] = [
        block_0x0020b9f0,
        block_0x0020b9fc,
        block_0x0020ba24,
        block_0x0020ba50,
        block_0x0020bac0,
        block_0x0020bac8,
        block_0x0020bb1c,
        block_0x0020bb20,
        block_0x0020bb24,
        block_0x0020bb44,
        block_0x0020bb6c,
        block_0x0020bb7c,
        block_0x0020bb88,
        block_0x0020bb8c,
        block_0x0020bbbc,
        block_0x0020bbe8,
        block_0x0020bc58,
        block_0x0020bc60,
        block_0x0020bcb4,
        block_0x0020bcb8,
        block_0x0020bcbc,
        block_0x0020bcdc,
        block_0x0020bd08,
        block_0x0020bd18,
        block_0x0020bd24,
        block_0x0020bd28,
        block_0x0020bd44,
        block_0x0020bd70,
        block_0x0020bde4,
        block_0x0020bdec,
        block_0x0020be3c,
        block_0x0020be40,
        block_0x0020be44,
        block_0x0020be58,
        block_0x0020be88,
        block_0x0020be98,
        block_0x0020be9c,
        block_0x0020bedc,
        block_0x0020befc,
        block_0x0020bf14,
        block_0x0020bf3c,
        block_0x0020bf44,
        block_0x0020bf54,
        block_0x0020bf74,
        block_0x0020bf94,
        block_0x0020bfb0,
        block_0x0020c010,
        block_0x0020c038,
        block_0x0020c054,
        block_0x0020c0cc,
        block_0x0020c0dc,
        block_0x0020c13c,
        block_0x0020c140,
        block_0x0020c15c,
        block_0x0020c168,
        block_0x0020c170,
        block_0x0020c18c,
        block_0x0020c1cc,
        block_0x0020c1ec,
        block_0x0020c228,
        block_0x0020c288,
        block_0x0020c2b8,
        block_0x0020c310,
        block_0x0020c36c,
        block_0x0020c384,
        block_0x0020c3ec,
        block_0x0020c3fc,
        block_0x0020c404,
        block_0x0020c418,
        block_0x0020c42c,
        block_0x0020c434,
        block_0x0020c440,
        block_0x0020c448,
        block_0x0020c458,
        block_0x0020c460,
        block_0x0020c468,
        block_0x0020c470,
        block_0x0020c47c,
        block_0x0020c480,
        block_0x0020c484,
        block_0x0020c488,
        block_0x0020c48c,
        block_0x0020c49c,
        block_0x0020c4a0,
        block_0x0020c4a8,
        block_0x0020c4ac,
        block_0x0020c4b0,
        block_0x0020c4bc,
        block_0x0020c4c4,
        block_0x0020c4c8,
        block_0x0020c4d4,
        block_0x0020c4e4,
        block_0x0020c4f0,
        block_0x0020c504,
        block_0x0020c51c,
        block_0x0020c540,
        block_0x0020c544,
        block_0x0020c54c,
        block_0x0020c558,
        block_0x0020c560,
        block_0x0020c574,
        block_0x0020c57c,
        block_0x0020c584,
        block_0x0020c590,
        block_0x0020c62c,
        block_0x0020c630,
        block_0x0020c69c,
    ];
    const IDX: [u16; 812usize] = [
        1u16, 0u16, 0u16, 2u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        3u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 4u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        5u16, 0u16, 6u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 7u16, 8u16, 9u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 10u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 11u16, 0u16, 0u16, 0u16, 12u16, 0u16, 0u16, 13u16, 14u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 15u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 16u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 17u16, 0u16,
        18u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 19u16, 20u16, 21u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 22u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 23u16, 0u16, 0u16, 0u16, 24u16, 0u16, 0u16, 25u16, 26u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 27u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 28u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 29u16, 0u16, 30u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 31u16, 32u16, 33u16, 0u16, 0u16, 0u16, 0u16, 34u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 35u16, 0u16, 0u16, 0u16, 36u16,
        37u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 38u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 39u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 40u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 41u16, 0u16, 42u16, 0u16, 0u16, 0u16, 43u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 44u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 45u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 46u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 47u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 48u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 49u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 50u16, 0u16, 0u16,
        0u16, 51u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 52u16,
        53u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 54u16, 0u16, 0u16, 55u16, 0u16, 56u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 57u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 58u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 59u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 60u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 61u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 62u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 63u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 64u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 65u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 66u16, 0u16, 0u16, 0u16, 67u16, 0u16, 68u16, 0u16, 0u16, 0u16, 0u16,
        69u16, 0u16, 0u16, 0u16, 0u16, 70u16, 0u16, 71u16, 0u16, 0u16, 72u16, 0u16,
        73u16, 0u16, 0u16, 0u16, 74u16, 0u16, 75u16, 0u16, 76u16, 0u16, 77u16, 0u16,
        0u16, 78u16, 79u16, 80u16, 81u16, 82u16, 0u16, 0u16, 0u16, 83u16, 84u16, 0u16,
        85u16, 86u16, 87u16, 0u16, 0u16, 88u16, 0u16, 89u16, 90u16, 0u16, 0u16, 91u16,
        0u16, 0u16, 0u16, 92u16, 0u16, 0u16, 93u16, 0u16, 0u16, 0u16, 0u16, 94u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 95u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        96u16, 97u16, 0u16, 98u16, 0u16, 0u16, 99u16, 0u16, 100u16, 0u16, 0u16, 0u16,
        0u16, 101u16, 0u16, 102u16, 0u16, 103u16, 0u16, 0u16, 104u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 105u16, 106u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 107u16,
    ];
    if pc < 2144752u32 || pc > 2147996u32 {
        return None;
    }
    let word_offset = ((pc - 2144752u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x0020b9f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2144756u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2144760u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2144764u32;
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
pub fn block_0x0020b9fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2144768u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2144772u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2144776u32)?;
    emu.adi_no_count(13usize, 11usize, 0u32, 2144780u32);
    emu.lw_no_count(11usize, 10usize, 0u32, 2144784u32)?;
    emu.adi_no_count(12usize, 0usize, 1000u32, 2144788u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2144792u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966834u32, 2144796u32);
    emu.adi_no_count(14usize, 0usize, 10u32, 2144800u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2145148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bb7c));
    } else {
        emu.pc = 2144804u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ba24));
    }
}
#[inline]
pub fn block_0x0020ba24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 2usize, 23u32, 2144808u32);
    let a = 0u32.wrapping_add(3518435328u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2144812u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2144816u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4096u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2144820u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 0usize, 100u32, 2144824u32);
    let a = 0u32.wrapping_add(9998336u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2144828u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 12usize, 1881u32, 2144832u32);
    emu.adi_no_count(5usize, 5usize, 1808u32, 2144836u32);
    emu.adi_no_count(6usize, 6usize, 1147u32, 2144840u32);
    emu.adi_no_count(7usize, 7usize, 1663u32, 2144844u32);
    emu.adi_no_count(12usize, 11usize, 0u32, 2144848u32);
    emu.add_memory_rw_events(11usize);
    emu.pc = 2144848u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ba50));
}
#[inline(never)]
pub fn block_0x0020ba50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(28usize, 12usize, 0u32, 2144852u32);
    emu.adi_no_count(14usize, 14usize, 4294967292u32, 2144856u32);
    emu.mulhu_no_count(12usize, 12usize, 17usize, 2144860u32);
    emu.sri_no_count(12usize, 12usize, 13u32, 2144864u32);
    emu.mul_no_count(29usize, 12usize, 5usize, 2144868u32);
    emu.sbr_no_count(29usize, 28usize, 29usize, 2144872u32);
    emu.sli_no_count(30usize, 29usize, 16u32, 2144876u32);
    emu.sri_no_count(30usize, 30usize, 18u32, 2144880u32);
    emu.mul_no_count(30usize, 30usize, 6usize, 2144884u32);
    emu.sri_no_count(31usize, 30usize, 16u32, 2144888u32);
    emu.sri_no_count(30usize, 30usize, 17u32, 2144892u32);
    emu.mul_no_count(30usize, 30usize, 16usize, 2144896u32);
    emu.ani_no_count(31usize, 31usize, 2046u32, 2144900u32);
    emu.sbr_no_count(29usize, 29usize, 30usize, 2144904u32);
    emu.adr_no_count(31usize, 10usize, 31usize, 2144908u32);
    emu.sli_no_count(29usize, 29usize, 17u32, 2144912u32);
    emu.sri_no_count(29usize, 29usize, 16u32, 2144916u32);
    emu.adr_no_count(29usize, 10usize, 29usize, 2144920u32);
    emu.lbu_no_count(30usize, 31usize, 0u32, 2144924u32);
    emu.lbu_no_count(31usize, 31usize, 1u32, 2144928u32);
    emu.lbu_no_count(8usize, 29usize, 0u32, 2144932u32);
    emu.lbu_no_count(29usize, 29usize, 1u32, 2144936u32);
    emu.sb_no_count(30usize, 15usize, 4294967293u32, 2144940u32);
    emu.sb_no_count(31usize, 15usize, 4294967294u32, 2144944u32);
    emu.sb_no_count(8usize, 15usize, 4294967295u32, 2144948u32);
    emu.sb_no_count(29usize, 15usize, 0u32, 2144952u32);
    emu.adi_no_count(15usize, 15usize, 4294967292u32, 2144956u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a < b {
        emu.pc = 2144848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ba50));
    } else {
        emu.pc = 2144960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bac0));
    }
}
#[inline(always)]
pub fn block_0x0020bac0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 9u32, 2144964u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2145052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bb1c));
    } else {
        emu.pc = 2144968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bac8));
    }
}
#[inline]
pub fn block_0x0020bac8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(15usize, 12usize, 16u32, 2144972u32);
    let a = 0u32.wrapping_add(4096u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2144976u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 0usize, 100u32, 2144980u32);
    emu.adi_no_count(5usize, 2usize, 14u32, 2144984u32);
    emu.sri_no_count(15usize, 15usize, 18u32, 2144988u32);
    emu.adi_no_count(16usize, 16usize, 1147u32, 2144992u32);
    emu.adr_no_count(6usize, 5usize, 14usize, 2144996u32);
    emu.mul_no_count(15usize, 15usize, 16usize, 2145000u32);
    emu.sri_no_count(15usize, 15usize, 17u32, 2145004u32);
    emu.mul_no_count(16usize, 15usize, 17usize, 2145008u32);
    emu.sbr_no_count(12usize, 12usize, 16usize, 2145012u32);
    emu.sli_no_count(12usize, 12usize, 17u32, 2145016u32);
    emu.sri_no_count(12usize, 12usize, 16u32, 2145020u32);
    emu.adr_no_count(12usize, 10usize, 12usize, 2145024u32);
    emu.lbu_no_count(16usize, 12usize, 0u32, 2145028u32);
    emu.lbu_no_count(12usize, 12usize, 1u32, 2145032u32);
    emu.adi_no_count(14usize, 14usize, 4294967294u32, 2145036u32);
    emu.adr_no_count(5usize, 5usize, 14usize, 2145040u32);
    emu.sb_no_count(16usize, 5usize, 0u32, 2145044u32);
    emu.sb_no_count(12usize, 6usize, 4294967295u32, 2145048u32);
    emu.adi_no_count(12usize, 15usize, 0u32, 2145052u32);
    emu.add_memory_rw_events(21usize);
    emu.pc = 2145052u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bb1c));
}
#[inline(always)]
pub fn block_0x0020bb1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2145060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bb24));
    } else {
        emu.pc = 2145056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bb20));
    }
}
#[inline(always)]
pub fn block_0x0020bb20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2145092u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bb44));
    } else {
        emu.pc = 2145060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bb24));
    }
}
#[inline(always)]
pub fn block_0x0020bb24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 1u32, 2145064u32);
    emu.ani_no_count(12usize, 12usize, 30u32, 2145068u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2145072u32);
    emu.lbu_no_count(10usize, 10usize, 1u32, 2145076u32);
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2145080u32);
    emu.adi_no_count(11usize, 2usize, 14u32, 2145084u32);
    emu.adr_no_count(11usize, 11usize, 14usize, 2145088u32);
    emu.sb_no_count(10usize, 11usize, 0u32, 2145092u32);
    emu.add_memory_rw_events(8usize);
    emu.pc = 2145092u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bb44));
}
#[inline]
pub fn block_0x0020bb44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 10u32, 2145096u32);
    emu.adi_no_count(10usize, 2usize, 14u32, 2145100u32);
    emu.sbr_no_count(15usize, 15usize, 14usize, 2145104u32);
    emu.adr_no_count(14usize, 10usize, 14usize, 2145108u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2145112u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2145116u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2145120u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2145124u32);
    emu.apc_no_count(1usize, 2145124u32, 16384u32, 2145128u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2145132u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1120u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020bb6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2145136u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2145140u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2145144u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2145148u32;
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
pub fn block_0x0020bb7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 11usize, 0u32, 2145152u32);
    emu.adi_no_count(15usize, 0usize, 9u32, 2145156u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a < b {
        emu.pc = 2144968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bac8));
    } else {
        emu.pc = 2145160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bb88));
    }
}
#[inline(always)]
pub fn block_0x0020bb88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2145164u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2145052u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bb1c));
}
#[inline]
pub fn block_0x0020bb8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2145168u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2145172u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2145176u32)?;
    emu.adi_no_count(13usize, 11usize, 0u32, 2145180u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2145184u32)?;
    emu.sai_no_count(11usize, 10usize, 1055u32, 2145188u32);
    emu.xrr_no_count(12usize, 10usize, 11usize, 2145192u32);
    emu.sbr_no_count(12usize, 12usize, 11usize, 2145196u32);
    emu.adi_no_count(14usize, 0usize, 1000u32, 2145200u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2145204u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966834u32, 2145208u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2145560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bd18));
    } else {
        emu.pc = 2145212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bbbc));
    }
}
#[inline]
pub fn block_0x0020bbbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 10u32, 2145216u32);
    emu.adi_no_count(15usize, 2usize, 23u32, 2145220u32);
    let a = 0u32.wrapping_add(3518435328u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2145224u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2145228u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4096u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2145232u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 0usize, 100u32, 2145236u32);
    let a = 0u32.wrapping_add(9998336u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2145240u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 17usize, 1881u32, 2145244u32);
    emu.adi_no_count(5usize, 5usize, 1808u32, 2145248u32);
    emu.adi_no_count(6usize, 6usize, 1147u32, 2145252u32);
    emu.adi_no_count(7usize, 7usize, 1663u32, 2145256u32);
    emu.add_memory_rw_events(11usize);
    emu.pc = 2145256u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bbe8));
}
#[inline(never)]
pub fn block_0x0020bbe8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(28usize, 12usize, 0u32, 2145260u32);
    emu.adi_no_count(14usize, 14usize, 4294967292u32, 2145264u32);
    emu.mulhu_no_count(12usize, 12usize, 17usize, 2145268u32);
    emu.sri_no_count(12usize, 12usize, 13u32, 2145272u32);
    emu.mul_no_count(29usize, 12usize, 5usize, 2145276u32);
    emu.sbr_no_count(29usize, 28usize, 29usize, 2145280u32);
    emu.sli_no_count(30usize, 29usize, 16u32, 2145284u32);
    emu.sri_no_count(30usize, 30usize, 18u32, 2145288u32);
    emu.mul_no_count(30usize, 30usize, 6usize, 2145292u32);
    emu.sri_no_count(31usize, 30usize, 16u32, 2145296u32);
    emu.sri_no_count(30usize, 30usize, 17u32, 2145300u32);
    emu.mul_no_count(30usize, 30usize, 16usize, 2145304u32);
    emu.ani_no_count(31usize, 31usize, 2046u32, 2145308u32);
    emu.sbr_no_count(29usize, 29usize, 30usize, 2145312u32);
    emu.adr_no_count(31usize, 11usize, 31usize, 2145316u32);
    emu.sli_no_count(29usize, 29usize, 17u32, 2145320u32);
    emu.sri_no_count(29usize, 29usize, 16u32, 2145324u32);
    emu.adr_no_count(29usize, 11usize, 29usize, 2145328u32);
    emu.lbu_no_count(30usize, 31usize, 0u32, 2145332u32);
    emu.lbu_no_count(31usize, 31usize, 1u32, 2145336u32);
    emu.lbu_no_count(8usize, 29usize, 0u32, 2145340u32);
    emu.lbu_no_count(29usize, 29usize, 1u32, 2145344u32);
    emu.sb_no_count(30usize, 15usize, 4294967293u32, 2145348u32);
    emu.sb_no_count(31usize, 15usize, 4294967294u32, 2145352u32);
    emu.sb_no_count(8usize, 15usize, 4294967295u32, 2145356u32);
    emu.sb_no_count(29usize, 15usize, 0u32, 2145360u32);
    emu.adi_no_count(15usize, 15usize, 4294967292u32, 2145364u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a < b {
        emu.pc = 2145256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bbe8));
    } else {
        emu.pc = 2145368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bc58));
    }
}
#[inline(always)]
pub fn block_0x0020bc58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 9u32, 2145372u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2145460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bcb4));
    } else {
        emu.pc = 2145376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bc60));
    }
}
#[inline]
pub fn block_0x0020bc60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(15usize, 12usize, 16u32, 2145380u32);
    let a = 0u32.wrapping_add(4096u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2145384u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 0usize, 100u32, 2145388u32);
    emu.adi_no_count(5usize, 2usize, 14u32, 2145392u32);
    emu.sri_no_count(15usize, 15usize, 18u32, 2145396u32);
    emu.adi_no_count(16usize, 16usize, 1147u32, 2145400u32);
    emu.adr_no_count(6usize, 5usize, 14usize, 2145404u32);
    emu.mul_no_count(15usize, 15usize, 16usize, 2145408u32);
    emu.sri_no_count(15usize, 15usize, 17u32, 2145412u32);
    emu.mul_no_count(16usize, 15usize, 17usize, 2145416u32);
    emu.sbr_no_count(12usize, 12usize, 16usize, 2145420u32);
    emu.sli_no_count(12usize, 12usize, 17u32, 2145424u32);
    emu.sri_no_count(12usize, 12usize, 16u32, 2145428u32);
    emu.adr_no_count(12usize, 11usize, 12usize, 2145432u32);
    emu.lbu_no_count(16usize, 12usize, 0u32, 2145436u32);
    emu.lbu_no_count(12usize, 12usize, 1u32, 2145440u32);
    emu.adi_no_count(14usize, 14usize, 4294967294u32, 2145444u32);
    emu.adr_no_count(5usize, 5usize, 14usize, 2145448u32);
    emu.sb_no_count(16usize, 5usize, 0u32, 2145452u32);
    emu.sb_no_count(12usize, 6usize, 4294967295u32, 2145456u32);
    emu.adi_no_count(12usize, 15usize, 0u32, 2145460u32);
    emu.add_memory_rw_events(21usize);
    emu.pc = 2145460u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bcb4));
}
#[inline(always)]
pub fn block_0x0020bcb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2145468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bcbc));
    } else {
        emu.pc = 2145464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bcb8));
    }
}
#[inline(always)]
pub fn block_0x0020bcb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2145500u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bcdc));
    } else {
        emu.pc = 2145468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bcbc));
    }
}
#[inline(always)]
pub fn block_0x0020bcbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 1u32, 2145472u32);
    emu.ani_no_count(12usize, 12usize, 30u32, 2145476u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2145480u32);
    emu.lbu_no_count(11usize, 11usize, 1u32, 2145484u32);
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2145488u32);
    emu.adi_no_count(12usize, 2usize, 14u32, 2145492u32);
    emu.adr_no_count(12usize, 12usize, 14usize, 2145496u32);
    emu.sb_no_count(11usize, 12usize, 0u32, 2145500u32);
    emu.add_memory_rw_events(8usize);
    emu.pc = 2145500u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bcdc));
}
#[inline]
pub fn block_0x0020bcdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 10u32, 2145504u32);
    emu.adi_no_count(11usize, 2usize, 14u32, 2145508u32);
    emu.sbr_no_count(15usize, 15usize, 14usize, 2145512u32);
    emu.adr_no_count(14usize, 11usize, 14usize, 2145516u32);
    emu.xri_no_count(11usize, 10usize, 4294967295u32, 2145520u32);
    emu.sri_no_count(11usize, 11usize, 31u32, 2145524u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2145528u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2145532u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2145536u32);
    emu.apc_no_count(1usize, 2145536u32, 16384u32, 2145540u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2145544u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(708u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020bd08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2145548u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2145552u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2145556u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2145560u32;
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
pub fn block_0x0020bd18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 10u32, 2145564u32);
    emu.adi_no_count(15usize, 0usize, 9u32, 2145568u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a < b {
        emu.pc = 2145376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bc60));
    } else {
        emu.pc = 2145572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bd24));
    }
}
#[inline(always)]
pub fn block_0x0020bd24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2145576u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2145460u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bcb4));
}
#[inline(always)]
pub fn block_0x0020bd28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2145580u32);
    emu.sw_no_count(8usize, 2usize, 12u32, 2145584u32)?;
    emu.sw_no_count(9usize, 2usize, 8u32, 2145588u32)?;
    emu.adi_no_count(13usize, 0usize, 1000u32, 2145592u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2145596u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966834u32, 2145600u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2145928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020be88));
    } else {
        emu.pc = 2145604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bd44));
    }
}
#[inline]
pub fn block_0x0020bd44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 11usize, 4294967294u32, 2145608u32);
    let a = 0u32.wrapping_add(3518435328u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2145612u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2145616u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4096u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2145620u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 0usize, 100u32, 2145624u32);
    let a = 0u32.wrapping_add(9998336u32);
    emu.write_reg_no_count(28usize, a);
    emu.pc = 2145628u32;
    emu.update_insn_clock();
    emu.adi_no_count(5usize, 13usize, 1881u32, 2145632u32);
    emu.adi_no_count(6usize, 15usize, 1808u32, 2145636u32);
    emu.adi_no_count(7usize, 7usize, 1147u32, 2145640u32);
    emu.adi_no_count(28usize, 28usize, 1663u32, 2145644u32);
    emu.adi_no_count(15usize, 10usize, 0u32, 2145648u32);
    emu.add_memory_rw_events(11usize);
    emu.pc = 2145648u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bd70));
}
#[inline(never)]
pub fn block_0x0020bd70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 29u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(29usize, 15usize, 0u32, 2145652u32);
    emu.adi_no_count(13usize, 12usize, 4294967292u32, 2145656u32);
    emu.mulhu_no_count(15usize, 15usize, 5usize, 2145660u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2145664u32);
    emu.sri_no_count(15usize, 15usize, 13u32, 2145668u32);
    emu.mul_no_count(30usize, 15usize, 6usize, 2145672u32);
    emu.sbr_no_count(30usize, 29usize, 30usize, 2145676u32);
    emu.sli_no_count(31usize, 30usize, 16u32, 2145680u32);
    emu.sri_no_count(31usize, 31usize, 18u32, 2145684u32);
    emu.mul_no_count(31usize, 31usize, 7usize, 2145688u32);
    emu.sri_no_count(8usize, 31usize, 16u32, 2145692u32);
    emu.sri_no_count(31usize, 31usize, 17u32, 2145696u32);
    emu.mul_no_count(31usize, 31usize, 17usize, 2145700u32);
    emu.ani_no_count(8usize, 8usize, 2046u32, 2145704u32);
    emu.sbr_no_count(30usize, 30usize, 31usize, 2145708u32);
    emu.adr_no_count(8usize, 14usize, 8usize, 2145712u32);
    emu.sli_no_count(30usize, 30usize, 17u32, 2145716u32);
    emu.sri_no_count(30usize, 30usize, 16u32, 2145720u32);
    emu.adr_no_count(30usize, 14usize, 30usize, 2145724u32);
    emu.lbu_no_count(31usize, 8usize, 0u32, 2145728u32);
    emu.lbu_no_count(8usize, 8usize, 1u32, 2145732u32);
    emu.lbu_no_count(9usize, 30usize, 0u32, 2145736u32);
    emu.lbu_no_count(30usize, 30usize, 1u32, 2145740u32);
    emu.sb_no_count(31usize, 12usize, 4294967294u32, 2145744u32);
    emu.sb_no_count(8usize, 12usize, 4294967295u32, 2145748u32);
    emu.sb_no_count(9usize, 12usize, 0u32, 2145752u32);
    emu.sb_no_count(30usize, 12usize, 1u32, 2145756u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2145760u32);
    emu.add_memory_rw_events(28usize);
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a < b {
        emu.pc = 2145648u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bd70));
    } else {
        emu.pc = 2145764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bde4));
    }
}
#[inline(always)]
pub fn block_0x0020bde4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 9u32, 2145768u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a >= b {
        emu.pc = 2145852u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020be3c));
    } else {
        emu.pc = 2145772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bdec));
    }
}
#[inline]
pub fn block_0x0020bdec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 15usize, 16u32, 2145776u32);
    let a = 0u32.wrapping_add(4096u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2145780u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 0usize, 100u32, 2145784u32);
    emu.adr_no_count(5usize, 11usize, 13usize, 2145788u32);
    emu.sri_no_count(12usize, 12usize, 18u32, 2145792u32);
    emu.adi_no_count(16usize, 16usize, 1147u32, 2145796u32);
    emu.mul_no_count(12usize, 12usize, 16usize, 2145800u32);
    emu.sri_no_count(12usize, 12usize, 17u32, 2145804u32);
    emu.mul_no_count(16usize, 12usize, 17usize, 2145808u32);
    emu.sbr_no_count(15usize, 15usize, 16usize, 2145812u32);
    emu.sli_no_count(15usize, 15usize, 17u32, 2145816u32);
    emu.sri_no_count(15usize, 15usize, 16u32, 2145820u32);
    emu.adr_no_count(15usize, 14usize, 15usize, 2145824u32);
    emu.lbu_no_count(16usize, 15usize, 0u32, 2145828u32);
    emu.lbu_no_count(15usize, 15usize, 1u32, 2145832u32);
    emu.adi_no_count(13usize, 13usize, 4294967294u32, 2145836u32);
    emu.adr_no_count(17usize, 11usize, 13usize, 2145840u32);
    emu.sb_no_count(16usize, 17usize, 0u32, 2145844u32);
    emu.sb_no_count(15usize, 5usize, 4294967295u32, 2145848u32);
    emu.adi_no_count(15usize, 12usize, 0u32, 2145852u32);
    emu.add_memory_rw_events(20usize);
    emu.pc = 2145852u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020be3c));
}
#[inline(always)]
pub fn block_0x0020be3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2145880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020be58));
    } else {
        emu.pc = 2145856u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020be40));
    }
}
#[inline(always)]
pub fn block_0x0020be40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2145880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020be58));
    } else {
        emu.pc = 2145860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020be44));
    }
}
#[inline(always)]
pub fn block_0x0020be44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 13usize, 0u32, 2145864u32);
    emu.lw_no_count(8usize, 2usize, 12u32, 2145868u32)?;
    emu.lw_no_count(9usize, 2usize, 8u32, 2145872u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2145876u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2145880u32;
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
pub fn block_0x0020be58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(15usize, 15usize, 1u32, 2145884u32);
    emu.ani_no_count(15usize, 15usize, 30u32, 2145888u32);
    emu.adr_no_count(14usize, 14usize, 15usize, 2145892u32);
    emu.lbu_no_count(10usize, 14usize, 1u32, 2145896u32);
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2145900u32);
    emu.adr_no_count(11usize, 11usize, 13usize, 2145904u32);
    emu.sb_no_count(10usize, 11usize, 0u32, 2145908u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2145912u32);
    emu.lw_no_count(8usize, 2usize, 12u32, 2145916u32)?;
    emu.lw_no_count(9usize, 2usize, 8u32, 2145920u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2145924u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2145928u32;
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
pub fn block_0x0020be88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 10usize, 0u32, 2145932u32);
    emu.adi_no_count(13usize, 12usize, 0u32, 2145936u32);
    emu.adi_no_count(12usize, 0usize, 9u32, 2145940u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2145772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bdec));
    } else {
        emu.pc = 2145944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020be98));
    }
}
#[inline(always)]
pub fn block_0x0020be98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2145948u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2145852u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020be3c));
}
#[inline]
pub fn block_0x0020be9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2145952u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2145956u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2145960u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2145964u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2145968u32)?;
    emu.lw_no_count(14usize, 10usize, 0u32, 2145972u32)?;
    emu.lw_no_count(15usize, 10usize, 4u32, 2145976u32)?;
    emu.adi_no_count(8usize, 11usize, 0u32, 2145980u32);
    emu.adi_no_count(9usize, 2usize, 12u32, 2145984u32);
    emu.adi_no_count(12usize, 2usize, 12u32, 2145988u32);
    emu.adi_no_count(13usize, 0usize, 20u32, 2145992u32);
    emu.adi_no_count(18usize, 0usize, 20u32, 2145996u32);
    emu.adi_no_count(10usize, 14usize, 0u32, 2146000u32);
    emu.adi_no_count(11usize, 15usize, 0u32, 2146004u32);
    emu.apc_no_count(1usize, 2146004u32, 0u32, 2146008u32);
    emu.add_memory_rw_events(16usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2146012u32;
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
pub fn block_0x0020bedc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(15usize, 18usize, 10usize, 2146016u32);
    emu.adr_no_count(14usize, 9usize, 10usize, 2146020u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2146024u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2146028u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2146032u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2146036u32);
    emu.apc_no_count(1usize, 2146036u32, 16384u32, 2146040u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2146044u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(208u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020befc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2146048u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2146052u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2146056u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2146060u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2146064u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2146068u32;
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
pub fn block_0x0020bf14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2146072u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2146076u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2146080u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2146084u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2146088u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2146092u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2146096u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2146100u32)?;
    emu.adi_no_count(8usize, 11usize, 0u32, 2146104u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2146116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bf44));
    } else {
        emu.pc = 2146108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bf3c));
    }
}
#[inline(always)]
pub fn block_0x0020bf3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 12usize, 0u32, 2146112u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2146116u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2146132u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bf54));
}
#[inline(always)]
pub fn block_0x0020bf44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(11usize, 0usize, 10usize, 2146120u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2146124u32);
    emu.sbr_no_count(13usize, 0usize, 12usize, 2146128u32);
    emu.sbr_no_count(11usize, 13usize, 11usize, 2146132u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2146132u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bf54));
}
#[inline(always)]
pub fn block_0x0020bf54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.slti_no_count(12usize, 12usize, 0u32, 2146136u32);
    emu.xri_no_count(9usize, 12usize, 1u32, 2146140u32);
    emu.adi_no_count(18usize, 2usize, 8u32, 2146144u32);
    emu.adi_no_count(12usize, 2usize, 8u32, 2146148u32);
    emu.adi_no_count(13usize, 0usize, 20u32, 2146152u32);
    emu.adi_no_count(19usize, 0usize, 20u32, 2146156u32);
    emu.apc_no_count(1usize, 2146156u32, 0u32, 2146160u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2146164u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(68u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020bf74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(15usize, 19usize, 10usize, 2146168u32);
    emu.adr_no_count(14usize, 18usize, 10usize, 2146172u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2146176u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2146180u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2146184u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2146188u32);
    emu.apc_no_count(1usize, 2146188u32, 16384u32, 2146192u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2146196u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(56u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020bf94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2146200u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2146204u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2146208u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2146212u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2146216u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2146220u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2146224u32;
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
pub fn block_0x0020bfb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2146228u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2146232u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2146236u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2146240u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2146244u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2146248u32)?;
    emu.sw_no_count(20usize, 2usize, 40u32, 2146252u32)?;
    emu.sw_no_count(21usize, 2usize, 36u32, 2146256u32)?;
    emu.sw_no_count(22usize, 2usize, 32u32, 2146260u32)?;
    emu.sw_no_count(23usize, 2usize, 28u32, 2146264u32)?;
    emu.sw_no_count(24usize, 2usize, 24u32, 2146268u32)?;
    emu.sw_no_count(25usize, 2usize, 20u32, 2146272u32)?;
    emu.sw_no_count(26usize, 2usize, 16u32, 2146276u32)?;
    emu.sw_no_count(27usize, 2usize, 12u32, 2146280u32)?;
    emu.adi_no_count(21usize, 13usize, 0u32, 2146284u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2146288u32);
    emu.adi_no_count(18usize, 10usize, 0u32, 2146292u32);
    emu.sltiu_no_count(10usize, 10usize, 1000u32, 2146296u32);
    emu.sltiu_no_count(11usize, 11usize, 1u32, 2146300u32);
    emu.anr_no_count(10usize, 11usize, 10usize, 2146304u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(24usize, a);
    emu.pc = 2146308u32;
    emu.update_insn_clock();
    emu.adi_no_count(24usize, 24usize, 4294966834u32, 2146312u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2146316u32)?;
    emu.add_memory_rw_events(23usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2146624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c140));
    } else {
        emu.pc = 2146320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c010));
    }
}
#[inline]
pub fn block_0x0020c010(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 12usize, 4294967294u32, 2146324u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2146328u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4096u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2146332u32;
    emu.update_insn_clock();
    emu.adi_no_count(26usize, 0usize, 100u32, 2146336u32);
    let a = 0u32.wrapping_add(9998336u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2146340u32;
    emu.update_insn_clock();
    emu.adi_no_count(20usize, 10usize, 1808u32, 2146344u32);
    emu.adi_no_count(27usize, 11usize, 1147u32, 2146348u32);
    emu.adi_no_count(8usize, 12usize, 1663u32, 2146352u32);
    emu.adi_no_count(22usize, 18usize, 0u32, 2146356u32);
    emu.adi_no_count(23usize, 9usize, 0u32, 2146360u32);
    emu.add_memory_rw_events(10usize);
    emu.pc = 2146360u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c038));
}
#[inline(always)]
pub fn block_0x0020c038(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 21usize, 4294967292u32, 2146364u32);
    emu.adi_no_count(10usize, 22usize, 0u32, 2146368u32);
    emu.adi_no_count(11usize, 23usize, 0u32, 2146372u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2146376u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2146380u32);
    emu.apc_no_count(1usize, 2146380u32, 32768u32, 2146384u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2146388u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966496u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0020c054(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 30u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.mul_no_count(12usize, 10usize, 20usize, 2146392u32);
    emu.adr_no_count(21usize, 25usize, 21usize, 2146396u32);
    emu.sltru_no_count(13usize, 8usize, 22usize, 2146400u32);
    emu.sltru_no_count(14usize, 0usize, 23usize, 2146404u32);
    emu.sbr_no_count(12usize, 22usize, 12usize, 2146408u32);
    emu.orr_no_count(13usize, 13usize, 14usize, 2146412u32);
    emu.sli_no_count(14usize, 12usize, 16u32, 2146416u32);
    emu.sri_no_count(14usize, 14usize, 18u32, 2146420u32);
    emu.mul_no_count(14usize, 14usize, 27usize, 2146424u32);
    emu.sri_no_count(15usize, 14usize, 16u32, 2146428u32);
    emu.sri_no_count(14usize, 14usize, 17u32, 2146432u32);
    emu.mul_no_count(14usize, 14usize, 26usize, 2146436u32);
    emu.ani_no_count(15usize, 15usize, 2046u32, 2146440u32);
    emu.sbr_no_count(12usize, 12usize, 14usize, 2146444u32);
    emu.adr_no_count(15usize, 24usize, 15usize, 2146448u32);
    emu.sli_no_count(12usize, 12usize, 17u32, 2146452u32);
    emu.sri_no_count(12usize, 12usize, 16u32, 2146456u32);
    emu.adr_no_count(12usize, 24usize, 12usize, 2146460u32);
    emu.lbu_no_count(14usize, 15usize, 0u32, 2146464u32);
    emu.lbu_no_count(15usize, 15usize, 1u32, 2146468u32);
    emu.lbu_no_count(16usize, 12usize, 0u32, 2146472u32);
    emu.lbu_no_count(12usize, 12usize, 1u32, 2146476u32);
    emu.sb_no_count(14usize, 21usize, 4294967294u32, 2146480u32);
    emu.sb_no_count(15usize, 21usize, 4294967295u32, 2146484u32);
    emu.sb_no_count(16usize, 21usize, 0u32, 2146488u32);
    emu.sb_no_count(12usize, 21usize, 1u32, 2146492u32);
    emu.adi_no_count(21usize, 19usize, 0u32, 2146496u32);
    emu.adi_no_count(22usize, 10usize, 0u32, 2146500u32);
    emu.adi_no_count(23usize, 11usize, 0u32, 2146504u32);
    emu.add_memory_rw_events(29usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2146360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c038));
    } else {
        emu.pc = 2146508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c0cc));
    }
}
#[inline(always)]
pub fn block_0x0020c0cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(12usize, 10usize, 10u32, 2146512u32);
    emu.sltiu_no_count(13usize, 11usize, 1u32, 2146516u32);
    emu.anr_no_count(12usize, 13usize, 12usize, 2146520u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2146652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c15c));
    } else {
        emu.pc = 2146524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c0dc));
    }
}
#[inline]
pub fn block_0x0020c0dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2146528u32);
    emu.sli_no_count(12usize, 10usize, 16u32, 2146532u32);
    let a = 0u32.wrapping_add(4096u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2146536u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 0usize, 100u32, 2146540u32);
    emu.lw_no_count(16usize, 2usize, 8u32, 2146544u32)?;
    emu.adr_no_count(15usize, 16usize, 19usize, 2146548u32);
    emu.sri_no_count(12usize, 12usize, 18u32, 2146552u32);
    emu.adi_no_count(13usize, 13usize, 1147u32, 2146556u32);
    emu.mul_no_count(12usize, 12usize, 13usize, 2146560u32);
    emu.sri_no_count(12usize, 12usize, 17u32, 2146564u32);
    emu.mul_no_count(13usize, 12usize, 14usize, 2146568u32);
    emu.sbr_no_count(10usize, 10usize, 13usize, 2146572u32);
    emu.sli_no_count(10usize, 10usize, 17u32, 2146576u32);
    emu.sri_no_count(10usize, 10usize, 16u32, 2146580u32);
    emu.adr_no_count(10usize, 24usize, 10usize, 2146584u32);
    emu.lbu_no_count(13usize, 10usize, 0u32, 2146588u32);
    emu.lbu_no_count(10usize, 10usize, 1u32, 2146592u32);
    emu.adi_no_count(19usize, 19usize, 4294967294u32, 2146596u32);
    emu.adr_no_count(14usize, 16usize, 19usize, 2146600u32);
    emu.sb_no_count(13usize, 14usize, 0u32, 2146604u32);
    emu.sb_no_count(10usize, 15usize, 4294967295u32, 2146608u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2146612u32);
    emu.orr_no_count(12usize, 18usize, 9usize, 2146616u32);
    emu.add_memory_rw_events(23usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2146664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c168));
    } else {
        emu.pc = 2146620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c13c));
    }
}
#[inline(always)]
pub fn block_0x0020c13c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2146624u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2146672u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c170));
}
#[inline(always)]
pub fn block_0x0020c140(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2146628u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2146632u32);
    emu.adi_no_count(19usize, 21usize, 0u32, 2146636u32);
    emu.sltiu_no_count(12usize, 18usize, 10u32, 2146640u32);
    emu.sltiu_no_count(13usize, 9usize, 1u32, 2146644u32);
    emu.anr_no_count(12usize, 13usize, 12usize, 2146648u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2146524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c0dc));
    } else {
        emu.pc = 2146652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c15c));
    }
}
#[inline(always)]
pub fn block_0x0020c15c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 2usize, 8u32, 2146656u32)?;
    emu.orr_no_count(12usize, 18usize, 9usize, 2146660u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2146672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c170));
    } else {
        emu.pc = 2146664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c168));
    }
}
#[inline(always)]
pub fn block_0x0020c168(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.orr_no_count(11usize, 10usize, 11usize, 2146668u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2146700u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c18c));
    } else {
        emu.pc = 2146672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c170));
    }
}
#[inline(always)]
pub fn block_0x0020c170(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 10usize, 1u32, 2146676u32);
    emu.ani_no_count(10usize, 10usize, 30u32, 2146680u32);
    emu.adr_no_count(10usize, 24usize, 10usize, 2146684u32);
    emu.lbu_no_count(10usize, 10usize, 1u32, 2146688u32);
    emu.adi_no_count(19usize, 19usize, 4294967295u32, 2146692u32);
    emu.adr_no_count(11usize, 16usize, 19usize, 2146696u32);
    emu.sb_no_count(10usize, 11usize, 0u32, 2146700u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2146700u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c18c));
}
#[inline]
pub fn block_0x0020c18c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 19usize, 0u32, 2146704u32);
    emu.lw_no_count(1usize, 2usize, 60u32, 2146708u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2146712u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2146716u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2146720u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2146724u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2146728u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2146732u32)?;
    emu.lw_no_count(22usize, 2usize, 32u32, 2146736u32)?;
    emu.lw_no_count(23usize, 2usize, 28u32, 2146740u32)?;
    emu.lw_no_count(24usize, 2usize, 24u32, 2146744u32)?;
    emu.lw_no_count(25usize, 2usize, 20u32, 2146748u32)?;
    emu.lw_no_count(26usize, 2usize, 16u32, 2146752u32)?;
    emu.lw_no_count(27usize, 2usize, 12u32, 2146756u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2146760u32);
    emu.add_memory_rw_events(16usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2146764u32;
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
pub fn block_0x0020c1cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2146768u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2146772u32);
    emu.sw_no_count(10usize, 2usize, 4u32, 2146776u32)?;
    emu.sw_no_count(11usize, 2usize, 8u32, 2146780u32)?;
    emu.sh_no_count(12usize, 2usize, 12u32, 2146784u32)?;
    emu.adi_no_count(10usize, 2usize, 4u32, 2146788u32);
    emu.apc_no_count(1usize, 2146788u32, 4294959104u32, 2146792u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2146796u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966128u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020c1ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2146800u32);
    emu.sw_no_count(10usize, 2usize, 24u32, 2146804u32)?;
    emu.sw_no_count(11usize, 2usize, 28u32, 2146808u32)?;
    emu.adi_no_count(10usize, 2usize, 24u32, 2146812u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2146816u32);
    emu.sw_no_count(0usize, 2usize, 16u32, 2146820u32)?;
    emu.adi_no_count(13usize, 0usize, 4u32, 2146824u32);
    emu.sw_no_count(10usize, 2usize, 0u32, 2146828u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2146832u32)?;
    emu.sw_no_count(13usize, 2usize, 8u32, 2146836u32)?;
    emu.sw_no_count(0usize, 2usize, 12u32, 2146840u32)?;
    emu.adi_no_count(10usize, 2usize, 0u32, 2146844u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2146848u32);
    emu.apc_no_count(1usize, 2146848u32, 0u32, 2146852u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2146856u32;
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
#[inline]
pub fn block_0x0020c228(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2146860u32);
    emu.sw_no_count(10usize, 2usize, 0u32, 2146864u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2146868u32)?;
    emu.adi_no_count(10usize, 2usize, 4u32, 2146872u32);
    let a = 0u32.wrapping_add(2146304u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2146876u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965756u32, 2146880u32);
    emu.adi_no_count(13usize, 2usize, 0u32, 2146884u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2146888u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294967052u32, 2146892u32);
    emu.adi_no_count(15usize, 0usize, 2u32, 2146896u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2146900u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2146904u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2146908u32)?;
    emu.sw_no_count(13usize, 2usize, 40u32, 2146912u32)?;
    emu.sw_no_count(11usize, 2usize, 44u32, 2146916u32)?;
    emu.adi_no_count(10usize, 2usize, 32u32, 2146920u32);
    emu.sw_no_count(14usize, 2usize, 8u32, 2146924u32)?;
    emu.sw_no_count(15usize, 2usize, 12u32, 2146928u32)?;
    emu.sw_no_count(10usize, 2usize, 16u32, 2146932u32)?;
    emu.sw_no_count(15usize, 2usize, 20u32, 2146936u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2146940u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2146944u32);
    emu.apc_no_count(1usize, 2146944u32, 0u32, 2146948u32);
    emu.add_memory_rw_events(24usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2146952u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967116u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020c288(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2146956u32);
    emu.adi_no_count(16usize, 14usize, 0u32, 2146960u32);
    emu.adi_no_count(15usize, 13usize, 0u32, 2146964u32);
    emu.sw_no_count(11usize, 2usize, 8u32, 2146968u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2146972u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2146976u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967068u32, 2146980u32);
    emu.adi_no_count(11usize, 2usize, 8u32, 2146984u32);
    emu.adi_no_count(13usize, 2usize, 12u32, 2146988u32);
    emu.adi_no_count(14usize, 12usize, 0u32, 2146992u32);
    emu.apc_no_count(1usize, 2146992u32, 0u32, 2146996u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2147000u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x0020c2b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 22u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967168u32, 2147004u32);
    emu.sw_no_count(1usize, 2usize, 124u32, 2147008u32)?;
    emu.sw_no_count(8usize, 2usize, 120u32, 2147012u32)?;
    emu.sw_no_count(9usize, 2usize, 116u32, 2147016u32)?;
    emu.adi_no_count(8usize, 16usize, 0u32, 2147020u32);
    emu.sw_no_count(11usize, 2usize, 12u32, 2147024u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2147028u32)?;
    emu.sli_no_count(10usize, 10usize, 2u32, 2147032u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2147036u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 96u32, 2147040u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2147044u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 108u32, 2147048u32);
    emu.adr_no_count(11usize, 11usize, 10usize, 2147052u32);
    emu.adr_no_count(10usize, 12usize, 10usize, 2147056u32);
    emu.lw_no_count(12usize, 15usize, 0u32, 2147060u32)?;
    emu.lw_no_count(11usize, 11usize, 0u32, 2147064u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2147068u32)?;
    emu.sw_no_count(13usize, 2usize, 20u32, 2147072u32)?;
    emu.sw_no_count(14usize, 2usize, 24u32, 2147076u32)?;
    emu.sw_no_count(11usize, 2usize, 28u32, 2147080u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2147084u32)?;
    emu.add_memory_rw_events(21usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2147180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c36c));
    } else {
        emu.pc = 2147088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c310));
    }
}
#[inline]
pub fn block_0x0020c310(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 23u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 28u32, 2147092u32);
    let a = 0u32.wrapping_add(2166784u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2147096u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1832u32, 2147100u32);
    emu.adi_no_count(12usize, 2usize, 12u32, 2147104u32);
    let a = 0u32.wrapping_add(2166784u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2147108u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1816u32, 2147112u32);
    emu.adi_no_count(14usize, 2usize, 20u32, 2147116u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2147120u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 4294967128u32, 2147124u32);
    emu.adi_no_count(16usize, 0usize, 3u32, 2147128u32);
    emu.sw_no_count(0usize, 2usize, 108u32, 2147132u32)?;
    emu.sw_no_count(10usize, 2usize, 60u32, 2147136u32)?;
    emu.sw_no_count(11usize, 2usize, 64u32, 2147140u32)?;
    emu.sw_no_count(12usize, 2usize, 68u32, 2147144u32)?;
    emu.sw_no_count(13usize, 2usize, 72u32, 2147148u32)?;
    emu.sw_no_count(14usize, 2usize, 76u32, 2147152u32)?;
    emu.sw_no_count(13usize, 2usize, 80u32, 2147156u32)?;
    emu.adi_no_count(10usize, 2usize, 60u32, 2147160u32);
    emu.sw_no_count(15usize, 2usize, 92u32, 2147164u32)?;
    emu.sw_no_count(16usize, 2usize, 96u32, 2147168u32)?;
    emu.sw_no_count(10usize, 2usize, 100u32, 2147172u32)?;
    emu.sw_no_count(16usize, 2usize, 104u32, 2147176u32)?;
    emu.add_memory_rw_events(23usize);
    let return_addr = 2147180u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2147308u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c3ec));
}
#[inline(always)]
pub fn block_0x0020c36c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 36u32, 2147184u32);
    emu.adi_no_count(12usize, 0usize, 24u32, 2147188u32);
    emu.adi_no_count(9usize, 2usize, 36u32, 2147192u32);
    emu.adi_no_count(11usize, 15usize, 0u32, 2147196u32);
    emu.apc_no_count(1usize, 2147196u32, 4294930432u32, 2147200u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2147204u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966128u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0020c384(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 26u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 28u32, 2147208u32);
    let a = 0u32.wrapping_add(2166784u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2147212u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1832u32, 2147216u32);
    let a = 0u32.wrapping_add(2162688u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2147220u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966564u32, 2147224u32);
    emu.adi_no_count(13usize, 2usize, 12u32, 2147228u32);
    let a = 0u32.wrapping_add(2166784u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2147232u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1816u32, 2147236u32);
    emu.adi_no_count(15usize, 2usize, 20u32, 2147240u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2147244u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 4294967164u32, 2147248u32);
    emu.sw_no_count(10usize, 2usize, 60u32, 2147252u32)?;
    emu.sw_no_count(11usize, 2usize, 64u32, 2147256u32)?;
    emu.sw_no_count(9usize, 2usize, 68u32, 2147260u32)?;
    emu.sw_no_count(12usize, 2usize, 72u32, 2147264u32)?;
    emu.adi_no_count(10usize, 0usize, 4u32, 2147268u32);
    emu.sw_no_count(0usize, 2usize, 108u32, 2147272u32)?;
    emu.sw_no_count(13usize, 2usize, 76u32, 2147276u32)?;
    emu.sw_no_count(14usize, 2usize, 80u32, 2147280u32)?;
    emu.sw_no_count(15usize, 2usize, 84u32, 2147284u32)?;
    emu.sw_no_count(14usize, 2usize, 88u32, 2147288u32)?;
    emu.adi_no_count(11usize, 2usize, 60u32, 2147292u32);
    emu.sw_no_count(16usize, 2usize, 92u32, 2147296u32)?;
    emu.sw_no_count(10usize, 2usize, 96u32, 2147300u32)?;
    emu.sw_no_count(11usize, 2usize, 100u32, 2147304u32)?;
    emu.sw_no_count(10usize, 2usize, 104u32, 2147308u32)?;
    emu.add_memory_rw_events(26usize);
    emu.pc = 2147308u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c3ec));
}
#[inline(always)]
pub fn block_0x0020c3ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 92u32, 2147312u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2147316u32);
    emu.apc_no_count(1usize, 2147316u32, 0u32, 2147320u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2147324u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966744u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020c3fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2147324u32, 0u32, 2147328u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2147332u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(8u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020c404(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967184u32, 2147336u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2147340u32)?;
    emu.adi_no_count(15usize, 0usize, 257u32, 2147344u32);
    emu.sw_no_count(13usize, 2usize, 12u32, 2147348u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2147372u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c42c));
    } else {
        emu.pc = 2147352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c418));
    }
}
#[inline(always)]
pub fn block_0x0020c418(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 0u32, 2147356u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2147360u32)?;
    emu.sw_no_count(11usize, 2usize, 20u32, 2147364u32)?;
    emu.adi_no_count(15usize, 0usize, 1u32, 2147368u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2147372u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2147440u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c470));
}
#[inline(always)]
pub fn block_0x0020c42c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 256u32, 2147376u32);
    emu.adi_no_count(16usize, 0usize, 4294967231u32, 2147380u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2147380u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c434));
}
#[inline(always)]
pub fn block_0x0020c434(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(17usize, 10usize, 15usize, 2147384u32);
    emu.lb_no_count(17usize, 17usize, 0u32, 2147388u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(16usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2147400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c448));
    } else {
        emu.pc = 2147392u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c440));
    }
}
#[inline(always)]
pub fn block_0x0020c440(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 15usize, 4294967295u32, 2147396u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a != b {
        emu.pc = 2147380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c434));
    } else {
        emu.pc = 2147400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c448));
    }
}
#[inline(always)]
pub fn block_0x0020c448(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 16u32, 2147404u32)?;
    emu.sw_no_count(15usize, 2usize, 20u32, 2147408u32)?;
    emu.sltru_no_count(16usize, 15usize, 11usize, 2147412u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a < b {
        emu.pc = 2147424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c460));
    } else {
        emu.pc = 2147416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c458));
    }
}
#[inline(always)]
pub fn block_0x0020c458(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 1u32, 2147420u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2147424u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2147432u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c468));
}
#[inline(always)]
pub fn block_0x0020c460(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2147428u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 4294967196u32, 2147432u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2147432u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c468));
}
#[inline(always)]
pub fn block_0x0020c468(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(16usize, 0usize, 16usize, 2147436u32);
    emu.ani_no_count(16usize, 16usize, 5u32, 2147440u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2147440u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c470));
}
#[inline(always)]
pub fn block_0x0020c470(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(15usize, 2usize, 24u32, 2147444u32)?;
    emu.sw_no_count(16usize, 2usize, 28u32, 2147448u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2147888u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c630));
    } else {
        emu.pc = 2147452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c47c));
    }
}
#[inline(always)]
pub fn block_0x0020c47c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2147884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c62c));
    } else {
        emu.pc = 2147456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c480));
    }
}
#[inline(always)]
pub fn block_0x0020c480(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2147996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c69c));
    } else {
        emu.pc = 2147460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c484));
    }
}
#[inline(always)]
pub fn block_0x0020c484(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2147488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c4a0));
    } else {
        emu.pc = 2147464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c488));
    }
}
#[inline(always)]
pub fn block_0x0020c488(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a >= b {
        emu.pc = 2147488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c4a0));
    } else {
        emu.pc = 2147468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c48c));
    }
}
#[inline(always)]
pub fn block_0x0020c48c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(15usize, 10usize, 12usize, 2147472u32);
    emu.lb_no_count(15usize, 15usize, 0u32, 2147476u32);
    emu.adi_no_count(16usize, 0usize, 4294967231u32, 2147480u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(16usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2147488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c4a0));
    } else {
        emu.pc = 2147484u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c49c));
    }
}
#[inline(always)]
pub fn block_0x0020c49c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 12usize, 0u32, 2147488u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2147488u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c4a0));
}
#[inline(always)]
pub fn block_0x0020c4a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(13usize, 2usize, 32u32, 2147492u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2147528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c4c8));
    } else {
        emu.pc = 2147496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c4a8));
    }
}
#[inline(always)]
pub fn block_0x0020c4a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2147524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c4c4));
    } else {
        emu.pc = 2147500u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c4ac));
    }
}
#[inline(always)]
pub fn block_0x0020c4ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 4294967231u32, 2147504u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2147504u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c4b0));
}
#[inline(always)]
pub fn block_0x0020c4b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(15usize, 10usize, 13usize, 2147508u32);
    emu.lb_no_count(15usize, 15usize, 0u32, 2147512u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2147524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c4c4));
    } else {
        emu.pc = 2147516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c4bc));
    }
}
#[inline(always)]
pub fn block_0x0020c4bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2147520u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2147504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c4b0));
    } else {
        emu.pc = 2147524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c4c4));
    }
}
#[inline(always)]
pub fn block_0x0020c4c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2147540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c4d4));
    } else {
        emu.pc = 2147528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c4c8));
    }
}
#[inline(always)]
pub fn block_0x0020c4c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 14usize, 0u32, 2147532u32);
    emu.apc_no_count(1usize, 2147532u32, 8192u32, 2147536u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2147540u32;
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
pub fn block_0x0020c4d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 10usize, 13usize, 2147544u32);
    emu.lb_no_count(12usize, 10usize, 0u32, 2147548u32);
    emu.ani_no_count(11usize, 12usize, 255u32, 2147552u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2147568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c4f0));
    } else {
        emu.pc = 2147556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c4e4));
    }
}
#[inline(always)]
pub fn block_0x0020c4e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 2usize, 36u32, 2147560u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2147564u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2147568u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2147728u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c590));
}
#[inline(always)]
pub fn block_0x0020c4f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(15usize, 10usize, 1u32, 2147572u32);
    emu.ani_no_count(12usize, 11usize, 31u32, 2147576u32);
    emu.adi_no_count(16usize, 0usize, 223u32, 2147580u32);
    emu.ani_no_count(15usize, 15usize, 63u32, 2147584u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a >= b {
        emu.pc = 2147652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c544));
    } else {
        emu.pc = 2147588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c504));
    }
}
#[inline(always)]
pub fn block_0x0020c504(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(16usize, 10usize, 2u32, 2147592u32);
    emu.sli_no_count(15usize, 15usize, 6u32, 2147596u32);
    emu.ani_no_count(16usize, 16usize, 63u32, 2147600u32);
    emu.adi_no_count(17usize, 0usize, 240u32, 2147604u32);
    emu.orr_no_count(15usize, 15usize, 16usize, 2147608u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2147680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c560));
    } else {
        emu.pc = 2147612u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c51c));
    }
}
#[inline]
pub fn block_0x0020c51c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 10usize, 3u32, 2147616u32);
    emu.sli_no_count(12usize, 12usize, 29u32, 2147620u32);
    emu.sli_no_count(15usize, 15usize, 6u32, 2147624u32);
    emu.sri_no_count(12usize, 12usize, 11u32, 2147628u32);
    emu.ani_no_count(10usize, 10usize, 63u32, 2147632u32);
    emu.orr_no_count(10usize, 15usize, 10usize, 2147636u32);
    emu.orr_no_count(10usize, 10usize, 12usize, 2147640u32);
    let a = 0u32.wrapping_add(1114112u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2147644u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2147528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c4c8));
    } else {
        emu.pc = 2147648u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c540));
    }
}
#[inline(always)]
pub fn block_0x0020c540(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2147652u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2147660u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c54c));
}
#[inline(always)]
pub fn block_0x0020c544(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 12usize, 6u32, 2147656u32);
    emu.orr_no_count(10usize, 10usize, 15usize, 2147660u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2147660u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c54c));
}
#[inline(always)]
pub fn block_0x0020c54c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 128u32, 2147664u32);
    emu.sw_no_count(10usize, 2usize, 36u32, 2147668u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2147700u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c574));
    } else {
        emu.pc = 2147672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c558));
    }
}
#[inline(always)]
pub fn block_0x0020c558(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2147676u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2147680u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2147728u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c590));
}
#[inline(always)]
pub fn block_0x0020c560(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 12usize, 12u32, 2147684u32);
    emu.orr_no_count(10usize, 15usize, 10usize, 2147688u32);
    emu.adi_no_count(11usize, 0usize, 128u32, 2147692u32);
    emu.sw_no_count(10usize, 2usize, 36u32, 2147696u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2147672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c558));
    } else {
        emu.pc = 2147700u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c574));
    }
}
#[inline(always)]
pub fn block_0x0020c574(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 10usize, 11u32, 2147704u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2147716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c584));
    } else {
        emu.pc = 2147708u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c57c));
    }
}
#[inline(always)]
pub fn block_0x0020c57c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2147712u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2147716u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2147728u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c590));
}
#[inline(always)]
pub fn block_0x0020c584(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 10usize, 16u32, 2147720u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2147724u32);
    emu.adi_no_count(10usize, 10usize, 3u32, 2147728u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2147728u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c590));
}
#[inline(never)]
pub fn block_0x0020c590(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 39u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 10usize, 13usize, 2147732u32);
    emu.adi_no_count(11usize, 2usize, 32u32, 2147736u32);
    let a = 0u32.wrapping_add(2146304u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2147740u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965756u32, 2147744u32);
    emu.adi_no_count(15usize, 2usize, 36u32, 2147748u32);
    let a = 0u32.wrapping_add(2166784u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2147752u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 972u32, 2147756u32);
    emu.adi_no_count(17usize, 2usize, 40u32, 2147760u32);
    let a = 0u32.wrapping_add(2150400u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2147764u32;
    emu.update_insn_clock();
    emu.adi_no_count(5usize, 5usize, 968u32, 2147768u32);
    emu.adi_no_count(6usize, 2usize, 16u32, 2147772u32);
    let a = 0u32.wrapping_add(2166784u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2147776u32;
    emu.update_insn_clock();
    emu.adi_no_count(7usize, 7usize, 1832u32, 2147780u32);
    emu.adi_no_count(28usize, 2usize, 24u32, 2147784u32);
    emu.sw_no_count(13usize, 2usize, 40u32, 2147788u32)?;
    emu.sw_no_count(10usize, 2usize, 44u32, 2147792u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2147796u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 8u32, 2147800u32);
    emu.sw_no_count(11usize, 2usize, 72u32, 2147804u32)?;
    emu.sw_no_count(12usize, 2usize, 76u32, 2147808u32)?;
    emu.sw_no_count(15usize, 2usize, 80u32, 2147812u32)?;
    emu.sw_no_count(16usize, 2usize, 84u32, 2147816u32)?;
    emu.adi_no_count(11usize, 0usize, 5u32, 2147820u32);
    emu.sw_no_count(0usize, 2usize, 64u32, 2147824u32)?;
    emu.sw_no_count(17usize, 2usize, 88u32, 2147828u32)?;
    emu.sw_no_count(5usize, 2usize, 92u32, 2147832u32)?;
    emu.sw_no_count(6usize, 2usize, 96u32, 2147836u32)?;
    emu.sw_no_count(7usize, 2usize, 100u32, 2147840u32)?;
    emu.sw_no_count(28usize, 2usize, 104u32, 2147844u32)?;
    emu.sw_no_count(7usize, 2usize, 108u32, 2147848u32)?;
    emu.adi_no_count(12usize, 2usize, 72u32, 2147852u32);
    emu.sw_no_count(10usize, 2usize, 48u32, 2147856u32)?;
    emu.sw_no_count(11usize, 2usize, 52u32, 2147860u32)?;
    emu.sw_no_count(12usize, 2usize, 56u32, 2147864u32)?;
    emu.sw_no_count(11usize, 2usize, 60u32, 2147868u32)?;
    emu.adi_no_count(10usize, 2usize, 48u32, 2147872u32);
    emu.adi_no_count(11usize, 14usize, 0u32, 2147876u32);
    emu.apc_no_count(1usize, 2147876u32, 0u32, 2147880u32);
    emu.add_memory_rw_events(39usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2147884u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966184u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020c62c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 13usize, 0u32, 2147888u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2147888u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c630));
}
#[inline(never)]
pub fn block_0x0020c630(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(12usize, 2usize, 40u32, 2147892u32)?;
    emu.adi_no_count(10usize, 2usize, 40u32, 2147896u32);
    let a = 0u32.wrapping_add(2146304u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2147900u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965756u32, 2147904u32);
    emu.adi_no_count(12usize, 2usize, 16u32, 2147908u32);
    let a = 0u32.wrapping_add(2166784u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2147912u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1832u32, 2147916u32);
    emu.adi_no_count(15usize, 2usize, 24u32, 2147920u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2147924u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 72u32, 2147928u32);
    emu.adi_no_count(17usize, 0usize, 3u32, 2147932u32);
    emu.sw_no_count(0usize, 2usize, 64u32, 2147936u32)?;
    emu.sw_no_count(10usize, 2usize, 72u32, 2147940u32)?;
    emu.sw_no_count(11usize, 2usize, 76u32, 2147944u32)?;
    emu.sw_no_count(12usize, 2usize, 80u32, 2147948u32)?;
    emu.sw_no_count(13usize, 2usize, 84u32, 2147952u32)?;
    emu.sw_no_count(15usize, 2usize, 88u32, 2147956u32)?;
    emu.sw_no_count(13usize, 2usize, 92u32, 2147960u32)?;
    emu.adi_no_count(10usize, 2usize, 72u32, 2147964u32);
    emu.sw_no_count(16usize, 2usize, 48u32, 2147968u32)?;
    emu.sw_no_count(17usize, 2usize, 52u32, 2147972u32)?;
    emu.sw_no_count(10usize, 2usize, 56u32, 2147976u32)?;
    emu.sw_no_count(17usize, 2usize, 60u32, 2147980u32)?;
    emu.adi_no_count(10usize, 2usize, 48u32, 2147984u32);
    emu.adi_no_count(11usize, 14usize, 0u32, 2147988u32);
    emu.apc_no_count(1usize, 2147988u32, 0u32, 2147992u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2147996u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966072u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0020c69c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 29u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 8u32, 2148000u32);
    let a = 0u32.wrapping_add(2146304u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2148004u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965756u32, 2148008u32);
    emu.adi_no_count(12usize, 2usize, 12u32, 2148012u32);
    emu.adi_no_count(13usize, 2usize, 16u32, 2148016u32);
    let a = 0u32.wrapping_add(2166784u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2148020u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 1832u32, 2148024u32);
    emu.adi_no_count(16usize, 2usize, 24u32, 2148028u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2148032u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 17usize, 4294967216u32, 2148036u32);
    emu.sw_no_count(10usize, 2usize, 72u32, 2148040u32)?;
    emu.sw_no_count(11usize, 2usize, 76u32, 2148044u32)?;
    emu.sw_no_count(12usize, 2usize, 80u32, 2148048u32)?;
    emu.sw_no_count(11usize, 2usize, 84u32, 2148052u32)?;
    emu.adi_no_count(10usize, 0usize, 4u32, 2148056u32);
    emu.sw_no_count(0usize, 2usize, 64u32, 2148060u32)?;
    emu.sw_no_count(13usize, 2usize, 88u32, 2148064u32)?;
    emu.sw_no_count(15usize, 2usize, 92u32, 2148068u32)?;
    emu.sw_no_count(16usize, 2usize, 96u32, 2148072u32)?;
    emu.sw_no_count(15usize, 2usize, 100u32, 2148076u32)?;
    emu.adi_no_count(11usize, 2usize, 72u32, 2148080u32);
    emu.sw_no_count(17usize, 2usize, 48u32, 2148084u32)?;
    emu.sw_no_count(10usize, 2usize, 52u32, 2148088u32)?;
    emu.sw_no_count(11usize, 2usize, 56u32, 2148092u32)?;
    emu.sw_no_count(10usize, 2usize, 60u32, 2148096u32)?;
    emu.adi_no_count(10usize, 2usize, 48u32, 2148100u32);
    emu.adi_no_count(11usize, 14usize, 0u32, 2148104u32);
    emu.apc_no_count(1usize, 2148104u32, 0u32, 2148108u32);
    emu.add_memory_rw_events(29usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2148112u32;
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
