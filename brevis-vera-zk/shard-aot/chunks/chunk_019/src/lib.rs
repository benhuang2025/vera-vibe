pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2165072u32;
pub const PC_MAX: u32 = 2167704u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 151usize] = [
        block_0x00210950,
        block_0x0021099c,
        block_0x002109ac,
        block_0x002109b0,
        block_0x002109b8,
        block_0x002109c0,
        block_0x00210a2c,
        block_0x00210a40,
        block_0x00210a44,
        block_0x00210a48,
        block_0x00210a58,
        block_0x00210a5c,
        block_0x00210a70,
        block_0x00210a74,
        block_0x00210a7c,
        block_0x00210ab4,
        block_0x00210abc,
        block_0x00210ac0,
        block_0x00210ad0,
        block_0x00210ad4,
        block_0x00210adc,
        block_0x00210ae0,
        block_0x00210ae4,
        block_0x00210ae8,
        block_0x00210af8,
        block_0x00210b00,
        block_0x00210b04,
        block_0x00210b0c,
        block_0x00210b10,
        block_0x00210b18,
        block_0x00210b1c,
        block_0x00210b28,
        block_0x00210b54,
        block_0x00210b64,
        block_0x00210b68,
        block_0x00210b80,
        block_0x00210b84,
        block_0x00210b8c,
        block_0x00210b90,
        block_0x00210bcc,
        block_0x00210be8,
        block_0x00210bf8,
        block_0x00210c2c,
        block_0x00210c4c,
        block_0x00210cac,
        block_0x00210cd4,
        block_0x00210cf0,
        block_0x00210d00,
        block_0x00210d08,
        block_0x00210d18,
        block_0x00210d34,
        block_0x00210d4c,
        block_0x00210d50,
        block_0x00210d84,
        block_0x00210dc0,
        block_0x00210dec,
        block_0x00210e38,
        block_0x00210e40,
        block_0x00210e70,
        block_0x00210e7c,
        block_0x00210e94,
        block_0x00210e98,
        block_0x00210ea8,
        block_0x00210eac,
        block_0x00210eb0,
        block_0x00210ec4,
        block_0x00210ec8,
        block_0x00210f14,
        block_0x00210f18,
        block_0x00210f34,
        block_0x00210f38,
        block_0x00210f3c,
        block_0x00210f48,
        block_0x00210f68,
        block_0x00210f6c,
        block_0x00210f88,
        block_0x00210f90,
        block_0x00210fa0,
        block_0x00210fac,
        block_0x00210fc0,
        block_0x00210fd4,
        block_0x0021102c,
        block_0x00211034,
        block_0x0021103c,
        block_0x0021107c,
        block_0x00211080,
        block_0x00211090,
        block_0x0021109c,
        block_0x002110ac,
        block_0x002110b0,
        block_0x002110b4,
        block_0x002110c0,
        block_0x002110c4,
        block_0x002110d4,
        block_0x002110ec,
        block_0x00211108,
        block_0x0021112c,
        block_0x00211138,
        block_0x00211144,
        block_0x00211158,
        block_0x0021116c,
        block_0x00211174,
        block_0x0021117c,
        block_0x00211180,
        block_0x0021118c,
        block_0x00211190,
        block_0x00211194,
        block_0x00211198,
        block_0x0021119c,
        block_0x002111b0,
        block_0x002111b4,
        block_0x002111c0,
        block_0x002111e0,
        block_0x002111e4,
        block_0x002111f0,
        block_0x00211200,
        block_0x00211204,
        block_0x00211208,
        block_0x00211220,
        block_0x00211224,
        block_0x0021122c,
        block_0x00211238,
        block_0x00211244,
        block_0x0021124c,
        block_0x00211258,
        block_0x00211264,
        block_0x0021126c,
        block_0x00211274,
        block_0x0021127c,
        block_0x0021128c,
        block_0x002112a0,
        block_0x002112a4,
        block_0x002112a8,
        block_0x002112b8,
        block_0x002112bc,
        block_0x002112c4,
        block_0x002112c8,
        block_0x002112cc,
        block_0x002112d0,
        block_0x002112e0,
        block_0x002112e8,
        block_0x002112f0,
        block_0x002112f4,
        block_0x0021130c,
        block_0x00211310,
        block_0x0021131c,
        block_0x00211320,
        block_0x00211360,
        block_0x00211368,
        block_0x0021137c,
        block_0x00211398,
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
    if pc < 2165072u32 || pc > 2167704u32 {
        return None;
    }
    let word_offset = ((pc - 2165072u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline]
pub fn block_0x00210950(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967216u32, 2165076u32);
    emu.sw_no_count(1usize, 2usize, 76u32, 2165080u32)?;
    emu.sw_no_count(8usize, 2usize, 72u32, 2165084u32)?;
    emu.sw_no_count(9usize, 2usize, 68u32, 2165088u32)?;
    emu.sw_no_count(18usize, 2usize, 64u32, 2165092u32)?;
    emu.sw_no_count(19usize, 2usize, 60u32, 2165096u32)?;
    emu.sw_no_count(20usize, 2usize, 56u32, 2165100u32)?;
    emu.sw_no_count(21usize, 2usize, 52u32, 2165104u32)?;
    emu.sw_no_count(22usize, 2usize, 48u32, 2165108u32)?;
    emu.sw_no_count(23usize, 2usize, 44u32, 2165112u32)?;
    emu.sw_no_count(24usize, 2usize, 40u32, 2165116u32)?;
    emu.sw_no_count(25usize, 2usize, 36u32, 2165120u32)?;
    emu.sw_no_count(26usize, 2usize, 32u32, 2165124u32)?;
    emu.sw_no_count(27usize, 2usize, 28u32, 2165128u32)?;
    emu.adi_no_count(18usize, 12usize, 0u32, 2165132u32);
    emu.adi_no_count(8usize, 11usize, 0u32, 2165136u32);
    emu.lw_no_count(12usize, 12usize, 4u32, 2165140u32)?;
    emu.adi_no_count(9usize, 10usize, 0u32, 2165144u32);
    emu.add_memory_rw_events(18usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2165176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002109b8));
    } else {
        emu.pc = 2165148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021099c));
    }
}
#[inline(always)]
pub fn block_0x0021099c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 18usize, 0u32, 2165152u32)?;
    emu.lw_no_count(13usize, 8usize, 12u32, 2165156u32)?;
    emu.adi_no_count(10usize, 9usize, 0u32, 2165160u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2165164u32;
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
pub fn block_0x002109ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2165176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002109b8));
    } else {
        emu.pc = 2165168u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002109b0));
    }
}
#[inline(always)]
pub fn block_0x002109b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2165172u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2165176u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2165648u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210b90));
}
#[inline(always)]
pub fn block_0x002109b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 12u32, 2165180u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2165648u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210b90));
    } else {
        emu.pc = 2165184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002109c0));
    }
}
#[inline(never)]
pub fn block_0x002109c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(20usize, 18usize, 8u32, 2165188u32)?;
    emu.sli_no_count(11usize, 10usize, 2u32, 2165192u32);
    emu.sli_no_count(10usize, 10usize, 4u32, 2165196u32);
    emu.adi_no_count(22usize, 0usize, 65u32, 2165200u32);
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(18usize, a);
    emu.pc = 2165204u32;
    emu.update_insn_clock();
    emu.adi_no_count(18usize, 18usize, 4294966976u32, 2165208u32);
    emu.adi_no_count(23usize, 0usize, 64u32, 2165212u32);
    emu.adi_no_count(24usize, 0usize, 1u32, 2165216u32);
    emu.adi_no_count(25usize, 0usize, 2u32, 2165220u32);
    let a = 0u32.wrapping_add(3435986944u32);
    emu.write_reg_no_count(27usize, a);
    emu.pc = 2165224u32;
    emu.update_insn_clock();
    emu.adi_no_count(21usize, 0usize, 10u32, 2165228u32);
    let a = 0u32.wrapping_add(393216u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2165232u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(524288u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2165236u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(917504u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2165240u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(516096u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2165244u32;
    emu.update_insn_clock();
    emu.sbr_no_count(10usize, 10usize, 11usize, 2165248u32);
    emu.adi_no_count(12usize, 12usize, 4294967286u32, 2165252u32);
    emu.sw_no_count(12usize, 2usize, 16u32, 2165256u32)?;
    emu.adi_no_count(11usize, 13usize, 4294967196u32, 2165260u32);
    emu.sw_no_count(11usize, 2usize, 12u32, 2165264u32)?;
    emu.adi_no_count(11usize, 14usize, 4294966296u32, 2165268u32);
    emu.sw_no_count(11usize, 2usize, 8u32, 2165272u32)?;
    emu.adr_no_count(26usize, 20usize, 10usize, 2165276u32);
    emu.adi_no_count(10usize, 20usize, 12u32, 2165280u32);
    emu.adi_no_count(11usize, 15usize, 4294965488u32, 2165284u32);
    emu.sw_no_count(11usize, 2usize, 4u32, 2165288u32)?;
    emu.add_memory_rw_events(27usize);
    let return_addr = 2165292u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2165320u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210a48));
}
#[inline(always)]
pub fn block_0x00210a2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 12usize, 4u32, 2165296u32)?;
    emu.lw_no_count(12usize, 12usize, 8u32, 2165300u32)?;
    emu.lw_no_count(13usize, 8usize, 12u32, 2165304u32)?;
    emu.adi_no_count(10usize, 9usize, 0u32, 2165308u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2165312u32;
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
pub fn block_0x00210a40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2165608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210b68));
    } else {
        emu.pc = 2165316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210a44));
    }
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
    emu.add_memory_rw_events(1usize);
    let return_addr = 2165320u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2165168u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002109b0));
}
#[inline(always)]
pub fn block_0x00210a48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 20usize, 0u32, 2165324u32);
    emu.lhu_no_count(11usize, 20usize, 0u32, 2165328u32)?;
    emu.adi_no_count(20usize, 10usize, 0u32, 2165332u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2165428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210ab4));
    } else {
        emu.pc = 2165336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210a58));
    }
}
#[inline(always)]
pub fn block_0x00210a58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2165292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210a2c));
    } else {
        emu.pc = 2165340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210a5c));
    }
}
#[inline(always)]
pub fn block_0x00210a5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lhu_no_count(10usize, 12usize, 2u32, 2165344u32)?;
    emu.sw_no_count(0usize, 2usize, 20u32, 2165348u32)?;
    emu.sb_no_count(0usize, 2usize, 24u32, 2165352u32);
    emu.lhu_no_count(11usize, 12usize, 0u32, 2165356u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(25usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2165508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210b04));
    } else {
        emu.pc = 2165360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210a70));
    }
}
#[inline(always)]
pub fn block_0x00210a70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2165516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210b0c));
    } else {
        emu.pc = 2165364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210a74));
    }
}
#[inline(always)]
pub fn block_0x00210a74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lhu_no_count(11usize, 12usize, 2u32, 2165368u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2165636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210b84));
    } else {
        emu.pc = 2165372u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210a7c));
    }
}
#[inline]
pub fn block_0x00210a7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 2usize, 16u32, 2165376u32)?;
    emu.adr_no_count(12usize, 11usize, 12usize, 2165380u32);
    emu.lw_no_count(13usize, 2usize, 12u32, 2165384u32)?;
    emu.adr_no_count(13usize, 11usize, 13usize, 2165388u32);
    emu.anr_no_count(12usize, 12usize, 13usize, 2165392u32);
    emu.lw_no_count(13usize, 2usize, 8u32, 2165396u32)?;
    emu.adr_no_count(13usize, 11usize, 13usize, 2165400u32);
    emu.lw_no_count(14usize, 2usize, 4u32, 2165404u32)?;
    emu.adr_no_count(11usize, 11usize, 14usize, 2165408u32);
    emu.anr_no_count(11usize, 13usize, 11usize, 2165412u32);
    emu.xrr_no_count(11usize, 12usize, 11usize, 2165416u32);
    emu.sri_no_count(11usize, 11usize, 17u32, 2165420u32);
    emu.adi_no_count(12usize, 11usize, 1u32, 2165424u32);
    emu.add_memory_rw_events(14usize);
    let return_addr = 2165428u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2165520u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210b10));
}
#[inline(always)]
pub fn block_0x00210ab4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(19usize, 12usize, 4u32, 2165432u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a < b {
        emu.pc = 2165472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210ae0));
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
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(22usize, 8usize, 12u32, 2165440u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2165440u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210ac0));
}
#[inline(always)]
pub fn block_0x00210ac0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 64u32, 2165444u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2165448u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2165452u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(22usize);
    let return_addr = 2165456u32;
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
pub fn block_0x00210ad0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2165168u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002109b0));
    } else {
        emu.pc = 2165460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210ad4));
    }
}
#[inline(always)]
pub fn block_0x00210ad4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 19usize, 4294967232u32, 2165464u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a < b {
        emu.pc = 2165440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210ac0));
    } else {
        emu.pc = 2165468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210adc));
    }
}
#[inline(always)]
pub fn block_0x00210adc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2165472u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2165480u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210ae8));
}
#[inline(always)]
pub fn block_0x00210ae0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2165608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210b68));
    } else {
        emu.pc = 2165476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210ae4));
    }
}
#[inline(always)]
pub fn block_0x00210ae4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(22usize, 8usize, 12u32, 2165480u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2165480u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210ae8));
}
#[inline(always)]
pub fn block_0x00210ae8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2165484u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2165488u32);
    emu.adi_no_count(12usize, 19usize, 0u32, 2165492u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(22usize);
    let return_addr = 2165496u32;
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
pub fn block_0x00210af8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(22usize, 0usize, 65u32, 2165500u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2165608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210b68));
    } else {
        emu.pc = 2165504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210b00));
    }
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
    emu.add_memory_rw_events(1usize);
    let return_addr = 2165508u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2165168u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002109b0));
}
#[inline(always)]
pub fn block_0x00210b04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 12usize, 8u32, 2165512u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2165516u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2165520u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210b10));
}
#[inline(always)]
pub fn block_0x00210b0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 12usize, 4u32, 2165520u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2165520u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210b10));
}
#[inline(always)]
pub fn block_0x00210b10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 6u32, 2165524u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a >= b {
        emu.pc = 2165708u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210bcc));
    } else {
        emu.pc = 2165528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210b18));
    }
}
#[inline(always)]
pub fn block_0x00210b18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2165588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210b54));
    } else {
        emu.pc = 2165532u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210b1c));
    }
}
#[inline(always)]
pub fn block_0x00210b1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 0usize, 12usize, 2165536u32);
    emu.adi_no_count(13usize, 2usize, 19u32, 2165540u32);
    emu.adr_no_count(13usize, 13usize, 12usize, 2165544u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2165544u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210b28));
}
#[inline]
pub fn block_0x00210b28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(14usize, 10usize, 16u32, 2165548u32);
    emu.adi_no_count(11usize, 11usize, 1u32, 2165552u32);
    emu.mulhu_no_count(14usize, 14usize, 27usize, 2165556u32);
    emu.sri_no_count(14usize, 14usize, 19u32, 2165560u32);
    emu.mul_no_count(15usize, 14usize, 21usize, 2165564u32);
    emu.sbr_no_count(10usize, 10usize, 15usize, 2165568u32);
    emu.ori_no_count(10usize, 10usize, 48u32, 2165572u32);
    emu.sb_no_count(10usize, 13usize, 0u32, 2165576u32);
    emu.adi_no_count(13usize, 13usize, 4294967295u32, 2165580u32);
    emu.adi_no_count(10usize, 14usize, 0u32, 2165584u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2165544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210b28));
    } else {
        emu.pc = 2165588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210b54));
    }
}
#[inline(always)]
pub fn block_0x00210b54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 8usize, 12u32, 2165592u32)?;
    emu.adi_no_count(11usize, 2usize, 20u32, 2165596u32);
    emu.adi_no_count(10usize, 9usize, 0u32, 2165600u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2165604u32;
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
pub fn block_0x00210b64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2165168u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002109b0));
    } else {
        emu.pc = 2165608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210b68));
    }
}
#[inline(always)]
pub fn block_0x00210b68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xrr_no_count(10usize, 20usize, 26usize, 2165612u32);
    emu.sltiu_no_count(10usize, 10usize, 1u32, 2165616u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2165620u32);
    emu.ani_no_count(10usize, 10usize, 12u32, 2165624u32);
    emu.adr_no_count(10usize, 20usize, 10usize, 2165628u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(26usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a != b {
        emu.pc = 2165320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210a48));
    } else {
        emu.pc = 2165632u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210b80));
    }
}
#[inline(always)]
pub fn block_0x00210b80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2165636u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2165644u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210b8c));
}
#[inline(always)]
pub fn block_0x00210b84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 1u32, 2165640u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2165644u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2165532u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210b1c));
}
#[inline(always)]
pub fn block_0x00210b8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 0u32, 2165648u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2165648u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210b90));
}
#[inline]
pub fn block_0x00210b90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 76u32, 2165652u32)?;
    emu.lw_no_count(8usize, 2usize, 72u32, 2165656u32)?;
    emu.lw_no_count(9usize, 2usize, 68u32, 2165660u32)?;
    emu.lw_no_count(18usize, 2usize, 64u32, 2165664u32)?;
    emu.lw_no_count(19usize, 2usize, 60u32, 2165668u32)?;
    emu.lw_no_count(20usize, 2usize, 56u32, 2165672u32)?;
    emu.lw_no_count(21usize, 2usize, 52u32, 2165676u32)?;
    emu.lw_no_count(22usize, 2usize, 48u32, 2165680u32)?;
    emu.lw_no_count(23usize, 2usize, 44u32, 2165684u32)?;
    emu.lw_no_count(24usize, 2usize, 40u32, 2165688u32)?;
    emu.lw_no_count(25usize, 2usize, 36u32, 2165692u32)?;
    emu.lw_no_count(26usize, 2usize, 32u32, 2165696u32)?;
    emu.lw_no_count(27usize, 2usize, 28u32, 2165700u32)?;
    emu.adi_no_count(2usize, 2usize, 80u32, 2165704u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2165708u32;
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
pub fn block_0x00210bcc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2165712u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294967040u32, 2165716u32);
    emu.adi_no_count(11usize, 0usize, 5u32, 2165720u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2165724u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2165728u32);
    emu.apc_no_count(1usize, 2165728u32, 12288u32, 2165732u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2165736u32;
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
pub fn block_0x00210be8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 10usize, 4u32, 2165740u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2165744u32)?;
    emu.lw_no_count(6usize, 13usize, 12u32, 2165748u32)?;
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2165752u32;
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
pub fn block_0x00210bf8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2165756u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2165760u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2165764u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2165768u32)?;
    emu.adi_no_count(8usize, 11usize, 0u32, 2165772u32);
    emu.lw_no_count(14usize, 11usize, 4u32, 2165776u32)?;
    emu.lw_no_count(11usize, 11usize, 0u32, 2165780u32)?;
    emu.lw_no_count(14usize, 14usize, 12u32, 2165784u32)?;
    emu.adi_no_count(9usize, 10usize, 0u32, 2165788u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2165792u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2165796u32);
    emu.adi_no_count(12usize, 13usize, 0u32, 2165800u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2165804u32;
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
pub fn block_0x00210c2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(8usize, 9usize, 0u32, 2165808u32)?;
    emu.sb_no_count(10usize, 9usize, 4u32, 2165812u32);
    emu.sb_no_count(0usize, 9usize, 5u32, 2165816u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2165820u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2165824u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2165828u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2165832u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2165836u32;
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
pub fn block_0x00210c4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 24u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2165840u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2165844u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2165848u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2165852u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2165856u32)?;
    emu.sw_no_count(19usize, 2usize, 28u32, 2165860u32)?;
    emu.sw_no_count(20usize, 2usize, 24u32, 2165864u32)?;
    emu.sw_no_count(21usize, 2usize, 20u32, 2165868u32)?;
    emu.sw_no_count(22usize, 2usize, 16u32, 2165872u32)?;
    emu.sw_no_count(23usize, 2usize, 12u32, 2165876u32)?;
    emu.sw_no_count(24usize, 2usize, 8u32, 2165880u32)?;
    emu.adi_no_count(8usize, 17usize, 0u32, 2165884u32);
    emu.adi_no_count(9usize, 16usize, 0u32, 2165888u32);
    emu.adi_no_count(18usize, 15usize, 0u32, 2165892u32);
    emu.adi_no_count(19usize, 14usize, 0u32, 2165896u32);
    emu.adi_no_count(20usize, 13usize, 0u32, 2165900u32);
    emu.adi_no_count(21usize, 10usize, 0u32, 2165904u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2165908u32)?;
    emu.lw_no_count(13usize, 21usize, 4u32, 2165912u32)?;
    emu.lw_no_count(22usize, 2usize, 48u32, 2165916u32)?;
    emu.lw_no_count(23usize, 2usize, 52u32, 2165920u32)?;
    emu.lw_no_count(24usize, 2usize, 56u32, 2165924u32)?;
    emu.lw_no_count(13usize, 13usize, 12u32, 2165928u32)?;
    emu.add_memory_rw_events(24usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2165932u32;
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
pub fn block_0x00210cac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(21usize, 2usize, 0u32, 2165936u32)?;
    emu.sb_no_count(10usize, 2usize, 4u32, 2165940u32);
    emu.sb_no_count(0usize, 2usize, 5u32, 2165944u32);
    emu.adi_no_count(10usize, 2usize, 0u32, 2165948u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2165952u32);
    emu.adi_no_count(12usize, 19usize, 0u32, 2165956u32);
    emu.adi_no_count(13usize, 18usize, 0u32, 2165960u32);
    emu.adi_no_count(14usize, 9usize, 0u32, 2165964u32);
    emu.apc_no_count(1usize, 2165964u32, 4294950912u32, 2165968u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2165972u32;
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
pub fn block_0x00210cd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 0u32, 2165976u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2165980u32);
    emu.adi_no_count(12usize, 22usize, 0u32, 2165984u32);
    emu.adi_no_count(13usize, 23usize, 0u32, 2165988u32);
    emu.adi_no_count(14usize, 24usize, 0u32, 2165992u32);
    emu.apc_no_count(1usize, 2165992u32, 4294950912u32, 2165996u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2166000u32;
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
pub fn block_0x00210cf0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 2usize, 4u32, 2166004u32);
    emu.lbu_no_count(12usize, 2usize, 5u32, 2166008u32);
    emu.orr_no_count(10usize, 12usize, 11usize, 2166012u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2166096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210d50));
    } else {
        emu.pc = 2166016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210d00));
    }
}
#[inline(always)]
pub fn block_0x00210d00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 11usize, 1u32, 2166020u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2166096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210d50));
    } else {
        emu.pc = 2166024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210d08));
    }
}
#[inline(always)]
pub fn block_0x00210d08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 0u32, 2166028u32)?;
    emu.lbu_no_count(11usize, 10usize, 10u32, 2166032u32);
    emu.ani_no_count(11usize, 11usize, 128u32, 2166036u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2166068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210d34));
    } else {
        emu.pc = 2166040u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210d18));
    }
}
#[inline(always)]
pub fn block_0x00210d18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 4u32, 2166044u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2166048u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2166052u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2166056u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 257u32, 2166060u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2166064u32);
    emu.add_memory_rw_events(7usize);
    let return_addr = 2166068u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2166092u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210d4c));
}
#[inline(always)]
pub fn block_0x00210d34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 10usize, 4u32, 2166072u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2166076u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2166080u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2166084u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 256u32, 2166088u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2166092u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2166092u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210d4c));
}
#[inline(always)]
pub fn block_0x00210d4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2166096u32;
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
pub fn block_0x00210d50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 1u32, 2166100u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2166104u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2166108u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2166112u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2166116u32)?;
    emu.lw_no_count(19usize, 2usize, 28u32, 2166120u32)?;
    emu.lw_no_count(20usize, 2usize, 24u32, 2166124u32)?;
    emu.lw_no_count(21usize, 2usize, 20u32, 2166128u32)?;
    emu.lw_no_count(22usize, 2usize, 16u32, 2166132u32)?;
    emu.lw_no_count(23usize, 2usize, 12u32, 2166136u32)?;
    emu.lw_no_count(24usize, 2usize, 8u32, 2166140u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2166144u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2166148u32;
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
pub fn block_0x00210d84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2166152u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2166156u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2166160u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2166164u32)?;
    emu.sw_no_count(18usize, 2usize, 0u32, 2166168u32)?;
    emu.adi_no_count(8usize, 13usize, 0u32, 2166172u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2166176u32);
    emu.lw_no_count(13usize, 11usize, 4u32, 2166180u32)?;
    emu.lw_no_count(11usize, 11usize, 0u32, 2166184u32)?;
    emu.lw_no_count(13usize, 13usize, 12u32, 2166188u32)?;
    emu.adi_no_count(18usize, 10usize, 0u32, 2166192u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2166196u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2166200u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2166204u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2166208u32;
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
pub fn block_0x00210dc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(11usize, 8usize, 1u32, 2166212u32);
    emu.sw_no_count(0usize, 18usize, 0u32, 2166216u32)?;
    emu.sw_no_count(9usize, 18usize, 4u32, 2166220u32)?;
    emu.sb_no_count(10usize, 18usize, 8u32, 2166224u32);
    emu.sb_no_count(11usize, 18usize, 9u32, 2166228u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2166232u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2166236u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2166240u32)?;
    emu.lw_no_count(18usize, 2usize, 0u32, 2166244u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2166248u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2166252u32;
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
pub fn block_0x00210dec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967216u32, 2166256u32);
    emu.sw_no_count(1usize, 2usize, 76u32, 2166260u32)?;
    emu.sw_no_count(8usize, 2usize, 72u32, 2166264u32)?;
    emu.sw_no_count(9usize, 2usize, 68u32, 2166268u32)?;
    emu.sw_no_count(18usize, 2usize, 64u32, 2166272u32)?;
    emu.sw_no_count(19usize, 2usize, 60u32, 2166276u32)?;
    emu.sw_no_count(20usize, 2usize, 56u32, 2166280u32)?;
    emu.sw_no_count(21usize, 2usize, 52u32, 2166284u32)?;
    emu.sw_no_count(22usize, 2usize, 48u32, 2166288u32)?;
    emu.sw_no_count(23usize, 2usize, 44u32, 2166292u32)?;
    emu.adi_no_count(20usize, 14usize, 0u32, 2166296u32);
    emu.adi_no_count(19usize, 13usize, 0u32, 2166300u32);
    emu.adi_no_count(9usize, 12usize, 0u32, 2166304u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2166308u32);
    emu.lw_no_count(22usize, 10usize, 4u32, 2166312u32)?;
    emu.lw_no_count(21usize, 10usize, 0u32, 2166316u32)?;
    emu.lw_no_count(23usize, 22usize, 12u32, 2166320u32)?;
    emu.adi_no_count(10usize, 21usize, 0u32, 2166324u32);
    emu.add_memory_rw_events(19usize);
    let base = emu.read_reg_b_tracked(23usize);
    let return_addr = 2166328u32;
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
pub fn block_0x00210e38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 1u32, 2166332u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2166384u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210e70));
    } else {
        emu.pc = 2166336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210e40));
    }
}
#[inline]
pub fn block_0x00210e40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2166340u32);
    emu.lw_no_count(1usize, 2usize, 76u32, 2166344u32)?;
    emu.lw_no_count(8usize, 2usize, 72u32, 2166348u32)?;
    emu.lw_no_count(9usize, 2usize, 68u32, 2166352u32)?;
    emu.lw_no_count(18usize, 2usize, 64u32, 2166356u32)?;
    emu.lw_no_count(19usize, 2usize, 60u32, 2166360u32)?;
    emu.lw_no_count(20usize, 2usize, 56u32, 2166364u32)?;
    emu.lw_no_count(21usize, 2usize, 52u32, 2166368u32)?;
    emu.lw_no_count(22usize, 2usize, 48u32, 2166372u32)?;
    emu.lw_no_count(23usize, 2usize, 44u32, 2166376u32)?;
    emu.adi_no_count(2usize, 2usize, 80u32, 2166380u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2166384u32;
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
pub fn block_0x00210e70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 8usize, 10u32, 2166388u32);
    emu.ani_no_count(10usize, 10usize, 128u32, 2166392u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2166448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210eb0));
    } else {
        emu.pc = 2166396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210e7c));
    }
}
#[inline(always)]
pub fn block_0x00210e7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2166400u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 259u32, 2166404u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2166408u32);
    emu.adi_no_count(18usize, 0usize, 1u32, 2166412u32);
    emu.adi_no_count(10usize, 21usize, 0u32, 2166416u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(23usize);
    let return_addr = 2166420u32;
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
pub fn block_0x00210e94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2166336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210e40));
    } else {
        emu.pc = 2166424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210e98));
    }
}
#[inline(always)]
pub fn block_0x00210e98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 20usize, 12u32, 2166428u32)?;
    emu.adi_no_count(10usize, 19usize, 0u32, 2166432u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2166436u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2166440u32;
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
pub fn block_0x00210ea8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2166336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210e40));
    } else {
        emu.pc = 2166444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210eac));
    }
}
#[inline(always)]
pub fn block_0x00210eac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2166448u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2166584u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210f38));
}
#[inline(always)]
pub fn block_0x00210eb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2166452u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 260u32, 2166456u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2166460u32);
    emu.adi_no_count(10usize, 21usize, 0u32, 2166464u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(23usize);
    let return_addr = 2166468u32;
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
pub fn block_0x00210ec4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2166336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210e40));
    } else {
        emu.pc = 2166472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210ec8));
    }
}
#[inline]
pub fn block_0x00210ec8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 1u32, 2166476u32);
    emu.adi_no_count(10usize, 2usize, 27u32, 2166480u32);
    emu.lw_no_count(11usize, 8usize, 8u32, 2166484u32)?;
    emu.lw_no_count(12usize, 8usize, 12u32, 2166488u32)?;
    emu.adi_no_count(13usize, 2usize, 12u32, 2166492u32);
    emu.sw_no_count(21usize, 2usize, 12u32, 2166496u32)?;
    emu.sw_no_count(22usize, 2usize, 16u32, 2166500u32)?;
    emu.sw_no_count(10usize, 2usize, 20u32, 2166504u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2166508u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 220u32, 2166512u32);
    emu.lw_no_count(14usize, 20usize, 12u32, 2166516u32)?;
    emu.sb_no_count(18usize, 2usize, 27u32, 2166520u32);
    emu.sw_no_count(13usize, 2usize, 28u32, 2166524u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2166528u32)?;
    emu.sw_no_count(11usize, 2usize, 36u32, 2166532u32)?;
    emu.sw_no_count(12usize, 2usize, 40u32, 2166536u32)?;
    emu.adi_no_count(11usize, 2usize, 28u32, 2166540u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2166544u32);
    emu.add_memory_rw_events(19usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2166548u32;
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
pub fn block_0x00210f14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2166336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210e40));
    } else {
        emu.pc = 2166552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210f18));
    }
}
#[inline(always)]
pub fn block_0x00210f18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 32u32, 2166556u32)?;
    emu.lw_no_count(10usize, 2usize, 28u32, 2166560u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2166564u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2166568u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 254u32, 2166572u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2166576u32);
    emu.add_memory_rw_events(7usize);
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
#[inline(always)]
pub fn block_0x00210f34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2166336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210e40));
    } else {
        emu.pc = 2166584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210f38));
    }
}
#[inline(always)]
pub fn block_0x00210f38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2166636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210f6c));
    } else {
        emu.pc = 2166588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210f3c));
    }
}
#[inline(always)]
pub fn block_0x00210f3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 8usize, 10u32, 2166592u32);
    emu.ani_no_count(10usize, 10usize, 128u32, 2166596u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2166636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210f6c));
    } else {
        emu.pc = 2166600u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210f48));
    }
}
#[inline(always)]
pub fn block_0x00210f48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 8usize, 4u32, 2166604u32)?;
    emu.lw_no_count(10usize, 8usize, 0u32, 2166608u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2166612u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2166616u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 263u32, 2166620u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2166624u32);
    emu.adi_no_count(18usize, 0usize, 1u32, 2166628u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2166632u32;
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
pub fn block_0x00210f68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2166336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210e40));
    } else {
        emu.pc = 2166636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210f6c));
    }
}
#[inline(always)]
pub fn block_0x00210f6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 8usize, 4u32, 2166640u32)?;
    emu.lw_no_count(10usize, 8usize, 0u32, 2166644u32)?;
    emu.lw_no_count(13usize, 11usize, 12u32, 2166648u32)?;
    let a = 0u32.wrapping_add(2191360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2166652u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 262u32, 2166656u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2166660u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2166664u32;
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
pub fn block_0x00210f88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2166668u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2166672u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2166336u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00210e40));
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
    emu.lw_no_count(12usize, 10usize, 4u32, 2166676u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2166680u32)?;
    emu.lw_no_count(6usize, 12usize, 16u32, 2166684u32)?;
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2166688u32;
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
pub fn block_0x00210fa0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(12usize, 10usize, 0u32, 2166692u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2166696u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2166720u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210fc0));
    } else {
        emu.pc = 2166700u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00210fac));
    }
}
#[inline(always)]
pub fn block_0x00210fac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2187264u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2166704u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 96u32, 2166708u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2166712u32);
    emu.apc_no_count(6usize, 2166712u32, 4294963200u32, 2166716u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2166720u32;
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
pub fn block_0x00210fc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2166724u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967056u32, 2166728u32);
    emu.adi_no_count(12usize, 0usize, 5u32, 2166732u32);
    emu.apc_no_count(6usize, 2166732u32, 4294963200u32, 2166736u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2166740u32;
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
pub fn block_0x00210fd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 22u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967200u32, 2166744u32);
    emu.sw_no_count(1usize, 2usize, 92u32, 2166748u32)?;
    emu.sw_no_count(8usize, 2usize, 88u32, 2166752u32)?;
    emu.sw_no_count(9usize, 2usize, 84u32, 2166756u32)?;
    emu.sw_no_count(18usize, 2usize, 80u32, 2166760u32)?;
    emu.sw_no_count(19usize, 2usize, 76u32, 2166764u32)?;
    emu.sw_no_count(20usize, 2usize, 72u32, 2166768u32)?;
    emu.sw_no_count(21usize, 2usize, 68u32, 2166772u32)?;
    emu.sw_no_count(22usize, 2usize, 64u32, 2166776u32)?;
    emu.sw_no_count(23usize, 2usize, 60u32, 2166780u32)?;
    emu.sw_no_count(24usize, 2usize, 56u32, 2166784u32)?;
    emu.sw_no_count(25usize, 2usize, 52u32, 2166788u32)?;
    emu.sw_no_count(26usize, 2usize, 48u32, 2166792u32)?;
    emu.sw_no_count(27usize, 2usize, 44u32, 2166796u32)?;
    emu.adi_no_count(19usize, 11usize, 0u32, 2166800u32);
    emu.lw_no_count(21usize, 12usize, 4u32, 2166804u32)?;
    emu.lw_no_count(8usize, 12usize, 0u32, 2166808u32)?;
    emu.lw_no_count(9usize, 21usize, 16u32, 2166812u32)?;
    emu.adi_no_count(23usize, 10usize, 0u32, 2166816u32);
    emu.adi_no_count(11usize, 0usize, 34u32, 2166820u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2166824u32);
    emu.add_memory_rw_events(22usize);
    let base = emu.read_reg_b_tracked(9usize);
    let return_addr = 2166828u32;
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
pub fn block_0x0021102c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 1u32, 2166832u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2167584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211320));
    } else {
        emu.pc = 2166836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211034));
    }
}
#[inline(always)]
pub fn block_0x00211034(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(8usize, 2usize, 24u32, 2166840u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2167484u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002112bc));
    } else {
        emu.pc = 2166844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021103c));
    }
}
#[inline]
pub fn block_0x0021103c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(21usize, 2usize, 16u32, 2166848u32)?;
    emu.sw_no_count(9usize, 2usize, 12u32, 2166852u32)?;
    emu.adi_no_count(20usize, 0usize, 0u32, 2166856u32);
    emu.adi_no_count(9usize, 0usize, 0u32, 2166860u32);
    emu.sbr_no_count(10usize, 0usize, 19usize, 2166864u32);
    emu.sw_no_count(10usize, 2usize, 8u32, 2166868u32)?;
    emu.adi_no_count(27usize, 0usize, 4294967201u32, 2166872u32);
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2166876u32;
    emu.update_insn_clock();
    emu.adi_no_count(24usize, 0usize, 1u32, 2166880u32);
    emu.adi_no_count(26usize, 0usize, 34u32, 2166884u32);
    emu.adi_no_count(10usize, 10usize, 1u32, 2166888u32);
    emu.sw_no_count(10usize, 2usize, 20u32, 2166892u32)?;
    emu.adi_no_count(21usize, 0usize, 92u32, 2166896u32);
    emu.adi_no_count(18usize, 23usize, 0u32, 2166900u32);
    emu.adi_no_count(13usize, 19usize, 0u32, 2166904u32);
    emu.add_memory_rw_events(16usize);
    let return_addr = 2166908u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2166928u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211090));
}
#[inline(always)]
pub fn block_0x0021107c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2166912u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2166912u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211080));
}
#[inline(always)]
pub fn block_0x00211080(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 10usize, 9usize, 2166916u32);
    emu.sbr_no_count(13usize, 8usize, 18usize, 2166920u32);
    emu.adr_no_count(9usize, 10usize, 25usize, 2166924u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2167656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211368));
    } else {
        emu.pc = 2166928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211090));
    }
}
#[inline(always)]
pub fn block_0x00211090(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 0usize, 0u32, 2166932u32);
    emu.adr_no_count(8usize, 18usize, 13usize, 2166936u32);
    emu.sbr_no_count(11usize, 0usize, 13usize, 2166940u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2166940u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021109c));
}
#[inline(always)]
pub fn block_0x0021109c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 18usize, 25usize, 2166944u32);
    emu.lbu_no_count(12usize, 10usize, 0u32, 2166948u32);
    emu.adi_no_count(14usize, 12usize, 4294967169u32, 2166952u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(27usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a < b {
        emu.pc = 2166980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002110c4));
    } else {
        emu.pc = 2166956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002110ac));
    }
}
#[inline(always)]
pub fn block_0x002110ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2166980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002110c4));
    } else {
        emu.pc = 2166960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002110b0));
    }
}
#[inline(always)]
pub fn block_0x002110b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2166980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002110c4));
    } else {
        emu.pc = 2166964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002110b4));
    }
}
#[inline(always)]
pub fn block_0x002110b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 25usize, 1u32, 2166968u32);
    emu.adr_no_count(10usize, 11usize, 25usize, 2166972u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2166940u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021109c));
    } else {
        emu.pc = 2166976u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002110c0));
    }
}
#[inline(always)]
pub fn block_0x002110c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2166980u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2167436u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021128c));
}
#[inline(always)]
pub fn block_0x002110c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(11usize, 10usize, 0u32, 2166984u32);
    emu.adi_no_count(18usize, 10usize, 1u32, 2166988u32);
    emu.ani_no_count(22usize, 11usize, 255u32, 2166992u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2167108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211144));
    } else {
        emu.pc = 2166996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002110d4));
    }
}
#[inline(always)]
pub fn block_0x002110d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(12usize, 18usize, 0u32, 2167000u32);
    emu.ani_no_count(11usize, 22usize, 31u32, 2167004u32);
    emu.adi_no_count(18usize, 10usize, 2u32, 2167008u32);
    emu.ani_no_count(13usize, 12usize, 63u32, 2167012u32);
    emu.adi_no_count(12usize, 0usize, 224u32, 2167016u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a < b {
        emu.pc = 2167084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021112c));
    } else {
        emu.pc = 2167020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002110ec));
    }
}
#[inline(always)]
pub fn block_0x002110ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(14usize, 18usize, 0u32, 2167024u32);
    emu.adi_no_count(12usize, 10usize, 3u32, 2167028u32);
    emu.sli_no_count(13usize, 13usize, 6u32, 2167032u32);
    emu.ani_no_count(14usize, 14usize, 63u32, 2167036u32);
    emu.orr_no_count(13usize, 13usize, 14usize, 2167040u32);
    emu.adi_no_count(14usize, 0usize, 240u32, 2167044u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a < b {
        emu.pc = 2167096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211138));
    } else {
        emu.pc = 2167048u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211108));
    }
}
#[inline]
pub fn block_0x00211108(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 4u32, 2167052u32);
    emu.lbu_no_count(10usize, 12usize, 0u32, 2167056u32);
    emu.sli_no_count(11usize, 11usize, 29u32, 2167060u32);
    emu.sri_no_count(11usize, 11usize, 11u32, 2167064u32);
    emu.sli_no_count(13usize, 13usize, 6u32, 2167068u32);
    emu.ani_no_count(10usize, 10usize, 63u32, 2167072u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2167076u32);
    emu.orr_no_count(22usize, 13usize, 10usize, 2167080u32);
    emu.add_memory_rw_events(9usize);
    let return_addr = 2167084u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2167108u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211144));
}
#[inline(always)]
pub fn block_0x0021112c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(11usize, 11usize, 6u32, 2167088u32);
    emu.orr_no_count(22usize, 11usize, 13usize, 2167092u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2167096u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2167108u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211144));
}
#[inline(always)]
pub fn block_0x00211138(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(11usize, 11usize, 12u32, 2167100u32);
    emu.orr_no_count(22usize, 13usize, 11usize, 2167104u32);
    emu.adi_no_count(18usize, 12usize, 0u32, 2167108u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2167108u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211144));
}
#[inline(always)]
pub fn block_0x00211144(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 28u32, 2167112u32);
    emu.adi_no_count(11usize, 22usize, 0u32, 2167116u32);
    emu.lw_no_count(12usize, 2usize, 20u32, 2167120u32)?;
    emu.apc_no_count(1usize, 2167120u32, 4294963200u32, 2167124u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2167128u32;
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
pub fn block_0x00211158(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 40u32, 2167132u32);
    emu.lbu_no_count(11usize, 2usize, 41u32, 2167136u32);
    emu.sbr_no_count(11usize, 11usize, 10usize, 2167140u32);
    emu.ani_no_count(10usize, 11usize, 255u32, 2167144u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2167396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211264));
    } else {
        emu.pc = 2167148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021116c));
    }
}
#[inline(always)]
pub fn block_0x0021116c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(13usize, 9usize, 25usize, 2167152u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2167704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211398));
    } else {
        emu.pc = 2167156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211174));
    }
}
#[inline(always)]
pub fn block_0x00211174(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(11usize, 23usize, 20usize, 2167160u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2167188u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211194));
    } else {
        emu.pc = 2167164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021117c));
    }
}
#[inline(always)]
pub fn block_0x0021117c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2167184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211190));
    } else {
        emu.pc = 2167168u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211180));
    }
}
#[inline(always)]
pub fn block_0x00211180(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lb_no_count(10usize, 11usize, 0u32, 2167172u32);
    emu.adi_no_count(12usize, 0usize, 4294967231u32, 2167176u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2167188u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211194));
    } else {
        emu.pc = 2167180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021118c));
    }
}
#[inline(always)]
pub fn block_0x0021118c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2167184u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2167704u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211398));
}
#[inline(always)]
pub fn block_0x00211190(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2167704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211398));
    } else {
        emu.pc = 2167188u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211194));
    }
}
#[inline(always)]
pub fn block_0x00211194(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2167232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002111c0));
    } else {
        emu.pc = 2167192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211198));
    }
}
#[inline(always)]
pub fn block_0x00211198(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2167220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002111b4));
    } else {
        emu.pc = 2167196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021119c));
    }
}
#[inline(always)]
pub fn block_0x0021119c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 23usize, 9usize, 2167200u32);
    emu.adr_no_count(10usize, 10usize, 25usize, 2167204u32);
    emu.lb_no_count(10usize, 10usize, 0u32, 2167208u32);
    emu.adi_no_count(12usize, 0usize, 4294967231u32, 2167212u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2167232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002111c0));
    } else {
        emu.pc = 2167216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002111b0));
    }
}
#[inline(always)]
pub fn block_0x002111b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2167220u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2167704u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211398));
}
#[inline(always)]
pub fn block_0x002111b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2167224u32)?;
    emu.adr_no_count(10usize, 13usize, 10usize, 2167228u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2167704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211398));
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
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(24usize, 23usize, 0u32, 2167236u32);
    emu.lw_no_count(10usize, 2usize, 16u32, 2167240u32)?;
    emu.lw_no_count(23usize, 10usize, 12u32, 2167244u32)?;
    emu.sbr_no_count(12usize, 9usize, 20usize, 2167248u32);
    emu.adr_no_count(12usize, 12usize, 25usize, 2167252u32);
    emu.lw_no_count(20usize, 2usize, 24u32, 2167256u32)?;
    emu.adi_no_count(10usize, 20usize, 0u32, 2167260u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(23usize);
    let return_addr = 2167264u32;
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
pub fn block_0x002111e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2167648u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211360));
    } else {
        emu.pc = 2167268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002111e4));
    }
}
#[inline(always)]
pub fn block_0x002111e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 41u32, 2167272u32);
    emu.adi_no_count(11usize, 0usize, 129u32, 2167276u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a < b {
        emu.pc = 2167304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211208));
    } else {
        emu.pc = 2167280u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002111f0));
    }
}
#[inline(always)]
pub fn block_0x002111f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 28u32, 2167284u32)?;
    emu.adi_no_count(10usize, 20usize, 0u32, 2167288u32);
    emu.lw_no_count(12usize, 2usize, 12u32, 2167292u32)?;
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2167296u32;
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
pub fn block_0x00211200(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2167332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211224));
    } else {
        emu.pc = 2167300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211204));
    }
}
#[inline(always)]
pub fn block_0x00211204(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2167304u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2167648u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211360));
}
#[inline(always)]
pub fn block_0x00211208(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 2usize, 40u32, 2167308u32);
    emu.sbr_no_count(12usize, 10usize, 11usize, 2167312u32);
    emu.adi_no_count(10usize, 2usize, 28u32, 2167316u32);
    emu.adr_no_count(11usize, 10usize, 11usize, 2167320u32);
    emu.adi_no_count(10usize, 20usize, 0u32, 2167324u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(23usize);
    let return_addr = 2167328u32;
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
pub fn block_0x00211220(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2167648u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211360));
    } else {
        emu.pc = 2167332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211224));
    }
}
#[inline(always)]
pub fn block_0x00211224(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 128u32, 2167336u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a >= b {
        emu.pc = 2167352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211238));
    } else {
        emu.pc = 2167340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021122c));
    }
}
#[inline(always)]
pub fn block_0x0021122c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 1u32, 2167344u32);
    emu.adi_no_count(23usize, 24usize, 0u32, 2167348u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2167352u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2167384u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211258));
}
#[inline(always)]
pub fn block_0x00211238(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 22usize, 11u32, 2167356u32);
    emu.adi_no_count(23usize, 24usize, 0u32, 2167360u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2167372u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021124c));
    } else {
        emu.pc = 2167364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211244));
    }
}
#[inline(always)]
pub fn block_0x00211244(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(20usize, 0usize, 2u32, 2167368u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2167372u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2167384u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211258));
}
#[inline(always)]
pub fn block_0x0021124c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 22usize, 16u32, 2167376u32);
    emu.sltru_no_count(20usize, 0usize, 10usize, 2167380u32);
    emu.adi_no_count(20usize, 20usize, 3u32, 2167384u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2167384u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211258));
}
#[inline(always)]
pub fn block_0x00211258(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(24usize, 0usize, 1u32, 2167388u32);
    emu.adr_no_count(10usize, 9usize, 25usize, 2167392u32);
    emu.adr_no_count(20usize, 20usize, 10usize, 2167396u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2167396u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211264));
}
#[inline(always)]
pub fn block_0x00211264(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 128u32, 2167400u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a < b {
        emu.pc = 2166908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021107c));
    } else {
        emu.pc = 2167404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021126c));
    }
}
#[inline(always)]
pub fn block_0x0021126c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 22usize, 11u32, 2167408u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2167420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021127c));
    } else {
        emu.pc = 2167412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211274));
    }
}
#[inline(always)]
pub fn block_0x00211274(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2167416u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2167420u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2166912u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211080));
}
#[inline(always)]
pub fn block_0x0021127c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 22usize, 16u32, 2167424u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2167428u32);
    emu.adi_no_count(10usize, 10usize, 3u32, 2167432u32);
    emu.add_memory_rw_events(4usize);
    let return_addr = 2167436u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2166912u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211080));
}
#[inline(always)]
pub fn block_0x0021128c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(13usize, 13usize, 9usize, 2167440u32);
    emu.lw_no_count(21usize, 2usize, 16u32, 2167444u32)?;
    emu.lw_no_count(9usize, 2usize, 12u32, 2167448u32)?;
    emu.adi_no_count(18usize, 0usize, 1u32, 2167452u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2167676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021137c));
    } else {
        emu.pc = 2167456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002112a0));
    }
}
#[inline(always)]
pub fn block_0x002112a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2167496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002112c8));
    } else {
        emu.pc = 2167460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002112a4));
    }
}
#[inline(always)]
pub fn block_0x002112a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2167492u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002112c4));
    } else {
        emu.pc = 2167464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002112a8));
    }
}
#[inline(always)]
pub fn block_0x002112a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 23usize, 20usize, 2167468u32);
    emu.lb_no_count(10usize, 10usize, 0u32, 2167472u32);
    emu.adi_no_count(11usize, 0usize, 4294967231u32, 2167476u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2167496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002112c8));
    } else {
        emu.pc = 2167480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002112b8));
    }
}
#[inline(always)]
pub fn block_0x002112b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2167484u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2167676u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021137c));
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
    emu.adi_no_count(20usize, 0usize, 0u32, 2167488u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2167492u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2167540u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002112f4));
}
#[inline(always)]
pub fn block_0x002112c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2167676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021137c));
    } else {
        emu.pc = 2167496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002112c8));
    }
}
#[inline(always)]
pub fn block_0x002112c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2167528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002112e8));
    } else {
        emu.pc = 2167500u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002112cc));
    }
}
#[inline(always)]
pub fn block_0x002112cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2167536u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002112f0));
    } else {
        emu.pc = 2167504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002112d0));
    }
}
#[inline(always)]
pub fn block_0x002112d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 23usize, 13usize, 2167508u32);
    emu.lb_no_count(10usize, 10usize, 0u32, 2167512u32);
    emu.adi_no_count(11usize, 0usize, 4294967231u32, 2167516u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2167676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021137c));
    } else {
        emu.pc = 2167520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002112e0));
    }
}
#[inline(always)]
pub fn block_0x002112e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 13usize, 0u32, 2167524u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2167528u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2167540u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002112f4));
}
#[inline(always)]
pub fn block_0x002112e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 0usize, 0u32, 2167532u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2167536u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2167540u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002112f4));
}
#[inline(always)]
pub fn block_0x002112f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2167676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021137c));
    } else {
        emu.pc = 2167540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002112f4));
    }
}
#[inline(always)]
pub fn block_0x002112f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(13usize, 21usize, 12u32, 2167544u32)?;
    emu.sbr_no_count(12usize, 19usize, 20usize, 2167548u32);
    emu.adr_no_count(11usize, 23usize, 20usize, 2167552u32);
    emu.lw_no_count(8usize, 2usize, 24u32, 2167556u32)?;
    emu.adi_no_count(10usize, 8usize, 0u32, 2167560u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2167564u32;
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
pub fn block_0x0021130c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2167584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211320));
    } else {
        emu.pc = 2167568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211310));
    }
}
#[inline(always)]
pub fn block_0x00211310(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 34u32, 2167572u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2167576u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(9usize);
    let return_addr = 2167580u32;
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
pub fn block_0x0021131c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 10usize, 0u32, 2167584u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2167584u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211320));
}
#[inline]
pub fn block_0x00211320(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 18usize, 0u32, 2167588u32);
    emu.lw_no_count(1usize, 2usize, 92u32, 2167592u32)?;
    emu.lw_no_count(8usize, 2usize, 88u32, 2167596u32)?;
    emu.lw_no_count(9usize, 2usize, 84u32, 2167600u32)?;
    emu.lw_no_count(18usize, 2usize, 80u32, 2167604u32)?;
    emu.lw_no_count(19usize, 2usize, 76u32, 2167608u32)?;
    emu.lw_no_count(20usize, 2usize, 72u32, 2167612u32)?;
    emu.lw_no_count(21usize, 2usize, 68u32, 2167616u32)?;
    emu.lw_no_count(22usize, 2usize, 64u32, 2167620u32)?;
    emu.lw_no_count(23usize, 2usize, 60u32, 2167624u32)?;
    emu.lw_no_count(24usize, 2usize, 56u32, 2167628u32)?;
    emu.lw_no_count(25usize, 2usize, 52u32, 2167632u32)?;
    emu.lw_no_count(26usize, 2usize, 48u32, 2167636u32)?;
    emu.lw_no_count(27usize, 2usize, 44u32, 2167640u32)?;
    emu.adi_no_count(2usize, 2usize, 96u32, 2167644u32);
    emu.add_memory_rw_events(16usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2167648u32;
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
pub fn block_0x00211360(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 1u32, 2167652u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2167656u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2167584u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211320));
}
#[inline(always)]
pub fn block_0x00211368(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(13usize, 10usize, 25usize, 2167660u32);
    emu.lw_no_count(21usize, 2usize, 16u32, 2167664u32)?;
    emu.lw_no_count(9usize, 2usize, 12u32, 2167668u32)?;
    emu.adi_no_count(18usize, 0usize, 1u32, 2167672u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a >= b {
        emu.pc = 2167456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002112a0));
    } else {
        emu.pc = 2167676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021137c));
    }
}
#[inline(always)]
pub fn block_0x0021137c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2167680u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294967080u32, 2167684u32);
    emu.adi_no_count(10usize, 23usize, 0u32, 2167688u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2167692u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2167696u32);
    emu.apc_no_count(1usize, 2167696u32, 4294946816u32, 2167700u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2167704u32;
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
pub fn block_0x00211398(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2195456u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2167708u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294967064u32, 2167712u32);
    emu.adi_no_count(10usize, 23usize, 0u32, 2167716u32);
    emu.adi_no_count(11usize, 19usize, 0u32, 2167720u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2167724u32);
    emu.apc_no_count(1usize, 2167724u32, 4294946816u32, 2167728u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2167732u32;
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
