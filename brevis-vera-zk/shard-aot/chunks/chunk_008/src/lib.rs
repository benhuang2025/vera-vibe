pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2138620u32;
pub const PC_MAX: u32 = 2141044u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 107usize] = [
        block_0x0020a1fc,
        block_0x0020a20c,
        block_0x0020a210,
        block_0x0020a240,
        block_0x0020a250,
        block_0x0020a25c,
        block_0x0020a2cc,
        block_0x0020a300,
        block_0x0020a30c,
        block_0x0020a318,
        block_0x0020a36c,
        block_0x0020a378,
        block_0x0020a390,
        block_0x0020a3b0,
        block_0x0020a3c0,
        block_0x0020a3c4,
        block_0x0020a3e8,
        block_0x0020a3f8,
        block_0x0020a404,
        block_0x0020a414,
        block_0x0020a42c,
        block_0x0020a444,
        block_0x0020a448,
        block_0x0020a44c,
        block_0x0020a458,
        block_0x0020a45c,
        block_0x0020a468,
        block_0x0020a494,
        block_0x0020a4c4,
        block_0x0020a4e8,
        block_0x0020a538,
        block_0x0020a544,
        block_0x0020a550,
        block_0x0020a5a0,
        block_0x0020a5a8,
        block_0x0020a5c8,
        block_0x0020a5e8,
        block_0x0020a630,
        block_0x0020a63c,
        block_0x0020a648,
        block_0x0020a664,
        block_0x0020a668,
        block_0x0020a688,
        block_0x0020a68c,
        block_0x0020a694,
        block_0x0020a6a0,
        block_0x0020a708,
        block_0x0020a754,
        block_0x0020a76c,
        block_0x0020a778,
        block_0x0020a77c,
        block_0x0020a7b0,
        block_0x0020a7c8,
        block_0x0020a7cc,
        block_0x0020a7dc,
        block_0x0020a7e4,
        block_0x0020a814,
        block_0x0020a81c,
        block_0x0020a824,
        block_0x0020a828,
        block_0x0020a834,
        block_0x0020a844,
        block_0x0020a898,
        block_0x0020a8a8,
        block_0x0020a8ac,
        block_0x0020a8b8,
        block_0x0020a8d4,
        block_0x0020a8f0,
        block_0x0020a8f4,
        block_0x0020a910,
        block_0x0020a914,
        block_0x0020a924,
        block_0x0020a92c,
        block_0x0020a934,
        block_0x0020a944,
        block_0x0020a964,
        block_0x0020a978,
        block_0x0020a98c,
        block_0x0020a9a4,
        block_0x0020a9cc,
        block_0x0020a9ec,
        block_0x0020a9f4,
        block_0x0020aa04,
        block_0x0020aa08,
        block_0x0020aa0c,
        block_0x0020aa10,
        block_0x0020aa1c,
        block_0x0020aa28,
        block_0x0020aa34,
        block_0x0020aa38,
        block_0x0020aa48,
        block_0x0020aa5c,
        block_0x0020aa70,
        block_0x0020aa78,
        block_0x0020aa8c,
        block_0x0020aa90,
        block_0x0020aa94,
        block_0x0020aa98,
        block_0x0020aaa8,
        block_0x0020aac4,
        block_0x0020aad8,
        block_0x0020aaec,
        block_0x0020ab10,
        block_0x0020ab14,
        block_0x0020ab4c,
        block_0x0020ab60,
        block_0x0020ab74,
    ];
    const IDX: [u16; 607usize] = [
        1u16, 0u16, 0u16, 0u16, 2u16, 3u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 4u16, 0u16, 0u16, 0u16, 5u16, 0u16, 0u16, 6u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        7u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        8u16, 0u16, 0u16, 9u16, 0u16, 0u16, 10u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 11u16, 0u16, 0u16, 12u16, 0u16, 0u16, 0u16, 0u16, 0u16, 13u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 14u16, 0u16, 0u16, 0u16, 15u16, 16u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 17u16, 0u16, 0u16, 0u16, 18u16, 0u16, 0u16,
        19u16, 0u16, 0u16, 0u16, 20u16, 0u16, 0u16, 0u16, 0u16, 0u16, 21u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 22u16, 23u16, 24u16, 0u16, 0u16, 25u16, 26u16, 0u16, 0u16,
        27u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 28u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 29u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 30u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        31u16, 0u16, 0u16, 32u16, 0u16, 0u16, 33u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        34u16, 0u16, 35u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 36u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 37u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 38u16, 0u16, 0u16,
        39u16, 0u16, 0u16, 40u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 41u16, 42u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 43u16, 44u16, 0u16, 45u16, 0u16, 0u16, 46u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 47u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 48u16, 0u16, 0u16, 0u16, 0u16, 0u16, 49u16, 0u16,
        0u16, 50u16, 51u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 52u16, 0u16, 0u16, 0u16, 0u16, 0u16, 53u16, 54u16, 0u16, 0u16, 0u16,
        55u16, 0u16, 56u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 57u16, 0u16, 58u16, 0u16, 59u16, 60u16, 0u16, 0u16, 61u16, 0u16, 0u16,
        0u16, 62u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 63u16, 0u16, 0u16, 0u16,
        64u16, 65u16, 0u16, 0u16, 66u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 67u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 68u16, 69u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        70u16, 71u16, 0u16, 0u16, 0u16, 72u16, 0u16, 73u16, 0u16, 74u16, 0u16, 0u16,
        0u16, 75u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 76u16, 0u16, 0u16, 0u16,
        0u16, 77u16, 0u16, 0u16, 0u16, 0u16, 78u16, 0u16, 0u16, 0u16, 0u16, 0u16, 79u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 80u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 81u16, 0u16, 82u16, 0u16, 0u16, 0u16, 83u16, 84u16,
        85u16, 86u16, 0u16, 0u16, 87u16, 0u16, 0u16, 88u16, 0u16, 0u16, 89u16, 90u16,
        0u16, 0u16, 0u16, 91u16, 0u16, 0u16, 0u16, 0u16, 92u16, 0u16, 0u16, 0u16, 0u16,
        93u16, 0u16, 94u16, 0u16, 0u16, 0u16, 0u16, 95u16, 96u16, 97u16, 98u16, 0u16,
        0u16, 0u16, 99u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 100u16, 0u16, 0u16, 0u16,
        0u16, 101u16, 0u16, 0u16, 0u16, 0u16, 102u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 103u16, 104u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 105u16, 0u16, 0u16, 0u16, 0u16, 106u16, 0u16, 0u16, 0u16,
        0u16, 107u16,
    ];
    if pc < 2138620u32 || pc > 2141044u32 {
        return None;
    }
    let word_offset = ((pc - 2138620u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x0020a1fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 12u32, 2138624u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2138628u32);
    emu.apc_no_count(1usize, 2138628u32, 4294930432u32, 2138632u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138636u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(748u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a20c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2138688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a240));
    } else {
        emu.pc = 2138640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a210));
    }
}
#[inline]
pub fn block_0x0020a210(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 2usize, 8u32, 2138644u32)?;
    emu.lw_no_count(13usize, 2usize, 12u32, 2138648u32)?;
    emu.lw_no_count(14usize, 2usize, 16u32, 2138652u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2138656u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966744u32, 2138660u32);
    emu.sw_no_count(12usize, 10usize, 0u32, 2138664u32)?;
    emu.sw_no_count(13usize, 10usize, 4u32, 2138668u32)?;
    emu.sw_no_count(14usize, 10usize, 8u32, 2138672u32)?;
    emu.lw_no_count(1usize, 2usize, 60u32, 2138676u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2138680u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2138684u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138688u32;
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
pub fn block_0x0020a240(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2138692u32);
    emu.adi_no_count(11usize, 0usize, 12u32, 2138696u32);
    emu.apc_no_count(1usize, 2138696u32, 4096u32, 2138700u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138704u32;
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
pub fn block_0x0020a250(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2138708u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2138712u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2138880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a300));
    } else {
        emu.pc = 2138716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a25c));
    }
}
#[inline(never)]
pub fn block_0x0020a25c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2138720u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2138724u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2138728u32)?;
    emu.lw_no_count(11usize, 10usize, 12u32, 2138732u32)?;
    emu.adi_no_count(12usize, 0usize, 1u32, 2138736u32);
    emu.sw_no_count(0usize, 2usize, 4u32, 2138740u32)?;
    emu.sw_no_count(12usize, 2usize, 8u32, 2138744u32)?;
    emu.sw_no_count(0usize, 2usize, 12u32, 2138748u32)?;
    emu.lw_no_count(11usize, 11usize, 0u32, 2138752u32)?;
    emu.lw_no_count(12usize, 11usize, 0u32, 2138756u32)?;
    emu.lw_no_count(13usize, 11usize, 4u32, 2138760u32)?;
    emu.lw_no_count(14usize, 11usize, 8u32, 2138764u32)?;
    emu.lw_no_count(15usize, 11usize, 12u32, 2138768u32)?;
    emu.lw_no_count(16usize, 11usize, 16u32, 2138772u32)?;
    emu.lw_no_count(11usize, 11usize, 20u32, 2138776u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2138780u32)?;
    emu.sw_no_count(13usize, 2usize, 20u32, 2138784u32)?;
    emu.sw_no_count(14usize, 2usize, 24u32, 2138788u32)?;
    emu.sw_no_count(15usize, 2usize, 28u32, 2138792u32)?;
    emu.sw_no_count(16usize, 2usize, 32u32, 2138796u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2138800u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2138804u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966720u32, 2138808u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2138812u32);
    emu.adi_no_count(10usize, 2usize, 4u32, 2138816u32);
    emu.adi_no_count(12usize, 2usize, 16u32, 2138820u32);
    emu.apc_no_count(1usize, 2138820u32, 24576u32, 2138824u32);
    emu.add_memory_rw_events(28usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138828u32;
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
pub fn block_0x0020a2cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 0u32, 2138832u32);
    emu.lw_no_count(11usize, 2usize, 4u32, 2138836u32)?;
    emu.lw_no_count(12usize, 2usize, 8u32, 2138840u32)?;
    emu.lw_no_count(13usize, 2usize, 12u32, 2138844u32)?;
    emu.sw_no_count(11usize, 2usize, 16u32, 2138848u32)?;
    emu.sw_no_count(12usize, 2usize, 20u32, 2138852u32)?;
    emu.sw_no_count(13usize, 2usize, 24u32, 2138856u32)?;
    emu.sw_no_count(11usize, 8usize, 0u32, 2138860u32)?;
    emu.sw_no_count(12usize, 8usize, 4u32, 2138864u32)?;
    emu.sw_no_count(13usize, 8usize, 8u32, 2138868u32)?;
    emu.lw_no_count(1usize, 2usize, 44u32, 2138872u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2138876u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2138880u32);
    emu.add_memory_rw_events(13usize);
    emu.pc = 2138880u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a300));
}
#[inline(always)]
pub fn block_0x0020a300(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2138884u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966744u32, 2138888u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138892u32;
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
pub fn block_0x0020a30c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2138896u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2138900u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2139000u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a378));
    } else {
        emu.pc = 2138904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a318));
    }
}
#[inline]
pub fn block_0x0020a318(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2138908u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2138912u32)?;
    emu.lw_no_count(10usize, 10usize, 12u32, 2138916u32)?;
    emu.lw_no_count(12usize, 10usize, 0u32, 2138920u32)?;
    emu.lw_no_count(10usize, 11usize, 0u32, 2138924u32)?;
    emu.lw_no_count(11usize, 11usize, 4u32, 2138928u32)?;
    emu.lw_no_count(13usize, 12usize, 0u32, 2138932u32)?;
    emu.lw_no_count(14usize, 12usize, 4u32, 2138936u32)?;
    emu.lw_no_count(15usize, 12usize, 8u32, 2138940u32)?;
    emu.lw_no_count(16usize, 12usize, 12u32, 2138944u32)?;
    emu.lw_no_count(17usize, 12usize, 16u32, 2138948u32)?;
    emu.lw_no_count(12usize, 12usize, 20u32, 2138952u32)?;
    emu.sw_no_count(13usize, 2usize, 4u32, 2138956u32)?;
    emu.sw_no_count(14usize, 2usize, 8u32, 2138960u32)?;
    emu.sw_no_count(15usize, 2usize, 12u32, 2138964u32)?;
    emu.sw_no_count(16usize, 2usize, 16u32, 2138968u32)?;
    emu.sw_no_count(17usize, 2usize, 20u32, 2138972u32)?;
    emu.sw_no_count(12usize, 2usize, 24u32, 2138976u32)?;
    emu.adi_no_count(12usize, 2usize, 4u32, 2138980u32);
    emu.apc_no_count(1usize, 2138980u32, 24576u32, 2138984u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2138988u32;
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
pub fn block_0x0020a36c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2138992u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2138996u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139000u32;
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
pub fn block_0x0020a378(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 10usize, 4u32, 2139004u32)?;
    emu.lw_no_count(12usize, 10usize, 8u32, 2139008u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2139012u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2139016u32);
    emu.apc_no_count(6usize, 2139016u32, 28672u32, 2139020u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2139024u32;
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
pub fn block_0x0020a390(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2139028u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2139032u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2139036u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2139040u32)?;
    emu.lw_no_count(8usize, 10usize, 0u32, 2139044u32)?;
    emu.lw_no_count(9usize, 10usize, 4u32, 2139048u32)?;
    emu.apc_no_count(1usize, 2139048u32, 4294934528u32, 2139052u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139056u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(880u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a3b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 8u32, 2139060u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2139064u32);
    emu.apc_no_count(1usize, 2139064u32, 4294930432u32, 2139068u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139072u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(312u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a3c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2139112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a3e8));
    } else {
        emu.pc = 2139076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a3c4));
    }
}
#[inline]
pub fn block_0x0020a3c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2139080u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966760u32, 2139084u32);
    emu.sw_no_count(8usize, 10usize, 0u32, 2139088u32)?;
    emu.sw_no_count(9usize, 10usize, 4u32, 2139092u32)?;
    emu.lw_no_count(1usize, 2usize, 12u32, 2139096u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2139100u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2139104u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2139108u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139112u32;
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
pub fn block_0x0020a3e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2139116u32);
    emu.adi_no_count(11usize, 0usize, 8u32, 2139120u32);
    emu.apc_no_count(1usize, 2139120u32, 4096u32, 2139124u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139128u32;
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
pub fn block_0x0020a3f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2139132u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966760u32, 2139136u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139140u32;
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
pub fn block_0x0020a404(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2139144u32)?;
    emu.lw_no_count(11usize, 10usize, 4u32, 2139148u32)?;
    emu.adi_no_count(10usize, 12usize, 0u32, 2139152u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139156u32;
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
pub fn block_0x0020a414(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 10usize, 0u32, 2139160u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2139164u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2139168u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2139172u32);
    emu.apc_no_count(6usize, 2139172u32, 28672u32, 2139176u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2139180u32;
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
pub fn block_0x0020a42c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2139184u32);
    emu.lw_no_count(11usize, 10usize, 0u32, 2139188u32)?;
    emu.lw_no_count(13usize, 11usize, 4u32, 2139192u32)?;
    emu.lw_no_count(12usize, 11usize, 12u32, 2139196u32)?;
    emu.adi_no_count(14usize, 0usize, 1u32, 2139200u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2139224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a458));
    } else {
        emu.pc = 2139204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a444));
    }
}
#[inline(always)]
pub fn block_0x0020a444(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2139284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a494));
    } else {
        emu.pc = 2139208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a448));
    }
}
#[inline(always)]
pub fn block_0x0020a448(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2139284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a494));
    } else {
        emu.pc = 2139212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a44c));
    }
}
#[inline(always)]
pub fn block_0x0020a44c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2139216u32);
    emu.adi_no_count(15usize, 0usize, 1u32, 2139220u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2139224u32;
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
pub fn block_0x0020a458(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2139284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a494));
    } else {
        emu.pc = 2139228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a45c));
    }
}
#[inline(always)]
pub fn block_0x0020a45c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 11usize, 0u32, 2139232u32)?;
    emu.lw_no_count(15usize, 11usize, 0u32, 2139236u32)?;
    emu.lw_no_count(11usize, 11usize, 4u32, 2139240u32)?;
    emu.add_memory_rw_events(3usize);
    emu.pc = 2139240u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a468));
}
#[inline]
pub fn block_0x0020a468(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(14usize, 10usize, 8u32, 2139244u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2139248u32)?;
    emu.lbu_no_count(13usize, 14usize, 8u32, 2139252u32);
    emu.lbu_no_count(14usize, 14usize, 9u32, 2139256u32);
    emu.sw_no_count(15usize, 2usize, 0u32, 2139260u32)?;
    emu.sw_no_count(11usize, 2usize, 4u32, 2139264u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2139268u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966776u32, 2139272u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2139276u32);
    emu.apc_no_count(1usize, 2139276u32, 0u32, 2139280u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139284u32;
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
pub fn block_0x0020a494(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 8u32, 2139288u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2139292u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2139296u32)?;
    emu.lbu_no_count(13usize, 11usize, 8u32, 2139300u32);
    emu.lbu_no_count(14usize, 11usize, 9u32, 2139304u32);
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2139308u32;
    emu.update_insn_clock();
    emu.sw_no_count(10usize, 2usize, 0u32, 2139312u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2139316u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966804u32, 2139320u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2139324u32);
    emu.apc_no_count(1usize, 2139324u32, 0u32, 2139328u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139332u32;
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
pub fn block_0x0020a4c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2139336u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2139340u32)?;
    emu.sw_no_count(8usize, 2usize, 24u32, 2139344u32)?;
    emu.sw_no_count(9usize, 2usize, 20u32, 2139348u32)?;
    emu.lw_no_count(9usize, 11usize, 12u32, 2139352u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2139356u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2139360u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2139364u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(9usize);
    let return_addr = 2139368u32;
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
pub fn block_0x0020a4e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2139372u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2139376u32)?;
    emu.lw_no_count(12usize, 2usize, 0u32, 2139380u32)?;
    emu.lw_no_count(13usize, 2usize, 4u32, 2139384u32)?;
    let a = 0u32.wrapping_add(3112902656u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2139388u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966129u32, 2139392u32);
    emu.xrr_no_count(11usize, 11usize, 14usize, 2139396u32);
    let a = 0u32.wrapping_add(1676365824u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2139400u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 44u32, 2139404u32);
    emu.xrr_no_count(13usize, 13usize, 14usize, 2139408u32);
    let a = 0u32.wrapping_add(1470513152u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2139412u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 376u32, 2139416u32);
    emu.xrr_no_count(10usize, 10usize, 14usize, 2139420u32);
    let a = 0u32.wrapping_add(3603652608u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2139424u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966637u32, 2139428u32);
    emu.xrr_no_count(12usize, 12usize, 14usize, 2139432u32);
    emu.orr_no_count(11usize, 13usize, 11usize, 2139436u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2139440u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2139444u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2139460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a544));
    } else {
        emu.pc = 2139448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a538));
    }
}
#[inline(always)]
pub fn block_0x0020a538(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 4u32, 2139452u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2139456u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2139460u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2139560u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a5a8));
}
#[inline(always)]
pub fn block_0x0020a544(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2139464u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2139468u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(9usize);
    let return_addr = 2139472u32;
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
pub fn block_0x0020a550(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2139476u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2139480u32)?;
    emu.lw_no_count(12usize, 2usize, 0u32, 2139484u32)?;
    emu.lw_no_count(13usize, 2usize, 4u32, 2139488u32)?;
    let a = 0u32.wrapping_add(2330587136u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2139492u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 428u32, 2139496u32);
    emu.xrr_no_count(11usize, 11usize, 14usize, 2139500u32);
    let a = 0u32.wrapping_add(2965131264u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2139504u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965685u32, 2139508u32);
    emu.xrr_no_count(13usize, 13usize, 14usize, 2139512u32);
    let a = 0u32.wrapping_add(4112453632u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2139516u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1473u32, 2139520u32);
    emu.xrr_no_count(10usize, 10usize, 14usize, 2139524u32);
    let a = 0u32.wrapping_add(2020298752u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2139528u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965580u32, 2139532u32);
    emu.xrr_no_count(12usize, 12usize, 14usize, 2139536u32);
    emu.orr_no_count(11usize, 13usize, 11usize, 2139540u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2139544u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2139548u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2139592u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a5c8));
    } else {
        emu.pc = 2139552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a5a0));
    }
}
#[inline(always)]
pub fn block_0x0020a5a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 4u32, 2139556u32);
    emu.adi_no_count(11usize, 0usize, 8u32, 2139560u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2139560u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a5a8));
}
#[inline(always)]
pub fn block_0x0020a5a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2139564u32)?;
    emu.adr_no_count(11usize, 8usize, 11usize, 2139568u32);
    emu.lw_no_count(11usize, 11usize, 0u32, 2139572u32)?;
    emu.lw_no_count(1usize, 2usize, 28u32, 2139576u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2139580u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2139584u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2139588u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139592u32;
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
pub fn block_0x0020a5c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2139596u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966832u32, 2139600u32);
    emu.adi_no_count(11usize, 0usize, 12u32, 2139604u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2139608u32)?;
    emu.lw_no_count(8usize, 2usize, 24u32, 2139612u32)?;
    emu.lw_no_count(9usize, 2usize, 20u32, 2139616u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2139620u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139624u32;
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
pub fn block_0x0020a5e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967200u32, 2139628u32);
    emu.sw_no_count(1usize, 2usize, 92u32, 2139632u32)?;
    emu.sw_no_count(8usize, 2usize, 88u32, 2139636u32)?;
    emu.sw_no_count(9usize, 2usize, 84u32, 2139640u32)?;
    emu.sw_no_count(18usize, 2usize, 80u32, 2139644u32)?;
    emu.sw_no_count(19usize, 2usize, 76u32, 2139648u32)?;
    emu.sw_no_count(20usize, 2usize, 72u32, 2139652u32)?;
    emu.adi_no_count(19usize, 14usize, 0u32, 2139656u32);
    emu.adi_no_count(18usize, 13usize, 0u32, 2139660u32);
    emu.adi_no_count(20usize, 12usize, 0u32, 2139664u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2139668u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2139672u32);
    emu.sw_no_count(10usize, 2usize, 4u32, 2139676u32)?;
    emu.sw_no_count(11usize, 2usize, 8u32, 2139680u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2139684u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2139688u32);
    emu.apc_no_count(1usize, 2139688u32, 4096u32, 2139692u32);
    emu.add_memory_rw_events(18usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139696u32;
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
pub fn block_0x0020a630(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 255u32, 2139700u32);
    emu.adi_no_count(11usize, 0usize, 2u32, 2139704u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2139788u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a68c));
    } else {
        emu.pc = 2139708u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a63c));
    }
}
#[inline(always)]
pub fn block_0x0020a63c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2139712u32;
    emu.update_insn_clock();
    emu.lw_no_count(11usize, 10usize, 4294966692u32, 2139716u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2140200u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a828));
    } else {
        emu.pc = 2139720u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a648));
    }
}
#[inline(always)]
pub fn block_0x0020a648(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 4294966692u32, 2139724u32);
    emu.lw_no_count(12usize, 12usize, 4u32, 2139728u32)?;
    emu.adi_no_count(11usize, 11usize, 1u32, 2139732u32);
    emu.sw_no_count(11usize, 10usize, 4294966692u32, 2139736u32)?;
    emu.lw_no_count(11usize, 9usize, 20u32, 2139740u32)?;
    emu.adi_no_count(10usize, 8usize, 0u32, 2139744u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2140024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a778));
    } else {
        emu.pc = 2139748u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a664));
    }
}
#[inline(always)]
pub fn block_0x0020a664(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2139752u32;
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
pub fn block_0x0020a668(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 48u32, 2139756u32)?;
    emu.sw_no_count(11usize, 2usize, 52u32, 2139760u32)?;
    emu.sw_no_count(20usize, 2usize, 56u32, 2139764u32)?;
    emu.sb_no_count(18usize, 2usize, 60u32, 2139768u32);
    emu.sb_no_count(19usize, 2usize, 61u32, 2139772u32);
    emu.adi_no_count(10usize, 2usize, 48u32, 2139776u32);
    emu.apc_no_count(1usize, 2139776u32, 0u32, 2139780u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2139784u32;
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
pub fn block_0x0020a688(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2139788u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2140080u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a7b0));
}
#[inline(always)]
pub fn block_0x0020a68c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 1u32, 2139792u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2139912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a708));
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
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 9usize, 24u32, 2139800u32)?;
    emu.adi_no_count(10usize, 8usize, 0u32, 2139804u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2139808u32;
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
pub fn block_0x0020a6a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 26u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(12usize, 10usize, 1u32, 2139812u32);
    emu.adi_no_count(13usize, 2usize, 12u32, 2139816u32);
    let a = 0u32.wrapping_add(2134016u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2139820u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 72u32, 2139824u32);
    emu.adi_no_count(15usize, 2usize, 16u32, 2139828u32);
    let a = 0u32.wrapping_add(2142208u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2139832u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 4294966656u32, 2139836u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2139840u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 17usize, 4294966960u32, 2139844u32);
    emu.sw_no_count(13usize, 2usize, 32u32, 2139848u32)?;
    emu.sw_no_count(14usize, 2usize, 36u32, 2139852u32)?;
    emu.sw_no_count(15usize, 2usize, 40u32, 2139856u32)?;
    emu.sw_no_count(16usize, 2usize, 44u32, 2139860u32)?;
    emu.adi_no_count(13usize, 0usize, 3u32, 2139864u32);
    emu.sw_no_count(0usize, 2usize, 64u32, 2139868u32)?;
    emu.adr_no_count(10usize, 10usize, 12usize, 2139872u32);
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2139876u32);
    emu.anr_no_count(11usize, 12usize, 11usize, 2139880u32);
    emu.adi_no_count(12usize, 2usize, 32u32, 2139884u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2139888u32)?;
    emu.sw_no_count(11usize, 2usize, 20u32, 2139892u32)?;
    emu.adi_no_count(10usize, 0usize, 2u32, 2139896u32);
    emu.sw_no_count(17usize, 2usize, 48u32, 2139900u32)?;
    emu.sw_no_count(13usize, 2usize, 52u32, 2139904u32)?;
    emu.sw_no_count(12usize, 2usize, 56u32, 2139908u32)?;
    emu.add_memory_rw_events(26usize);
    let return_addr = 2139912u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2139988u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a754));
}
#[inline]
pub fn block_0x0020a708(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 12u32, 2139916u32);
    let a = 0u32.wrapping_add(2134016u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2139920u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 72u32, 2139924u32);
    emu.adi_no_count(12usize, 2usize, 4u32, 2139928u32);
    let a = 0u32.wrapping_add(2142208u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2139932u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966984u32, 2139936u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2139940u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966872u32, 2139944u32);
    emu.adi_no_count(15usize, 0usize, 3u32, 2139948u32);
    emu.sw_no_count(0usize, 2usize, 64u32, 2139952u32)?;
    emu.adi_no_count(16usize, 2usize, 32u32, 2139956u32);
    emu.sw_no_count(10usize, 2usize, 32u32, 2139960u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2139964u32)?;
    emu.sw_no_count(12usize, 2usize, 40u32, 2139968u32)?;
    emu.sw_no_count(13usize, 2usize, 44u32, 2139972u32)?;
    emu.adi_no_count(10usize, 0usize, 2u32, 2139976u32);
    emu.sw_no_count(14usize, 2usize, 48u32, 2139980u32)?;
    emu.sw_no_count(15usize, 2usize, 52u32, 2139984u32)?;
    emu.sw_no_count(16usize, 2usize, 56u32, 2139988u32)?;
    emu.add_memory_rw_events(19usize);
    emu.pc = 2139988u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a754));
}
#[inline(always)]
pub fn block_0x0020a754(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 60u32, 2139992u32)?;
    emu.adi_no_count(10usize, 2usize, 24u32, 2139996u32);
    emu.adi_no_count(11usize, 2usize, 24u32, 2140000u32);
    emu.adi_no_count(12usize, 2usize, 48u32, 2140004u32);
    emu.apc_no_count(1usize, 2140004u32, 0u32, 2140008u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140012u32;
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
pub fn block_0x0020a76c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 24u32, 2140016u32);
    emu.lw_no_count(11usize, 2usize, 28u32, 2140020u32)?;
    emu.add_memory_rw_events(3usize);
    let return_addr = 2140024u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2140188u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a81c));
}
#[inline(always)]
pub fn block_0x0020a778(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(11usize);
    let return_addr = 2140028u32;
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
pub fn block_0x0020a77c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2140032u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966692u32, 2140036u32);
    emu.lw_no_count(13usize, 12usize, 8u32, 2140040u32)?;
    emu.lw_no_count(12usize, 12usize, 4u32, 2140044u32)?;
    emu.lw_no_count(13usize, 13usize, 20u32, 2140048u32)?;
    emu.sw_no_count(10usize, 2usize, 48u32, 2140052u32)?;
    emu.sw_no_count(11usize, 2usize, 52u32, 2140056u32)?;
    emu.sw_no_count(20usize, 2usize, 56u32, 2140060u32)?;
    emu.sb_no_count(18usize, 2usize, 60u32, 2140064u32);
    emu.sb_no_count(19usize, 2usize, 61u32, 2140068u32);
    emu.adi_no_count(11usize, 2usize, 48u32, 2140072u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2140076u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2140080u32;
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
pub fn block_0x0020a7b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2203648u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2140084u32;
    emu.update_insn_clock();
    emu.lw_no_count(11usize, 10usize, 4294966692u32, 2140088u32)?;
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2140092u32);
    emu.sw_no_count(11usize, 10usize, 4294966692u32, 2140096u32)?;
    emu.apc_no_count(1usize, 2140096u32, 4096u32, 2140100u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140104u32;
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
pub fn block_0x0020a7c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2140124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a7dc));
    } else {
        emu.pc = 2140108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a7cc));
    }
}
#[inline(always)]
pub fn block_0x0020a7cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 0u32, 2140112u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2140116u32);
    emu.apc_no_count(1usize, 2140116u32, 0u32, 2140120u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140124u32;
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
pub fn block_0x0020a7dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2140128u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294967032u32, 2140132u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2140132u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a7e4));
}
#[inline]
pub fn block_0x0020a7e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2140136u32);
    emu.sw_no_count(0usize, 2usize, 64u32, 2140140u32)?;
    emu.adi_no_count(12usize, 0usize, 4u32, 2140144u32);
    emu.sw_no_count(10usize, 2usize, 48u32, 2140148u32)?;
    emu.sw_no_count(11usize, 2usize, 52u32, 2140152u32)?;
    emu.sw_no_count(12usize, 2usize, 56u32, 2140156u32)?;
    emu.sw_no_count(0usize, 2usize, 60u32, 2140160u32)?;
    emu.adi_no_count(10usize, 2usize, 32u32, 2140164u32);
    emu.adi_no_count(11usize, 2usize, 24u32, 2140168u32);
    emu.adi_no_count(12usize, 2usize, 48u32, 2140172u32);
    emu.apc_no_count(1usize, 2140172u32, 0u32, 2140176u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140180u32;
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
pub fn block_0x0020a814(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 32u32, 2140184u32);
    emu.lw_no_count(11usize, 2usize, 36u32, 2140188u32)?;
    emu.add_memory_rw_events(2usize);
    emu.pc = 2140188u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a81c));
}
#[inline(always)]
pub fn block_0x0020a81c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2140188u32, 4294963200u32, 2140192u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140196u32;
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
pub fn block_0x0020a824(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.update_insn_clock();
    return Err(format!("Unimplemented instruction (UNIMP) at PC {:#x}", 2140196u32));
}
#[inline(always)]
pub fn block_0x0020a828(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2140204u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294967180u32, 2140208u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2140212u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2140132u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a7e4));
}
#[inline(always)]
pub fn block_0x0020a834(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2140216u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2140220u32)?;
    emu.apc_no_count(1usize, 2140220u32, 4294934528u32, 2140224u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140228u32;
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
pub fn block_0x0020a844(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 12u32, 2140232u32)?;
    emu.adi_no_count(10usize, 2usize, 12u32, 2140236u32);
    let a = 0u32.wrapping_add(2146304u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2140240u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966704u32, 2140244u32);
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2140248u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967104u32, 2140252u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2140256u32);
    emu.sw_no_count(0usize, 2usize, 40u32, 2140260u32)?;
    emu.adi_no_count(14usize, 2usize, 48u32, 2140264u32);
    emu.sw_no_count(10usize, 2usize, 48u32, 2140268u32)?;
    emu.sw_no_count(11usize, 2usize, 52u32, 2140272u32)?;
    emu.adi_no_count(10usize, 0usize, 1u32, 2140276u32);
    emu.sw_no_count(12usize, 2usize, 24u32, 2140280u32)?;
    emu.sw_no_count(13usize, 2usize, 28u32, 2140284u32)?;
    emu.sw_no_count(14usize, 2usize, 32u32, 2140288u32)?;
    emu.sw_no_count(10usize, 2usize, 36u32, 2140292u32)?;
    emu.adi_no_count(10usize, 2usize, 16u32, 2140296u32);
    emu.adi_no_count(11usize, 2usize, 59u32, 2140300u32);
    emu.adi_no_count(12usize, 2usize, 24u32, 2140304u32);
    emu.apc_no_count(1usize, 2140304u32, 0u32, 2140308u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140312u32;
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
pub fn block_0x0020a898(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 16u32, 2140316u32);
    emu.lw_no_count(11usize, 2usize, 20u32, 2140320u32)?;
    emu.apc_no_count(1usize, 2140320u32, 4294963200u32, 2140324u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140328u32;
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
pub fn block_0x0020a8a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.update_insn_clock();
    return Err(format!("Unimplemented instruction (UNIMP) at PC {:#x}", 2140328u32));
}
#[inline(always)]
pub fn block_0x0020a8ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 4u32, 2140336u32);
    emu.sb_no_count(11usize, 10usize, 0u32, 2140340u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140344u32;
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
pub fn block_0x0020a8b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2140348u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2140352u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2140356u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2140360u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2140364u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2140368u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140372u32;
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
pub fn block_0x0020a8d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2140376u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2140380u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2140384u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2140388u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2140392u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2140396u32)?;
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(5usize);
    let return_addr = 2140400u32;
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
pub fn block_0x0020a8f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.update_insn_clock();
    return Err(format!("Unimplemented instruction (UNIMP) at PC {:#x}", 2140400u32));
}
#[inline(always)]
pub fn block_0x0020a8f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2140408u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2140412u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2140416u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2140420u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2140424u32);
    emu.adi_no_count(12usize, 12usize, 4294967288u32, 2140428u32);
    emu.sli_no_count(13usize, 13usize, 3u32, 2140432u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2140432u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a910));
}
#[inline(always)]
pub fn block_0x0020a910(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2140460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a92c));
    } else {
        emu.pc = 2140436u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a914));
    }
}
#[inline(always)]
pub fn block_0x0020a914(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(9usize, 12usize, 12u32, 2140440u32)?;
    emu.adi_no_count(12usize, 12usize, 8u32, 2140444u32);
    emu.adi_no_count(13usize, 13usize, 4294967288u32, 2140448u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2140432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a910));
    } else {
        emu.pc = 2140452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a924));
    }
}
#[inline(always)]
pub fn block_0x0020a924(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 12usize, 0u32, 2140456u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2140460u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2140468u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a934));
}
#[inline(always)]
pub fn block_0x0020a92c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2140464u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2140468u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2140468u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a934));
}
#[inline(always)]
pub fn block_0x0020a934(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2140472u32);
    emu.adi_no_count(12usize, 9usize, 0u32, 2140476u32);
    emu.apc_no_count(1usize, 2140476u32, 4294934528u32, 2140480u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140484u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1596u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020a944(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2140488u32);
    emu.sb_no_count(10usize, 8usize, 0u32, 2140492u32);
    emu.sw_no_count(9usize, 8usize, 4u32, 2140496u32)?;
    emu.lw_no_count(1usize, 2usize, 12u32, 2140500u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2140504u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2140508u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2140512u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140516u32;
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
pub fn block_0x0020a964(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2140520u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2140524u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2140528u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2140532u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2140556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a98c));
    } else {
        emu.pc = 2140536u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a978));
    }
}
#[inline(always)]
pub fn block_0x0020a978(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2140540u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2140544u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2140548u32);
    emu.apc_no_count(1usize, 2140548u32, 4294934528u32, 2140552u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140556u32;
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
pub fn block_0x0020a98c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2140560u32);
    emu.sb_no_count(10usize, 8usize, 0u32, 2140564u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2140568u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2140572u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2140576u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140580u32;
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
pub fn block_0x0020a9a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2140584u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2140588u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2140592u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2140596u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2140600u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2140604u32)?;
    emu.sw_no_count(20usize, 2usize, 40u32, 2140608u32)?;
    emu.sw_no_count(21usize, 2usize, 36u32, 2140612u32)?;
    emu.adi_no_count(8usize, 10usize, 0u32, 2140616u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2140688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aa10));
    } else {
        emu.pc = 2140620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a9cc));
    }
}
#[inline(always)]
pub fn block_0x0020a9cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2140624u32);
    let a = 0u32.wrapping_add(536870912u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2140628u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 13usize, 4294967295u32, 2140632u32);
    emu.sli_no_count(11usize, 13usize, 3u32, 2140636u32);
    emu.adi_no_count(19usize, 19usize, 4294967295u32, 2140640u32);
    emu.anr_no_count(14usize, 14usize, 19usize, 2140644u32);
    emu.adi_no_count(14usize, 14usize, 1u32, 2140648u32);
    emu.adi_no_count(15usize, 12usize, 4u32, 2140652u32);
    emu.add_memory_rw_events(8usize);
    emu.pc = 2140652u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a9ec));
}
#[inline(always)]
pub fn block_0x0020a9ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(16usize, 15usize, 0u32, 2140656u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a != b {
        emu.pc = 2140680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aa08));
    } else {
        emu.pc = 2140660u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a9f4));
    }
}
#[inline(always)]
pub fn block_0x0020a9f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 10usize, 1u32, 2140664u32);
    emu.adi_no_count(11usize, 11usize, 4294967288u32, 2140668u32);
    emu.adi_no_count(15usize, 15usize, 8u32, 2140672u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2140652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a9ec));
    } else {
        emu.pc = 2140676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aa04));
    }
}
#[inline(always)]
pub fn block_0x0020aa04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 14usize, 0u32, 2140680u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2140680u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020aa08));
}
#[inline(always)]
pub fn block_0x0020aa08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2141004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ab4c));
    } else {
        emu.pc = 2140684u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aa0c));
    }
}
#[inline(always)]
pub fn block_0x0020aa0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2140700u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aa1c));
    } else {
        emu.pc = 2140688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aa10));
    }
}
#[inline(always)]
pub fn block_0x0020aa10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2140692u32);
    emu.sb_no_count(10usize, 8usize, 0u32, 2140696u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2140700u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2140908u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020aaec));
}
#[inline(always)]
pub fn block_0x0020aa1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(20usize, 10usize, 3u32, 2140704u32);
    emu.adr_no_count(20usize, 12usize, 20usize, 2140708u32);
    emu.sbr_no_count(9usize, 13usize, 10usize, 2140712u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2140712u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020aa28));
}
#[inline(always)]
pub fn block_0x0020aa28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(21usize, 9usize, 3u32, 2140716u32);
    emu.adi_no_count(10usize, 20usize, 4294967288u32, 2140720u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2140724u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2140724u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020aa34));
}
#[inline(always)]
pub fn block_0x0020aa34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2140868u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aac4));
    } else {
        emu.pc = 2140728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aa38));
    }
}
#[inline(always)]
pub fn block_0x0020aa38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 10usize, 12u32, 2140732u32)?;
    emu.adi_no_count(10usize, 10usize, 8u32, 2140736u32);
    emu.adi_no_count(11usize, 11usize, 4294967288u32, 2140740u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2140724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aa34));
    } else {
        emu.pc = 2140744u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aa48));
    }
}
#[inline(always)]
pub fn block_0x0020aa48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 0u32, 2140748u32)?;
    emu.adi_no_count(10usize, 0usize, 2u32, 2140752u32);
    emu.adi_no_count(12usize, 18usize, 0u32, 2140756u32);
    emu.apc_no_count(1usize, 2140756u32, 4294934528u32, 2140760u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140764u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1316u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020aa5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2140768u32);
    emu.adr_no_count(11usize, 9usize, 19usize, 2140772u32);
    emu.anr_no_count(11usize, 11usize, 19usize, 2140776u32);
    emu.adi_no_count(11usize, 11usize, 1u32, 2140780u32);
    emu.adi_no_count(12usize, 20usize, 4u32, 2140784u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2140784u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020aa70));
}
#[inline(always)]
pub fn block_0x0020aa70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 12usize, 0u32, 2140788u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a < b {
        emu.pc = 2140816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aa90));
    } else {
        emu.pc = 2140792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aa78));
    }
}
#[inline(always)]
pub fn block_0x0020aa78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(18usize, 18usize, 13usize, 2140796u32);
    emu.adi_no_count(10usize, 10usize, 1u32, 2140800u32);
    emu.adi_no_count(21usize, 21usize, 4294967288u32, 2140804u32);
    emu.adi_no_count(12usize, 12usize, 8u32, 2140808u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a != b {
        emu.pc = 2140784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aa70));
    } else {
        emu.pc = 2140812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aa8c));
    }
}
#[inline(always)]
pub fn block_0x0020aa8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 11usize, 0u32, 2140816u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2140816u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020aa90));
}
#[inline(always)]
pub fn block_0x0020aa90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2141024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ab60));
    } else {
        emu.pc = 2140820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aa94));
    }
}
#[inline(always)]
pub fn block_0x0020aa94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2140944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ab10));
    } else {
        emu.pc = 2140824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aa98));
    }
}
#[inline(always)]
pub fn block_0x0020aa98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(11usize, 10usize, 3u32, 2140828u32);
    emu.adr_no_count(20usize, 20usize, 11usize, 2140832u32);
    emu.lw_no_count(11usize, 20usize, 4u32, 2140836u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2141044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ab74));
    } else {
        emu.pc = 2140840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aaa8));
    }
}
#[inline(always)]
pub fn block_0x0020aaa8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 20usize, 0u32, 2140844u32)?;
    emu.sbr_no_count(9usize, 9usize, 10usize, 2140848u32);
    emu.sbr_no_count(10usize, 11usize, 18usize, 2140852u32);
    emu.adr_no_count(12usize, 12usize, 18usize, 2140856u32);
    emu.sw_no_count(12usize, 20usize, 0u32, 2140860u32)?;
    emu.sw_no_count(10usize, 20usize, 4u32, 2140864u32)?;
    emu.add_memory_rw_events(7usize);
    let return_addr = 2140868u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2140712u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020aa28));
}
#[inline(always)]
pub fn block_0x0020aac4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2140872u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2140876u32);
    emu.adi_no_count(12usize, 0usize, 0u32, 2140880u32);
    emu.apc_no_count(1usize, 2140880u32, 4294934528u32, 2140884u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140888u32;
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
#[inline(always)]
pub fn block_0x0020aad8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2140892u32;
    emu.update_insn_clock();
    emu.lw_no_count(11usize, 10usize, 32u32, 2140896u32)?;
    emu.lw_no_count(10usize, 10usize, 36u32, 2140900u32)?;
    emu.sw_no_count(11usize, 8usize, 0u32, 2140904u32)?;
    emu.sw_no_count(10usize, 8usize, 4u32, 2140908u32)?;
    emu.add_memory_rw_events(5usize);
    emu.pc = 2140908u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020aaec));
}
#[inline]
pub fn block_0x0020aaec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2140912u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2140916u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2140920u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2140924u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2140928u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2140932u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2140936u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2140940u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140944u32;
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
pub fn block_0x0020ab10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2140688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aa10));
    } else {
        emu.pc = 2140948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ab14));
    }
}
#[inline]
pub fn block_0x0020ab14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2140952u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294967228u32, 2140956u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2140960u32);
    emu.sw_no_count(0usize, 2usize, 28u32, 2140964u32)?;
    emu.adi_no_count(12usize, 0usize, 4u32, 2140968u32);
    emu.sw_no_count(10usize, 2usize, 12u32, 2140972u32)?;
    emu.sw_no_count(11usize, 2usize, 16u32, 2140976u32)?;
    emu.sw_no_count(12usize, 2usize, 20u32, 2140980u32)?;
    emu.sw_no_count(0usize, 2usize, 24u32, 2140984u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2140988u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967236u32, 2140992u32);
    emu.adi_no_count(10usize, 2usize, 12u32, 2140996u32);
    emu.apc_no_count(1usize, 2140996u32, 8192u32, 2141000u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2141004u32;
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
pub fn block_0x0020ab4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2141008u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967268u32, 2141012u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2141016u32);
    emu.apc_no_count(1usize, 2141016u32, 36864u32, 2141020u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2141024u32;
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
pub fn block_0x0020ab60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2141028u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967268u32, 2141032u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2141036u32);
    emu.apc_no_count(1usize, 2141036u32, 36864u32, 2141040u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2141044u32;
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
pub fn block_0x0020ab74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2141048u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294967252u32, 2141052u32);
    emu.adi_no_count(10usize, 18usize, 0u32, 2141056u32);
    emu.apc_no_count(1usize, 2141056u32, 36864u32, 2141060u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2141064u32;
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
