pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2159200u32;
pub const PC_MAX: u32 = 2161752u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 127usize] = [
        block_0x0020f260,
        block_0x0020f2a8,
        block_0x0020f2b0,
        block_0x0020f2b4,
        block_0x0020f2e0,
        block_0x0020f340,
        block_0x0020f39c,
        block_0x0020f3d8,
        block_0x0020f3e4,
        block_0x0020f3ec,
        block_0x0020f3fc,
        block_0x0020f49c,
        block_0x0020f4a4,
        block_0x0020f4b4,
        block_0x0020f4f4,
        block_0x0020f500,
        block_0x0020f508,
        block_0x0020f514,
        block_0x0020f530,
        block_0x0020f534,
        block_0x0020f53c,
        block_0x0020f554,
        block_0x0020f55c,
        block_0x0020f568,
        block_0x0020f578,
        block_0x0020f580,
        block_0x0020f598,
        block_0x0020f5a4,
        block_0x0020f5ac,
        block_0x0020f5b8,
        block_0x0020f5bc,
        block_0x0020f5d4,
        block_0x0020f63c,
        block_0x0020f644,
        block_0x0020f650,
        block_0x0020f660,
        block_0x0020f668,
        block_0x0020f680,
        block_0x0020f684,
        block_0x0020f68c,
        block_0x0020f6a8,
        block_0x0020f6b0,
        block_0x0020f6d0,
        block_0x0020f6d8,
        block_0x0020f6ec,
        block_0x0020f6fc,
        block_0x0020f704,
        block_0x0020f70c,
        block_0x0020f714,
        block_0x0020f72c,
        block_0x0020f754,
        block_0x0020f758,
        block_0x0020f784,
        block_0x0020f78c,
        block_0x0020f79c,
        block_0x0020f7a0,
        block_0x0020f7bc,
        block_0x0020f7c0,
        block_0x0020f7d4,
        block_0x0020f7e8,
        block_0x0020f7f0,
        block_0x0020f810,
        block_0x0020f818,
        block_0x0020f820,
        block_0x0020f834,
        block_0x0020f850,
        block_0x0020f854,
        block_0x0020f864,
        block_0x0020f868,
        block_0x0020f880,
        block_0x0020f884,
        block_0x0020f8a0,
        block_0x0020f8a8,
        block_0x0020f8e0,
        block_0x0020f8fc,
        block_0x0020f918,
        block_0x0020f934,
        block_0x0020f94c,
        block_0x0020f964,
        block_0x0020f97c,
        block_0x0020f9a8,
        block_0x0020f9b0,
        block_0x0020f9b4,
        block_0x0020f9bc,
        block_0x0020f9cc,
        block_0x0020f9d4,
        block_0x0020f9d8,
        block_0x0020f9e4,
        block_0x0020f9f4,
        block_0x0020f9fc,
        block_0x0020fa00,
        block_0x0020fa0c,
        block_0x0020fa38,
        block_0x0020fa40,
        block_0x0020fa44,
        block_0x0020fa60,
        block_0x0020fa70,
        block_0x0020fa74,
        block_0x0020fa7c,
        block_0x0020fa80,
        block_0x0020fa88,
        block_0x0020faa8,
        block_0x0020fab0,
        block_0x0020fab4,
        block_0x0020fac0,
        block_0x0020fac8,
        block_0x0020fad8,
        block_0x0020faf4,
        block_0x0020fafc,
        block_0x0020fb2c,
        block_0x0020fb44,
        block_0x0020fb50,
        block_0x0020fb54,
        block_0x0020fb80,
        block_0x0020fb84,
        block_0x0020fb94,
        block_0x0020fbc8,
        block_0x0020fbe4,
        block_0x0020fbec,
        block_0x0020fbf0,
        block_0x0020fc00,
        block_0x0020fc04,
        block_0x0020fc0c,
        block_0x0020fc10,
        block_0x0020fc20,
        block_0x0020fc3c,
        block_0x0020fc58,
    ];
    const IDX: [u16; 639usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 3u16, 4u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 5u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 6u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 7u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 8u16, 0u16, 0u16, 9u16, 0u16, 10u16, 0u16, 0u16, 0u16, 11u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        12u16, 0u16, 13u16, 0u16, 0u16, 0u16, 14u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 15u16, 0u16, 0u16, 16u16,
        0u16, 17u16, 0u16, 0u16, 18u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 19u16, 20u16,
        0u16, 21u16, 0u16, 0u16, 0u16, 0u16, 0u16, 22u16, 0u16, 23u16, 0u16, 0u16, 24u16,
        0u16, 0u16, 0u16, 25u16, 0u16, 26u16, 0u16, 0u16, 0u16, 0u16, 0u16, 27u16, 0u16,
        0u16, 28u16, 0u16, 29u16, 0u16, 0u16, 30u16, 31u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        32u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        33u16, 0u16, 34u16, 0u16, 0u16, 35u16, 0u16, 0u16, 0u16, 36u16, 0u16, 37u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 38u16, 39u16, 0u16, 40u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 41u16, 0u16, 42u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 43u16,
        0u16, 44u16, 0u16, 0u16, 0u16, 0u16, 45u16, 0u16, 0u16, 0u16, 46u16, 0u16, 47u16,
        0u16, 48u16, 0u16, 49u16, 0u16, 0u16, 0u16, 0u16, 0u16, 50u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 51u16, 52u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 53u16, 0u16, 54u16, 0u16, 0u16, 0u16, 55u16, 56u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 57u16, 58u16, 0u16, 0u16, 0u16, 0u16, 59u16,
        0u16, 0u16, 0u16, 0u16, 60u16, 0u16, 61u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 62u16, 0u16, 63u16, 0u16, 64u16, 0u16, 0u16, 0u16, 0u16, 65u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 66u16, 67u16, 0u16, 0u16, 0u16, 68u16, 69u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 70u16, 71u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 72u16, 0u16,
        73u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 74u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 75u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 76u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 77u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 78u16, 0u16, 0u16, 0u16, 0u16, 0u16, 79u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 80u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 81u16,
        0u16, 82u16, 83u16, 0u16, 84u16, 0u16, 0u16, 0u16, 85u16, 0u16, 86u16, 87u16,
        0u16, 0u16, 88u16, 0u16, 0u16, 0u16, 89u16, 0u16, 90u16, 91u16, 0u16, 0u16,
        92u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 93u16, 0u16,
        94u16, 95u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 96u16, 0u16, 0u16, 0u16, 97u16,
        98u16, 0u16, 99u16, 100u16, 0u16, 101u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 102u16, 0u16, 103u16, 104u16, 0u16, 0u16, 105u16, 0u16, 106u16, 0u16, 0u16,
        0u16, 107u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 108u16, 0u16, 109u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 110u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 111u16, 0u16, 0u16, 112u16, 113u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 114u16, 115u16, 0u16, 0u16, 0u16, 116u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 117u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 118u16, 0u16, 119u16, 120u16, 0u16, 0u16,
        0u16, 121u16, 122u16, 0u16, 123u16, 124u16, 0u16, 0u16, 0u16, 125u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 126u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 127u16,
    ];
    if pc < 2159200u32 || pc > 2161752u32 {
        return None;
    }
    let word_offset = ((pc - 2159200u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline]
pub fn block_0x0020f260(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2159204u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2159208u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2159212u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2159216u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2159220u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2159224u32)?;
    emu.sw_no_count(20usize, 2usize, 24u32, 2159228u32)?;
    emu.sw_no_count(21usize, 2usize, 20u32, 2159232u32)?;
    emu.sw_no_count(22usize, 2usize, 16u32, 2159236u32)?;
    emu.sw_no_count(23usize, 2usize, 12u32, 2159240u32)?;
    emu.sw_no_count(24usize, 2usize, 8u32, 2159244u32)?;
    emu.sw_no_count(25usize, 2usize, 4u32, 2159248u32)?;
    emu.sw_no_count(26usize, 2usize, 0u32, 2159252u32)?;
    emu.adi_no_count(15usize, 14usize, 0u32, 2159256u32);
    emu.lw_no_count(17usize, 11usize, 0u32, 2159260u32)?;
    emu.lw_no_count(16usize, 11usize, 4u32, 2159264u32)?;
    emu.orr_no_count(14usize, 17usize, 16usize, 2159268u32);
    emu.add_memory_rw_events(17usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2160864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f8e0));
    } else {
        emu.pc = 2159272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f2a8));
    }
}
#[inline(always)]
pub fn block_0x0020f2a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(14usize, 16usize, 29u32, 2159276u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a != b {
        emu.pc = 2160892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f8fc));
    } else {
        emu.pc = 2159280u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f2b0));
    }
}
#[inline(always)]
pub fn block_0x0020f2b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2160920u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f918));
    } else {
        emu.pc = 2159284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f2b4));
    }
}
#[inline]
pub fn block_0x0020f2b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lh_no_count(14usize, 11usize, 24u32, 2159288u32)?;
    emu.sri_no_count(11usize, 17usize, 1u32, 2159292u32);
    let a = 0u32.wrapping_add(1431654400u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2159296u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(858992640u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2159300u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(252645376u32);
    emu.write_reg_no_count(29usize, a);
    emu.pc = 2159304u32;
    emu.update_insn_clock();
    emu.adi_no_count(28usize, 5usize, 1365u32, 2159308u32);
    emu.adi_no_count(7usize, 6usize, 819u32, 2159312u32);
    emu.adi_no_count(5usize, 29usize, 4294967055u32, 2159316u32);
    let a = 0u32.wrapping_add(16842752u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2159320u32;
    emu.update_insn_clock();
    emu.adi_no_count(6usize, 6usize, 257u32, 2159324u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a != b {
        emu.pc = 2159424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f340));
    } else {
        emu.pc = 2159328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f2e0));
    }
}
#[inline]
pub fn block_0x0020f2e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.orr_no_count(29usize, 17usize, 11usize, 2159332u32);
    emu.sri_no_count(30usize, 29usize, 2u32, 2159336u32);
    emu.orr_no_count(29usize, 29usize, 30usize, 2159340u32);
    emu.sri_no_count(30usize, 29usize, 4u32, 2159344u32);
    emu.orr_no_count(29usize, 29usize, 30usize, 2159348u32);
    emu.sri_no_count(30usize, 29usize, 8u32, 2159352u32);
    emu.orr_no_count(29usize, 29usize, 30usize, 2159356u32);
    emu.sri_no_count(30usize, 29usize, 16u32, 2159360u32);
    emu.orr_no_count(29usize, 29usize, 30usize, 2159364u32);
    emu.xri_no_count(29usize, 29usize, 4294967295u32, 2159368u32);
    emu.sri_no_count(30usize, 29usize, 1u32, 2159372u32);
    emu.anr_no_count(28usize, 30usize, 28usize, 2159376u32);
    emu.sbr_no_count(28usize, 29usize, 28usize, 2159380u32);
    emu.anr_no_count(29usize, 28usize, 7usize, 2159384u32);
    emu.sri_no_count(28usize, 28usize, 2u32, 2159388u32);
    emu.anr_no_count(7usize, 28usize, 7usize, 2159392u32);
    emu.adr_no_count(7usize, 29usize, 7usize, 2159396u32);
    emu.sri_no_count(28usize, 7usize, 4u32, 2159400u32);
    emu.adr_no_count(7usize, 7usize, 28usize, 2159404u32);
    emu.anr_no_count(5usize, 7usize, 5usize, 2159408u32);
    emu.mul_no_count(5usize, 5usize, 6usize, 2159412u32);
    emu.sri_no_count(5usize, 5usize, 24u32, 2159416u32);
    emu.adi_no_count(6usize, 5usize, 32u32, 2159420u32);
    emu.add_memory_rw_events(24usize);
    let return_addr = 2159424u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2159516u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f39c));
}
#[inline]
pub fn block_0x0020f340(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 23u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(29usize, 16usize, 1u32, 2159428u32);
    emu.orr_no_count(29usize, 16usize, 29usize, 2159432u32);
    emu.sri_no_count(30usize, 29usize, 2u32, 2159436u32);
    emu.orr_no_count(29usize, 29usize, 30usize, 2159440u32);
    emu.sri_no_count(30usize, 29usize, 4u32, 2159444u32);
    emu.orr_no_count(29usize, 29usize, 30usize, 2159448u32);
    emu.sri_no_count(30usize, 29usize, 8u32, 2159452u32);
    emu.orr_no_count(29usize, 29usize, 30usize, 2159456u32);
    emu.sri_no_count(30usize, 29usize, 16u32, 2159460u32);
    emu.orr_no_count(29usize, 29usize, 30usize, 2159464u32);
    emu.xri_no_count(29usize, 29usize, 4294967295u32, 2159468u32);
    emu.sri_no_count(30usize, 29usize, 1u32, 2159472u32);
    emu.anr_no_count(28usize, 30usize, 28usize, 2159476u32);
    emu.sbr_no_count(28usize, 29usize, 28usize, 2159480u32);
    emu.anr_no_count(29usize, 28usize, 7usize, 2159484u32);
    emu.sri_no_count(28usize, 28usize, 2u32, 2159488u32);
    emu.anr_no_count(7usize, 28usize, 7usize, 2159492u32);
    emu.adr_no_count(7usize, 29usize, 7usize, 2159496u32);
    emu.sri_no_count(28usize, 7usize, 4u32, 2159500u32);
    emu.adr_no_count(7usize, 7usize, 28usize, 2159504u32);
    emu.anr_no_count(5usize, 7usize, 5usize, 2159508u32);
    emu.mul_no_count(5usize, 5usize, 6usize, 2159512u32);
    emu.sri_no_count(6usize, 5usize, 24u32, 2159516u32);
    emu.add_memory_rw_events(23usize);
    emu.pc = 2159516u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f39c));
}
#[inline]
pub fn block_0x0020f39c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(5usize, 14usize, 6usize, 2159520u32);
    emu.adi_no_count(14usize, 0usize, 4294967200u32, 2159524u32);
    emu.adi_no_count(7usize, 0usize, 80u32, 2159528u32);
    let a = 0u32.wrapping_add(2068697088u32);
    emu.write_reg_no_count(28usize, a);
    emu.pc = 2159532u32;
    emu.update_insn_clock();
    emu.sbr_no_count(14usize, 14usize, 5usize, 2159536u32);
    emu.adi_no_count(28usize, 28usize, 4294965651u32, 2159540u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2159544u32);
    emu.sai_no_count(14usize, 14usize, 1040u32, 2159548u32);
    emu.adi_no_count(14usize, 14usize, 1087u32, 2159552u32);
    emu.mul_no_count(14usize, 14usize, 7usize, 2159556u32);
    emu.mulh_no_count(14usize, 14usize, 28usize, 2159560u32);
    emu.sri_no_count(28usize, 14usize, 31u32, 2159564u32);
    emu.sai_no_count(14usize, 14usize, 1034u32, 2159568u32);
    emu.adr_no_count(14usize, 14usize, 28usize, 2159572u32);
    emu.add_memory_rw_events(14usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a < b {
        emu.pc = 2160996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f964));
    } else {
        emu.pc = 2159576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f3d8));
    }
}
#[inline(always)]
pub fn block_0x0020f3d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(7usize, 6usize, 4294967264u32, 2159580u32);
    emu.slr_no_count(17usize, 17usize, 6usize, 2159584u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(7usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2159596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f3ec));
    } else {
        emu.pc = 2159588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f3e4));
    }
}
#[inline(always)]
pub fn block_0x0020f3e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 17usize, 0u32, 2159592u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2159596u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2159612u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f3fc));
}
#[inline(always)]
pub fn block_0x0020f3ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(28usize, 6usize, 4294967295u32, 2159600u32);
    emu.srr_no_count(11usize, 11usize, 28usize, 2159604u32);
    emu.slr_no_count(16usize, 16usize, 6usize, 2159608u32);
    emu.orr_no_count(11usize, 16usize, 11usize, 2159612u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2159612u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f3fc));
}
#[inline(never)]
pub fn block_0x0020f3fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 40u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sai_no_count(16usize, 7usize, 1055u32, 2159616u32);
    emu.sli_no_count(14usize, 14usize, 4u32, 2159620u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2159624u32;
    emu.update_insn_clock();
    emu.adi_no_count(6usize, 6usize, 4294965776u32, 2159628u32);
    emu.adr_no_count(14usize, 6usize, 14usize, 2159632u32);
    emu.lw_no_count(6usize, 14usize, 0u32, 2159636u32)?;
    emu.lw_no_count(7usize, 14usize, 4u32, 2159640u32)?;
    emu.anr_no_count(16usize, 16usize, 17usize, 2159644u32);
    emu.lh_no_count(17usize, 14usize, 8u32, 2159648u32)?;
    emu.mulhu_no_count(28usize, 6usize, 16usize, 2159652u32);
    emu.mul_no_count(29usize, 7usize, 16usize, 2159656u32);
    emu.mulhu_no_count(30usize, 7usize, 16usize, 2159660u32);
    emu.mul_no_count(31usize, 6usize, 11usize, 2159664u32);
    emu.mulhu_no_count(6usize, 6usize, 11usize, 2159668u32);
    emu.mul_no_count(8usize, 7usize, 11usize, 2159672u32);
    emu.mulhu_no_count(11usize, 7usize, 11usize, 2159676u32);
    emu.adr_no_count(17usize, 5usize, 17usize, 2159680u32);
    emu.adi_no_count(16usize, 0usize, 4294967232u32, 2159684u32);
    emu.adr_no_count(28usize, 29usize, 28usize, 2159688u32);
    emu.sbr_no_count(5usize, 16usize, 17usize, 2159692u32);
    emu.sbr_no_count(16usize, 0usize, 17usize, 2159696u32);
    emu.sltru_no_count(17usize, 28usize, 29usize, 2159700u32);
    emu.adr_no_count(28usize, 31usize, 28usize, 2159704u32);
    emu.adr_no_count(17usize, 30usize, 17usize, 2159708u32);
    emu.ani_no_count(29usize, 5usize, 63u32, 2159712u32);
    emu.sltru_no_count(7usize, 28usize, 31usize, 2159716u32);
    emu.sri_no_count(30usize, 28usize, 31u32, 2159720u32);
    emu.adr_no_count(6usize, 6usize, 7usize, 2159724u32);
    emu.adi_no_count(28usize, 29usize, 4294967264u32, 2159728u32);
    emu.adr_no_count(6usize, 17usize, 6usize, 2159732u32);
    emu.sltru_no_count(17usize, 6usize, 17usize, 2159736u32);
    emu.adr_no_count(6usize, 8usize, 6usize, 2159740u32);
    emu.sltru_no_count(7usize, 6usize, 8usize, 2159744u32);
    emu.adr_no_count(11usize, 11usize, 17usize, 2159748u32);
    emu.adr_no_count(6usize, 30usize, 6usize, 2159752u32);
    emu.adr_no_count(11usize, 11usize, 7usize, 2159756u32);
    emu.sltru_no_count(7usize, 6usize, 30usize, 2159760u32);
    emu.adr_no_count(7usize, 11usize, 7usize, 2159764u32);
    emu.xri_no_count(8usize, 29usize, 4294967295u32, 2159768u32);
    emu.add_memory_rw_events(39usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(28usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2159780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f4a4));
    } else {
        emu.pc = 2159772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f49c));
    }
}
#[inline(always)]
pub fn block_0x0020f49c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.srr_no_count(11usize, 7usize, 29usize, 2159776u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2159780u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2159796u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f4b4));
}
#[inline(always)]
pub fn block_0x0020f4a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(11usize, 7usize, 1u32, 2159784u32);
    emu.slr_no_count(11usize, 11usize, 8usize, 2159788u32);
    emu.srr_no_count(17usize, 6usize, 16usize, 2159792u32);
    emu.orr_no_count(11usize, 17usize, 11usize, 2159796u32);
    emu.add_memory_rw_events(4usize);
    emu.pc = 2159796u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f4b4));
}
#[inline]
pub fn block_0x0020f4b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lhu_no_count(14usize, 14usize, 10u32, 2159800u32)?;
    emu.slti_no_count(17usize, 28usize, 0u32, 2159804u32);
    emu.adi_no_count(30usize, 0usize, 1u32, 2159808u32);
    emu.slr_no_count(31usize, 30usize, 29usize, 2159812u32);
    emu.adi_no_count(9usize, 17usize, 4294967295u32, 2159816u32);
    emu.sbr_no_count(17usize, 0usize, 17usize, 2159820u32);
    emu.slr_no_count(16usize, 30usize, 16usize, 2159824u32);
    emu.anr_no_count(31usize, 9usize, 31usize, 2159828u32);
    emu.anr_no_count(30usize, 17usize, 16usize, 2159832u32);
    emu.sltiu_no_count(16usize, 30usize, 1u32, 2159836u32);
    emu.adi_no_count(9usize, 30usize, 4294967295u32, 2159840u32);
    emu.sbr_no_count(18usize, 31usize, 16usize, 2159844u32);
    emu.anr_no_count(17usize, 18usize, 7usize, 2159848u32);
    emu.anr_no_count(16usize, 9usize, 6usize, 2159852u32);
    emu.orr_no_count(19usize, 16usize, 17usize, 2159856u32);
    emu.add_memory_rw_events(15usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2159924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f534));
    } else {
        emu.pc = 2159860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f4f4));
    }
}
#[inline(always)]
pub fn block_0x0020f4f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(19usize, 11usize, 4u32, 2159864u32);
    emu.adi_no_count(20usize, 0usize, 625u32, 2159868u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a >= b {
        emu.pc = 2159964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f55c));
    } else {
        emu.pc = 2159872u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f500));
    }
}
#[inline(always)]
pub fn block_0x0020f500(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 100u32, 2159876u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2160044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f5ac));
    } else {
        emu.pc = 2159880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f508));
    }
}
#[inline(always)]
pub fn block_0x0020f508(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 9u32, 2159884u32);
    emu.sltiu_no_count(20usize, 11usize, 10u32, 2159888u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a < b {
        emu.pc = 2160692u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f834));
    } else {
        emu.pc = 2159892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f514));
    }
}
#[inline(always)]
pub fn block_0x0020f514(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 1u32, 2159896u32);
    emu.xri_no_count(20usize, 20usize, 1u32, 2159900u32);
    emu.sbr_no_count(14usize, 20usize, 14usize, 2159904u32);
    emu.adi_no_count(14usize, 14usize, 1u32, 2159908u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2159912u32);
    emu.sai_no_count(14usize, 14usize, 1040u32, 2159916u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(15usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2160084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f5d4));
    } else {
        emu.pc = 2159920u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f530));
    }
}
#[inline(always)]
pub fn block_0x0020f530(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2159924u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2160024u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f598));
}
#[inline(always)]
pub fn block_0x0020f534(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 10u32, 2159928u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a < b {
        emu.pc = 2159956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f554));
    } else {
        emu.pc = 2159932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f53c));
    }
}
#[inline(always)]
pub fn block_0x0020f53c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(19usize, 13usize, 2u32, 2159936u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(20usize, a);
    emu.pc = 2159940u32;
    emu.update_insn_clock();
    emu.adi_no_count(20usize, 20usize, 368u32, 2159944u32);
    emu.adr_no_count(19usize, 20usize, 19usize, 2159948u32);
    emu.lw_no_count(19usize, 19usize, 4294967292u32, 2159952u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2159860u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f4f4));
    } else {
        emu.pc = 2159956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f554));
    }
}
#[inline(always)]
pub fn block_0x0020f554(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 10usize, 0u32, 2159960u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2159964u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2160808u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f8a8));
}
#[inline(always)]
pub fn block_0x0020f55c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(999424u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2159968u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 19usize, 576u32, 2159972u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2160196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f644));
    } else {
        emu.pc = 2159976u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f568));
    }
}
#[inline(always)]
pub fn block_0x0020f568(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(98304u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2159980u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 19usize, 1696u32, 2159984u32);
    emu.sltru_no_count(20usize, 11usize, 19usize, 2159988u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2160000u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f580));
    } else {
        emu.pc = 2159992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f578));
    }
}
#[inline(always)]
pub fn block_0x0020f578(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2159996u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 19usize, 1808u32, 2160000u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2160000u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f580));
}
#[inline(always)]
pub fn block_0x0020f580(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(20usize, 20usize, 5u32, 2160004u32);
    emu.sbr_no_count(14usize, 20usize, 14usize, 2160008u32);
    emu.adi_no_count(14usize, 14usize, 1u32, 2160012u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2160016u32);
    emu.sai_no_count(14usize, 14usize, 1040u32, 2160020u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(15usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2160084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f5d4));
    } else {
        emu.pc = 2160024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f598));
    }
}
#[inline(always)]
pub fn block_0x0020f598(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(7usize, 14usize, 15usize, 2160028u32);
    emu.sli_no_count(6usize, 5usize, 16u32, 2160032u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a < b {
        emu.pc = 2160260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f684));
    } else {
        emu.pc = 2160036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f5a4));
    }
}
#[inline(always)]
pub fn block_0x0020f5a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 13usize, 0u32, 2160040u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2160044u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2160268u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f68c));
}
#[inline(always)]
pub fn block_0x0020f5ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(21usize, 0usize, 1000u32, 2160048u32);
    emu.sltiu_no_count(20usize, 11usize, 1000u32, 2160052u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2160060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f5bc));
    } else {
        emu.pc = 2160056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f5b8));
    }
}
#[inline(always)]
pub fn block_0x0020f5b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 1000u32, 2160060u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2160060u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f5bc));
}
#[inline(always)]
pub fn block_0x0020f5bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(20usize, 20usize, 3u32, 2160064u32);
    emu.sbr_no_count(14usize, 20usize, 14usize, 2160068u32);
    emu.adi_no_count(14usize, 14usize, 1u32, 2160072u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2160076u32);
    emu.sai_no_count(14usize, 14usize, 1040u32, 2160080u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(15usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2160024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f598));
    } else {
        emu.pc = 2160084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f5d4));
    }
}
#[inline(never)]
pub fn block_0x0020f5d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 26u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 6usize, 1u32, 2160088u32);
    emu.sli_no_count(16usize, 7usize, 31u32, 2160092u32);
    emu.sri_no_count(17usize, 7usize, 1u32, 2160096u32);
    let a = 0u32.wrapping_add(3435974656u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2160100u32;
    emu.update_insn_clock();
    emu.orr_no_count(11usize, 11usize, 16usize, 2160104u32);
    emu.adi_no_count(6usize, 5usize, 4294966477u32, 2160108u32);
    emu.adi_no_count(16usize, 5usize, 4294966476u32, 2160112u32);
    emu.adr_no_count(5usize, 11usize, 17usize, 2160116u32);
    emu.sltru_no_count(7usize, 5usize, 11usize, 2160120u32);
    emu.adr_no_count(5usize, 5usize, 7usize, 2160124u32);
    emu.mulhu_no_count(7usize, 5usize, 6usize, 2160128u32);
    emu.sri_no_count(9usize, 7usize, 2u32, 2160132u32);
    emu.ani_no_count(7usize, 7usize, 4294967292u32, 2160136u32);
    emu.adr_no_count(7usize, 7usize, 9usize, 2160140u32);
    emu.sbr_no_count(5usize, 5usize, 7usize, 2160144u32);
    emu.sbr_no_count(7usize, 11usize, 5usize, 2160148u32);
    emu.sltru_no_count(11usize, 11usize, 5usize, 2160152u32);
    emu.mul_no_count(5usize, 7usize, 16usize, 2160156u32);
    emu.mulhu_no_count(9usize, 7usize, 6usize, 2160160u32);
    emu.sbr_no_count(11usize, 17usize, 11usize, 2160164u32);
    emu.mul_no_count(16usize, 7usize, 6usize, 2160168u32);
    emu.adr_no_count(17usize, 9usize, 5usize, 2160172u32);
    emu.mul_no_count(11usize, 11usize, 6usize, 2160176u32);
    emu.adr_no_count(17usize, 17usize, 11usize, 2160180u32);
    emu.slr_no_count(11usize, 19usize, 29usize, 2160184u32);
    emu.add_memory_rw_events(25usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(28usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2160396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f70c));
    } else {
        emu.pc = 2160188u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f63c));
    }
}
#[inline(always)]
pub fn block_0x0020f63c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(29usize, 11usize, 0u32, 2160192u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2160196u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2160404u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f714));
}
#[inline(always)]
pub fn block_0x0020f644(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(99999744u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2160200u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 19usize, 256u32, 2160204u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2160724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f854));
    } else {
        emu.pc = 2160208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f650));
    }
}
#[inline(always)]
pub fn block_0x0020f650(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(9998336u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2160212u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 19usize, 1664u32, 2160216u32);
    emu.sltru_no_count(20usize, 11usize, 19usize, 2160220u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2160232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f668));
    } else {
        emu.pc = 2160224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f660));
    }
}
#[inline(always)]
pub fn block_0x0020f660(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(999424u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2160228u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 19usize, 576u32, 2160232u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2160232u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f668));
}
#[inline(always)]
pub fn block_0x0020f668(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(20usize, 20usize, 7u32, 2160236u32);
    emu.sbr_no_count(14usize, 20usize, 14usize, 2160240u32);
    emu.adi_no_count(14usize, 14usize, 1u32, 2160244u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2160248u32);
    emu.sai_no_count(14usize, 14usize, 1040u32, 2160252u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(15usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2160024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f598));
    } else {
        emu.pc = 2160256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f680));
    }
}
#[inline(always)]
pub fn block_0x0020f680(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2160260u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2160084u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f5d4));
}
#[inline(always)]
pub fn block_0x0020f684(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(7usize, 7usize, 16u32, 2160264u32);
    emu.sai_no_count(5usize, 7usize, 1040u32, 2160268u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2160268u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f68c));
}
#[inline(always)]
pub fn block_0x0020f68c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(7usize, 6usize, 16u32, 2160272u32);
    emu.adi_no_count(22usize, 0usize, 4294967295u32, 2160276u32);
    let a = 0u32.wrapping_add(3435974656u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2160280u32;
    emu.update_insn_clock();
    emu.adi_no_count(23usize, 0usize, 10u32, 2160284u32);
    emu.adi_no_count(24usize, 6usize, 4294966477u32, 2160288u32);
    emu.adi_no_count(21usize, 0usize, 4294967295u32, 2160292u32);
    emu.adi_no_count(25usize, 12usize, 0u32, 2160296u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2160296u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f6a8));
}
#[inline(always)]
pub fn block_0x0020f6a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(26usize, 13usize, 21usize, 2160300u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a == b {
        emu.pc = 2160948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f934));
    } else {
        emu.pc = 2160304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f6b0));
    }
}
#[inline(always)]
pub fn block_0x0020f6b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(6usize, 19usize, 0u32, 2160308u32);
    emu.divu_no_count(19usize, 11usize, 19usize, 2160312u32);
    emu.mul_no_count(26usize, 19usize, 6usize, 2160316u32);
    emu.sbr_no_count(11usize, 11usize, 26usize, 2160320u32);
    emu.adr_no_count(26usize, 5usize, 21usize, 2160324u32);
    emu.adi_no_count(19usize, 19usize, 48u32, 2160328u32);
    emu.sb_no_count(19usize, 25usize, 0u32, 2160332u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a == b {
        emu.pc = 2160380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f6fc));
    } else {
        emu.pc = 2160336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f6d0));
    }
}
#[inline(always)]
pub fn block_0x0020f6d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(19usize, 20usize, 21usize, 2160340u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2160428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f72c));
    } else {
        emu.pc = 2160344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f6d8));
    }
}
#[inline(always)]
pub fn block_0x0020f6d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.mulhu_no_count(19usize, 6usize, 24usize, 2160348u32);
    emu.adi_no_count(25usize, 25usize, 1u32, 2160352u32);
    emu.sri_no_count(19usize, 19usize, 3u32, 2160356u32);
    emu.adi_no_count(21usize, 21usize, 4294967295u32, 2160360u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a >= b {
        emu.pc = 2160296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f6a8));
    } else {
        emu.pc = 2160364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f6ec));
    }
}
#[inline(always)]
pub fn block_0x0020f6ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2160368u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 408u32, 2160372u32);
    emu.apc_no_count(1usize, 2160372u32, 0u32, 2160376u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160380u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1516u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020f6fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.slr_no_count(7usize, 11usize, 29usize, 2160384u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(28usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2160616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f7e8));
    } else {
        emu.pc = 2160388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f704));
    }
}
#[inline(always)]
pub fn block_0x0020f704(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 7usize, 0u32, 2160392u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2160396u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2160624u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f7f0));
}
#[inline(always)]
pub fn block_0x0020f70c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(5usize, 19usize, 1u32, 2160400u32);
    emu.srr_no_count(29usize, 5usize, 8usize, 2160404u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2160404u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f714));
}
#[inline(always)]
pub fn block_0x0020f714(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sai_no_count(5usize, 28usize, 1055u32, 2160408u32);
    emu.anr_no_count(28usize, 5usize, 11usize, 2160412u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2160416u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2160420u32);
    emu.adi_no_count(13usize, 0usize, 0u32, 2160424u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2160428u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2160800u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f8a0));
}
#[inline]
pub fn block_0x0020f72c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(6usize, 0usize, 0u32, 2160432u32);
    emu.adi_no_count(19usize, 7usize, 4294967295u32, 2160436u32);
    emu.sbr_no_count(11usize, 0usize, 21usize, 2160440u32);
    emu.adi_no_count(7usize, 0usize, 1u32, 2160444u32);
    emu.ani_no_count(19usize, 19usize, 63u32, 2160448u32);
    emu.xri_no_count(20usize, 19usize, 4294967295u32, 2160452u32);
    emu.adi_no_count(21usize, 19usize, 4294967264u32, 2160456u32);
    emu.sai_no_count(22usize, 21usize, 1055u32, 2160460u32);
    emu.adi_no_count(23usize, 0usize, 10u32, 2160464u32);
    emu.add_memory_rw_events(10usize);
    let return_addr = 2160468u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2160516u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f784));
}
#[inline(always)]
pub fn block_0x0020f754(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.srr_no_count(24usize, 17usize, 29usize, 2160472u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2160472u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f758));
}
#[inline]
pub fn block_0x0020f758(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(17usize, 17usize, 18usize, 2160476u32);
    emu.anr_no_count(16usize, 16usize, 9usize, 2160480u32);
    emu.mulhu_no_count(25usize, 7usize, 23usize, 2160484u32);
    emu.mul_no_count(6usize, 6usize, 23usize, 2160488u32);
    emu.mul_no_count(7usize, 7usize, 23usize, 2160492u32);
    emu.adi_no_count(24usize, 24usize, 48u32, 2160496u32);
    emu.adr_no_count(6usize, 25usize, 6usize, 2160500u32);
    emu.adr_no_count(25usize, 12usize, 11usize, 2160504u32);
    emu.adi_no_count(11usize, 11usize, 1u32, 2160508u32);
    emu.sb_no_count(24usize, 25usize, 0u32, 2160512u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2160772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f884));
    } else {
        emu.pc = 2160516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f784));
    }
}
#[inline(always)]
pub fn block_0x0020f784(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.srr_no_count(24usize, 6usize, 19usize, 2160520u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(21usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2160544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f7a0));
    } else {
        emu.pc = 2160524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f78c));
    }
}
#[inline(always)]
pub fn block_0x0020f78c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 24usize, 0u32, 2160528u32);
    emu.anr_no_count(24usize, 22usize, 24usize, 2160532u32);
    emu.orr_no_count(24usize, 25usize, 24usize, 2160536u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(24usize);
    if a == b {
        emu.pc = 2160572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f7bc));
    } else {
        emu.pc = 2160540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f79c));
    }
}
#[inline(always)]
pub fn block_0x0020f79c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2160544u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2159956u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f554));
}
#[inline(always)]
pub fn block_0x0020f7a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.srr_no_count(25usize, 7usize, 19usize, 2160548u32);
    emu.sli_no_count(26usize, 6usize, 1u32, 2160552u32);
    emu.slr_no_count(26usize, 26usize, 20usize, 2160556u32);
    emu.orr_no_count(25usize, 25usize, 26usize, 2160560u32);
    emu.anr_no_count(24usize, 22usize, 24usize, 2160564u32);
    emu.orr_no_count(24usize, 25usize, 24usize, 2160568u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(24usize);
    if a != b {
        emu.pc = 2159956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f554));
    } else {
        emu.pc = 2160572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f7bc));
    }
}
#[inline(always)]
pub fn block_0x0020f7bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a >= b {
        emu.pc = 2160972u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f94c));
    } else {
        emu.pc = 2160576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f7c0));
    }
}
#[inline(always)]
pub fn block_0x0020f7c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.mulhu_no_count(24usize, 16usize, 23usize, 2160580u32);
    emu.mul_no_count(17usize, 17usize, 23usize, 2160584u32);
    emu.adr_no_count(17usize, 24usize, 17usize, 2160588u32);
    emu.mul_no_count(16usize, 16usize, 23usize, 2160592u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(28usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2160468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f754));
    } else {
        emu.pc = 2160596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f7d4));
    }
}
#[inline(always)]
pub fn block_0x0020f7d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(24usize, 17usize, 1u32, 2160600u32);
    emu.slr_no_count(24usize, 24usize, 8usize, 2160604u32);
    emu.srr_no_count(25usize, 16usize, 29usize, 2160608u32);
    emu.orr_no_count(24usize, 25usize, 24usize, 2160612u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2160616u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2160472u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f758));
}
#[inline(always)]
pub fn block_0x0020f7e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(11usize, 11usize, 1u32, 2160620u32);
    emu.srr_no_count(9usize, 11usize, 8usize, 2160624u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2160624u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f7f0));
}
#[inline(always)]
pub fn block_0x0020f7f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sai_no_count(11usize, 28usize, 1055u32, 2160628u32);
    emu.adr_no_count(17usize, 9usize, 17usize, 2160632u32);
    emu.anr_no_count(7usize, 11usize, 7usize, 2160636u32);
    emu.adr_no_count(16usize, 7usize, 16usize, 2160640u32);
    emu.sltru_no_count(7usize, 16usize, 7usize, 2160644u32);
    emu.adr_no_count(17usize, 17usize, 7usize, 2160648u32);
    emu.slr_no_count(7usize, 6usize, 29usize, 2160652u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(28usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2160664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f818));
    } else {
        emu.pc = 2160656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f810));
    }
}
#[inline(always)]
pub fn block_0x0020f810(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(29usize, 7usize, 0u32, 2160660u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2160664u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2160672u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f820));
}
#[inline(always)]
pub fn block_0x0020f818(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(6usize, 6usize, 1u32, 2160668u32);
    emu.srr_no_count(29usize, 6usize, 8usize, 2160672u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2160672u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f820));
}
#[inline(always)]
pub fn block_0x0020f820(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(28usize, 11usize, 7usize, 2160676u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2160680u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2160684u32);
    emu.adi_no_count(13usize, 5usize, 0u32, 2160688u32);
    emu.add_memory_rw_events(5usize);
    let return_addr = 2160692u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2160800u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f8a0));
}
#[inline(always)]
pub fn block_0x0020f834(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 10u32, 2160696u32);
    emu.xri_no_count(20usize, 20usize, 1u32, 2160700u32);
    emu.sbr_no_count(14usize, 20usize, 14usize, 2160704u32);
    emu.adi_no_count(14usize, 14usize, 1u32, 2160708u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2160712u32);
    emu.sai_no_count(14usize, 14usize, 1040u32, 2160716u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(15usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2160084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f5d4));
    } else {
        emu.pc = 2160720u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f850));
    }
}
#[inline(always)]
pub fn block_0x0020f850(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2160724u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2160024u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f598));
}
#[inline(always)]
pub fn block_0x0020f854(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(1000001536u32);
    emu.write_reg_no_count(20usize, a);
    emu.pc = 2160728u32;
    emu.update_insn_clock();
    emu.adi_no_count(21usize, 20usize, 4294965760u32, 2160732u32);
    emu.sltru_no_count(20usize, 11usize, 21usize, 2160736u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2160744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f868));
    } else {
        emu.pc = 2160740u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f864));
    }
}
#[inline(always)]
pub fn block_0x0020f864(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 21usize, 0u32, 2160744u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2160744u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f868));
}
#[inline(always)]
pub fn block_0x0020f868(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(20usize, 20usize, 9u32, 2160748u32);
    emu.sbr_no_count(14usize, 20usize, 14usize, 2160752u32);
    emu.adi_no_count(14usize, 14usize, 1u32, 2160756u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2160760u32);
    emu.sai_no_count(14usize, 14usize, 1040u32, 2160764u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(15usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2160024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f598));
    } else {
        emu.pc = 2160768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f880));
    }
}
#[inline(always)]
pub fn block_0x0020f880(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2160772u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2160084u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f5d4));
}
#[inline(always)]
pub fn block_0x0020f884(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 12usize, 0u32, 2160776u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2160780u32);
    emu.adi_no_count(13usize, 5usize, 0u32, 2160784u32);
    emu.adi_no_count(28usize, 30usize, 0u32, 2160788u32);
    emu.adi_no_count(29usize, 31usize, 0u32, 2160792u32);
    emu.adi_no_count(30usize, 7usize, 0u32, 2160796u32);
    emu.adi_no_count(31usize, 6usize, 0u32, 2160800u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2160800u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020f8a0));
}
#[inline(always)]
pub fn block_0x0020f8a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2160800u32, 0u32, 2160804u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160808u32;
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
#[inline]
pub fn block_0x0020f8a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2160812u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2160816u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2160820u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2160824u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2160828u32)?;
    emu.lw_no_count(20usize, 2usize, 24u32, 2160832u32)?;
    emu.lw_no_count(21usize, 2usize, 20u32, 2160836u32)?;
    emu.lw_no_count(22usize, 2usize, 16u32, 2160840u32)?;
    emu.lw_no_count(23usize, 2usize, 12u32, 2160844u32)?;
    emu.lw_no_count(24usize, 2usize, 8u32, 2160848u32)?;
    emu.lw_no_count(25usize, 2usize, 4u32, 2160852u32)?;
    emu.lw_no_count(26usize, 2usize, 0u32, 2160856u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2160860u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160864u32;
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
pub fn block_0x0020f8e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2160868u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294967088u32, 2160872u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2160876u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 284u32, 2160880u32);
    emu.adi_no_count(11usize, 0usize, 28u32, 2160884u32);
    emu.apc_no_count(1usize, 2160884u32, 4294955008u32, 2160888u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160892u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966444u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020f8fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2160896u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 300u32, 2160900u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2160904u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 336u32, 2160908u32);
    emu.adi_no_count(11usize, 0usize, 36u32, 2160912u32);
    emu.apc_no_count(1usize, 2160912u32, 4294955008u32, 2160916u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160920u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966416u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020f918(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2160924u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 248u32, 2160928u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2160932u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 352u32, 2160936u32);
    emu.adi_no_count(11usize, 0usize, 33u32, 2160940u32);
    emu.apc_no_count(1usize, 2160940u32, 4294955008u32, 2160944u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160948u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966388u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020f934(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2160952u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 424u32, 2160956u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2160960u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2160964u32);
    emu.apc_no_count(1usize, 2160964u32, 4294955008u32, 2160968u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160972u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966424u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020f94c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2160976u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 440u32, 2160980u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2160984u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2160988u32);
    emu.apc_no_count(1usize, 2160988u32, 4294955008u32, 2160992u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2160996u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966400u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020f964(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2161000u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967072u32, 2161004u32);
    emu.adi_no_count(11usize, 0usize, 81u32, 2161008u32);
    emu.adi_no_count(10usize, 14usize, 0u32, 2161012u32);
    emu.apc_no_count(1usize, 2161012u32, 4294955008u32, 2161016u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161020u32;
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
#[inline]
pub fn block_0x0020f97c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2161024u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2161028u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2161032u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2161036u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2161040u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2161044u32)?;
    emu.sw_no_count(20usize, 2usize, 24u32, 2161048u32)?;
    emu.sw_no_count(21usize, 2usize, 20u32, 2161052u32)?;
    emu.sw_no_count(22usize, 2usize, 16u32, 2161056u32)?;
    emu.sw_no_count(23usize, 2usize, 12u32, 2161060u32)?;
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(29usize);
    if a == b {
        emu.pc = 2161076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f9b4));
    } else {
        emu.pc = 2161064u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f9a8));
    }
}
#[inline(always)]
pub fn block_0x0020f9a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 31usize, 29usize, 2161068u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a != b {
        emu.pc = 2161084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f9bc));
    } else {
        emu.pc = 2161072u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f9b0));
    }
}
#[inline(always)]
pub fn block_0x0020f9b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2161076u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2161488u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fb50));
}
#[inline(always)]
pub fn block_0x0020f9b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 30usize, 28usize, 2161080u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2161488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fb50));
    } else {
        emu.pc = 2161084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f9bc));
    }
}
#[inline(always)]
pub fn block_0x0020f9bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 28usize, 30usize, 2161088u32);
    emu.sbr_no_count(6usize, 29usize, 31usize, 2161092u32);
    emu.sbr_no_count(5usize, 6usize, 5usize, 2161096u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2161112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f9d8));
    } else {
        emu.pc = 2161100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f9cc));
    }
}
#[inline(always)]
pub fn block_0x0020f9cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 31usize, 5usize, 2161104u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a != b {
        emu.pc = 2161124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f9e4));
    } else {
        emu.pc = 2161108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f9d4));
    }
}
#[inline(always)]
pub fn block_0x0020f9d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2161112u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2161488u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fb50));
}
#[inline(always)]
pub fn block_0x0020f9d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(5usize, 28usize, 30usize, 2161116u32);
    emu.sltru_no_count(5usize, 30usize, 5usize, 2161120u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2161488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fb50));
    } else {
        emu.pc = 2161124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f9e4));
    }
}
#[inline(always)]
pub fn block_0x0020f9e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 28usize, 16usize, 2161128u32);
    emu.sbr_no_count(6usize, 29usize, 17usize, 2161132u32);
    emu.sbr_no_count(5usize, 6usize, 5usize, 2161136u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2161152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fa00));
    } else {
        emu.pc = 2161140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f9f4));
    }
}
#[inline(always)]
pub fn block_0x0020f9f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 17usize, 5usize, 2161144u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a != b {
        emu.pc = 2161164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fa0c));
    } else {
        emu.pc = 2161148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020f9fc));
    }
}
#[inline(always)]
pub fn block_0x0020f9fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2161152u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2161264u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fa70));
}
#[inline(always)]
pub fn block_0x0020fa00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(5usize, 28usize, 16usize, 2161156u32);
    emu.sltru_no_count(5usize, 16usize, 5usize, 2161160u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2161264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fa70));
    } else {
        emu.pc = 2161164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fa0c));
    }
}
#[inline]
pub fn block_0x0020fa0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(6usize, 16usize, 31u32, 2161168u32);
    emu.sli_no_count(7usize, 17usize, 1u32, 2161172u32);
    emu.sli_no_count(5usize, 16usize, 1u32, 2161176u32);
    emu.sri_no_count(8usize, 30usize, 31u32, 2161180u32);
    emu.orr_no_count(6usize, 7usize, 6usize, 2161184u32);
    emu.sltru_no_count(7usize, 28usize, 5usize, 2161188u32);
    emu.sbr_no_count(6usize, 29usize, 6usize, 2161192u32);
    emu.sbr_no_count(6usize, 6usize, 7usize, 2161196u32);
    emu.sli_no_count(7usize, 31usize, 1u32, 2161200u32);
    emu.orr_no_count(7usize, 7usize, 8usize, 2161204u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2161248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fa60));
    } else {
        emu.pc = 2161208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fa38));
    }
}
#[inline(always)]
pub fn block_0x0020fa38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 6usize, 7usize, 2161212u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a != b {
        emu.pc = 2161264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fa70));
    } else {
        emu.pc = 2161216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fa40));
    }
}
#[inline(always)]
pub fn block_0x0020fa40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2161680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fc10));
    } else {
        emu.pc = 2161220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fa44));
    }
}
#[inline(always)]
pub fn block_0x0020fa44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2161224u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 488u32, 2161228u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2161232u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2161236u32);
    emu.adi_no_count(12usize, 14usize, 0u32, 2161240u32);
    emu.apc_no_count(1usize, 2161240u32, 16384u32, 2161244u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161248u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(836u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020fa60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(5usize, 28usize, 5usize, 2161252u32);
    emu.sli_no_count(6usize, 30usize, 1u32, 2161256u32);
    emu.sltru_no_count(5usize, 5usize, 6usize, 2161260u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2161216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fa40));
    } else {
        emu.pc = 2161264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fa70));
    }
}
#[inline(always)]
pub fn block_0x0020fa70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2161280u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fa80));
    } else {
        emu.pc = 2161268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fa74));
    }
}
#[inline(always)]
pub fn block_0x0020fa74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 31usize, 17usize, 2161272u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a != b {
        emu.pc = 2161288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fa88));
    } else {
        emu.pc = 2161276u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fa7c));
    }
}
#[inline(always)]
pub fn block_0x0020fa7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2161280u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2161488u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fb50));
}
#[inline(always)]
pub fn block_0x0020fa80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 30usize, 16usize, 2161284u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2161488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fb50));
    } else {
        emu.pc = 2161288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fa88));
    }
}
#[inline(always)]
pub fn block_0x0020fa88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 16usize, 30usize, 2161292u32);
    emu.sbr_no_count(17usize, 17usize, 31usize, 2161296u32);
    emu.sbr_no_count(16usize, 16usize, 30usize, 2161300u32);
    emu.sbr_no_count(17usize, 17usize, 5usize, 2161304u32);
    emu.sbr_no_count(5usize, 29usize, 17usize, 2161308u32);
    emu.sltru_no_count(6usize, 28usize, 16usize, 2161312u32);
    emu.sbr_no_count(5usize, 5usize, 6usize, 2161316u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2161476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fb44));
    } else {
        emu.pc = 2161320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020faa8));
    }
}
#[inline(always)]
pub fn block_0x0020faa8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 17usize, 5usize, 2161324u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a != b {
        emu.pc = 2161488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fb50));
    } else {
        emu.pc = 2161328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fab0));
    }
}
#[inline(always)]
pub fn block_0x0020fab0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2161696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fc20));
    } else {
        emu.pc = 2161332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fab4));
    }
}
#[inline(always)]
pub fn block_0x0020fab4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(16usize, 0usize, 0u32, 2161336u32);
    emu.adr_no_count(8usize, 11usize, 13usize, 2161340u32);
    emu.adi_no_count(17usize, 0usize, 57u32, 2161344u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2161344u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fac0));
}
#[inline(always)]
pub fn block_0x0020fac0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(5usize, 13usize, 16usize, 2161348u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2161536u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fb80));
    } else {
        emu.pc = 2161352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fac8));
    }
}
#[inline(always)]
pub fn block_0x0020fac8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(5usize, 8usize, 16usize, 2161356u32);
    emu.lbu_no_count(5usize, 5usize, 4294967295u32, 2161360u32);
    emu.adi_no_count(16usize, 16usize, 4294967295u32, 2161364u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2161344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fac0));
    } else {
        emu.pc = 2161368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fad8));
    }
}
#[inline(always)]
pub fn block_0x0020fad8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 8usize, 16usize, 2161372u32);
    emu.lbu_no_count(15usize, 8usize, 0u32, 2161376u32);
    emu.adr_no_count(17usize, 13usize, 16usize, 2161380u32);
    emu.adi_no_count(5usize, 15usize, 1u32, 2161384u32);
    emu.adi_no_count(15usize, 17usize, 1u32, 2161388u32);
    emu.sb_no_count(5usize, 8usize, 0u32, 2161392u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2161752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fc58));
    } else {
        emu.pc = 2161396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020faf4));
    }
}
#[inline(always)]
pub fn block_0x0020faf4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 0usize, 4294967295u32, 2161400u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2161676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fc0c));
    } else {
        emu.pc = 2161404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fafc));
    }
}
#[inline]
pub fn block_0x0020fafc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(15usize, 16usize, 4294967295u32, 2161408u32);
    emu.adi_no_count(16usize, 8usize, 1u32, 2161412u32);
    emu.adi_no_count(8usize, 11usize, 0u32, 2161416u32);
    emu.adi_no_count(11usize, 0usize, 48u32, 2161420u32);
    emu.adi_no_count(9usize, 10usize, 0u32, 2161424u32);
    emu.adi_no_count(10usize, 16usize, 0u32, 2161428u32);
    emu.adi_no_count(18usize, 12usize, 0u32, 2161432u32);
    emu.adi_no_count(12usize, 15usize, 0u32, 2161436u32);
    emu.adi_no_count(19usize, 13usize, 0u32, 2161440u32);
    emu.adi_no_count(20usize, 14usize, 0u32, 2161444u32);
    emu.apc_no_count(1usize, 2161444u32, 4294914048u32, 2161448u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161452u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1132u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020fb2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 8usize, 0u32, 2161456u32);
    emu.adi_no_count(14usize, 20usize, 0u32, 2161460u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2161464u32);
    emu.adi_no_count(12usize, 18usize, 0u32, 2161468u32);
    emu.adi_no_count(13usize, 19usize, 0u32, 2161472u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2161476u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2161676u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fc0c));
}
#[inline(always)]
pub fn block_0x0020fb44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(17usize, 28usize, 16usize, 2161480u32);
    emu.sltru_no_count(16usize, 16usize, 17usize, 2161484u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2161328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fab0));
    } else {
        emu.pc = 2161488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fb50));
    }
}
#[inline(always)]
pub fn block_0x0020fb50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 10usize, 0u32, 2161492u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2161492u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fb54));
}
#[inline]
pub fn block_0x0020fb54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2161496u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2161500u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2161504u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2161508u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2161512u32)?;
    emu.lw_no_count(20usize, 2usize, 24u32, 2161516u32)?;
    emu.lw_no_count(21usize, 2usize, 20u32, 2161520u32)?;
    emu.lw_no_count(22usize, 2usize, 16u32, 2161524u32)?;
    emu.lw_no_count(23usize, 2usize, 12u32, 2161528u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2161532u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161536u32;
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
pub fn block_0x0020fb80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2161636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fbe4));
    } else {
        emu.pc = 2161540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fb84));
    }
}
#[inline(always)]
pub fn block_0x0020fb84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 0usize, 49u32, 2161544u32);
    emu.adi_no_count(16usize, 13usize, 4294967295u32, 2161548u32);
    emu.sb_no_count(17usize, 11usize, 0u32, 2161552u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2161644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fbec));
    } else {
        emu.pc = 2161556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fb94));
    }
}
#[inline]
pub fn block_0x0020fb94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 11usize, 1u32, 2161560u32);
    emu.adi_no_count(18usize, 11usize, 0u32, 2161564u32);
    emu.adi_no_count(11usize, 0usize, 48u32, 2161568u32);
    emu.adi_no_count(9usize, 0usize, 48u32, 2161572u32);
    emu.adi_no_count(19usize, 10usize, 0u32, 2161576u32);
    emu.adi_no_count(10usize, 17usize, 0u32, 2161580u32);
    emu.adi_no_count(20usize, 12usize, 0u32, 2161584u32);
    emu.adi_no_count(12usize, 16usize, 0u32, 2161588u32);
    emu.adi_no_count(21usize, 13usize, 0u32, 2161592u32);
    emu.adi_no_count(23usize, 14usize, 0u32, 2161596u32);
    emu.adi_no_count(22usize, 15usize, 0u32, 2161600u32);
    emu.apc_no_count(1usize, 2161600u32, 4294914048u32, 2161604u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161608u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(976u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020fbc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 22usize, 0u32, 2161612u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2161616u32);
    emu.adi_no_count(14usize, 23usize, 0u32, 2161620u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2161624u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2161628u32);
    emu.adi_no_count(13usize, 21usize, 0u32, 2161632u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2161636u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2161648u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fbf0));
}
#[inline(always)]
pub fn block_0x0020fbe4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 49u32, 2161640u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2161644u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2161648u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fbf0));
}
#[inline(always)]
pub fn block_0x0020fbec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 48u32, 2161648u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2161648u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fbf0));
}
#[inline(always)]
pub fn block_0x0020fbf0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(14usize, 14usize, 1u32, 2161652u32);
    emu.sli_no_count(14usize, 14usize, 16u32, 2161656u32);
    emu.sai_no_count(14usize, 14usize, 1040u32, 2161660u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2161676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fc0c));
    } else {
        emu.pc = 2161664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fc00));
    }
}
#[inline(always)]
pub fn block_0x0020fc00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(15usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2161676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fc0c));
    } else {
        emu.pc = 2161668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fc04));
    }
}
#[inline(always)]
pub fn block_0x0020fc04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(9usize, 8usize, 0u32, 2161672u32);
    emu.adi_no_count(13usize, 13usize, 1u32, 2161676u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2161676u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fc0c));
}
#[inline(always)]
pub fn block_0x0020fc0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a < b {
        emu.pc = 2161724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fc3c));
    } else {
        emu.pc = 2161680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020fc10));
    }
}
#[inline(always)]
pub fn block_0x0020fc10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 10usize, 0u32, 2161684u32)?;
    emu.sw_no_count(13usize, 10usize, 4u32, 2161688u32)?;
    emu.sh_no_count(14usize, 10usize, 8u32, 2161692u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2161696u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2161492u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020fb54));
}
#[inline(always)]
pub fn block_0x0020fc20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2161700u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 456u32, 2161704u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2161708u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2161712u32);
    emu.adi_no_count(12usize, 14usize, 0u32, 2161716u32);
    emu.apc_no_count(1usize, 2161716u32, 16384u32, 2161720u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161724u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(360u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020fc3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2161728u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 472u32, 2161732u32);
    emu.adi_no_count(10usize, 13usize, 0u32, 2161736u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2161740u32);
    emu.adi_no_count(12usize, 14usize, 0u32, 2161744u32);
    emu.apc_no_count(1usize, 2161744u32, 16384u32, 2161748u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161752u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(332u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020fc58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2161756u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1308u32, 2161760u32);
    emu.adi_no_count(10usize, 15usize, 0u32, 2161764u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2161768u32);
    emu.apc_no_count(1usize, 2161768u32, 16384u32, 2161772u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2161776u32;
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
