pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2145156u32;
pub const PC_MAX: u32 = 2148400u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 107usize] = [
        block_0x0020bb84,
        block_0x0020bb90,
        block_0x0020bbb8,
        block_0x0020bbe4,
        block_0x0020bc54,
        block_0x0020bc5c,
        block_0x0020bcb0,
        block_0x0020bcb4,
        block_0x0020bcb8,
        block_0x0020bcd8,
        block_0x0020bd00,
        block_0x0020bd10,
        block_0x0020bd1c,
        block_0x0020bd20,
        block_0x0020bd50,
        block_0x0020bd7c,
        block_0x0020bdec,
        block_0x0020bdf4,
        block_0x0020be48,
        block_0x0020be4c,
        block_0x0020be50,
        block_0x0020be70,
        block_0x0020be9c,
        block_0x0020beac,
        block_0x0020beb8,
        block_0x0020bebc,
        block_0x0020bed8,
        block_0x0020bf04,
        block_0x0020bf78,
        block_0x0020bf80,
        block_0x0020bfd0,
        block_0x0020bfd4,
        block_0x0020bfd8,
        block_0x0020bfec,
        block_0x0020c01c,
        block_0x0020c02c,
        block_0x0020c030,
        block_0x0020c070,
        block_0x0020c090,
        block_0x0020c0a8,
        block_0x0020c0d0,
        block_0x0020c0d8,
        block_0x0020c0e8,
        block_0x0020c108,
        block_0x0020c128,
        block_0x0020c144,
        block_0x0020c1a4,
        block_0x0020c1cc,
        block_0x0020c1e8,
        block_0x0020c260,
        block_0x0020c270,
        block_0x0020c2d0,
        block_0x0020c2d4,
        block_0x0020c2f0,
        block_0x0020c2fc,
        block_0x0020c304,
        block_0x0020c320,
        block_0x0020c360,
        block_0x0020c380,
        block_0x0020c3bc,
        block_0x0020c41c,
        block_0x0020c44c,
        block_0x0020c4a4,
        block_0x0020c500,
        block_0x0020c518,
        block_0x0020c580,
        block_0x0020c590,
        block_0x0020c598,
        block_0x0020c5ac,
        block_0x0020c5c0,
        block_0x0020c5c8,
        block_0x0020c5d4,
        block_0x0020c5dc,
        block_0x0020c5ec,
        block_0x0020c5f4,
        block_0x0020c5fc,
        block_0x0020c604,
        block_0x0020c610,
        block_0x0020c614,
        block_0x0020c618,
        block_0x0020c61c,
        block_0x0020c620,
        block_0x0020c630,
        block_0x0020c634,
        block_0x0020c63c,
        block_0x0020c640,
        block_0x0020c644,
        block_0x0020c650,
        block_0x0020c658,
        block_0x0020c65c,
        block_0x0020c668,
        block_0x0020c678,
        block_0x0020c684,
        block_0x0020c698,
        block_0x0020c6b0,
        block_0x0020c6d4,
        block_0x0020c6d8,
        block_0x0020c6e0,
        block_0x0020c6ec,
        block_0x0020c6f4,
        block_0x0020c708,
        block_0x0020c710,
        block_0x0020c718,
        block_0x0020c724,
        block_0x0020c7c0,
        block_0x0020c7c4,
        block_0x0020c830,
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
    if pc < 2145156u32 || pc > 2148400u32 {
        return None;
    }
    let word_offset = ((pc - 2145156u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x0020bb84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2145160u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2145164u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2145168u32;
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
pub fn block_0x0020bb90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2145172u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2145176u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2145180u32)?;
    emu.adi_no_count(13usize, 11usize, 0u32, 2145184u32);
    emu.lw_no_count(11usize, 10usize, 0u32, 2145188u32)?;
    emu.adi_no_count(12usize, 0usize, 1000u32, 2145192u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2145196u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 98u32, 2145200u32);
    emu.adi_no_count(14usize, 0usize, 10u32, 2145204u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2145552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bd10));
    } else {
        emu.pc = 2145208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bbb8));
    }
}
#[inline]
pub fn block_0x0020bbb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 2usize, 23u32, 2145212u32);
    let a = 0u32.wrapping_add(3518435328u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2145216u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2145220u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4096u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2145224u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 0usize, 100u32, 2145228u32);
    let a = 0u32.wrapping_add(9998336u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2145232u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 12usize, 1881u32, 2145236u32);
    emu.adi_no_count(5usize, 5usize, 1808u32, 2145240u32);
    emu.adi_no_count(6usize, 6usize, 1147u32, 2145244u32);
    emu.adi_no_count(7usize, 7usize, 1663u32, 2145248u32);
    emu.adi_no_count(12usize, 11usize, 0u32, 2145252u32);
    emu.add_memory_rw_events(11usize);
    emu.pc = 2145252u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bbe4));
}
#[inline(never)]
pub fn block_0x0020bbe4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(28usize, 12usize, 0u32, 2145256u32);
    emu.adi_no_count(14usize, 14usize, 4294967292u32, 2145260u32);
    emu.mulhu_no_count(12usize, 12usize, 17usize, 2145264u32);
    emu.sri_no_count(12usize, 12usize, 13u32, 2145268u32);
    emu.mul_no_count(29usize, 12usize, 5usize, 2145272u32);
    emu.sbr_no_count(29usize, 28usize, 29usize, 2145276u32);
    emu.sli_no_count(30usize, 29usize, 16u32, 2145280u32);
    emu.sri_no_count(30usize, 30usize, 18u32, 2145284u32);
    emu.mul_no_count(30usize, 30usize, 6usize, 2145288u32);
    emu.sri_no_count(31usize, 30usize, 16u32, 2145292u32);
    emu.sri_no_count(30usize, 30usize, 17u32, 2145296u32);
    emu.mul_no_count(30usize, 30usize, 16usize, 2145300u32);
    emu.ani_no_count(31usize, 31usize, 2046u32, 2145304u32);
    emu.sbr_no_count(29usize, 29usize, 30usize, 2145308u32);
    emu.adr_no_count(31usize, 10usize, 31usize, 2145312u32);
    emu.sli_no_count(29usize, 29usize, 17u32, 2145316u32);
    emu.sri_no_count(29usize, 29usize, 16u32, 2145320u32);
    emu.adr_no_count(29usize, 10usize, 29usize, 2145324u32);
    emu.lbu_no_count(30usize, 31usize, 0u32, 2145328u32);
    emu.lbu_no_count(31usize, 31usize, 1u32, 2145332u32);
    emu.lbu_no_count(8usize, 29usize, 0u32, 2145336u32);
    emu.lbu_no_count(29usize, 29usize, 1u32, 2145340u32);
    emu.sb_no_count(30usize, 15usize, 4294967293u32, 2145344u32);
    emu.sb_no_count(31usize, 15usize, 4294967294u32, 2145348u32);
    emu.sb_no_count(8usize, 15usize, 4294967295u32, 2145352u32);
    emu.sb_no_count(29usize, 15usize, 0u32, 2145356u32);
    emu.adi_no_count(15usize, 15usize, 4294967292u32, 2145360u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a < b {
        emu.pc = 2145252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bbe4));
    } else {
        emu.pc = 2145364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bc54));
    }
}
#[inline(always)]
pub fn block_0x0020bc54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 9u32, 2145368u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2145456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bcb0));
    } else {
        emu.pc = 2145372u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bc5c));
    }
}
#[inline]
pub fn block_0x0020bc5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(15usize, 12usize, 16u32, 2145376u32);
    let a = 0u32.wrapping_add(4096u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2145380u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 0usize, 100u32, 2145384u32);
    emu.adi_no_count(5usize, 2usize, 14u32, 2145388u32);
    emu.sri_no_count(15usize, 15usize, 18u32, 2145392u32);
    emu.adi_no_count(16usize, 16usize, 1147u32, 2145396u32);
    emu.adr_no_count(6usize, 5usize, 14usize, 2145400u32);
    emu.mul_no_count(15usize, 15usize, 16usize, 2145404u32);
    emu.sri_no_count(15usize, 15usize, 17u32, 2145408u32);
    emu.mul_no_count(16usize, 15usize, 17usize, 2145412u32);
    emu.sbr_no_count(12usize, 12usize, 16usize, 2145416u32);
    emu.sli_no_count(12usize, 12usize, 17u32, 2145420u32);
    emu.sri_no_count(12usize, 12usize, 16u32, 2145424u32);
    emu.adr_no_count(12usize, 10usize, 12usize, 2145428u32);
    emu.lbu_no_count(16usize, 12usize, 0u32, 2145432u32);
    emu.lbu_no_count(12usize, 12usize, 1u32, 2145436u32);
    emu.adi_no_count(14usize, 14usize, 4294967294u32, 2145440u32);
    emu.adr_no_count(5usize, 5usize, 14usize, 2145444u32);
    emu.sb_no_count(16usize, 5usize, 0u32, 2145448u32);
    emu.sb_no_count(12usize, 6usize, 4294967295u32, 2145452u32);
    emu.adi_no_count(12usize, 15usize, 0u32, 2145456u32);
    emu.add_memory_rw_events(21usize);
    emu.pc = 2145456u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bcb0));
}
#[inline(always)]
pub fn block_0x0020bcb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2145464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bcb8));
    } else {
        emu.pc = 2145460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bcb4));
    }
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
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2145496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bcd8));
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
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 1u32, 2145468u32);
    emu.ani_no_count(12usize, 12usize, 30u32, 2145472u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2145476u32);
    emu.lbu_no_count(10usize, 10usize, 1u32, 2145480u32);
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2145484u32);
    emu.adi_no_count(11usize, 2usize, 14u32, 2145488u32);
    emu.adr_no_count(11usize, 11usize, 14usize, 2145492u32);
    emu.sb_no_count(10usize, 11usize, 0u32, 2145496u32);
    emu.add_memory_rw_events(8usize);
    emu.pc = 2145496u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bcd8));
}
#[inline]
pub fn block_0x0020bcd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 10u32, 2145500u32);
    emu.adi_no_count(10usize, 2usize, 14u32, 2145504u32);
    emu.sbr_no_count(15usize, 15usize, 14usize, 2145508u32);
    emu.adr_no_count(14usize, 10usize, 14usize, 2145512u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2145516u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2145520u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2145524u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2145528u32);
    emu.apc_no_count(1usize, 2145528u32, 16384u32, 2145532u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2145536u32;
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
pub fn block_0x0020bd00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2145540u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2145544u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2145548u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2145552u32;
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
pub fn block_0x0020bd10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 11usize, 0u32, 2145556u32);
    emu.adi_no_count(15usize, 0usize, 9u32, 2145560u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a < b {
        emu.pc = 2145372u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bc5c));
    } else {
        emu.pc = 2145564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bd1c));
    }
}
#[inline(always)]
pub fn block_0x0020bd1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2145568u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2145456u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bcb0));
}
#[inline]
pub fn block_0x0020bd20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2145572u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2145576u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2145580u32)?;
    emu.adi_no_count(13usize, 11usize, 0u32, 2145584u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2145588u32)?;
    emu.sai_no_count(11usize, 10usize, 1055u32, 2145592u32);
    emu.xrr_no_count(12usize, 10usize, 11usize, 2145596u32);
    emu.sbr_no_count(12usize, 12usize, 11usize, 2145600u32);
    emu.adi_no_count(14usize, 0usize, 1000u32, 2145604u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2145608u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 98u32, 2145612u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2145964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020beac));
    } else {
        emu.pc = 2145616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bd50));
    }
}
#[inline]
pub fn block_0x0020bd50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 10u32, 2145620u32);
    emu.adi_no_count(15usize, 2usize, 23u32, 2145624u32);
    let a = 0u32.wrapping_add(3518435328u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2145628u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2145632u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4096u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2145636u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 0usize, 100u32, 2145640u32);
    let a = 0u32.wrapping_add(9998336u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2145644u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 17usize, 1881u32, 2145648u32);
    emu.adi_no_count(5usize, 5usize, 1808u32, 2145652u32);
    emu.adi_no_count(6usize, 6usize, 1147u32, 2145656u32);
    emu.adi_no_count(7usize, 7usize, 1663u32, 2145660u32);
    emu.add_memory_rw_events(11usize);
    emu.pc = 2145660u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bd7c));
}
#[inline(never)]
pub fn block_0x0020bd7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(28usize, 12usize, 0u32, 2145664u32);
    emu.adi_no_count(14usize, 14usize, 4294967292u32, 2145668u32);
    emu.mulhu_no_count(12usize, 12usize, 17usize, 2145672u32);
    emu.sri_no_count(12usize, 12usize, 13u32, 2145676u32);
    emu.mul_no_count(29usize, 12usize, 5usize, 2145680u32);
    emu.sbr_no_count(29usize, 28usize, 29usize, 2145684u32);
    emu.sli_no_count(30usize, 29usize, 16u32, 2145688u32);
    emu.sri_no_count(30usize, 30usize, 18u32, 2145692u32);
    emu.mul_no_count(30usize, 30usize, 6usize, 2145696u32);
    emu.sri_no_count(31usize, 30usize, 16u32, 2145700u32);
    emu.sri_no_count(30usize, 30usize, 17u32, 2145704u32);
    emu.mul_no_count(30usize, 30usize, 16usize, 2145708u32);
    emu.ani_no_count(31usize, 31usize, 2046u32, 2145712u32);
    emu.sbr_no_count(29usize, 29usize, 30usize, 2145716u32);
    emu.adr_no_count(31usize, 11usize, 31usize, 2145720u32);
    emu.sli_no_count(29usize, 29usize, 17u32, 2145724u32);
    emu.sri_no_count(29usize, 29usize, 16u32, 2145728u32);
    emu.adr_no_count(29usize, 11usize, 29usize, 2145732u32);
    emu.lbu_no_count(30usize, 31usize, 0u32, 2145736u32);
    emu.lbu_no_count(31usize, 31usize, 1u32, 2145740u32);
    emu.lbu_no_count(8usize, 29usize, 0u32, 2145744u32);
    emu.lbu_no_count(29usize, 29usize, 1u32, 2145748u32);
    emu.sb_no_count(30usize, 15usize, 4294967293u32, 2145752u32);
    emu.sb_no_count(31usize, 15usize, 4294967294u32, 2145756u32);
    emu.sb_no_count(8usize, 15usize, 4294967295u32, 2145760u32);
    emu.sb_no_count(29usize, 15usize, 0u32, 2145764u32);
    emu.adi_no_count(15usize, 15usize, 4294967292u32, 2145768u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a < b {
        emu.pc = 2145660u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bd7c));
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
#[inline(always)]
pub fn block_0x0020bdec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 9u32, 2145776u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a >= b {
        emu.pc = 2145864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020be48));
    } else {
        emu.pc = 2145780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bdf4));
    }
}
#[inline]
pub fn block_0x0020bdf4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(15usize, 12usize, 16u32, 2145784u32);
    let a = 0u32.wrapping_add(4096u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2145788u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 0usize, 100u32, 2145792u32);
    emu.adi_no_count(5usize, 2usize, 14u32, 2145796u32);
    emu.sri_no_count(15usize, 15usize, 18u32, 2145800u32);
    emu.adi_no_count(16usize, 16usize, 1147u32, 2145804u32);
    emu.adr_no_count(6usize, 5usize, 14usize, 2145808u32);
    emu.mul_no_count(15usize, 15usize, 16usize, 2145812u32);
    emu.sri_no_count(15usize, 15usize, 17u32, 2145816u32);
    emu.mul_no_count(16usize, 15usize, 17usize, 2145820u32);
    emu.sbr_no_count(12usize, 12usize, 16usize, 2145824u32);
    emu.sli_no_count(12usize, 12usize, 17u32, 2145828u32);
    emu.sri_no_count(12usize, 12usize, 16u32, 2145832u32);
    emu.adr_no_count(12usize, 11usize, 12usize, 2145836u32);
    emu.lbu_no_count(16usize, 12usize, 0u32, 2145840u32);
    emu.lbu_no_count(12usize, 12usize, 1u32, 2145844u32);
    emu.adi_no_count(14usize, 14usize, 4294967294u32, 2145848u32);
    emu.adr_no_count(5usize, 5usize, 14usize, 2145852u32);
    emu.sb_no_count(16usize, 5usize, 0u32, 2145856u32);
    emu.sb_no_count(12usize, 6usize, 4294967295u32, 2145860u32);
    emu.adi_no_count(12usize, 15usize, 0u32, 2145864u32);
    emu.add_memory_rw_events(21usize);
    emu.pc = 2145864u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020be48));
}
#[inline(always)]
pub fn block_0x0020be48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2145872u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020be50));
    } else {
        emu.pc = 2145868u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020be4c));
    }
}
#[inline(always)]
pub fn block_0x0020be4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2145904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020be70));
    } else {
        emu.pc = 2145872u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020be50));
    }
}
#[inline(always)]
pub fn block_0x0020be50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 1u32, 2145876u32);
    emu.ani_no_count(12usize, 12usize, 30u32, 2145880u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2145884u32);
    emu.lbu_no_count(11usize, 11usize, 1u32, 2145888u32);
    emu.adi_no_count(14usize, 14usize, 4294967295u32, 2145892u32);
    emu.adi_no_count(12usize, 2usize, 14u32, 2145896u32);
    emu.adr_no_count(12usize, 12usize, 14usize, 2145900u32);
    emu.sb_no_count(11usize, 12usize, 0u32, 2145904u32);
    emu.add_memory_rw_events(8usize);
    emu.pc = 2145904u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020be70));
}
#[inline]
pub fn block_0x0020be70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 10u32, 2145908u32);
    emu.adi_no_count(11usize, 2usize, 14u32, 2145912u32);
    emu.sbr_no_count(15usize, 15usize, 14usize, 2145916u32);
    emu.adr_no_count(14usize, 11usize, 14usize, 2145920u32);
    emu.xri_no_count(11usize, 10usize, 4294967295u32, 2145924u32);
    emu.sri_no_count(11usize, 11usize, 31u32, 2145928u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2145932u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2145936u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2145940u32);
    emu.apc_no_count(1usize, 2145940u32, 16384u32, 2145944u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2145948u32;
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
pub fn block_0x0020be9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2145952u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2145956u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2145960u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2145964u32;
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
pub fn block_0x0020beac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 0usize, 10u32, 2145968u32);
    emu.adi_no_count(15usize, 0usize, 9u32, 2145972u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a < b {
        emu.pc = 2145780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bdf4));
    } else {
        emu.pc = 2145976u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020beb8));
    }
}
#[inline(always)]
pub fn block_0x0020beb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2145980u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2145864u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020be48));
}
#[inline(always)]
pub fn block_0x0020bebc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2145984u32);
    emu.sw_no_count(8usize, 2usize, 12u32, 2145988u32)?;
    emu.sw_no_count(9usize, 2usize, 8u32, 2145992u32)?;
    emu.adi_no_count(13usize, 0usize, 1000u32, 2145996u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2146000u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 98u32, 2146004u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2146332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c01c));
    } else {
        emu.pc = 2146008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bed8));
    }
}
#[inline]
pub fn block_0x0020bed8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 11usize, 4294967294u32, 2146012u32);
    let a = 0u32.wrapping_add(3518435328u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2146016u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2146020u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4096u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2146024u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 0usize, 100u32, 2146028u32);
    let a = 0u32.wrapping_add(9998336u32);
    emu.write_reg_no_count(28usize, a);
    emu.pc = 2146032u32;
    emu.update_insn_clock();
    emu.adi_no_count(5usize, 13usize, 1881u32, 2146036u32);
    emu.adi_no_count(6usize, 15usize, 1808u32, 2146040u32);
    emu.adi_no_count(7usize, 7usize, 1147u32, 2146044u32);
    emu.adi_no_count(28usize, 28usize, 1663u32, 2146048u32);
    emu.adi_no_count(15usize, 10usize, 0u32, 2146052u32);
    emu.add_memory_rw_events(11usize);
    emu.pc = 2146052u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bf04));
}
#[inline(never)]
pub fn block_0x0020bf04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 29u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(29usize, 15usize, 0u32, 2146056u32);
    emu.adi_no_count(13usize, 12usize, 4294967292u32, 2146060u32);
    emu.mulhu_no_count(15usize, 15usize, 5usize, 2146064u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2146068u32);
    emu.sri_no_count(15usize, 15usize, 13u32, 2146072u32);
    emu.mul_no_count(30usize, 15usize, 6usize, 2146076u32);
    emu.sbr_no_count(30usize, 29usize, 30usize, 2146080u32);
    emu.sli_no_count(31usize, 30usize, 16u32, 2146084u32);
    emu.sri_no_count(31usize, 31usize, 18u32, 2146088u32);
    emu.mul_no_count(31usize, 31usize, 7usize, 2146092u32);
    emu.sri_no_count(8usize, 31usize, 16u32, 2146096u32);
    emu.sri_no_count(31usize, 31usize, 17u32, 2146100u32);
    emu.mul_no_count(31usize, 31usize, 17usize, 2146104u32);
    emu.ani_no_count(8usize, 8usize, 2046u32, 2146108u32);
    emu.sbr_no_count(30usize, 30usize, 31usize, 2146112u32);
    emu.adr_no_count(8usize, 14usize, 8usize, 2146116u32);
    emu.sli_no_count(30usize, 30usize, 17u32, 2146120u32);
    emu.sri_no_count(30usize, 30usize, 16u32, 2146124u32);
    emu.adr_no_count(30usize, 14usize, 30usize, 2146128u32);
    emu.lbu_no_count(31usize, 8usize, 0u32, 2146132u32);
    emu.lbu_no_count(8usize, 8usize, 1u32, 2146136u32);
    emu.lbu_no_count(9usize, 30usize, 0u32, 2146140u32);
    emu.lbu_no_count(30usize, 30usize, 1u32, 2146144u32);
    emu.sb_no_count(31usize, 12usize, 4294967294u32, 2146148u32);
    emu.sb_no_count(8usize, 12usize, 4294967295u32, 2146152u32);
    emu.sb_no_count(9usize, 12usize, 0u32, 2146156u32);
    emu.sb_no_count(30usize, 12usize, 1u32, 2146160u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2146164u32);
    emu.add_memory_rw_events(28usize);
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a < b {
        emu.pc = 2146052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bf04));
    } else {
        emu.pc = 2146168u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bf78));
    }
}
#[inline(always)]
pub fn block_0x0020bf78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 9u32, 2146172u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a >= b {
        emu.pc = 2146256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bfd0));
    } else {
        emu.pc = 2146176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bf80));
    }
}
#[inline]
pub fn block_0x0020bf80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 15usize, 16u32, 2146180u32);
    let a = 0u32.wrapping_add(4096u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2146184u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 0usize, 100u32, 2146188u32);
    emu.adr_no_count(5usize, 11usize, 13usize, 2146192u32);
    emu.sri_no_count(12usize, 12usize, 18u32, 2146196u32);
    emu.adi_no_count(16usize, 16usize, 1147u32, 2146200u32);
    emu.mul_no_count(12usize, 12usize, 16usize, 2146204u32);
    emu.sri_no_count(12usize, 12usize, 17u32, 2146208u32);
    emu.mul_no_count(16usize, 12usize, 17usize, 2146212u32);
    emu.sbr_no_count(15usize, 15usize, 16usize, 2146216u32);
    emu.sli_no_count(15usize, 15usize, 17u32, 2146220u32);
    emu.sri_no_count(15usize, 15usize, 16u32, 2146224u32);
    emu.adr_no_count(15usize, 14usize, 15usize, 2146228u32);
    emu.lbu_no_count(16usize, 15usize, 0u32, 2146232u32);
    emu.lbu_no_count(15usize, 15usize, 1u32, 2146236u32);
    emu.adi_no_count(13usize, 13usize, 4294967294u32, 2146240u32);
    emu.adr_no_count(17usize, 11usize, 13usize, 2146244u32);
    emu.sb_no_count(16usize, 17usize, 0u32, 2146248u32);
    emu.sb_no_count(15usize, 5usize, 4294967295u32, 2146252u32);
    emu.adi_no_count(15usize, 12usize, 0u32, 2146256u32);
    emu.add_memory_rw_events(20usize);
    emu.pc = 2146256u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bfd0));
}
#[inline(always)]
pub fn block_0x0020bfd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2146284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bfec));
    } else {
        emu.pc = 2146260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bfd4));
    }
}
#[inline(always)]
pub fn block_0x0020bfd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2146284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bfec));
    } else {
        emu.pc = 2146264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bfd8));
    }
}
#[inline(always)]
pub fn block_0x0020bfd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 13usize, 0u32, 2146268u32);
    emu.lw_no_count(8usize, 2usize, 12u32, 2146272u32)?;
    emu.lw_no_count(9usize, 2usize, 8u32, 2146276u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2146280u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2146284u32;
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
pub fn block_0x0020bfec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(15usize, 15usize, 1u32, 2146288u32);
    emu.ani_no_count(15usize, 15usize, 30u32, 2146292u32);
    emu.adr_no_count(14usize, 14usize, 15usize, 2146296u32);
    emu.lbu_no_count(10usize, 14usize, 1u32, 2146300u32);
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2146304u32);
    emu.adr_no_count(11usize, 11usize, 13usize, 2146308u32);
    emu.sb_no_count(10usize, 11usize, 0u32, 2146312u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2146316u32);
    emu.lw_no_count(8usize, 2usize, 12u32, 2146320u32)?;
    emu.lw_no_count(9usize, 2usize, 8u32, 2146324u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2146328u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2146332u32;
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
pub fn block_0x0020c01c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 10usize, 0u32, 2146336u32);
    emu.adi_no_count(13usize, 12usize, 0u32, 2146340u32);
    emu.adi_no_count(12usize, 0usize, 9u32, 2146344u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2146176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bf80));
    } else {
        emu.pc = 2146348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c02c));
    }
}
#[inline(always)]
pub fn block_0x0020c02c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2146352u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2146256u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bfd0));
}
#[inline]
pub fn block_0x0020c030(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2146356u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2146360u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2146364u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2146368u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2146372u32)?;
    emu.lw_no_count(14usize, 10usize, 0u32, 2146376u32)?;
    emu.lw_no_count(15usize, 10usize, 4u32, 2146380u32)?;
    emu.adi_no_count(8usize, 11usize, 0u32, 2146384u32);
    emu.adi_no_count(9usize, 2usize, 12u32, 2146388u32);
    emu.adi_no_count(12usize, 2usize, 12u32, 2146392u32);
    emu.adi_no_count(13usize, 0usize, 20u32, 2146396u32);
    emu.adi_no_count(18usize, 0usize, 20u32, 2146400u32);
    emu.adi_no_count(10usize, 14usize, 0u32, 2146404u32);
    emu.adi_no_count(11usize, 15usize, 0u32, 2146408u32);
    emu.apc_no_count(1usize, 2146408u32, 0u32, 2146412u32);
    emu.add_memory_rw_events(16usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2146416u32;
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
pub fn block_0x0020c070(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(15usize, 18usize, 10usize, 2146420u32);
    emu.adr_no_count(14usize, 9usize, 10usize, 2146424u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2146428u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2146432u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2146436u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2146440u32);
    emu.apc_no_count(1usize, 2146440u32, 16384u32, 2146444u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2146448u32;
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
pub fn block_0x0020c090(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2146452u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2146456u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2146460u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2146464u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2146468u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2146472u32;
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
pub fn block_0x0020c0a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2146476u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2146480u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2146484u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2146488u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2146492u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2146496u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2146500u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2146504u32)?;
    emu.adi_no_count(8usize, 11usize, 0u32, 2146508u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2146520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c0d8));
    } else {
        emu.pc = 2146512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c0d0));
    }
}
#[inline(always)]
pub fn block_0x0020c0d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 12usize, 0u32, 2146516u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2146520u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2146536u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c0e8));
}
#[inline(always)]
pub fn block_0x0020c0d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(11usize, 0usize, 10usize, 2146524u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2146528u32);
    emu.sbr_no_count(13usize, 0usize, 12usize, 2146532u32);
    emu.sbr_no_count(11usize, 13usize, 11usize, 2146536u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2146536u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c0e8));
}
#[inline(always)]
pub fn block_0x0020c0e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.slti_no_count(12usize, 12usize, 0u32, 2146540u32);
    emu.xri_no_count(9usize, 12usize, 1u32, 2146544u32);
    emu.adi_no_count(18usize, 2usize, 8u32, 2146548u32);
    emu.adi_no_count(12usize, 2usize, 8u32, 2146552u32);
    emu.adi_no_count(13usize, 0usize, 20u32, 2146556u32);
    emu.adi_no_count(19usize, 0usize, 20u32, 2146560u32);
    emu.apc_no_count(1usize, 2146560u32, 0u32, 2146564u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2146568u32;
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
pub fn block_0x0020c108(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(15usize, 19usize, 10usize, 2146572u32);
    emu.adr_no_count(14usize, 18usize, 10usize, 2146576u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2146580u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2146584u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2146588u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2146592u32);
    emu.apc_no_count(1usize, 2146592u32, 16384u32, 2146596u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2146600u32;
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
pub fn block_0x0020c128(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2146604u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2146608u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2146612u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2146616u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2146620u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2146624u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2146628u32;
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
pub fn block_0x0020c144(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2146632u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2146636u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2146640u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2146644u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2146648u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2146652u32)?;
    emu.sw_no_count(20usize, 2usize, 40u32, 2146656u32)?;
    emu.sw_no_count(21usize, 2usize, 36u32, 2146660u32)?;
    emu.sw_no_count(22usize, 2usize, 32u32, 2146664u32)?;
    emu.sw_no_count(23usize, 2usize, 28u32, 2146668u32)?;
    emu.sw_no_count(24usize, 2usize, 24u32, 2146672u32)?;
    emu.sw_no_count(25usize, 2usize, 20u32, 2146676u32)?;
    emu.sw_no_count(26usize, 2usize, 16u32, 2146680u32)?;
    emu.sw_no_count(27usize, 2usize, 12u32, 2146684u32)?;
    emu.adi_no_count(21usize, 13usize, 0u32, 2146688u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2146692u32);
    emu.adi_no_count(18usize, 10usize, 0u32, 2146696u32);
    emu.sltiu_no_count(10usize, 10usize, 1000u32, 2146700u32);
    emu.sltiu_no_count(11usize, 11usize, 1u32, 2146704u32);
    emu.anr_no_count(10usize, 11usize, 10usize, 2146708u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(24usize, a);
    emu.pc = 2146712u32;
    emu.update_insn_clock();
    emu.adi_no_count(24usize, 24usize, 98u32, 2146716u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2146720u32)?;
    emu.add_memory_rw_events(23usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2147028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c2d4));
    } else {
        emu.pc = 2146724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c1a4));
    }
}
#[inline]
pub fn block_0x0020c1a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 12usize, 4294967294u32, 2146728u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2146732u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4096u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2146736u32;
    emu.update_insn_clock();
    emu.adi_no_count(26usize, 0usize, 100u32, 2146740u32);
    let a = 0u32.wrapping_add(9998336u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2146744u32;
    emu.update_insn_clock();
    emu.adi_no_count(20usize, 10usize, 1808u32, 2146748u32);
    emu.adi_no_count(27usize, 11usize, 1147u32, 2146752u32);
    emu.adi_no_count(8usize, 12usize, 1663u32, 2146756u32);
    emu.adi_no_count(22usize, 18usize, 0u32, 2146760u32);
    emu.adi_no_count(23usize, 9usize, 0u32, 2146764u32);
    emu.add_memory_rw_events(10usize);
    emu.pc = 2146764u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c1cc));
}
#[inline(always)]
pub fn block_0x0020c1cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 21usize, 4294967292u32, 2146768u32);
    emu.adi_no_count(10usize, 22usize, 0u32, 2146772u32);
    emu.adi_no_count(11usize, 23usize, 0u32, 2146776u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2146780u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2146784u32);
    emu.apc_no_count(1usize, 2146784u32, 32768u32, 2146788u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2146792u32;
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
pub fn block_0x0020c1e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 30u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.mul_no_count(12usize, 10usize, 20usize, 2146796u32);
    emu.adr_no_count(21usize, 25usize, 21usize, 2146800u32);
    emu.sltru_no_count(13usize, 8usize, 22usize, 2146804u32);
    emu.sltru_no_count(14usize, 0usize, 23usize, 2146808u32);
    emu.sbr_no_count(12usize, 22usize, 12usize, 2146812u32);
    emu.orr_no_count(13usize, 13usize, 14usize, 2146816u32);
    emu.sli_no_count(14usize, 12usize, 16u32, 2146820u32);
    emu.sri_no_count(14usize, 14usize, 18u32, 2146824u32);
    emu.mul_no_count(14usize, 14usize, 27usize, 2146828u32);
    emu.sri_no_count(15usize, 14usize, 16u32, 2146832u32);
    emu.sri_no_count(14usize, 14usize, 17u32, 2146836u32);
    emu.mul_no_count(14usize, 14usize, 26usize, 2146840u32);
    emu.ani_no_count(15usize, 15usize, 2046u32, 2146844u32);
    emu.sbr_no_count(12usize, 12usize, 14usize, 2146848u32);
    emu.adr_no_count(15usize, 24usize, 15usize, 2146852u32);
    emu.sli_no_count(12usize, 12usize, 17u32, 2146856u32);
    emu.sri_no_count(12usize, 12usize, 16u32, 2146860u32);
    emu.adr_no_count(12usize, 24usize, 12usize, 2146864u32);
    emu.lbu_no_count(14usize, 15usize, 0u32, 2146868u32);
    emu.lbu_no_count(15usize, 15usize, 1u32, 2146872u32);
    emu.lbu_no_count(16usize, 12usize, 0u32, 2146876u32);
    emu.lbu_no_count(12usize, 12usize, 1u32, 2146880u32);
    emu.sb_no_count(14usize, 21usize, 4294967294u32, 2146884u32);
    emu.sb_no_count(15usize, 21usize, 4294967295u32, 2146888u32);
    emu.sb_no_count(16usize, 21usize, 0u32, 2146892u32);
    emu.sb_no_count(12usize, 21usize, 1u32, 2146896u32);
    emu.adi_no_count(21usize, 19usize, 0u32, 2146900u32);
    emu.adi_no_count(22usize, 10usize, 0u32, 2146904u32);
    emu.adi_no_count(23usize, 11usize, 0u32, 2146908u32);
    emu.add_memory_rw_events(29usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2146764u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c1cc));
    } else {
        emu.pc = 2146912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c260));
    }
}
#[inline(always)]
pub fn block_0x0020c260(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(12usize, 10usize, 10u32, 2146916u32);
    emu.sltiu_no_count(13usize, 11usize, 1u32, 2146920u32);
    emu.anr_no_count(12usize, 13usize, 12usize, 2146924u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2147056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c2f0));
    } else {
        emu.pc = 2146928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c270));
    }
}
#[inline]
pub fn block_0x0020c270(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2146932u32);
    emu.sli_no_count(12usize, 10usize, 16u32, 2146936u32);
    let a = 0u32.wrapping_add(4096u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2146940u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 0usize, 100u32, 2146944u32);
    emu.lw_no_count(16usize, 2usize, 8u32, 2146948u32)?;
    emu.adr_no_count(15usize, 16usize, 19usize, 2146952u32);
    emu.sri_no_count(12usize, 12usize, 18u32, 2146956u32);
    emu.adi_no_count(13usize, 13usize, 1147u32, 2146960u32);
    emu.mul_no_count(12usize, 12usize, 13usize, 2146964u32);
    emu.sri_no_count(12usize, 12usize, 17u32, 2146968u32);
    emu.mul_no_count(13usize, 12usize, 14usize, 2146972u32);
    emu.sbr_no_count(10usize, 10usize, 13usize, 2146976u32);
    emu.sli_no_count(10usize, 10usize, 17u32, 2146980u32);
    emu.sri_no_count(10usize, 10usize, 16u32, 2146984u32);
    emu.adr_no_count(10usize, 24usize, 10usize, 2146988u32);
    emu.lbu_no_count(13usize, 10usize, 0u32, 2146992u32);
    emu.lbu_no_count(10usize, 10usize, 1u32, 2146996u32);
    emu.adi_no_count(19usize, 19usize, 4294967294u32, 2147000u32);
    emu.adr_no_count(14usize, 16usize, 19usize, 2147004u32);
    emu.sb_no_count(13usize, 14usize, 0u32, 2147008u32);
    emu.sb_no_count(10usize, 15usize, 4294967295u32, 2147012u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2147016u32);
    emu.orr_no_count(12usize, 18usize, 9usize, 2147020u32);
    emu.add_memory_rw_events(23usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2147068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c2fc));
    } else {
        emu.pc = 2147024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c2d0));
    }
}
#[inline(always)]
pub fn block_0x0020c2d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2147028u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2147076u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c304));
}
#[inline(always)]
pub fn block_0x0020c2d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2147032u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2147036u32);
    emu.adi_no_count(19usize, 21usize, 0u32, 2147040u32);
    emu.sltiu_no_count(12usize, 18usize, 10u32, 2147044u32);
    emu.sltiu_no_count(13usize, 9usize, 1u32, 2147048u32);
    emu.anr_no_count(12usize, 13usize, 12usize, 2147052u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2146928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c270));
    } else {
        emu.pc = 2147056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c2f0));
    }
}
#[inline(always)]
pub fn block_0x0020c2f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 2usize, 8u32, 2147060u32)?;
    emu.orr_no_count(12usize, 18usize, 9usize, 2147064u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2147076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c304));
    } else {
        emu.pc = 2147068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c2fc));
    }
}
#[inline(always)]
pub fn block_0x0020c2fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.orr_no_count(11usize, 10usize, 11usize, 2147072u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2147104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c320));
    } else {
        emu.pc = 2147076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c304));
    }
}
#[inline(always)]
pub fn block_0x0020c304(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 10usize, 1u32, 2147080u32);
    emu.ani_no_count(10usize, 10usize, 30u32, 2147084u32);
    emu.adr_no_count(10usize, 24usize, 10usize, 2147088u32);
    emu.lbu_no_count(10usize, 10usize, 1u32, 2147092u32);
    emu.adi_no_count(19usize, 19usize, 4294967295u32, 2147096u32);
    emu.adr_no_count(11usize, 16usize, 19usize, 2147100u32);
    emu.sb_no_count(10usize, 11usize, 0u32, 2147104u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2147104u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c320));
}
#[inline]
pub fn block_0x0020c320(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 19usize, 0u32, 2147108u32);
    emu.lw_no_count(1usize, 2usize, 60u32, 2147112u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2147116u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2147120u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2147124u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2147128u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2147132u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2147136u32)?;
    emu.lw_no_count(22usize, 2usize, 32u32, 2147140u32)?;
    emu.lw_no_count(23usize, 2usize, 28u32, 2147144u32)?;
    emu.lw_no_count(24usize, 2usize, 24u32, 2147148u32)?;
    emu.lw_no_count(25usize, 2usize, 20u32, 2147152u32)?;
    emu.lw_no_count(26usize, 2usize, 16u32, 2147156u32)?;
    emu.lw_no_count(27usize, 2usize, 12u32, 2147160u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2147164u32);
    emu.add_memory_rw_events(16usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2147168u32;
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
pub fn block_0x0020c360(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2147172u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2147176u32);
    emu.sw_no_count(10usize, 2usize, 4u32, 2147180u32)?;
    emu.sw_no_count(11usize, 2usize, 8u32, 2147184u32)?;
    emu.sh_no_count(12usize, 2usize, 12u32, 2147188u32)?;
    emu.adi_no_count(10usize, 2usize, 4u32, 2147192u32);
    emu.apc_no_count(1usize, 2147192u32, 4294959104u32, 2147196u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2147200u32;
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
pub fn block_0x0020c380(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2147204u32);
    emu.sw_no_count(10usize, 2usize, 24u32, 2147208u32)?;
    emu.sw_no_count(11usize, 2usize, 28u32, 2147212u32)?;
    emu.adi_no_count(10usize, 2usize, 24u32, 2147216u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2147220u32);
    emu.sw_no_count(0usize, 2usize, 16u32, 2147224u32)?;
    emu.adi_no_count(13usize, 0usize, 4u32, 2147228u32);
    emu.sw_no_count(10usize, 2usize, 0u32, 2147232u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2147236u32)?;
    emu.sw_no_count(13usize, 2usize, 8u32, 2147240u32)?;
    emu.sw_no_count(0usize, 2usize, 12u32, 2147244u32)?;
    emu.adi_no_count(10usize, 2usize, 0u32, 2147248u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2147252u32);
    emu.apc_no_count(1usize, 2147252u32, 0u32, 2147256u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2147260u32;
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
pub fn block_0x0020c3bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2147264u32);
    emu.sw_no_count(10usize, 2usize, 0u32, 2147268u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2147272u32)?;
    emu.adi_no_count(10usize, 2usize, 4u32, 2147276u32);
    let a = 0u32.wrapping_add(2146304u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2147280u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966160u32, 2147284u32);
    emu.adi_no_count(13usize, 2usize, 0u32, 2147288u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2147292u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 316u32, 2147296u32);
    emu.adi_no_count(15usize, 0usize, 2u32, 2147300u32);
    emu.sw_no_count(0usize, 2usize, 24u32, 2147304u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2147308u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2147312u32)?;
    emu.sw_no_count(13usize, 2usize, 40u32, 2147316u32)?;
    emu.sw_no_count(11usize, 2usize, 44u32, 2147320u32)?;
    emu.adi_no_count(10usize, 2usize, 32u32, 2147324u32);
    emu.sw_no_count(14usize, 2usize, 8u32, 2147328u32)?;
    emu.sw_no_count(15usize, 2usize, 12u32, 2147332u32)?;
    emu.sw_no_count(10usize, 2usize, 16u32, 2147336u32)?;
    emu.sw_no_count(15usize, 2usize, 20u32, 2147340u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2147344u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2147348u32);
    emu.apc_no_count(1usize, 2147348u32, 0u32, 2147352u32);
    emu.add_memory_rw_events(24usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2147356u32;
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
pub fn block_0x0020c41c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2147360u32);
    emu.adi_no_count(16usize, 14usize, 0u32, 2147364u32);
    emu.adi_no_count(15usize, 13usize, 0u32, 2147368u32);
    emu.sw_no_count(11usize, 2usize, 8u32, 2147372u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2147376u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2147380u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 332u32, 2147384u32);
    emu.adi_no_count(11usize, 2usize, 8u32, 2147388u32);
    emu.adi_no_count(13usize, 2usize, 12u32, 2147392u32);
    emu.adi_no_count(14usize, 12usize, 0u32, 2147396u32);
    emu.apc_no_count(1usize, 2147396u32, 0u32, 2147400u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2147404u32;
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
pub fn block_0x0020c44c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 22u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967168u32, 2147408u32);
    emu.sw_no_count(1usize, 2usize, 124u32, 2147412u32)?;
    emu.sw_no_count(8usize, 2usize, 120u32, 2147416u32)?;
    emu.sw_no_count(9usize, 2usize, 116u32, 2147420u32)?;
    emu.adi_no_count(8usize, 16usize, 0u32, 2147424u32);
    emu.sw_no_count(11usize, 2usize, 12u32, 2147428u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2147432u32)?;
    emu.sli_no_count(10usize, 10usize, 2u32, 2147436u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2147440u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 656u32, 2147444u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2147448u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 668u32, 2147452u32);
    emu.adr_no_count(11usize, 11usize, 10usize, 2147456u32);
    emu.adr_no_count(10usize, 12usize, 10usize, 2147460u32);
    emu.lw_no_count(12usize, 15usize, 0u32, 2147464u32)?;
    emu.lw_no_count(11usize, 11usize, 0u32, 2147468u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2147472u32)?;
    emu.sw_no_count(13usize, 2usize, 20u32, 2147476u32)?;
    emu.sw_no_count(14usize, 2usize, 24u32, 2147480u32)?;
    emu.sw_no_count(11usize, 2usize, 28u32, 2147484u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2147488u32)?;
    emu.add_memory_rw_events(21usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2147584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c500));
    } else {
        emu.pc = 2147492u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c4a4));
    }
}
#[inline]
pub fn block_0x0020c4a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 23u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 28u32, 2147496u32);
    let a = 0u32.wrapping_add(2170880u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2147500u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965436u32, 2147504u32);
    emu.adi_no_count(12usize, 2usize, 12u32, 2147508u32);
    let a = 0u32.wrapping_add(2170880u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2147512u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294965420u32, 2147516u32);
    emu.adi_no_count(14usize, 2usize, 20u32, 2147520u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2147524u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 392u32, 2147528u32);
    emu.adi_no_count(16usize, 0usize, 3u32, 2147532u32);
    emu.sw_no_count(0usize, 2usize, 108u32, 2147536u32)?;
    emu.sw_no_count(10usize, 2usize, 60u32, 2147540u32)?;
    emu.sw_no_count(11usize, 2usize, 64u32, 2147544u32)?;
    emu.sw_no_count(12usize, 2usize, 68u32, 2147548u32)?;
    emu.sw_no_count(13usize, 2usize, 72u32, 2147552u32)?;
    emu.sw_no_count(14usize, 2usize, 76u32, 2147556u32)?;
    emu.sw_no_count(13usize, 2usize, 80u32, 2147560u32)?;
    emu.adi_no_count(10usize, 2usize, 60u32, 2147564u32);
    emu.sw_no_count(15usize, 2usize, 92u32, 2147568u32)?;
    emu.sw_no_count(16usize, 2usize, 96u32, 2147572u32)?;
    emu.sw_no_count(10usize, 2usize, 100u32, 2147576u32)?;
    emu.sw_no_count(16usize, 2usize, 104u32, 2147580u32)?;
    emu.add_memory_rw_events(23usize);
    let return_addr = 2147584u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2147712u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c580));
}
#[inline(always)]
pub fn block_0x0020c500(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 36u32, 2147588u32);
    emu.adi_no_count(12usize, 0usize, 24u32, 2147592u32);
    emu.adi_no_count(9usize, 2usize, 36u32, 2147596u32);
    emu.adi_no_count(11usize, 15usize, 0u32, 2147600u32);
    emu.apc_no_count(1usize, 2147600u32, 4294930432u32, 2147604u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2147608u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966100u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0020c518(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 26u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 28u32, 2147612u32);
    let a = 0u32.wrapping_add(2170880u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2147616u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965436u32, 2147620u32);
    let a = 0u32.wrapping_add(2162688u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2147624u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966968u32, 2147628u32);
    emu.adi_no_count(13usize, 2usize, 12u32, 2147632u32);
    let a = 0u32.wrapping_add(2170880u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2147636u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965420u32, 2147640u32);
    emu.adi_no_count(15usize, 2usize, 20u32, 2147644u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2147648u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 428u32, 2147652u32);
    emu.sw_no_count(10usize, 2usize, 60u32, 2147656u32)?;
    emu.sw_no_count(11usize, 2usize, 64u32, 2147660u32)?;
    emu.sw_no_count(9usize, 2usize, 68u32, 2147664u32)?;
    emu.sw_no_count(12usize, 2usize, 72u32, 2147668u32)?;
    emu.adi_no_count(10usize, 0usize, 4u32, 2147672u32);
    emu.sw_no_count(0usize, 2usize, 108u32, 2147676u32)?;
    emu.sw_no_count(13usize, 2usize, 76u32, 2147680u32)?;
    emu.sw_no_count(14usize, 2usize, 80u32, 2147684u32)?;
    emu.sw_no_count(15usize, 2usize, 84u32, 2147688u32)?;
    emu.sw_no_count(14usize, 2usize, 88u32, 2147692u32)?;
    emu.adi_no_count(11usize, 2usize, 60u32, 2147696u32);
    emu.sw_no_count(16usize, 2usize, 92u32, 2147700u32)?;
    emu.sw_no_count(10usize, 2usize, 96u32, 2147704u32)?;
    emu.sw_no_count(11usize, 2usize, 100u32, 2147708u32)?;
    emu.sw_no_count(10usize, 2usize, 104u32, 2147712u32)?;
    emu.add_memory_rw_events(26usize);
    emu.pc = 2147712u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c580));
}
#[inline(always)]
pub fn block_0x0020c580(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 92u32, 2147716u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2147720u32);
    emu.apc_no_count(1usize, 2147720u32, 0u32, 2147724u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2147728u32;
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
pub fn block_0x0020c590(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2147728u32, 0u32, 2147732u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2147736u32;
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
pub fn block_0x0020c598(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967184u32, 2147740u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2147744u32)?;
    emu.adi_no_count(15usize, 0usize, 257u32, 2147748u32);
    emu.sw_no_count(13usize, 2usize, 12u32, 2147752u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2147776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c5c0));
    } else {
        emu.pc = 2147756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c5ac));
    }
}
#[inline(always)]
pub fn block_0x0020c5ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 0u32, 2147760u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2147764u32)?;
    emu.sw_no_count(11usize, 2usize, 20u32, 2147768u32)?;
    emu.adi_no_count(15usize, 0usize, 1u32, 2147772u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2147776u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2147844u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c604));
}
#[inline(always)]
pub fn block_0x0020c5c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 256u32, 2147780u32);
    emu.adi_no_count(16usize, 0usize, 4294967231u32, 2147784u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2147784u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c5c8));
}
#[inline(always)]
pub fn block_0x0020c5c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(17usize, 10usize, 15usize, 2147788u32);
    emu.lb_no_count(17usize, 17usize, 0u32, 2147792u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(16usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2147804u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c5dc));
    } else {
        emu.pc = 2147796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c5d4));
    }
}
#[inline(always)]
pub fn block_0x0020c5d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 15usize, 4294967295u32, 2147800u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a != b {
        emu.pc = 2147784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c5c8));
    } else {
        emu.pc = 2147804u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c5dc));
    }
}
#[inline(always)]
pub fn block_0x0020c5dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 16u32, 2147808u32)?;
    emu.sw_no_count(15usize, 2usize, 20u32, 2147812u32)?;
    emu.sltru_no_count(16usize, 15usize, 11usize, 2147816u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a < b {
        emu.pc = 2147828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c5f4));
    } else {
        emu.pc = 2147820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c5ec));
    }
}
#[inline(always)]
pub fn block_0x0020c5ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 1u32, 2147824u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2147828u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2147836u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c5fc));
}
#[inline(always)]
pub fn block_0x0020c5f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2147832u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 460u32, 2147836u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2147836u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c5fc));
}
#[inline(always)]
pub fn block_0x0020c5fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(16usize, 0usize, 16usize, 2147840u32);
    emu.ani_no_count(16usize, 16usize, 5u32, 2147844u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2147844u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c604));
}
#[inline(always)]
pub fn block_0x0020c604(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(15usize, 2usize, 24u32, 2147848u32)?;
    emu.sw_no_count(16usize, 2usize, 28u32, 2147852u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2148292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c7c4));
    } else {
        emu.pc = 2147856u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c610));
    }
}
#[inline(always)]
pub fn block_0x0020c610(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2148288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c7c0));
    } else {
        emu.pc = 2147860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c614));
    }
}
#[inline(always)]
pub fn block_0x0020c614(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2148400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c830));
    } else {
        emu.pc = 2147864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c618));
    }
}
#[inline(always)]
pub fn block_0x0020c618(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2147892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c634));
    } else {
        emu.pc = 2147868u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c61c));
    }
}
#[inline(always)]
pub fn block_0x0020c61c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2147892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c634));
    } else {
        emu.pc = 2147872u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c620));
    }
}
#[inline(always)]
pub fn block_0x0020c620(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(15usize, 10usize, 12usize, 2147876u32);
    emu.lb_no_count(15usize, 15usize, 0u32, 2147880u32);
    emu.adi_no_count(16usize, 0usize, 4294967231u32, 2147884u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(16usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2147892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c634));
    } else {
        emu.pc = 2147888u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c630));
    }
}
#[inline(always)]
pub fn block_0x0020c630(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 12usize, 0u32, 2147892u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2147892u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c634));
}
#[inline(always)]
pub fn block_0x0020c634(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(13usize, 2usize, 32u32, 2147896u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2147932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c65c));
    } else {
        emu.pc = 2147900u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c63c));
    }
}
#[inline(always)]
pub fn block_0x0020c63c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2147928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c658));
    } else {
        emu.pc = 2147904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c640));
    }
}
#[inline(always)]
pub fn block_0x0020c640(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 4294967231u32, 2147908u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2147908u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c644));
}
#[inline(always)]
pub fn block_0x0020c644(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(15usize, 10usize, 13usize, 2147912u32);
    emu.lb_no_count(15usize, 15usize, 0u32, 2147916u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2147928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c658));
    } else {
        emu.pc = 2147920u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c650));
    }
}
#[inline(always)]
pub fn block_0x0020c650(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2147924u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2147908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c644));
    } else {
        emu.pc = 2147928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c658));
    }
}
#[inline(always)]
pub fn block_0x0020c658(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2147944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c668));
    } else {
        emu.pc = 2147932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c65c));
    }
}
#[inline(always)]
pub fn block_0x0020c65c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 14usize, 0u32, 2147936u32);
    emu.apc_no_count(1usize, 2147936u32, 8192u32, 2147940u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2147944u32;
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
pub fn block_0x0020c668(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 10usize, 13usize, 2147948u32);
    emu.lb_no_count(12usize, 10usize, 0u32, 2147952u32);
    emu.ani_no_count(11usize, 12usize, 255u32, 2147956u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2147972u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c684));
    } else {
        emu.pc = 2147960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c678));
    }
}
#[inline(always)]
pub fn block_0x0020c678(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 2usize, 36u32, 2147964u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2147968u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2147972u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2148132u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c724));
}
#[inline(always)]
pub fn block_0x0020c684(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(15usize, 10usize, 1u32, 2147976u32);
    emu.ani_no_count(12usize, 11usize, 31u32, 2147980u32);
    emu.adi_no_count(16usize, 0usize, 223u32, 2147984u32);
    emu.ani_no_count(15usize, 15usize, 63u32, 2147988u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a >= b {
        emu.pc = 2148056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c6d8));
    } else {
        emu.pc = 2147992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c698));
    }
}
#[inline(always)]
pub fn block_0x0020c698(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(16usize, 10usize, 2u32, 2147996u32);
    emu.sli_no_count(15usize, 15usize, 6u32, 2148000u32);
    emu.ani_no_count(16usize, 16usize, 63u32, 2148004u32);
    emu.adi_no_count(17usize, 0usize, 240u32, 2148008u32);
    emu.orr_no_count(15usize, 15usize, 16usize, 2148012u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2148084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c6f4));
    } else {
        emu.pc = 2148016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c6b0));
    }
}
#[inline]
pub fn block_0x0020c6b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 10usize, 3u32, 2148020u32);
    emu.sli_no_count(12usize, 12usize, 29u32, 2148024u32);
    emu.sli_no_count(15usize, 15usize, 6u32, 2148028u32);
    emu.sri_no_count(12usize, 12usize, 11u32, 2148032u32);
    emu.ani_no_count(10usize, 10usize, 63u32, 2148036u32);
    emu.orr_no_count(10usize, 15usize, 10usize, 2148040u32);
    emu.orr_no_count(10usize, 10usize, 12usize, 2148044u32);
    let a = 0u32.wrapping_add(1114112u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2148048u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2147932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c65c));
    } else {
        emu.pc = 2148052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c6d4));
    }
}
#[inline(always)]
pub fn block_0x0020c6d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2148056u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2148064u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c6e0));
}
#[inline(always)]
pub fn block_0x0020c6d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 12usize, 6u32, 2148060u32);
    emu.orr_no_count(10usize, 10usize, 15usize, 2148064u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2148064u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c6e0));
}
#[inline(always)]
pub fn block_0x0020c6e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 128u32, 2148068u32);
    emu.sw_no_count(10usize, 2usize, 36u32, 2148072u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2148104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c708));
    } else {
        emu.pc = 2148076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c6ec));
    }
}
#[inline(always)]
pub fn block_0x0020c6ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2148080u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2148084u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2148132u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c724));
}
#[inline(always)]
pub fn block_0x0020c6f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 12usize, 12u32, 2148088u32);
    emu.orr_no_count(10usize, 15usize, 10usize, 2148092u32);
    emu.adi_no_count(11usize, 0usize, 128u32, 2148096u32);
    emu.sw_no_count(10usize, 2usize, 36u32, 2148100u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2148076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c6ec));
    } else {
        emu.pc = 2148104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c708));
    }
}
#[inline(always)]
pub fn block_0x0020c708(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 10usize, 11u32, 2148108u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2148120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c718));
    } else {
        emu.pc = 2148112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c710));
    }
}
#[inline(always)]
pub fn block_0x0020c710(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2148116u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2148120u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2148132u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c724));
}
#[inline(always)]
pub fn block_0x0020c718(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 10usize, 16u32, 2148124u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2148128u32);
    emu.adi_no_count(10usize, 10usize, 3u32, 2148132u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2148132u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c724));
}
#[inline(never)]
pub fn block_0x0020c724(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 39u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 10usize, 13usize, 2148136u32);
    emu.adi_no_count(11usize, 2usize, 32u32, 2148140u32);
    let a = 0u32.wrapping_add(2146304u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2148144u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966160u32, 2148148u32);
    emu.adi_no_count(15usize, 2usize, 36u32, 2148152u32);
    let a = 0u32.wrapping_add(2166784u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2148156u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 1376u32, 2148160u32);
    emu.adi_no_count(17usize, 2usize, 40u32, 2148164u32);
    let a = 0u32.wrapping_add(2150400u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2148168u32;
    emu.update_insn_clock();
    emu.adi_no_count(5usize, 5usize, 1372u32, 2148172u32);
    emu.adi_no_count(6usize, 2usize, 16u32, 2148176u32);
    let a = 0u32.wrapping_add(2170880u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2148180u32;
    emu.update_insn_clock();
    emu.adi_no_count(7usize, 7usize, 4294965436u32, 2148184u32);
    emu.adi_no_count(28usize, 2usize, 24u32, 2148188u32);
    emu.sw_no_count(13usize, 2usize, 40u32, 2148192u32)?;
    emu.sw_no_count(10usize, 2usize, 44u32, 2148196u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2148200u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 568u32, 2148204u32);
    emu.sw_no_count(11usize, 2usize, 72u32, 2148208u32)?;
    emu.sw_no_count(12usize, 2usize, 76u32, 2148212u32)?;
    emu.sw_no_count(15usize, 2usize, 80u32, 2148216u32)?;
    emu.sw_no_count(16usize, 2usize, 84u32, 2148220u32)?;
    emu.adi_no_count(11usize, 0usize, 5u32, 2148224u32);
    emu.sw_no_count(0usize, 2usize, 64u32, 2148228u32)?;
    emu.sw_no_count(17usize, 2usize, 88u32, 2148232u32)?;
    emu.sw_no_count(5usize, 2usize, 92u32, 2148236u32)?;
    emu.sw_no_count(6usize, 2usize, 96u32, 2148240u32)?;
    emu.sw_no_count(7usize, 2usize, 100u32, 2148244u32)?;
    emu.sw_no_count(28usize, 2usize, 104u32, 2148248u32)?;
    emu.sw_no_count(7usize, 2usize, 108u32, 2148252u32)?;
    emu.adi_no_count(12usize, 2usize, 72u32, 2148256u32);
    emu.sw_no_count(10usize, 2usize, 48u32, 2148260u32)?;
    emu.sw_no_count(11usize, 2usize, 52u32, 2148264u32)?;
    emu.sw_no_count(12usize, 2usize, 56u32, 2148268u32)?;
    emu.sw_no_count(11usize, 2usize, 60u32, 2148272u32)?;
    emu.adi_no_count(10usize, 2usize, 48u32, 2148276u32);
    emu.adi_no_count(11usize, 14usize, 0u32, 2148280u32);
    emu.apc_no_count(1usize, 2148280u32, 0u32, 2148284u32);
    emu.add_memory_rw_events(39usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2148288u32;
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
pub fn block_0x0020c7c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 13usize, 0u32, 2148292u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2148292u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c7c4));
}
#[inline(never)]
pub fn block_0x0020c7c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(12usize, 2usize, 40u32, 2148296u32)?;
    emu.adi_no_count(10usize, 2usize, 40u32, 2148300u32);
    let a = 0u32.wrapping_add(2146304u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2148304u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966160u32, 2148308u32);
    emu.adi_no_count(12usize, 2usize, 16u32, 2148312u32);
    let a = 0u32.wrapping_add(2170880u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2148316u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294965436u32, 2148320u32);
    emu.adi_no_count(15usize, 2usize, 24u32, 2148324u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2148328u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 632u32, 2148332u32);
    emu.adi_no_count(17usize, 0usize, 3u32, 2148336u32);
    emu.sw_no_count(0usize, 2usize, 64u32, 2148340u32)?;
    emu.sw_no_count(10usize, 2usize, 72u32, 2148344u32)?;
    emu.sw_no_count(11usize, 2usize, 76u32, 2148348u32)?;
    emu.sw_no_count(12usize, 2usize, 80u32, 2148352u32)?;
    emu.sw_no_count(13usize, 2usize, 84u32, 2148356u32)?;
    emu.sw_no_count(15usize, 2usize, 88u32, 2148360u32)?;
    emu.sw_no_count(13usize, 2usize, 92u32, 2148364u32)?;
    emu.adi_no_count(10usize, 2usize, 72u32, 2148368u32);
    emu.sw_no_count(16usize, 2usize, 48u32, 2148372u32)?;
    emu.sw_no_count(17usize, 2usize, 52u32, 2148376u32)?;
    emu.sw_no_count(10usize, 2usize, 56u32, 2148380u32)?;
    emu.sw_no_count(17usize, 2usize, 60u32, 2148384u32)?;
    emu.adi_no_count(10usize, 2usize, 48u32, 2148388u32);
    emu.adi_no_count(11usize, 14usize, 0u32, 2148392u32);
    emu.apc_no_count(1usize, 2148392u32, 0u32, 2148396u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2148400u32;
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
pub fn block_0x0020c830(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 29u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 8u32, 2148404u32);
    let a = 0u32.wrapping_add(2146304u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2148408u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966160u32, 2148412u32);
    emu.adi_no_count(12usize, 2usize, 12u32, 2148416u32);
    emu.adi_no_count(13usize, 2usize, 16u32, 2148420u32);
    let a = 0u32.wrapping_add(2170880u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2148424u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 4294965436u32, 2148428u32);
    emu.adi_no_count(16usize, 2usize, 24u32, 2148432u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2148436u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 17usize, 480u32, 2148440u32);
    emu.sw_no_count(10usize, 2usize, 72u32, 2148444u32)?;
    emu.sw_no_count(11usize, 2usize, 76u32, 2148448u32)?;
    emu.sw_no_count(12usize, 2usize, 80u32, 2148452u32)?;
    emu.sw_no_count(11usize, 2usize, 84u32, 2148456u32)?;
    emu.adi_no_count(10usize, 0usize, 4u32, 2148460u32);
    emu.sw_no_count(0usize, 2usize, 64u32, 2148464u32)?;
    emu.sw_no_count(13usize, 2usize, 88u32, 2148468u32)?;
    emu.sw_no_count(15usize, 2usize, 92u32, 2148472u32)?;
    emu.sw_no_count(16usize, 2usize, 96u32, 2148476u32)?;
    emu.sw_no_count(15usize, 2usize, 100u32, 2148480u32)?;
    emu.adi_no_count(11usize, 2usize, 72u32, 2148484u32);
    emu.sw_no_count(17usize, 2usize, 48u32, 2148488u32)?;
    emu.sw_no_count(10usize, 2usize, 52u32, 2148492u32)?;
    emu.sw_no_count(11usize, 2usize, 56u32, 2148496u32)?;
    emu.sw_no_count(10usize, 2usize, 60u32, 2148500u32)?;
    emu.adi_no_count(10usize, 2usize, 48u32, 2148504u32);
    emu.adi_no_count(11usize, 14usize, 0u32, 2148508u32);
    emu.apc_no_count(1usize, 2148508u32, 0u32, 2148512u32);
    emu.add_memory_rw_events(29usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2148516u32;
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
