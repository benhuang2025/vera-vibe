pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2116140u32;
pub const PC_MAX: u32 = 2118508u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 114usize] = [
        block_0x00204a2c,
        block_0x00204ac0,
        block_0x00204ad4,
        block_0x00204ae4,
        block_0x00204b38,
        block_0x00204b48,
        block_0x00204b9c,
        block_0x00204bb0,
        block_0x00204c04,
        block_0x00204c18,
        block_0x00204cac,
        block_0x00204cbc,
        block_0x00204d10,
        block_0x00204d24,
        block_0x00204d38,
        block_0x00204d48,
        block_0x00204d58,
        block_0x00204dcc,
        block_0x00204de8,
        block_0x00204df0,
        block_0x00204df8,
        block_0x00204e14,
        block_0x00204e1c,
        block_0x00204e20,
        block_0x00204e3c,
        block_0x00204e70,
        block_0x00204e7c,
        block_0x00204ea0,
        block_0x00204eac,
        block_0x00204ec8,
        block_0x00204ecc,
        block_0x00204ef4,
        block_0x00204f00,
        block_0x00204f08,
        block_0x00204f24,
        block_0x00204f40,
        block_0x00204f68,
        block_0x00204f74,
        block_0x00204fdc,
        block_0x00204fe8,
        block_0x00204ff4,
        block_0x00204ffc,
        block_0x00205004,
        block_0x0020500c,
        block_0x00205014,
        block_0x00205024,
        block_0x00205030,
        block_0x00205040,
        block_0x00205048,
        block_0x0020504c,
        block_0x00205088,
        block_0x00205094,
        block_0x002050b4,
        block_0x002050bc,
        block_0x002050dc,
        block_0x002050e8,
        block_0x00205108,
        block_0x00205110,
        block_0x00205120,
        block_0x00205124,
        block_0x00205134,
        block_0x00205138,
        block_0x00205148,
        block_0x0020514c,
        block_0x0020515c,
        block_0x00205160,
        block_0x00205170,
        block_0x00205174,
        block_0x00205184,
        block_0x00205188,
        block_0x00205198,
        block_0x0020519c,
        block_0x002051ac,
        block_0x002051b0,
        block_0x002051c0,
        block_0x002051c4,
        block_0x002051d4,
        block_0x002051d8,
        block_0x002051e8,
        block_0x002051ec,
        block_0x002051fc,
        block_0x00205200,
        block_0x00205210,
        block_0x00205214,
        block_0x00205224,
        block_0x00205228,
        block_0x00205238,
        block_0x0020523c,
        block_0x0020524c,
        block_0x00205250,
        block_0x00205260,
        block_0x00205264,
        block_0x00205274,
        block_0x00205278,
        block_0x00205288,
        block_0x0020528c,
        block_0x0020529c,
        block_0x002052a0,
        block_0x002052b0,
        block_0x002052b4,
        block_0x002052c4,
        block_0x002052c8,
        block_0x002052d8,
        block_0x002052dc,
        block_0x002052ec,
        block_0x002052f0,
        block_0x00205300,
        block_0x00205304,
        block_0x00205314,
        block_0x00205318,
        block_0x00205328,
        block_0x0020532c,
        block_0x00205350,
        block_0x0020536c,
    ];
    const IDX: [u16; 593usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16,
        0u16, 0u16, 0u16, 3u16, 0u16, 0u16, 0u16, 4u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 5u16, 0u16, 0u16, 0u16, 6u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 7u16, 0u16, 0u16, 0u16, 0u16, 8u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 9u16, 0u16, 0u16, 0u16, 0u16, 10u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 11u16, 0u16, 0u16, 0u16, 12u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 13u16, 0u16, 0u16, 0u16, 0u16, 14u16, 0u16, 0u16, 0u16, 0u16,
        15u16, 0u16, 0u16, 0u16, 16u16, 0u16, 0u16, 0u16, 17u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 18u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 19u16, 0u16, 20u16, 0u16, 21u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 22u16, 0u16, 23u16, 24u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        25u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        26u16, 0u16, 0u16, 27u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 28u16,
        0u16, 0u16, 29u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 30u16, 31u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 32u16, 0u16, 0u16, 33u16, 0u16, 34u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 35u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        36u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 37u16, 0u16, 0u16,
        38u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        39u16, 0u16, 0u16, 40u16, 0u16, 0u16, 41u16, 0u16, 42u16, 0u16, 43u16, 0u16,
        44u16, 0u16, 45u16, 0u16, 0u16, 0u16, 46u16, 0u16, 0u16, 47u16, 0u16, 0u16, 0u16,
        48u16, 0u16, 49u16, 50u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 51u16, 0u16, 0u16, 52u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 53u16, 0u16, 54u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        55u16, 0u16, 0u16, 56u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 57u16, 0u16,
        58u16, 0u16, 0u16, 0u16, 59u16, 60u16, 0u16, 0u16, 0u16, 61u16, 62u16, 0u16,
        0u16, 0u16, 63u16, 64u16, 0u16, 0u16, 0u16, 65u16, 66u16, 0u16, 0u16, 0u16,
        67u16, 68u16, 0u16, 0u16, 0u16, 69u16, 70u16, 0u16, 0u16, 0u16, 71u16, 72u16,
        0u16, 0u16, 0u16, 73u16, 74u16, 0u16, 0u16, 0u16, 75u16, 76u16, 0u16, 0u16, 0u16,
        77u16, 78u16, 0u16, 0u16, 0u16, 79u16, 80u16, 0u16, 0u16, 0u16, 81u16, 82u16,
        0u16, 0u16, 0u16, 83u16, 84u16, 0u16, 0u16, 0u16, 85u16, 86u16, 0u16, 0u16, 0u16,
        87u16, 88u16, 0u16, 0u16, 0u16, 89u16, 90u16, 0u16, 0u16, 0u16, 91u16, 92u16,
        0u16, 0u16, 0u16, 93u16, 94u16, 0u16, 0u16, 0u16, 95u16, 96u16, 0u16, 0u16, 0u16,
        97u16, 98u16, 0u16, 0u16, 0u16, 99u16, 100u16, 0u16, 0u16, 0u16, 101u16, 102u16,
        0u16, 0u16, 0u16, 103u16, 104u16, 0u16, 0u16, 0u16, 105u16, 106u16, 0u16, 0u16,
        0u16, 107u16, 108u16, 0u16, 0u16, 0u16, 109u16, 110u16, 0u16, 0u16, 0u16, 111u16,
        112u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 113u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 114u16,
    ];
    if pc < 2116140u32 || pc > 2118508u32 {
        return None;
    }
    let word_offset = ((pc - 2116140u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(never)]
pub fn block_0x00204a2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 20u32, 2116144u32)?;
    emu.lw_no_count(11usize, 2usize, 24u32, 2116148u32)?;
    emu.lw_no_count(12usize, 2usize, 28u32, 2116152u32)?;
    emu.lw_no_count(13usize, 2usize, 32u32, 2116156u32)?;
    emu.sw_no_count(10usize, 2usize, 628u32, 2116160u32)?;
    emu.sw_no_count(11usize, 2usize, 632u32, 2116164u32)?;
    emu.sw_no_count(12usize, 2usize, 636u32, 2116168u32)?;
    emu.sw_no_count(13usize, 2usize, 640u32, 2116172u32)?;
    emu.lw_no_count(10usize, 2usize, 4u32, 2116176u32)?;
    emu.lw_no_count(11usize, 2usize, 8u32, 2116180u32)?;
    emu.lw_no_count(12usize, 2usize, 12u32, 2116184u32)?;
    emu.lw_no_count(13usize, 2usize, 16u32, 2116188u32)?;
    emu.sw_no_count(10usize, 2usize, 612u32, 2116192u32)?;
    emu.sw_no_count(11usize, 2usize, 616u32, 2116196u32)?;
    emu.sw_no_count(12usize, 2usize, 620u32, 2116200u32)?;
    emu.sw_no_count(13usize, 2usize, 624u32, 2116204u32)?;
    emu.lw_no_count(10usize, 2usize, 372u32, 2116208u32)?;
    emu.lw_no_count(11usize, 2usize, 376u32, 2116212u32)?;
    emu.lw_no_count(12usize, 2usize, 380u32, 2116216u32)?;
    emu.lw_no_count(13usize, 2usize, 384u32, 2116220u32)?;
    emu.sw_no_count(10usize, 2usize, 660u32, 2116224u32)?;
    emu.sw_no_count(11usize, 2usize, 664u32, 2116228u32)?;
    emu.sw_no_count(12usize, 2usize, 668u32, 2116232u32)?;
    emu.sw_no_count(13usize, 2usize, 672u32, 2116236u32)?;
    emu.lw_no_count(10usize, 2usize, 356u32, 2116240u32)?;
    emu.lw_no_count(11usize, 2usize, 360u32, 2116244u32)?;
    emu.lw_no_count(12usize, 2usize, 364u32, 2116248u32)?;
    emu.lw_no_count(13usize, 2usize, 368u32, 2116252u32)?;
    emu.sw_no_count(10usize, 2usize, 644u32, 2116256u32)?;
    emu.sw_no_count(11usize, 2usize, 648u32, 2116260u32)?;
    emu.sw_no_count(12usize, 2usize, 652u32, 2116264u32)?;
    emu.sw_no_count(13usize, 2usize, 656u32, 2116268u32)?;
    emu.adi_no_count(10usize, 2usize, 580u32, 2116272u32);
    emu.adi_no_count(11usize, 2usize, 644u32, 2116276u32);
    emu.adi_no_count(12usize, 2usize, 612u32, 2116280u32);
    emu.apc_no_count(1usize, 2116280u32, 20480u32, 2116284u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116288u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1936u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204ac0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 388u32, 2116292u32);
    emu.adi_no_count(11usize, 2usize, 420u32, 2116296u32);
    emu.adi_no_count(12usize, 2usize, 580u32, 2116300u32);
    emu.apc_no_count(1usize, 2116300u32, 24576u32, 2116304u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116308u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965840u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204ad4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 612u32, 2116312u32);
    emu.adi_no_count(11usize, 2usize, 388u32, 2116316u32);
    emu.apc_no_count(1usize, 2116316u32, 53248u32, 2116320u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116324u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966704u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00204ae4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 404u32, 2116328u32)?;
    emu.lw_no_count(11usize, 2usize, 408u32, 2116332u32)?;
    emu.lw_no_count(12usize, 2usize, 412u32, 2116336u32)?;
    emu.lw_no_count(13usize, 2usize, 416u32, 2116340u32)?;
    emu.sw_no_count(10usize, 2usize, 660u32, 2116344u32)?;
    emu.sw_no_count(11usize, 2usize, 664u32, 2116348u32)?;
    emu.sw_no_count(12usize, 2usize, 668u32, 2116352u32)?;
    emu.sw_no_count(13usize, 2usize, 672u32, 2116356u32)?;
    emu.lw_no_count(10usize, 2usize, 388u32, 2116360u32)?;
    emu.lw_no_count(11usize, 2usize, 392u32, 2116364u32)?;
    emu.lw_no_count(12usize, 2usize, 396u32, 2116368u32)?;
    emu.lw_no_count(13usize, 2usize, 400u32, 2116372u32)?;
    emu.sw_no_count(10usize, 2usize, 644u32, 2116376u32)?;
    emu.sw_no_count(11usize, 2usize, 648u32, 2116380u32)?;
    emu.sw_no_count(12usize, 2usize, 652u32, 2116384u32)?;
    emu.sw_no_count(13usize, 2usize, 656u32, 2116388u32)?;
    emu.adi_no_count(10usize, 2usize, 452u32, 2116392u32);
    emu.adi_no_count(11usize, 2usize, 612u32, 2116396u32);
    emu.adi_no_count(12usize, 2usize, 644u32, 2116400u32);
    emu.apc_no_count(1usize, 2116400u32, 20480u32, 2116404u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116408u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1816u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204b38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 612u32, 2116412u32);
    emu.adi_no_count(11usize, 2usize, 4u32, 2116416u32);
    emu.apc_no_count(1usize, 2116416u32, 53248u32, 2116420u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116424u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966604u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00204b48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 20u32, 2116428u32)?;
    emu.lw_no_count(11usize, 2usize, 24u32, 2116432u32)?;
    emu.lw_no_count(12usize, 2usize, 28u32, 2116436u32)?;
    emu.lw_no_count(13usize, 2usize, 32u32, 2116440u32)?;
    emu.sw_no_count(10usize, 2usize, 660u32, 2116444u32)?;
    emu.sw_no_count(11usize, 2usize, 664u32, 2116448u32)?;
    emu.sw_no_count(12usize, 2usize, 668u32, 2116452u32)?;
    emu.sw_no_count(13usize, 2usize, 672u32, 2116456u32)?;
    emu.lw_no_count(10usize, 2usize, 4u32, 2116460u32)?;
    emu.lw_no_count(11usize, 2usize, 8u32, 2116464u32)?;
    emu.lw_no_count(12usize, 2usize, 12u32, 2116468u32)?;
    emu.lw_no_count(13usize, 2usize, 16u32, 2116472u32)?;
    emu.sw_no_count(10usize, 2usize, 644u32, 2116476u32)?;
    emu.sw_no_count(11usize, 2usize, 648u32, 2116480u32)?;
    emu.sw_no_count(12usize, 2usize, 652u32, 2116484u32)?;
    emu.sw_no_count(13usize, 2usize, 656u32, 2116488u32)?;
    emu.adi_no_count(10usize, 2usize, 580u32, 2116492u32);
    emu.adi_no_count(11usize, 2usize, 612u32, 2116496u32);
    emu.adi_no_count(12usize, 2usize, 644u32, 2116500u32);
    emu.apc_no_count(1usize, 2116500u32, 20480u32, 2116504u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116508u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1716u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204b9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 484u32, 2116512u32);
    emu.adi_no_count(11usize, 2usize, 580u32, 2116516u32);
    emu.adi_no_count(12usize, 2usize, 356u32, 2116520u32);
    emu.apc_no_count(1usize, 2116520u32, 24576u32, 2116524u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116528u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965620u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00204bb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 468u32, 2116532u32)?;
    emu.lw_no_count(11usize, 2usize, 472u32, 2116536u32)?;
    emu.lw_no_count(12usize, 2usize, 476u32, 2116540u32)?;
    emu.lw_no_count(13usize, 2usize, 480u32, 2116544u32)?;
    emu.sw_no_count(10usize, 2usize, 660u32, 2116548u32)?;
    emu.sw_no_count(11usize, 2usize, 664u32, 2116552u32)?;
    emu.sw_no_count(12usize, 2usize, 668u32, 2116556u32)?;
    emu.sw_no_count(13usize, 2usize, 672u32, 2116560u32)?;
    emu.lw_no_count(10usize, 2usize, 452u32, 2116564u32)?;
    emu.lw_no_count(11usize, 2usize, 456u32, 2116568u32)?;
    emu.lw_no_count(12usize, 2usize, 460u32, 2116572u32)?;
    emu.lw_no_count(13usize, 2usize, 464u32, 2116576u32)?;
    emu.sw_no_count(10usize, 2usize, 644u32, 2116580u32)?;
    emu.sw_no_count(11usize, 2usize, 648u32, 2116584u32)?;
    emu.sw_no_count(12usize, 2usize, 652u32, 2116588u32)?;
    emu.sw_no_count(13usize, 2usize, 656u32, 2116592u32)?;
    emu.adi_no_count(10usize, 2usize, 612u32, 2116596u32);
    emu.adi_no_count(11usize, 2usize, 484u32, 2116600u32);
    emu.adi_no_count(12usize, 2usize, 644u32, 2116604u32);
    emu.apc_no_count(1usize, 2116604u32, 24576u32, 2116608u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116612u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966104u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204c04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 516u32, 2116616u32);
    emu.adi_no_count(11usize, 2usize, 292u32, 2116620u32);
    emu.adi_no_count(12usize, 2usize, 612u32, 2116624u32);
    emu.apc_no_count(1usize, 2116624u32, 20480u32, 2116628u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116632u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1592u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00204c18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 9usize, 48u32, 2116636u32)?;
    emu.lw_no_count(11usize, 9usize, 52u32, 2116640u32)?;
    emu.lw_no_count(12usize, 9usize, 56u32, 2116644u32)?;
    emu.lw_no_count(13usize, 9usize, 60u32, 2116648u32)?;
    emu.sw_no_count(10usize, 2usize, 628u32, 2116652u32)?;
    emu.sw_no_count(11usize, 2usize, 632u32, 2116656u32)?;
    emu.sw_no_count(12usize, 2usize, 636u32, 2116660u32)?;
    emu.sw_no_count(13usize, 2usize, 640u32, 2116664u32)?;
    emu.lw_no_count(10usize, 9usize, 32u32, 2116668u32)?;
    emu.lw_no_count(11usize, 9usize, 36u32, 2116672u32)?;
    emu.lw_no_count(12usize, 9usize, 40u32, 2116676u32)?;
    emu.lw_no_count(13usize, 9usize, 44u32, 2116680u32)?;
    emu.sw_no_count(10usize, 2usize, 612u32, 2116684u32)?;
    emu.sw_no_count(11usize, 2usize, 616u32, 2116688u32)?;
    emu.sw_no_count(12usize, 2usize, 620u32, 2116692u32)?;
    emu.sw_no_count(13usize, 2usize, 624u32, 2116696u32)?;
    emu.lw_no_count(10usize, 9usize, 80u32, 2116700u32)?;
    emu.lw_no_count(11usize, 9usize, 84u32, 2116704u32)?;
    emu.lw_no_count(12usize, 9usize, 88u32, 2116708u32)?;
    emu.lw_no_count(13usize, 9usize, 92u32, 2116712u32)?;
    emu.sw_no_count(10usize, 2usize, 660u32, 2116716u32)?;
    emu.sw_no_count(11usize, 2usize, 664u32, 2116720u32)?;
    emu.sw_no_count(12usize, 2usize, 668u32, 2116724u32)?;
    emu.sw_no_count(13usize, 2usize, 672u32, 2116728u32)?;
    emu.lw_no_count(10usize, 9usize, 64u32, 2116732u32)?;
    emu.lw_no_count(11usize, 9usize, 68u32, 2116736u32)?;
    emu.lw_no_count(12usize, 9usize, 72u32, 2116740u32)?;
    emu.lw_no_count(13usize, 9usize, 76u32, 2116744u32)?;
    emu.sw_no_count(10usize, 2usize, 644u32, 2116748u32)?;
    emu.sw_no_count(11usize, 2usize, 648u32, 2116752u32)?;
    emu.sw_no_count(12usize, 2usize, 652u32, 2116756u32)?;
    emu.sw_no_count(13usize, 2usize, 656u32, 2116760u32)?;
    emu.adi_no_count(10usize, 2usize, 580u32, 2116764u32);
    emu.adi_no_count(11usize, 2usize, 612u32, 2116768u32);
    emu.adi_no_count(12usize, 2usize, 644u32, 2116772u32);
    emu.apc_no_count(1usize, 2116772u32, 24576u32, 2116776u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116780u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965936u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204cac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 548u32, 2116784u32);
    emu.adi_no_count(11usize, 2usize, 580u32, 2116788u32);
    emu.apc_no_count(1usize, 2116788u32, 53248u32, 2116792u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116796u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966232u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00204cbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 564u32, 2116800u32)?;
    emu.lw_no_count(11usize, 2usize, 568u32, 2116804u32)?;
    emu.lw_no_count(12usize, 2usize, 572u32, 2116808u32)?;
    emu.lw_no_count(13usize, 2usize, 576u32, 2116812u32)?;
    emu.sw_no_count(10usize, 2usize, 660u32, 2116816u32)?;
    emu.sw_no_count(11usize, 2usize, 664u32, 2116820u32)?;
    emu.sw_no_count(12usize, 2usize, 668u32, 2116824u32)?;
    emu.sw_no_count(13usize, 2usize, 672u32, 2116828u32)?;
    emu.lw_no_count(10usize, 2usize, 548u32, 2116832u32)?;
    emu.lw_no_count(11usize, 2usize, 552u32, 2116836u32)?;
    emu.lw_no_count(12usize, 2usize, 556u32, 2116840u32)?;
    emu.lw_no_count(13usize, 2usize, 560u32, 2116844u32)?;
    emu.sw_no_count(10usize, 2usize, 644u32, 2116848u32)?;
    emu.sw_no_count(11usize, 2usize, 648u32, 2116852u32)?;
    emu.sw_no_count(12usize, 2usize, 652u32, 2116856u32)?;
    emu.sw_no_count(13usize, 2usize, 656u32, 2116860u32)?;
    emu.adi_no_count(10usize, 2usize, 612u32, 2116864u32);
    emu.adi_no_count(11usize, 2usize, 452u32, 2116868u32);
    emu.adi_no_count(12usize, 2usize, 644u32, 2116872u32);
    emu.apc_no_count(1usize, 2116872u32, 24576u32, 2116876u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116880u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965836u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204d10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 2usize, 324u32, 2116884u32);
    emu.adi_no_count(12usize, 2usize, 612u32, 2116888u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2116892u32);
    emu.apc_no_count(1usize, 2116892u32, 24576u32, 2116896u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116900u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965248u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204d24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 644u32, 2116904u32);
    emu.adi_no_count(11usize, 2usize, 548u32, 2116908u32);
    emu.adi_no_count(12usize, 2usize, 36u32, 2116912u32);
    emu.apc_no_count(1usize, 2116912u32, 24576u32, 2116916u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116920u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965796u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204d38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 612u32, 2116924u32);
    emu.adi_no_count(11usize, 2usize, 644u32, 2116928u32);
    emu.apc_no_count(1usize, 2116928u32, 53248u32, 2116932u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116936u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966092u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204d48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 8usize, 64u32, 2116940u32);
    emu.adi_no_count(11usize, 2usize, 612u32, 2116944u32);
    emu.apc_no_count(1usize, 2116944u32, 53248u32, 2116948u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2116952u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966076u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00204d58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 29u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 532u32, 2116956u32)?;
    emu.lw_no_count(11usize, 2usize, 536u32, 2116960u32)?;
    emu.lw_no_count(12usize, 2usize, 540u32, 2116964u32)?;
    emu.lw_no_count(13usize, 2usize, 544u32, 2116968u32)?;
    emu.sw_no_count(10usize, 8usize, 48u32, 2116972u32)?;
    emu.sw_no_count(11usize, 8usize, 52u32, 2116976u32)?;
    emu.sw_no_count(12usize, 8usize, 56u32, 2116980u32)?;
    emu.sw_no_count(13usize, 8usize, 60u32, 2116984u32)?;
    emu.lw_no_count(10usize, 2usize, 516u32, 2116988u32)?;
    emu.lw_no_count(11usize, 2usize, 520u32, 2116992u32)?;
    emu.lw_no_count(12usize, 2usize, 524u32, 2116996u32)?;
    emu.lw_no_count(13usize, 2usize, 528u32, 2117000u32)?;
    emu.sw_no_count(10usize, 8usize, 32u32, 2117004u32)?;
    emu.sw_no_count(11usize, 8usize, 36u32, 2117008u32)?;
    emu.sw_no_count(12usize, 8usize, 40u32, 2117012u32)?;
    emu.sw_no_count(13usize, 8usize, 44u32, 2117016u32)?;
    emu.lw_no_count(1usize, 2usize, 716u32, 2117020u32)?;
    emu.lw_no_count(8usize, 2usize, 712u32, 2117024u32)?;
    emu.lw_no_count(9usize, 2usize, 708u32, 2117028u32)?;
    emu.lw_no_count(18usize, 2usize, 704u32, 2117032u32)?;
    emu.lw_no_count(19usize, 2usize, 700u32, 2117036u32)?;
    emu.lw_no_count(20usize, 2usize, 696u32, 2117040u32)?;
    emu.lw_no_count(21usize, 2usize, 692u32, 2117044u32)?;
    emu.lw_no_count(22usize, 2usize, 688u32, 2117048u32)?;
    emu.lw_no_count(23usize, 2usize, 684u32, 2117052u32)?;
    emu.lw_no_count(24usize, 2usize, 680u32, 2117056u32)?;
    emu.lw_no_count(25usize, 2usize, 676u32, 2117060u32)?;
    emu.adi_no_count(2usize, 2usize, 720u32, 2117064u32);
    emu.add_memory_rw_events(29usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2117068u32;
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
pub fn block_0x00204dcc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2117072u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2117076u32)?;
    emu.lw_no_count(12usize, 10usize, 0u32, 2117080u32)?;
    emu.lw_no_count(14usize, 12usize, 0u32, 2117084u32)?;
    emu.adi_no_count(13usize, 0usize, 1u32, 2117088u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2117092u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a < b {
        emu.pc = 2117140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204e14));
    } else {
        emu.pc = 2117096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204de8));
    }
}
#[inline(always)]
pub fn block_0x00204de8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 1u32, 2117100u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(13usize);
    if (a as i32) >= (b as i32) {
        emu.pc = 2117148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204e1c));
    } else {
        emu.pc = 2117104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204df0));
    }
}
#[inline(always)]
pub fn block_0x00204df0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 2u32, 2117108u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2117180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204e3c));
    } else {
        emu.pc = 2117112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204df8));
    }
}
#[inline(always)]
pub fn block_0x00204df8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2117116u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967100u32, 2117120u32);
    emu.adi_no_count(12usize, 0usize, 12u32, 2117124u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2117128u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2117132u32);
    emu.apc_no_count(6usize, 2117132u32, 102400u32, 2117136u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2117140u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966356u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204e14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 14usize, 4294967294u32, 2117144u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(13usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2117104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204df0));
    } else {
        emu.pc = 2117148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204e1c));
    }
}
#[inline(always)]
pub fn block_0x00204e1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2117244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204e7c));
    } else {
        emu.pc = 2117152u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204e20));
    }
}
#[inline(always)]
pub fn block_0x00204e20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2117156u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966996u32, 2117160u32);
    emu.adi_no_count(12usize, 0usize, 26u32, 2117164u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2117168u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2117172u32);
    emu.apc_no_count(6usize, 2117172u32, 102400u32, 2117176u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2117180u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966316u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00204e3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 4u32, 2117184u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2117188u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2117192u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967128u32, 2117196u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2117200u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294967138u32, 2117204u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2117208u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 4294967112u32, 2117212u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2117216u32);
    emu.adi_no_count(14usize, 0usize, 3u32, 2117220u32);
    emu.adi_no_count(15usize, 2usize, 8u32, 2117224u32);
    emu.apc_no_count(1usize, 2117224u32, 102400u32, 2117228u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2117232u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966364u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204e70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2117236u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2117240u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2117244u32;
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
pub fn block_0x00204e7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(12usize, 2usize, 4u32, 2117248u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2117252u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967088u32, 2117256u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2117260u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294967024u32, 2117264u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2117268u32);
    emu.adi_no_count(13usize, 2usize, 4u32, 2117272u32);
    emu.apc_no_count(1usize, 2117272u32, 102400u32, 2117276u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2117280u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966968u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204ea0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2117284u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2117288u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2117292u32;
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
pub fn block_0x00204eac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2117296u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2117300u32)?;
    emu.lw_no_count(12usize, 10usize, 0u32, 2117304u32)?;
    emu.lw_no_count(13usize, 12usize, 0u32, 2117308u32)?;
    emu.adi_no_count(14usize, 0usize, 1u32, 2117312u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2117316u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(14usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2117376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204f00));
    } else {
        emu.pc = 2117320u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204ec8));
    }
}
#[inline(always)]
pub fn block_0x00204ec8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2117412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204f24));
    } else {
        emu.pc = 2117324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204ecc));
    }
}
#[inline]
pub fn block_0x00204ecc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 4u32, 2117328u32);
    emu.sw_no_count(12usize, 2usize, 4u32, 2117332u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2117336u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967088u32, 2117340u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2117344u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294967024u32, 2117348u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2117352u32);
    emu.adi_no_count(13usize, 2usize, 4u32, 2117356u32);
    emu.apc_no_count(1usize, 2117356u32, 102400u32, 2117360u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2117364u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966884u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204ef4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2117368u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2117372u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2117376u32;
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
pub fn block_0x00204f00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 2u32, 2117380u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2117440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204f40));
    } else {
        emu.pc = 2117384u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204f08));
    }
}
#[inline(always)]
pub fn block_0x00204f08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2117388u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967141u32, 2117392u32);
    emu.adi_no_count(12usize, 0usize, 19u32, 2117396u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2117400u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2117404u32);
    emu.apc_no_count(6usize, 2117404u32, 102400u32, 2117408u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2117412u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966084u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204f24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2117416u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967100u32, 2117420u32);
    emu.adi_no_count(12usize, 0usize, 12u32, 2117424u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2117428u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2117432u32);
    emu.apc_no_count(6usize, 2117432u32, 102400u32, 2117436u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2117440u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966056u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00204f40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 4u32, 2117444u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2117448u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2117452u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967176u32, 2117456u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2117460u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294967160u32, 2117464u32);
    emu.adi_no_count(12usize, 0usize, 9u32, 2117468u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2117472u32);
    emu.apc_no_count(1usize, 2117472u32, 102400u32, 2117476u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2117480u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966768u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204f68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2117484u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2117488u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2117492u32;
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
#[inline(never)]
pub fn block_0x00204f74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 26u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2117496u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2117500u32)?;
    emu.adi_no_count(5usize, 11usize, 0u32, 2117504u32);
    emu.lw_no_count(15usize, 10usize, 0u32, 2117508u32)?;
    emu.adi_no_count(10usize, 15usize, 4u32, 2117512u32);
    emu.sw_no_count(10usize, 2usize, 24u32, 2117516u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2117520u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294967204u32, 2117524u32);
    emu.adi_no_count(6usize, 2usize, 24u32, 2117528u32);
    emu.adi_no_count(7usize, 0usize, 9u32, 2117532u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2117536u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967220u32, 2117540u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2117544u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294967229u32, 2117548u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2117552u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 4294967188u32, 2117556u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2117560u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 17usize, 4294967240u32, 2117564u32);
    emu.adi_no_count(12usize, 0usize, 9u32, 2117568u32);
    emu.adi_no_count(14usize, 0usize, 11u32, 2117572u32);
    emu.sw_no_count(7usize, 2usize, 0u32, 2117576u32)?;
    emu.sw_no_count(6usize, 2usize, 4u32, 2117580u32)?;
    emu.sw_no_count(10usize, 2usize, 8u32, 2117584u32)?;
    emu.adi_no_count(10usize, 5usize, 0u32, 2117588u32);
    emu.apc_no_count(1usize, 2117588u32, 102400u32, 2117592u32);
    emu.add_memory_rw_events(26usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2117596u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966236u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00204fdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2117600u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2117604u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2117608u32;
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
pub fn block_0x00204fe8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 11usize, 8u32, 2117612u32)?;
    emu.sli_no_count(13usize, 12usize, 6u32, 2117616u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(13usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2117636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205004));
    } else {
        emu.pc = 2117620u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204ff4));
    }
}
#[inline(always)]
pub fn block_0x00204ff4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(12usize, 12usize, 5u32, 2117624u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if (a as i32) < (b as i32) {
        emu.pc = 2117644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020500c));
    } else {
        emu.pc = 2117628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00204ffc));
    }
}
#[inline(always)]
pub fn block_0x00204ffc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2117628u32, 90112u32, 2117632u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2117636u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1068u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205004(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2117636u32, 90112u32, 2117640u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2117644u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(544u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020500c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2117644u32, 90112u32, 2117648u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2117652u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(676u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205014(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2117656u32)?;
    emu.lw_no_count(11usize, 10usize, 0u32, 2117660u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2117664u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2117704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205048));
    } else {
        emu.pc = 2117668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205024));
    }
}
#[inline(always)]
pub fn block_0x00205024(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 10usize, 4u32, 2117672u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2117676u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2117704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205048));
    } else {
        emu.pc = 2117680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205030));
    }
}
#[inline(always)]
pub fn block_0x00205030(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 8u32, 2117684u32)?;
    emu.lw_no_count(11usize, 10usize, 4u32, 2117688u32)?;
    emu.lw_no_count(6usize, 11usize, 0u32, 2117692u32)?;
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2117704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205048));
    } else {
        emu.pc = 2117696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205040));
    }
}
#[inline(always)]
pub fn block_0x00205040(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2117700u32)?;
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2117704u32;
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
pub fn block_0x00205048(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2117708u32;
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
pub fn block_0x0020504c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2117712u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2117716u32)?;
    emu.sw_no_count(8usize, 2usize, 40u32, 2117720u32)?;
    emu.sw_no_count(9usize, 2usize, 36u32, 2117724u32)?;
    emu.sw_no_count(18usize, 2usize, 32u32, 2117728u32)?;
    emu.adi_no_count(9usize, 11usize, 0u32, 2117732u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2117736u32);
    emu.lbu_no_count(10usize, 11usize, 0u32, 2117740u32);
    emu.sb_no_count(10usize, 2usize, 31u32, 2117744u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2117748u32);
    emu.adi_no_count(12usize, 2usize, 31u32, 2117752u32);
    emu.adi_no_count(13usize, 0usize, 1u32, 2117756u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2117760u32);
    emu.apc_no_count(1usize, 2117760u32, 4096u32, 2117764u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2117768u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966584u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205088(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 8u32, 2117772u32);
    emu.adi_no_count(18usize, 0usize, 4u32, 2117776u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2118480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205350));
    } else {
        emu.pc = 2117780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205094));
    }
}
#[inline(always)]
pub fn block_0x00205094(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 9usize, 1u32, 2117784u32);
    emu.sb_no_count(10usize, 2usize, 31u32, 2117788u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2117792u32);
    emu.adi_no_count(12usize, 2usize, 31u32, 2117796u32);
    emu.adi_no_count(13usize, 0usize, 1u32, 2117800u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2117804u32);
    emu.apc_no_count(1usize, 2117804u32, 4096u32, 2117808u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2117812u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966540u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002050b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 8u32, 2117816u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2118480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205350));
    } else {
        emu.pc = 2117820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002050bc));
    }
}
#[inline(always)]
pub fn block_0x002050bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 9usize, 2u32, 2117824u32);
    emu.sb_no_count(10usize, 2usize, 31u32, 2117828u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2117832u32);
    emu.adi_no_count(12usize, 2usize, 31u32, 2117836u32);
    emu.adi_no_count(13usize, 0usize, 1u32, 2117840u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2117844u32);
    emu.apc_no_count(1usize, 2117844u32, 4096u32, 2117848u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2117852u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966500u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002050dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 8u32, 2117856u32);
    emu.adi_no_count(18usize, 0usize, 4u32, 2117860u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2118480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205350));
    } else {
        emu.pc = 2117864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002050e8));
    }
}
#[inline(always)]
pub fn block_0x002050e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 9usize, 3u32, 2117868u32);
    emu.sb_no_count(10usize, 2usize, 31u32, 2117872u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2117876u32);
    emu.adi_no_count(12usize, 2usize, 31u32, 2117880u32);
    emu.adi_no_count(13usize, 0usize, 1u32, 2117884u32);
    emu.adi_no_count(11usize, 8usize, 0u32, 2117888u32);
    emu.apc_no_count(1usize, 2117888u32, 4096u32, 2117892u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2117896u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966456u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205108(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 8u32, 2117900u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2118480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205350));
    } else {
        emu.pc = 2117904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205110));
    }
}
#[inline(always)]
pub fn block_0x00205110(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 4u32, 2117908u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2117912u32);
    emu.apc_no_count(1usize, 2117912u32, 4294959104u32, 2117916u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2117920u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(356u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205120(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2118508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020536c));
    } else {
        emu.pc = 2117924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205124));
    }
}
#[inline(always)]
pub fn block_0x00205124(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 5u32, 2117928u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2117932u32);
    emu.apc_no_count(1usize, 2117932u32, 4294959104u32, 2117936u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2117940u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(336u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205134(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2118508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020536c));
    } else {
        emu.pc = 2117944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205138));
    }
}
#[inline(always)]
pub fn block_0x00205138(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 6u32, 2117948u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2117952u32);
    emu.apc_no_count(1usize, 2117952u32, 4294959104u32, 2117956u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2117960u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(316u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205148(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2118508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020536c));
    } else {
        emu.pc = 2117964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020514c));
    }
}
#[inline(always)]
pub fn block_0x0020514c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 7u32, 2117968u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2117972u32);
    emu.apc_no_count(1usize, 2117972u32, 4294959104u32, 2117976u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2117980u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(296u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020515c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2118508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020536c));
    } else {
        emu.pc = 2117984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205160));
    }
}
#[inline(always)]
pub fn block_0x00205160(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 8u32, 2117988u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2117992u32);
    emu.apc_no_count(1usize, 2117992u32, 4294959104u32, 2117996u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118000u32;
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
pub fn block_0x00205170(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2118508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020536c));
    } else {
        emu.pc = 2118004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205174));
    }
}
#[inline(always)]
pub fn block_0x00205174(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 9u32, 2118008u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2118012u32);
    emu.apc_no_count(1usize, 2118012u32, 4294959104u32, 2118016u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118020u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(256u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205184(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2118508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020536c));
    } else {
        emu.pc = 2118024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205188));
    }
}
#[inline(always)]
pub fn block_0x00205188(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 10u32, 2118028u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2118032u32);
    emu.apc_no_count(1usize, 2118032u32, 4294959104u32, 2118036u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118040u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(236u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205198(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2118508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020536c));
    } else {
        emu.pc = 2118044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020519c));
    }
}
#[inline(always)]
pub fn block_0x0020519c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 11u32, 2118048u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2118052u32);
    emu.apc_no_count(1usize, 2118052u32, 4294959104u32, 2118056u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118060u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(216u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002051ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2118508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020536c));
    } else {
        emu.pc = 2118064u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002051b0));
    }
}
#[inline(always)]
pub fn block_0x002051b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 12u32, 2118068u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2118072u32);
    emu.apc_no_count(1usize, 2118072u32, 4294959104u32, 2118076u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118080u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(196u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002051c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2118508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020536c));
    } else {
        emu.pc = 2118084u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002051c4));
    }
}
#[inline(always)]
pub fn block_0x002051c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 13u32, 2118088u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2118092u32);
    emu.apc_no_count(1usize, 2118092u32, 4294959104u32, 2118096u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118100u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(176u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002051d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2118508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020536c));
    } else {
        emu.pc = 2118104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002051d8));
    }
}
#[inline(always)]
pub fn block_0x002051d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 14u32, 2118108u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2118112u32);
    emu.apc_no_count(1usize, 2118112u32, 4294959104u32, 2118116u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118120u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(156u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002051e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2118508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020536c));
    } else {
        emu.pc = 2118124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002051ec));
    }
}
#[inline(always)]
pub fn block_0x002051ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 15u32, 2118128u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2118132u32);
    emu.apc_no_count(1usize, 2118132u32, 4294959104u32, 2118136u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118140u32;
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
pub fn block_0x002051fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2118508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020536c));
    } else {
        emu.pc = 2118144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205200));
    }
}
#[inline(always)]
pub fn block_0x00205200(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 16u32, 2118148u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2118152u32);
    emu.apc_no_count(1usize, 2118152u32, 4294959104u32, 2118156u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118160u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(116u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205210(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2118508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020536c));
    } else {
        emu.pc = 2118164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205214));
    }
}
#[inline(always)]
pub fn block_0x00205214(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 17u32, 2118168u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2118172u32);
    emu.apc_no_count(1usize, 2118172u32, 4294959104u32, 2118176u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118180u32;
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
pub fn block_0x00205224(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2118508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020536c));
    } else {
        emu.pc = 2118184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205228));
    }
}
#[inline(always)]
pub fn block_0x00205228(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 18u32, 2118188u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2118192u32);
    emu.apc_no_count(1usize, 2118192u32, 4294959104u32, 2118196u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118200u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(76u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205238(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2118508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020536c));
    } else {
        emu.pc = 2118204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020523c));
    }
}
#[inline(always)]
pub fn block_0x0020523c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 19u32, 2118208u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2118212u32);
    emu.apc_no_count(1usize, 2118212u32, 4294959104u32, 2118216u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118220u32;
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
pub fn block_0x0020524c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2118508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020536c));
    } else {
        emu.pc = 2118224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205250));
    }
}
#[inline(always)]
pub fn block_0x00205250(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 20u32, 2118228u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2118232u32);
    emu.apc_no_count(1usize, 2118232u32, 4294959104u32, 2118236u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118240u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(36u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205260(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2118508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020536c));
    } else {
        emu.pc = 2118244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205264));
    }
}
#[inline(always)]
pub fn block_0x00205264(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 21u32, 2118248u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2118252u32);
    emu.apc_no_count(1usize, 2118252u32, 4294959104u32, 2118256u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118260u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(16u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205274(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2118508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020536c));
    } else {
        emu.pc = 2118264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205278));
    }
}
#[inline(always)]
pub fn block_0x00205278(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 22u32, 2118268u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2118272u32);
    emu.apc_no_count(1usize, 2118272u32, 4294959104u32, 2118276u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118280u32;
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
pub fn block_0x00205288(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2118508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020536c));
    } else {
        emu.pc = 2118284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020528c));
    }
}
#[inline(always)]
pub fn block_0x0020528c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 23u32, 2118288u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2118292u32);
    emu.apc_no_count(1usize, 2118292u32, 4294959104u32, 2118296u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118300u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967272u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020529c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2118508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020536c));
    } else {
        emu.pc = 2118304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002052a0));
    }
}
#[inline(always)]
pub fn block_0x002052a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 24u32, 2118308u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2118312u32);
    emu.apc_no_count(1usize, 2118312u32, 4294959104u32, 2118316u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118320u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967252u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002052b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2118508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020536c));
    } else {
        emu.pc = 2118324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002052b4));
    }
}
#[inline(always)]
pub fn block_0x002052b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 25u32, 2118328u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2118332u32);
    emu.apc_no_count(1usize, 2118332u32, 4294959104u32, 2118336u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118340u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967232u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002052c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2118508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020536c));
    } else {
        emu.pc = 2118344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002052c8));
    }
}
#[inline(always)]
pub fn block_0x002052c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 26u32, 2118348u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2118352u32);
    emu.apc_no_count(1usize, 2118352u32, 4294959104u32, 2118356u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118360u32;
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
#[inline(always)]
pub fn block_0x002052d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2118508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020536c));
    } else {
        emu.pc = 2118364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002052dc));
    }
}
#[inline(always)]
pub fn block_0x002052dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 27u32, 2118368u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2118372u32);
    emu.apc_no_count(1usize, 2118372u32, 4294959104u32, 2118376u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118380u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967192u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002052ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2118508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020536c));
    } else {
        emu.pc = 2118384u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002052f0));
    }
}
#[inline(always)]
pub fn block_0x002052f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 28u32, 2118388u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2118392u32);
    emu.apc_no_count(1usize, 2118392u32, 4294959104u32, 2118396u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118400u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967172u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205300(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2118508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020536c));
    } else {
        emu.pc = 2118404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205304));
    }
}
#[inline(always)]
pub fn block_0x00205304(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 29u32, 2118408u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2118412u32);
    emu.apc_no_count(1usize, 2118412u32, 4294959104u32, 2118416u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118420u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967152u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205314(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2118508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020536c));
    } else {
        emu.pc = 2118424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205318));
    }
}
#[inline(always)]
pub fn block_0x00205318(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 30u32, 2118428u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2118432u32);
    emu.apc_no_count(1usize, 2118432u32, 4294959104u32, 2118436u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118440u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967132u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205328(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2118508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020536c));
    } else {
        emu.pc = 2118444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020532c));
    }
}
#[inline]
pub fn block_0x0020532c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(11usize, 9usize, 31u32, 2118448u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2118452u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2118456u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2118460u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2118464u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2118468u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2118472u32);
    emu.apc_no_count(6usize, 2118472u32, 4294959104u32, 2118476u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2118480u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294967092u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205350(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2118484u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2118488u32)?;
    emu.sw_no_count(10usize, 2usize, 16u32, 2118492u32)?;
    emu.sw_no_count(11usize, 2usize, 20u32, 2118496u32)?;
    emu.adi_no_count(10usize, 2usize, 16u32, 2118500u32);
    emu.apc_no_count(1usize, 2118500u32, 20480u32, 2118504u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118508u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966440u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020536c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2118512u32)?;
    emu.lw_no_count(8usize, 2usize, 40u32, 2118516u32)?;
    emu.lw_no_count(9usize, 2usize, 36u32, 2118520u32)?;
    emu.lw_no_count(18usize, 2usize, 32u32, 2118524u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2118528u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2118532u32;
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
