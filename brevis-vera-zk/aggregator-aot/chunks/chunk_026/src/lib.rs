pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2214096u32;
pub const PC_MAX: u32 = 2216692u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 113usize] = [
        block_0x0021c8d0,
        block_0x0021c940,
        block_0x0021c960,
        block_0x0021c978,
        block_0x0021c988,
        block_0x0021c990,
        block_0x0021c9a0,
        block_0x0021c9a8,
        block_0x0021c9b0,
        block_0x0021c9c0,
        block_0x0021c9c8,
        block_0x0021cb00,
        block_0x0021cb10,
        block_0x0021cb18,
        block_0x0021cb28,
        block_0x0021cb34,
        block_0x0021cb44,
        block_0x0021cb50,
        block_0x0021cb60,
        block_0x0021cb64,
        block_0x0021cb74,
        block_0x0021cca8,
        block_0x0021ccbc,
        block_0x0021ccc0,
        block_0x0021ccc8,
        block_0x0021cccc,
        block_0x0021cce0,
        block_0x0021cd00,
        block_0x0021cd1c,
        block_0x0021cd70,
        block_0x0021cd78,
        block_0x0021cdb4,
        block_0x0021cdc0,
        block_0x0021cdd4,
        block_0x0021cdd8,
        block_0x0021cde0,
        block_0x0021cde4,
        block_0x0021cdfc,
        block_0x0021ce00,
        block_0x0021ce14,
        block_0x0021ce20,
        block_0x0021ce28,
        block_0x0021ce34,
        block_0x0021ce38,
        block_0x0021ce3c,
        block_0x0021ce68,
        block_0x0021ce6c,
        block_0x0021ce90,
        block_0x0021ce94,
        block_0x0021ce9c,
        block_0x0021cec8,
        block_0x0021ced4,
        block_0x0021cee8,
        block_0x0021ceec,
        block_0x0021cefc,
        block_0x0021cf00,
        block_0x0021cf20,
        block_0x0021cf28,
        block_0x0021cf2c,
        block_0x0021cf38,
        block_0x0021cf5c,
        block_0x0021cf60,
        block_0x0021cf68,
        block_0x0021cf6c,
        block_0x0021cfa0,
        block_0x0021cff0,
        block_0x0021d004,
        block_0x0021d008,
        block_0x0021d018,
        block_0x0021d020,
        block_0x0021d028,
        block_0x0021d030,
        block_0x0021d048,
        block_0x0021d054,
        block_0x0021d05c,
        block_0x0021d078,
        block_0x0021d07c,
        block_0x0021d084,
        block_0x0021d098,
        block_0x0021d0a4,
        block_0x0021d0c8,
        block_0x0021d0cc,
        block_0x0021d118,
        block_0x0021d128,
        block_0x0021d134,
        block_0x0021d138,
        block_0x0021d17c,
        block_0x0021d184,
        block_0x0021d198,
        block_0x0021d1a0,
        block_0x0021d1b4,
        block_0x0021d1b8,
        block_0x0021d1bc,
        block_0x0021d1c8,
        block_0x0021d1d4,
        block_0x0021d1e8,
        block_0x0021d1ec,
        block_0x0021d1fc,
        block_0x0021d200,
        block_0x0021d21c,
        block_0x0021d224,
        block_0x0021d238,
        block_0x0021d23c,
        block_0x0021d240,
        block_0x0021d280,
        block_0x0021d29c,
        block_0x0021d2a4,
        block_0x0021d2b8,
        block_0x0021d2bc,
        block_0x0021d2d0,
        block_0x0021d2dc,
        block_0x0021d2f0,
        block_0x0021d2f4,
    ];
    const IDX: [u16; 650usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 2u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 3u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 4u16, 0u16, 0u16, 0u16, 5u16, 0u16, 6u16, 0u16, 0u16, 0u16,
        7u16, 0u16, 8u16, 0u16, 9u16, 0u16, 0u16, 0u16, 10u16, 0u16, 11u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 12u16, 0u16, 0u16,
        0u16, 13u16, 0u16, 14u16, 0u16, 0u16, 0u16, 15u16, 0u16, 0u16, 16u16, 0u16, 0u16,
        0u16, 17u16, 0u16, 0u16, 18u16, 0u16, 0u16, 0u16, 19u16, 20u16, 0u16, 0u16, 0u16,
        21u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 22u16,
        0u16, 0u16, 0u16, 0u16, 23u16, 24u16, 0u16, 25u16, 26u16, 0u16, 0u16, 0u16, 0u16,
        27u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 28u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 29u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 30u16, 0u16, 31u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 32u16, 0u16, 0u16, 33u16, 0u16, 0u16, 0u16, 0u16, 34u16, 35u16, 0u16,
        36u16, 37u16, 0u16, 0u16, 0u16, 0u16, 0u16, 38u16, 39u16, 0u16, 0u16, 0u16, 0u16,
        40u16, 0u16, 0u16, 41u16, 0u16, 42u16, 0u16, 0u16, 43u16, 44u16, 45u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 46u16, 47u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 48u16, 49u16, 0u16, 50u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 51u16, 0u16, 0u16, 52u16, 0u16, 0u16,
        0u16, 0u16, 53u16, 54u16, 0u16, 0u16, 0u16, 55u16, 56u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 57u16, 0u16, 58u16, 59u16, 0u16, 0u16, 60u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 61u16, 62u16, 0u16, 63u16, 64u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 65u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 66u16, 0u16, 0u16, 0u16, 0u16, 67u16, 68u16, 0u16, 0u16, 0u16,
        69u16, 0u16, 70u16, 0u16, 71u16, 0u16, 72u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        73u16, 0u16, 0u16, 74u16, 0u16, 75u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 76u16,
        77u16, 0u16, 78u16, 0u16, 0u16, 0u16, 0u16, 79u16, 0u16, 0u16, 80u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 81u16, 82u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        83u16, 0u16, 0u16, 0u16, 84u16, 0u16, 0u16, 85u16, 86u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 87u16,
        0u16, 88u16, 0u16, 0u16, 0u16, 0u16, 89u16, 0u16, 90u16, 0u16, 0u16, 0u16, 0u16,
        91u16, 92u16, 93u16, 0u16, 0u16, 94u16, 0u16, 0u16, 95u16, 0u16, 0u16, 0u16,
        0u16, 96u16, 97u16, 0u16, 0u16, 0u16, 98u16, 99u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 100u16, 0u16, 101u16, 0u16, 0u16, 0u16, 0u16, 102u16, 103u16, 104u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 105u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 106u16, 0u16, 107u16, 0u16,
        0u16, 0u16, 0u16, 108u16, 109u16, 0u16, 0u16, 0u16, 0u16, 110u16, 0u16, 0u16,
        111u16, 0u16, 0u16, 0u16, 0u16, 112u16, 113u16,
    ];
    if pc < 2214096u32 || pc > 2216692u32 {
        return None;
    }
    let word_offset = ((pc - 2214096u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(never)]
pub fn block_0x0021c8d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2214100u32);
    emu.sw_no_count(10usize, 2usize, 8u32, 2214104u32)?;
    emu.sw_no_count(11usize, 2usize, 12u32, 2214108u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2214112u32)?;
    emu.sw_no_count(13usize, 2usize, 20u32, 2214116u32)?;
    emu.adi_no_count(10usize, 2usize, 8u32, 2214120u32);
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2214124u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4u32, 2214128u32);
    emu.adi_no_count(12usize, 2usize, 16u32, 2214132u32);
    let a = 0u32.wrapping_add(2220032u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2214136u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294967284u32, 2214140u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2214144u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 4294966152u32, 2214148u32);
    emu.adi_no_count(16usize, 0usize, 2u32, 2214152u32);
    emu.sw_no_count(0usize, 2usize, 40u32, 2214156u32)?;
    emu.adi_no_count(17usize, 2usize, 48u32, 2214160u32);
    emu.sw_no_count(10usize, 2usize, 48u32, 2214164u32)?;
    emu.sw_no_count(11usize, 2usize, 52u32, 2214168u32)?;
    emu.sw_no_count(12usize, 2usize, 56u32, 2214172u32)?;
    emu.sw_no_count(13usize, 2usize, 60u32, 2214176u32)?;
    emu.sw_no_count(15usize, 2usize, 24u32, 2214180u32)?;
    emu.sw_no_count(16usize, 2usize, 28u32, 2214184u32)?;
    emu.sw_no_count(17usize, 2usize, 32u32, 2214188u32)?;
    emu.sw_no_count(16usize, 2usize, 36u32, 2214192u32)?;
    emu.adi_no_count(10usize, 2usize, 24u32, 2214196u32);
    emu.adi_no_count(11usize, 14usize, 0u32, 2214200u32);
    emu.apc_no_count(1usize, 2214200u32, 4294963200u32, 2214204u32);
    emu.add_memory_rw_events(28usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2214208u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966308u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021c940(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2214212u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2214216u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2214220u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2214224u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2214228u32)?;
    emu.adi_no_count(13usize, 0usize, 39u32, 2214232u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2214236u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2214280u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c988));
    } else {
        emu.pc = 2214240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c960));
    }
}
#[inline(always)]
pub fn block_0x0021c960(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 11usize, 2u32, 2214244u32);
    let a = 0u32.wrapping_add(2240512u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2214248u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294966168u32, 2214252u32);
    emu.adr_no_count(10usize, 10usize, 13usize, 2214256u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2214260u32)?;
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(10usize);
    let return_addr = 2214264u32;
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
pub fn block_0x0021c978(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2214268u32);
    let a = 0u32.wrapping_add(12288u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2214272u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 92u32, 2214276u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2214280u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2215116u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021cccc));
}
#[inline(always)]
pub fn block_0x0021c988(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 92u32, 2214284u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2214304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c9a0));
    } else {
        emu.pc = 2214288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c990));
    }
}
#[inline(always)]
pub fn block_0x0021c990(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2214292u32);
    let a = 0u32.wrapping_add(24576u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2214296u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966364u32, 2214300u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2214304u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2215116u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021cccc));
}
#[inline(always)]
pub fn block_0x0021c9a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(12usize, 12usize, 1u32, 2214308u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2214736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cb50));
    } else {
        emu.pc = 2214312u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c9a8));
    }
}
#[inline(always)]
pub fn block_0x0021c9a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 767u32, 2214316u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2214736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cb50));
    } else {
        emu.pc = 2214320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c9b0));
    }
}
#[inline(always)]
pub fn block_0x0021c9b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 11usize, 0u32, 2214324u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2214328u32);
    emu.apc_no_count(1usize, 2214328u32, 0u32, 2214332u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2214336u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966712u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021c9c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 9usize, 0u32, 2214340u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2214736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cb50));
    } else {
        emu.pc = 2214344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021c9c8));
    }
}
#[inline(never)]
pub fn block_0x0021c9c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 78u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(14usize, 11usize, 1u32, 2214348u32);
    let a = 0u32.wrapping_add(1431654400u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2214352u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(858992640u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2214356u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(252645376u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2214360u32;
    emu.update_insn_clock();
    emu.sri_no_count(15usize, 11usize, 20u32, 2214364u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2214368u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 4294965902u32, 2214372u32);
    emu.sli_no_count(17usize, 11usize, 12u32, 2214376u32);
    emu.sli_no_count(5usize, 11usize, 16u32, 2214380u32);
    emu.sli_no_count(6usize, 11usize, 20u32, 2214384u32);
    emu.sli_no_count(7usize, 11usize, 24u32, 2214388u32);
    emu.orr_no_count(14usize, 11usize, 14usize, 2214392u32);
    emu.ani_no_count(11usize, 11usize, 15u32, 2214396u32);
    emu.adr_no_count(15usize, 16usize, 15usize, 2214400u32);
    emu.sri_no_count(17usize, 17usize, 28u32, 2214404u32);
    emu.sri_no_count(5usize, 5usize, 28u32, 2214408u32);
    emu.sri_no_count(6usize, 6usize, 28u32, 2214412u32);
    emu.sri_no_count(7usize, 7usize, 28u32, 2214416u32);
    emu.adr_no_count(11usize, 16usize, 11usize, 2214420u32);
    emu.adr_no_count(17usize, 16usize, 17usize, 2214424u32);
    emu.adr_no_count(5usize, 16usize, 5usize, 2214428u32);
    emu.adr_no_count(6usize, 16usize, 6usize, 2214432u32);
    emu.adr_no_count(16usize, 16usize, 7usize, 2214436u32);
    emu.sri_no_count(7usize, 14usize, 2u32, 2214440u32);
    emu.orr_no_count(14usize, 14usize, 7usize, 2214444u32);
    emu.sri_no_count(7usize, 14usize, 4u32, 2214448u32);
    emu.orr_no_count(14usize, 14usize, 7usize, 2214452u32);
    emu.sri_no_count(7usize, 14usize, 8u32, 2214456u32);
    emu.orr_no_count(14usize, 14usize, 7usize, 2214460u32);
    let a = 0u32.wrapping_add(16842752u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2214464u32;
    emu.update_insn_clock();
    emu.lbu_no_count(15usize, 15usize, 0u32, 2214468u32);
    emu.lbu_no_count(17usize, 17usize, 0u32, 2214472u32);
    emu.lbu_no_count(5usize, 5usize, 0u32, 2214476u32);
    emu.lbu_no_count(6usize, 6usize, 0u32, 2214480u32);
    emu.sh_no_count(0usize, 2usize, 12u32, 2214484u32)?;
    emu.sb_no_count(0usize, 2usize, 14u32, 2214488u32);
    emu.sb_no_count(15usize, 2usize, 15u32, 2214492u32);
    emu.sb_no_count(17usize, 2usize, 16u32, 2214496u32);
    emu.adi_no_count(15usize, 0usize, 125u32, 2214500u32);
    emu.adi_no_count(13usize, 13usize, 1365u32, 2214504u32);
    emu.lbu_no_count(16usize, 16usize, 0u32, 2214508u32);
    emu.lbu_no_count(11usize, 11usize, 0u32, 2214512u32);
    emu.sb_no_count(5usize, 2usize, 17u32, 2214516u32);
    emu.sb_no_count(6usize, 2usize, 18u32, 2214520u32);
    emu.sb_no_count(16usize, 2usize, 19u32, 2214524u32);
    emu.sri_no_count(16usize, 14usize, 16u32, 2214528u32);
    emu.orr_no_count(14usize, 14usize, 16usize, 2214532u32);
    emu.xri_no_count(14usize, 14usize, 4294967295u32, 2214536u32);
    emu.sri_no_count(16usize, 14usize, 1u32, 2214540u32);
    emu.anr_no_count(13usize, 16usize, 13usize, 2214544u32);
    emu.adi_no_count(16usize, 2usize, 12u32, 2214548u32);
    emu.adi_no_count(12usize, 12usize, 819u32, 2214552u32);
    emu.ani_no_count(14usize, 14usize, 4294967294u32, 2214556u32);
    emu.sbr_no_count(14usize, 14usize, 13usize, 2214560u32);
    emu.anr_no_count(13usize, 14usize, 12usize, 2214564u32);
    emu.sri_no_count(14usize, 14usize, 2u32, 2214568u32);
    emu.anr_no_count(12usize, 14usize, 12usize, 2214572u32);
    emu.adi_no_count(14usize, 0usize, 92u32, 2214576u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2214580u32);
    emu.sri_no_count(13usize, 12usize, 4u32, 2214584u32);
    emu.adr_no_count(12usize, 12usize, 13usize, 2214588u32);
    emu.adi_no_count(13usize, 0usize, 117u32, 2214592u32);
    emu.adi_no_count(10usize, 10usize, 4294967055u32, 2214596u32);
    emu.anr_no_count(10usize, 12usize, 10usize, 2214600u32);
    emu.adi_no_count(12usize, 0usize, 123u32, 2214604u32);
    emu.adi_no_count(17usize, 7usize, 257u32, 2214608u32);
    emu.mul_no_count(10usize, 10usize, 17usize, 2214612u32);
    emu.sri_no_count(10usize, 10usize, 26u32, 2214616u32);
    emu.adi_no_count(9usize, 10usize, 4294967294u32, 2214620u32);
    emu.adr_no_count(10usize, 16usize, 10usize, 2214624u32);
    emu.adr_no_count(16usize, 16usize, 9usize, 2214628u32);
    emu.sb_no_count(14usize, 16usize, 0u32, 2214632u32);
    emu.sb_no_count(13usize, 10usize, 4294967295u32, 2214636u32);
    emu.sb_no_count(12usize, 10usize, 0u32, 2214640u32);
    emu.sb_no_count(11usize, 2usize, 20u32, 2214644u32);
    emu.sb_no_count(15usize, 2usize, 21u32, 2214648u32);
    emu.adi_no_count(11usize, 2usize, 12u32, 2214652u32);
    emu.add_memory_rw_events(78usize);
    let return_addr = 2214656u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2215080u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021cca8));
}
#[inline(always)]
pub fn block_0x0021cb00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2214660u32);
    let a = 0u32.wrapping_add(28672u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2214664u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294966876u32, 2214668u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2214672u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2215116u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021cccc));
}
#[inline(always)]
pub fn block_0x0021cb10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 12usize, 256u32, 2214676u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2214736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cb50));
    } else {
        emu.pc = 2214680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cb18));
    }
}
#[inline(always)]
pub fn block_0x0021cb18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2214684u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2214688u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1884u32, 2214692u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2214696u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2215116u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021cccc));
}
#[inline(always)]
pub fn block_0x0021cb28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2214700u32);
    let a = 0u32.wrapping_add(28672u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2214704u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(3usize);
    let return_addr = 2214708u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2215112u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ccc8));
}
#[inline(always)]
pub fn block_0x0021cb34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2214712u32);
    let a = 0u32.wrapping_add(28672u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2214716u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1116u32, 2214720u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2214724u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2215116u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021cccc));
}
#[inline(always)]
pub fn block_0x0021cb44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 8u32, 2214728u32);
    emu.sri_no_count(12usize, 12usize, 24u32, 2214732u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2215104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ccc0));
    } else {
        emu.pc = 2214736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cb50));
    }
}
#[inline(always)]
pub fn block_0x0021cb50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 11usize, 0u32, 2214740u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2214744u32);
    emu.apc_no_count(1usize, 2214744u32, 8192u32, 2214748u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2214752u32;
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
pub fn block_0x0021cb60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2214772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cb74));
    } else {
        emu.pc = 2214756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cb64));
    }
}
#[inline(always)]
pub fn block_0x0021cb64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(9usize, 8usize, 0u32, 2214760u32)?;
    emu.adi_no_count(18usize, 0usize, 129u32, 2214764u32);
    emu.adi_no_count(9usize, 0usize, 128u32, 2214768u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2214772u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2215136u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021cce0));
}
#[inline(never)]
pub fn block_0x0021cb74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 77u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(14usize, 9usize, 1u32, 2214776u32);
    let a = 0u32.wrapping_add(1431654400u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2214780u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(858992640u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2214784u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(252645376u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2214788u32;
    emu.update_insn_clock();
    emu.sri_no_count(15usize, 9usize, 20u32, 2214792u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2214796u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 4294965902u32, 2214800u32);
    emu.sli_no_count(17usize, 9usize, 12u32, 2214804u32);
    emu.sli_no_count(5usize, 9usize, 16u32, 2214808u32);
    emu.sli_no_count(6usize, 9usize, 20u32, 2214812u32);
    emu.sli_no_count(7usize, 9usize, 24u32, 2214816u32);
    emu.orr_no_count(14usize, 9usize, 14usize, 2214820u32);
    emu.ani_no_count(13usize, 9usize, 15u32, 2214824u32);
    emu.adr_no_count(15usize, 16usize, 15usize, 2214828u32);
    emu.sri_no_count(17usize, 17usize, 28u32, 2214832u32);
    emu.sri_no_count(5usize, 5usize, 28u32, 2214836u32);
    emu.sri_no_count(6usize, 6usize, 28u32, 2214840u32);
    emu.sri_no_count(7usize, 7usize, 28u32, 2214844u32);
    emu.adr_no_count(13usize, 16usize, 13usize, 2214848u32);
    emu.adr_no_count(17usize, 16usize, 17usize, 2214852u32);
    emu.adr_no_count(5usize, 16usize, 5usize, 2214856u32);
    emu.adr_no_count(6usize, 16usize, 6usize, 2214860u32);
    emu.adr_no_count(16usize, 16usize, 7usize, 2214864u32);
    emu.sri_no_count(7usize, 14usize, 2u32, 2214868u32);
    emu.orr_no_count(14usize, 14usize, 7usize, 2214872u32);
    emu.sri_no_count(7usize, 14usize, 4u32, 2214876u32);
    emu.orr_no_count(14usize, 14usize, 7usize, 2214880u32);
    emu.sri_no_count(7usize, 14usize, 8u32, 2214884u32);
    emu.orr_no_count(14usize, 14usize, 7usize, 2214888u32);
    let a = 0u32.wrapping_add(16842752u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2214892u32;
    emu.update_insn_clock();
    emu.lbu_no_count(15usize, 15usize, 0u32, 2214896u32);
    emu.lbu_no_count(17usize, 17usize, 0u32, 2214900u32);
    emu.lbu_no_count(5usize, 5usize, 0u32, 2214904u32);
    emu.lbu_no_count(6usize, 6usize, 0u32, 2214908u32);
    emu.sh_no_count(0usize, 2usize, 22u32, 2214912u32)?;
    emu.sb_no_count(0usize, 2usize, 24u32, 2214916u32);
    emu.sb_no_count(15usize, 2usize, 25u32, 2214920u32);
    emu.sb_no_count(17usize, 2usize, 26u32, 2214924u32);
    emu.adi_no_count(15usize, 0usize, 125u32, 2214928u32);
    emu.adi_no_count(12usize, 12usize, 1365u32, 2214932u32);
    emu.lbu_no_count(16usize, 16usize, 0u32, 2214936u32);
    emu.lbu_no_count(13usize, 13usize, 0u32, 2214940u32);
    emu.sb_no_count(5usize, 2usize, 27u32, 2214944u32);
    emu.sb_no_count(6usize, 2usize, 28u32, 2214948u32);
    emu.sb_no_count(16usize, 2usize, 29u32, 2214952u32);
    emu.sri_no_count(16usize, 14usize, 16u32, 2214956u32);
    emu.orr_no_count(14usize, 14usize, 16usize, 2214960u32);
    emu.xri_no_count(14usize, 14usize, 4294967295u32, 2214964u32);
    emu.sri_no_count(16usize, 14usize, 1u32, 2214968u32);
    emu.anr_no_count(12usize, 16usize, 12usize, 2214972u32);
    emu.adi_no_count(16usize, 2usize, 22u32, 2214976u32);
    emu.adi_no_count(11usize, 11usize, 819u32, 2214980u32);
    emu.ani_no_count(14usize, 14usize, 4294967294u32, 2214984u32);
    emu.sbr_no_count(14usize, 14usize, 12usize, 2214988u32);
    emu.anr_no_count(12usize, 14usize, 11usize, 2214992u32);
    emu.sri_no_count(14usize, 14usize, 2u32, 2214996u32);
    emu.anr_no_count(11usize, 14usize, 11usize, 2215000u32);
    emu.adi_no_count(14usize, 0usize, 92u32, 2215004u32);
    emu.adr_no_count(11usize, 12usize, 11usize, 2215008u32);
    emu.sri_no_count(12usize, 11usize, 4u32, 2215012u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2215016u32);
    emu.adi_no_count(12usize, 0usize, 117u32, 2215020u32);
    emu.adi_no_count(10usize, 10usize, 4294967055u32, 2215024u32);
    emu.anr_no_count(10usize, 11usize, 10usize, 2215028u32);
    emu.adi_no_count(11usize, 0usize, 123u32, 2215032u32);
    emu.adi_no_count(17usize, 7usize, 257u32, 2215036u32);
    emu.mul_no_count(10usize, 10usize, 17usize, 2215040u32);
    emu.sri_no_count(10usize, 10usize, 26u32, 2215044u32);
    emu.adi_no_count(9usize, 10usize, 4294967294u32, 2215048u32);
    emu.adr_no_count(10usize, 16usize, 10usize, 2215052u32);
    emu.adr_no_count(16usize, 16usize, 9usize, 2215056u32);
    emu.sb_no_count(14usize, 16usize, 0u32, 2215060u32);
    emu.sb_no_count(12usize, 10usize, 4294967295u32, 2215064u32);
    emu.sb_no_count(11usize, 10usize, 0u32, 2215068u32);
    emu.sb_no_count(13usize, 2usize, 30u32, 2215072u32);
    emu.sb_no_count(15usize, 2usize, 31u32, 2215076u32);
    emu.adi_no_count(11usize, 2usize, 22u32, 2215080u32);
    emu.add_memory_rw_events(77usize);
    emu.pc = 2215080u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021cca8));
}
#[inline(always)]
pub fn block_0x0021cca8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 10u32, 2215084u32);
    emu.adi_no_count(18usize, 0usize, 10u32, 2215088u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2215092u32);
    emu.apc_no_count(1usize, 2215092u32, 4294885376u32, 2215096u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2215100u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(128u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021ccbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2215104u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2215136u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021cce0));
}
#[inline(always)]
pub fn block_0x0021ccc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2215108u32);
    let a = 0u32.wrapping_add(8192u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2215112u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(2usize);
    emu.pc = 2215112u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ccc8));
}
#[inline(always)]
pub fn block_0x0021ccc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 10usize, 604u32, 2215116u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2215116u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021cccc));
}
#[inline(always)]
pub fn block_0x0021cccc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 8usize, 0u32, 2215120u32)?;
    emu.sh_no_count(0usize, 8usize, 4u32, 2215124u32)?;
    emu.sh_no_count(0usize, 8usize, 6u32, 2215128u32)?;
    emu.sh_no_count(0usize, 8usize, 8u32, 2215132u32)?;
    emu.adi_no_count(18usize, 0usize, 2u32, 2215136u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2215136u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021cce0));
}
#[inline(always)]
pub fn block_0x0021cce0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(9usize, 8usize, 12u32, 2215140u32);
    emu.sb_no_count(18usize, 8usize, 13u32, 2215144u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2215148u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2215152u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2215156u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2215160u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2215164u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2215168u32;
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
pub fn block_0x0021cd00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 11usize, 0u32, 2215172u32)?;
    emu.lw_no_count(11usize, 11usize, 4u32, 2215176u32)?;
    emu.adi_no_count(13usize, 10usize, 0u32, 2215180u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2215184u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2215188u32);
    emu.apc_no_count(6usize, 2215188u32, 0u32, 2215192u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2215196u32;
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
pub fn block_0x0021cd1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2215200u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2215204u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2215208u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2215212u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2215216u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2215220u32)?;
    emu.sw_no_count(20usize, 2usize, 40u32, 2215224u32)?;
    emu.sw_no_count(21usize, 2usize, 36u32, 2215228u32)?;
    emu.sw_no_count(22usize, 2usize, 32u32, 2215232u32)?;
    emu.sw_no_count(23usize, 2usize, 28u32, 2215236u32)?;
    emu.sw_no_count(24usize, 2usize, 24u32, 2215240u32)?;
    emu.sw_no_count(25usize, 2usize, 20u32, 2215244u32)?;
    emu.adi_no_count(8usize, 12usize, 0u32, 2215248u32);
    let a = 0u32.wrapping_add(3758096384u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2215252u32;
    emu.update_insn_clock();
    emu.lw_no_count(21usize, 8usize, 16u32, 2215256u32)?;
    emu.adi_no_count(12usize, 12usize, 32u32, 2215260u32);
    emu.sw_no_count(10usize, 2usize, 4u32, 2215264u32)?;
    emu.sw_no_count(11usize, 2usize, 8u32, 2215268u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2215272u32)?;
    emu.sw_no_count(0usize, 2usize, 16u32, 2215276u32)?;
    emu.add_memory_rw_events(20usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a == b {
        emu.pc = 2215572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ce94));
    } else {
        emu.pc = 2215280u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cd70));
    }
}
#[inline(always)]
pub fn block_0x0021cd70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 8usize, 20u32, 2215284u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2215724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cf2c));
    } else {
        emu.pc = 2215288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cd78));
    }
}
#[inline]
pub fn block_0x0021cd78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 0u32, 2215292u32);
    emu.sli_no_count(12usize, 11usize, 3u32, 2215296u32);
    emu.sli_no_count(13usize, 11usize, 5u32, 2215300u32);
    emu.adi_no_count(10usize, 21usize, 24u32, 2215304u32);
    emu.lw_no_count(23usize, 8usize, 0u32, 2215308u32)?;
    emu.lw_no_count(19usize, 8usize, 8u32, 2215312u32)?;
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2215316u32);
    emu.adi_no_count(20usize, 0usize, 2u32, 2215320u32);
    emu.sbr_no_count(13usize, 13usize, 12usize, 2215324u32);
    emu.sli_no_count(11usize, 11usize, 3u32, 2215328u32);
    emu.adr_no_count(22usize, 21usize, 13usize, 2215332u32);
    emu.sri_no_count(11usize, 11usize, 3u32, 2215336u32);
    emu.adi_no_count(9usize, 11usize, 1u32, 2215340u32);
    emu.adi_no_count(23usize, 23usize, 4u32, 2215344u32);
    emu.adi_no_count(24usize, 0usize, 1u32, 2215348u32);
    emu.add_memory_rw_events(15usize);
    emu.pc = 2215348u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021cdb4));
}
#[inline(always)]
pub fn block_0x0021cdb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 23usize, 0u32, 2215352u32)?;
    emu.adi_no_count(25usize, 10usize, 0u32, 2215356u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2215384u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cdd8));
    } else {
        emu.pc = 2215360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cdc0));
    }
}
#[inline(always)]
pub fn block_0x0021cdc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 2usize, 8u32, 2215364u32)?;
    emu.lw_no_count(10usize, 2usize, 4u32, 2215368u32)?;
    emu.lw_no_count(11usize, 23usize, 4294967292u32, 2215372u32)?;
    emu.lw_no_count(13usize, 13usize, 12u32, 2215376u32)?;
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2215380u32;
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
pub fn block_0x0021cdd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2215776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cf60));
    } else {
        emu.pc = 2215384u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cdd8));
    }
}
#[inline(always)]
pub fn block_0x0021cdd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lhu_no_count(10usize, 21usize, 8u32, 2215388u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2215444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ce14));
    } else {
        emu.pc = 2215392u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cde0));
    }
}
#[inline(always)]
pub fn block_0x0021cde0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2215464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ce28));
    } else {
        emu.pc = 2215396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cde4));
    }
}
#[inline(always)]
pub fn block_0x0021cde4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 21usize, 12u32, 2215400u32)?;
    emu.sli_no_count(10usize, 10usize, 3u32, 2215404u32);
    emu.adr_no_count(10usize, 19usize, 10usize, 2215408u32);
    emu.lhu_no_count(11usize, 10usize, 4u32, 2215412u32)?;
    emu.lhu_no_count(10usize, 21usize, 0u32, 2215416u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2215456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ce20));
    } else {
        emu.pc = 2215420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cdfc));
    }
}
#[inline(always)]
pub fn block_0x0021cdfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2215480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ce38));
    } else {
        emu.pc = 2215424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ce00));
    }
}
#[inline(always)]
pub fn block_0x0021ce00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 21usize, 4u32, 2215428u32)?;
    emu.sli_no_count(10usize, 10usize, 3u32, 2215432u32);
    emu.adr_no_count(10usize, 19usize, 10usize, 2215436u32);
    emu.lhu_no_count(12usize, 10usize, 4u32, 2215440u32)?;
    emu.add_memory_rw_events(5usize);
    let return_addr = 2215444u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2215484u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ce3c));
}
#[inline(always)]
pub fn block_0x0021ce14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lhu_no_count(11usize, 21usize, 10u32, 2215448u32)?;
    emu.lhu_no_count(10usize, 21usize, 0u32, 2215452u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2215420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cdfc));
    } else {
        emu.pc = 2215456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ce20));
    }
}
#[inline(always)]
pub fn block_0x0021ce20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 0u32, 2215460u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2215464u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2215484u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ce3c));
}
#[inline(always)]
pub fn block_0x0021ce28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2215468u32);
    emu.lhu_no_count(10usize, 21usize, 0u32, 2215472u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2215420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cdfc));
    } else {
        emu.pc = 2215476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ce34));
    }
}
#[inline(always)]
pub fn block_0x0021ce34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2215480u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2215456u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ce20));
}
#[inline(always)]
pub fn block_0x0021ce38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lhu_no_count(12usize, 21usize, 2u32, 2215484u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2215484u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021ce3c));
}
#[inline]
pub fn block_0x0021ce3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 21usize, 16u32, 2215488u32)?;
    emu.lw_no_count(13usize, 21usize, 20u32, 2215492u32)?;
    emu.sli_no_count(10usize, 10usize, 3u32, 2215496u32);
    emu.adr_no_count(14usize, 19usize, 10usize, 2215500u32);
    emu.lw_no_count(10usize, 14usize, 0u32, 2215504u32)?;
    emu.lw_no_count(14usize, 14usize, 4u32, 2215508u32)?;
    emu.sw_no_count(13usize, 2usize, 12u32, 2215512u32)?;
    emu.sh_no_count(11usize, 2usize, 16u32, 2215516u32)?;
    emu.sh_no_count(12usize, 2usize, 18u32, 2215520u32)?;
    emu.adi_no_count(11usize, 2usize, 4u32, 2215524u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2215528u32;
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
pub fn block_0x0021ce68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2215776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cf60));
    } else {
        emu.pc = 2215532u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ce6c));
    }
}
#[inline]
pub fn block_0x0021ce6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 18usize, 1u32, 2215536u32);
    emu.xrr_no_count(10usize, 25usize, 22usize, 2215540u32);
    emu.sltiu_no_count(10usize, 10usize, 1u32, 2215544u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2215548u32);
    emu.ani_no_count(10usize, 10usize, 24u32, 2215552u32);
    emu.adr_no_count(10usize, 25usize, 10usize, 2215556u32);
    emu.adi_no_count(23usize, 23usize, 8u32, 2215560u32);
    emu.adi_no_count(21usize, 25usize, 0u32, 2215564u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a != b {
        emu.pc = 2215348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cdb4));
    } else {
        emu.pc = 2215568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ce90));
    }
}
#[inline(always)]
pub fn block_0x0021ce90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2215572u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2215712u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021cf20));
}
#[inline(always)]
pub fn block_0x0021ce94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 12u32, 2215576u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2215724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cf2c));
    } else {
        emu.pc = 2215580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ce9c));
    }
}
#[inline]
pub fn block_0x0021ce9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 0u32, 2215584u32);
    emu.lw_no_count(20usize, 8usize, 0u32, 2215588u32)?;
    emu.lw_no_count(21usize, 8usize, 8u32, 2215592u32)?;
    emu.sli_no_count(19usize, 10usize, 3u32, 2215596u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2215600u32);
    emu.sli_no_count(10usize, 10usize, 3u32, 2215604u32);
    emu.sri_no_count(10usize, 10usize, 3u32, 2215608u32);
    emu.adi_no_count(9usize, 10usize, 1u32, 2215612u32);
    emu.adr_no_count(19usize, 21usize, 19usize, 2215616u32);
    emu.adi_no_count(10usize, 21usize, 8u32, 2215620u32);
    emu.adi_no_count(20usize, 20usize, 4u32, 2215624u32);
    emu.add_memory_rw_events(11usize);
    emu.pc = 2215624u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021cec8));
}
#[inline(always)]
pub fn block_0x0021cec8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 20usize, 0u32, 2215628u32)?;
    emu.adi_no_count(22usize, 10usize, 0u32, 2215632u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2215660u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ceec));
    } else {
        emu.pc = 2215636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ced4));
    }
}
#[inline(always)]
pub fn block_0x0021ced4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 2usize, 8u32, 2215640u32)?;
    emu.lw_no_count(10usize, 2usize, 4u32, 2215644u32)?;
    emu.lw_no_count(11usize, 20usize, 4294967292u32, 2215648u32)?;
    emu.lw_no_count(13usize, 13usize, 12u32, 2215652u32)?;
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2215656u32;
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
pub fn block_0x0021cee8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2215776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cf60));
    } else {
        emu.pc = 2215660u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021ceec));
    }
}
#[inline(always)]
pub fn block_0x0021ceec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 21usize, 0u32, 2215664u32)?;
    emu.lw_no_count(12usize, 21usize, 4u32, 2215668u32)?;
    emu.adi_no_count(11usize, 2usize, 4u32, 2215672u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2215676u32;
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
pub fn block_0x0021cefc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2215776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cf60));
    } else {
        emu.pc = 2215680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cf00));
    }
}
#[inline(always)]
pub fn block_0x0021cf00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 18usize, 1u32, 2215684u32);
    emu.xrr_no_count(10usize, 22usize, 19usize, 2215688u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2215692u32);
    emu.sli_no_count(10usize, 10usize, 3u32, 2215696u32);
    emu.adr_no_count(10usize, 22usize, 10usize, 2215700u32);
    emu.adi_no_count(20usize, 20usize, 8u32, 2215704u32);
    emu.adi_no_count(21usize, 22usize, 0u32, 2215708u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a != b {
        emu.pc = 2215624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cec8));
    } else {
        emu.pc = 2215712u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cf20));
    }
}
#[inline(always)]
pub fn block_0x0021cf20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 4u32, 2215716u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a < b {
        emu.pc = 2215736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cf38));
    } else {
        emu.pc = 2215720u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cf28));
    }
}
#[inline(always)]
pub fn block_0x0021cf28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2215724u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2215784u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021cf68));
}
#[inline(always)]
pub fn block_0x0021cf2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 0u32, 2215728u32);
    emu.lw_no_count(10usize, 8usize, 4u32, 2215732u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(0usize);
    if a >= b {
        emu.pc = 2215784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cf68));
    } else {
        emu.pc = 2215736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cf38));
    }
}
#[inline]
pub fn block_0x0021cf38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 8usize, 0u32, 2215740u32)?;
    emu.sli_no_count(9usize, 9usize, 3u32, 2215744u32);
    emu.lw_no_count(10usize, 2usize, 4u32, 2215748u32)?;
    emu.lw_no_count(13usize, 2usize, 8u32, 2215752u32)?;
    emu.adr_no_count(9usize, 11usize, 9usize, 2215756u32);
    emu.lw_no_count(11usize, 9usize, 0u32, 2215760u32)?;
    emu.lw_no_count(12usize, 9usize, 4u32, 2215764u32)?;
    emu.lw_no_count(13usize, 13usize, 12u32, 2215768u32)?;
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2215772u32;
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
pub fn block_0x0021cf5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2215784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cf68));
    } else {
        emu.pc = 2215776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cf60));
    }
}
#[inline(always)]
pub fn block_0x0021cf60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2215780u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2215784u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2215788u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021cf6c));
}
#[inline(always)]
pub fn block_0x0021cf68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2215788u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2215788u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021cf6c));
}
#[inline]
pub fn block_0x0021cf6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2215792u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2215796u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2215800u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2215804u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2215808u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2215812u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2215816u32)?;
    emu.lw_no_count(22usize, 2usize, 32u32, 2215820u32)?;
    emu.lw_no_count(23usize, 2usize, 28u32, 2215824u32)?;
    emu.lw_no_count(24usize, 2usize, 24u32, 2215828u32)?;
    emu.lw_no_count(25usize, 2usize, 20u32, 2215832u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2215836u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2215840u32;
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
pub fn block_0x0021cfa0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2215844u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2215848u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2215852u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2215856u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2215860u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2215864u32)?;
    emu.sw_no_count(20usize, 2usize, 40u32, 2215868u32)?;
    emu.sw_no_count(21usize, 2usize, 36u32, 2215872u32)?;
    emu.sw_no_count(22usize, 2usize, 32u32, 2215876u32)?;
    emu.sw_no_count(23usize, 2usize, 28u32, 2215880u32)?;
    emu.sw_no_count(24usize, 2usize, 24u32, 2215884u32)?;
    emu.sw_no_count(25usize, 2usize, 20u32, 2215888u32)?;
    emu.sw_no_count(26usize, 2usize, 16u32, 2215892u32)?;
    emu.sw_no_count(27usize, 2usize, 12u32, 2215896u32)?;
    emu.adi_no_count(8usize, 15usize, 0u32, 2215900u32);
    emu.adi_no_count(9usize, 14usize, 0u32, 2215904u32);
    emu.adi_no_count(19usize, 13usize, 0u32, 2215908u32);
    emu.adi_no_count(20usize, 12usize, 0u32, 2215912u32);
    emu.adi_no_count(18usize, 10usize, 0u32, 2215916u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2216068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d084));
    } else {
        emu.pc = 2215920u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021cff0));
    }
}
#[inline(always)]
pub fn block_0x0021cff0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(22usize, 18usize, 8u32, 2215924u32)?;
    let a = 0u32.wrapping_add(2097152u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2215928u32;
    emu.update_insn_clock();
    emu.anr_no_count(10usize, 22usize, 10usize, 2215932u32);
    let a = 0u32.wrapping_add(1114112u32);
    emu.write_reg_no_count(21usize, a);
    emu.pc = 2215936u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2215944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d008));
    } else {
        emu.pc = 2215940u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d004));
    }
}
#[inline(always)]
pub fn block_0x0021d004(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(21usize, 0usize, 43u32, 2215944u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2215944u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021d008));
}
#[inline(always)]
pub fn block_0x0021d008(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 10usize, 21u32, 2215948u32);
    emu.adr_no_count(24usize, 10usize, 8usize, 2215952u32);
    emu.sli_no_count(10usize, 22usize, 8u32, 2215956u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2216088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d098));
    } else {
        emu.pc = 2215960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d018));
    }
}
#[inline(always)]
pub fn block_0x0021d018(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 16u32, 2215964u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a >= b {
        emu.pc = 2216216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d118));
    } else {
        emu.pc = 2215968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d020));
    }
}
#[inline(always)]
pub fn block_0x0021d020(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2215972u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2216008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d048));
    } else {
        emu.pc = 2215976u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d028));
    }
}
#[inline(always)]
pub fn block_0x0021d028(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(11usize, 20usize, 19usize, 2215980u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2215984u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2215984u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021d030));
}
#[inline(always)]
pub fn block_0x0021d030(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(13usize, 12usize, 0u32, 2215988u32);
    emu.adi_no_count(12usize, 12usize, 1u32, 2215992u32);
    emu.slti_no_count(13usize, 13usize, 4294967232u32, 2215996u32);
    emu.xri_no_count(13usize, 13usize, 1u32, 2216000u32);
    emu.adr_no_count(10usize, 10usize, 13usize, 2216004u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2215984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d030));
    } else {
        emu.pc = 2216008u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d048));
    }
}
#[inline(always)]
pub fn block_0x0021d048(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(24usize, 10usize, 24usize, 2216012u32);
    emu.lhu_no_count(27usize, 18usize, 12u32, 2216016u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(27usize);
    let a = emu.read_reg_a_tracked(24usize);
    if a >= b {
        emu.pc = 2216100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d0a4));
    } else {
        emu.pc = 2216020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d054));
    }
}
#[inline(always)]
pub fn block_0x0021d054(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 22usize, 7u32, 2216024u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2216248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d138));
    } else {
        emu.pc = 2216028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d05c));
    }
}
#[inline(always)]
pub fn block_0x0021d05c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(24usize, 27usize, 24usize, 2216032u32);
    emu.sli_no_count(10usize, 22usize, 1u32, 2216036u32);
    emu.sri_no_count(10usize, 10usize, 30u32, 2216040u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2216044u32);
    emu.sli_no_count(22usize, 22usize, 11u32, 2216048u32);
    emu.sw_no_count(9usize, 2usize, 8u32, 2216052u32)?;
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2216380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d1bc));
    } else {
        emu.pc = 2216056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d078));
    }
}
#[inline(always)]
pub fn block_0x0021d078(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2216444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d1fc));
    } else {
        emu.pc = 2216060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d07c));
    }
}
#[inline(always)]
pub fn block_0x0021d07c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 0usize, 0u32, 2216064u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2216068u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2216448u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021d200));
}
#[inline(always)]
pub fn block_0x0021d084(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(22usize, 18usize, 8u32, 2216072u32)?;
    emu.adi_no_count(24usize, 8usize, 1u32, 2216076u32);
    emu.adi_no_count(21usize, 0usize, 45u32, 2216080u32);
    emu.sli_no_count(10usize, 22usize, 8u32, 2216084u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2215960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d018));
    } else {
        emu.pc = 2216088u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d098));
    }
}
#[inline(always)]
pub fn block_0x0021d098(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 0u32, 2216092u32);
    emu.lhu_no_count(27usize, 18usize, 12u32, 2216096u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(27usize);
    let a = emu.read_reg_a_tracked(24usize);
    if a < b {
        emu.pc = 2216020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d054));
    } else {
        emu.pc = 2216100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d0a4));
    }
}
#[inline]
pub fn block_0x0021d0a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(22usize, 18usize, 0u32, 2216104u32)?;
    emu.lw_no_count(18usize, 18usize, 4u32, 2216108u32)?;
    emu.adi_no_count(10usize, 22usize, 0u32, 2216112u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2216116u32);
    emu.adi_no_count(12usize, 21usize, 0u32, 2216120u32);
    emu.adi_no_count(13usize, 20usize, 0u32, 2216124u32);
    emu.adi_no_count(14usize, 19usize, 0u32, 2216128u32);
    emu.apc_no_count(1usize, 2216128u32, 0u32, 2216132u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2216136u32;
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
pub fn block_0x0021d0c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2216508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d23c));
    } else {
        emu.pc = 2216140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d0cc));
    }
}
#[inline]
pub fn block_0x0021d0cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(6usize, 18usize, 12u32, 2216144u32)?;
    emu.adi_no_count(10usize, 22usize, 0u32, 2216148u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2216152u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2216156u32);
    emu.lw_no_count(1usize, 2usize, 60u32, 2216160u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2216164u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2216168u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2216172u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2216176u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2216180u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2216184u32)?;
    emu.lw_no_count(22usize, 2usize, 32u32, 2216188u32)?;
    emu.lw_no_count(23usize, 2usize, 28u32, 2216192u32)?;
    emu.lw_no_count(24usize, 2usize, 24u32, 2216196u32)?;
    emu.lw_no_count(25usize, 2usize, 20u32, 2216200u32)?;
    emu.lw_no_count(26usize, 2usize, 16u32, 2216204u32)?;
    emu.lw_no_count(27usize, 2usize, 12u32, 2216208u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2216212u32);
    emu.add_memory_rw_events(19usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2216216u32;
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
pub fn block_0x0021d118(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 20usize, 0u32, 2216220u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2216224u32);
    emu.apc_no_count(1usize, 2216224u32, 4096u32, 2216228u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2216232u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1092u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0021d128(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(24usize, 10usize, 24usize, 2216236u32);
    emu.lhu_no_count(27usize, 18usize, 12u32, 2216240u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(27usize);
    let a = emu.read_reg_a_tracked(24usize);
    if a >= b {
        emu.pc = 2216100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d0a4));
    } else {
        emu.pc = 2216244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d134));
    }
}
#[inline(always)]
pub fn block_0x0021d134(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2216248u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2216020u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021d054));
}
#[inline]
pub fn block_0x0021d138(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(22usize, 18usize, 0u32, 2216252u32)?;
    emu.lw_no_count(23usize, 18usize, 4u32, 2216256u32)?;
    emu.lw_no_count(25usize, 18usize, 8u32, 2216260u32)?;
    emu.lw_no_count(26usize, 18usize, 12u32, 2216264u32)?;
    let a = 0u32.wrapping_add(2682257408u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2216268u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(536870912u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2216272u32;
    emu.update_insn_clock();
    emu.anr_no_count(10usize, 25usize, 10usize, 2216276u32);
    emu.adi_no_count(11usize, 11usize, 48u32, 2216280u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2216284u32);
    emu.sw_no_count(10usize, 18usize, 8u32, 2216288u32)?;
    emu.adi_no_count(10usize, 22usize, 0u32, 2216292u32);
    emu.adi_no_count(11usize, 23usize, 0u32, 2216296u32);
    emu.adi_no_count(12usize, 21usize, 0u32, 2216300u32);
    emu.adi_no_count(13usize, 20usize, 0u32, 2216304u32);
    emu.adi_no_count(14usize, 19usize, 0u32, 2216308u32);
    emu.apc_no_count(1usize, 2216308u32, 0u32, 2216312u32);
    emu.add_memory_rw_events(17usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2216316u32;
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
pub fn block_0x0021d17c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 1u32, 2216320u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2216512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d240));
    } else {
        emu.pc = 2216324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d184));
    }
}
#[inline(always)]
pub fn block_0x0021d184(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 0u32, 2216328u32);
    emu.sbr_no_count(10usize, 27usize, 24usize, 2216332u32);
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(21usize, a);
    emu.pc = 2216336u32;
    emu.update_insn_clock();
    emu.adi_no_count(21usize, 21usize, 4294967295u32, 2216340u32);
    emu.anr_no_count(24usize, 10usize, 21usize, 2216344u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2216344u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021d198));
}
#[inline(always)]
pub fn block_0x0021d198(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(10usize, 20usize, 21usize, 2216348u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2216404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d1d4));
    } else {
        emu.pc = 2216352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d1a0));
    }
}
#[inline(always)]
pub fn block_0x0021d1a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 23usize, 16u32, 2216356u32)?;
    emu.adi_no_count(20usize, 20usize, 1u32, 2216360u32);
    emu.adi_no_count(11usize, 0usize, 48u32, 2216364u32);
    emu.adi_no_count(10usize, 22usize, 0u32, 2216368u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2216372u32;
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
pub fn block_0x0021d1b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2216344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d198));
    } else {
        emu.pc = 2216376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d1b8));
    }
}
#[inline(always)]
pub fn block_0x0021d1b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2216380u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2216512u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021d240));
}
#[inline(always)]
pub fn block_0x0021d1bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 2u32, 2216384u32);
    emu.adi_no_count(25usize, 24usize, 0u32, 2216388u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2216448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d200));
    } else {
        emu.pc = 2216392u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d1c8));
    }
}
#[inline(always)]
pub fn block_0x0021d1c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 24usize, 16u32, 2216396u32);
    emu.sri_no_count(25usize, 10usize, 17u32, 2216400u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2216404u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2216448u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021d200));
}
#[inline(always)]
pub fn block_0x0021d1d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 23usize, 12u32, 2216408u32)?;
    emu.adi_no_count(10usize, 22usize, 0u32, 2216412u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2216416u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2216420u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2216424u32;
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
pub fn block_0x0021d1e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2216512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d240));
    } else {
        emu.pc = 2216428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d1ec));
    }
}
#[inline(always)]
pub fn block_0x0021d1ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 0u32, 2216432u32);
    emu.sw_no_count(25usize, 18usize, 8u32, 2216436u32)?;
    emu.sw_no_count(26usize, 18usize, 12u32, 2216440u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2216444u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2216512u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021d240));
}
#[inline(always)]
pub fn block_0x0021d1fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 24usize, 0u32, 2216448u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2216448u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021d200));
}
#[inline(always)]
pub fn block_0x0021d200(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(26usize, 0usize, 0u32, 2216452u32);
    emu.sri_no_count(22usize, 22usize, 11u32, 2216456u32);
    emu.lw_no_count(23usize, 18usize, 0u32, 2216460u32)?;
    emu.lw_no_count(18usize, 18usize, 4u32, 2216464u32)?;
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(27usize, a);
    emu.pc = 2216468u32;
    emu.update_insn_clock();
    emu.adi_no_count(27usize, 27usize, 4294967295u32, 2216472u32);
    emu.anr_no_count(9usize, 25usize, 27usize, 2216476u32);
    emu.add_memory_rw_events(7usize);
    emu.pc = 2216476u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021d21c));
}
#[inline(always)]
pub fn block_0x0021d21c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(10usize, 26usize, 27usize, 2216480u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2216576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d280));
    } else {
        emu.pc = 2216484u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d224));
    }
}
#[inline(always)]
pub fn block_0x0021d224(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 16u32, 2216488u32)?;
    emu.adi_no_count(26usize, 26usize, 1u32, 2216492u32);
    emu.adi_no_count(10usize, 23usize, 0u32, 2216496u32);
    emu.adi_no_count(11usize, 22usize, 0u32, 2216500u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2216504u32;
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
pub fn block_0x0021d238(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2216476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d21c));
    } else {
        emu.pc = 2216508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d23c));
    }
}
#[inline(always)]
pub fn block_0x0021d23c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 1u32, 2216512u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2216512u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021d240));
}
#[inline]
pub fn block_0x0021d240(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 19usize, 0u32, 2216516u32);
    emu.lw_no_count(1usize, 2usize, 60u32, 2216520u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2216524u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2216528u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2216532u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2216536u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2216540u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2216544u32)?;
    emu.lw_no_count(22usize, 2usize, 32u32, 2216548u32)?;
    emu.lw_no_count(23usize, 2usize, 28u32, 2216552u32)?;
    emu.lw_no_count(24usize, 2usize, 24u32, 2216556u32)?;
    emu.lw_no_count(25usize, 2usize, 20u32, 2216560u32)?;
    emu.lw_no_count(26usize, 2usize, 16u32, 2216564u32)?;
    emu.lw_no_count(27usize, 2usize, 12u32, 2216568u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2216572u32);
    emu.add_memory_rw_events(16usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2216576u32;
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
pub fn block_0x0021d280(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 23usize, 0u32, 2216580u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2216584u32);
    emu.adi_no_count(12usize, 21usize, 0u32, 2216588u32);
    emu.adi_no_count(13usize, 20usize, 0u32, 2216592u32);
    emu.adi_no_count(14usize, 19usize, 0u32, 2216596u32);
    emu.apc_no_count(1usize, 2216596u32, 0u32, 2216600u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2216604u32;
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
pub fn block_0x0021d29c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 1u32, 2216608u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2216512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d240));
    } else {
        emu.pc = 2216612u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d2a4));
    }
}
#[inline(always)]
pub fn block_0x0021d2a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 18usize, 12u32, 2216616u32)?;
    emu.adi_no_count(10usize, 23usize, 0u32, 2216620u32);
    emu.lw_no_count(11usize, 2usize, 8u32, 2216624u32)?;
    emu.adi_no_count(12usize, 8usize, 0u32, 2216628u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2216632u32;
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
pub fn block_0x0021d2b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2216512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d240));
    } else {
        emu.pc = 2216636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d2bc));
    }
}
#[inline(always)]
pub fn block_0x0021d2bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 0usize, 0u32, 2216640u32);
    emu.sbr_no_count(10usize, 24usize, 25usize, 2216644u32);
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(9usize, a);
    emu.pc = 2216648u32;
    emu.update_insn_clock();
    emu.adi_no_count(9usize, 9usize, 4294967295u32, 2216652u32);
    emu.anr_no_count(20usize, 10usize, 9usize, 2216656u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2216656u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021d2d0));
}
#[inline(always)]
pub fn block_0x0021d2d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(10usize, 8usize, 9usize, 2216660u32);
    emu.sltru_no_count(19usize, 10usize, 20usize, 2216664u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a >= b {
        emu.pc = 2216512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d240));
    } else {
        emu.pc = 2216668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d2dc));
    }
}
#[inline(always)]
pub fn block_0x0021d2dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 18usize, 16u32, 2216672u32)?;
    emu.adi_no_count(8usize, 8usize, 1u32, 2216676u32);
    emu.adi_no_count(10usize, 23usize, 0u32, 2216680u32);
    emu.adi_no_count(11usize, 22usize, 0u32, 2216684u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2216688u32;
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
pub fn block_0x0021d2f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2216656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d2d0));
    } else {
        emu.pc = 2216692u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021d2f4));
    }
}
#[inline(always)]
pub fn block_0x0021d2f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2216696u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2216512u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021d240));
}
