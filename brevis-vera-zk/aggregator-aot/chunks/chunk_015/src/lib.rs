pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2166088u32;
pub const PC_MAX: u32 = 2171204u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 106usize] = [
        block_0x00210d48,
        block_0x00210d5c,
        block_0x00210d68,
        block_0x00210e50,
        block_0x00210eec,
        block_0x00210f00,
        block_0x00210f0c,
        block_0x00210f2c,
        block_0x00210f78,
        block_0x00210f88,
        block_0x00210fa0,
        block_0x00210fb0,
        block_0x00210fbc,
        block_0x0021106c,
        block_0x002110dc,
        block_0x002110f8,
        block_0x00211114,
        block_0x00211118,
        block_0x00211144,
        block_0x00211170,
        block_0x0021119c,
        block_0x002111c8,
        block_0x002111f4,
        block_0x0021121c,
        block_0x00211238,
        block_0x0021123c,
        block_0x00211264,
        block_0x0021126c,
        block_0x00211294,
        block_0x0021129c,
        block_0x002112c4,
        block_0x002112cc,
        block_0x002112f4,
        block_0x002112fc,
        block_0x00211320,
        block_0x00211328,
        block_0x0021134c,
        block_0x00211414,
        block_0x002116f4,
        block_0x00211924,
        block_0x00211ab0,
        block_0x00211ac4,
        block_0x00211af8,
        block_0x00211b00,
        block_0x00211b08,
        block_0x00211b5c,
        block_0x00211b60,
        block_0x00211b78,
        block_0x00211b7c,
        block_0x00211b98,
        block_0x00211bb0,
        block_0x00211bb4,
        block_0x00211bb8,
        block_0x00211bc4,
        block_0x00211be8,
        block_0x00211c0c,
        block_0x00211c30,
        block_0x00211c54,
        block_0x00211c78,
        block_0x00211c9c,
        block_0x00211cc0,
        block_0x00211ce4,
        block_0x00211d08,
        block_0x00211d2c,
        block_0x00211d50,
        block_0x00211d74,
        block_0x00211d98,
        block_0x00211dbc,
        block_0x00211dec,
        block_0x00211e10,
        block_0x00211e40,
        block_0x00211e64,
        block_0x00211e88,
        block_0x00211eac,
        block_0x00211ed0,
        block_0x00211ef4,
        block_0x00211f20,
        block_0x00211f70,
        block_0x00211f7c,
        block_0x00211fa0,
        block_0x00211fec,
        block_0x00211ff4,
        block_0x00211ffc,
        block_0x00212004,
        block_0x0021200c,
        block_0x00212014,
        block_0x0021201c,
        block_0x00212024,
        block_0x0021202c,
        block_0x00212034,
        block_0x0021203c,
        block_0x00212044,
        block_0x0021204c,
        block_0x00212054,
        block_0x0021205c,
        block_0x00212064,
        block_0x0021206c,
        block_0x00212074,
        block_0x0021207c,
        block_0x00212084,
        block_0x00212088,
        block_0x002120a0,
        block_0x002120a4,
        block_0x00212118,
        block_0x00212124,
        block_0x00212144,
    ];
    const IDX: [u16; 1280usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 0u16, 3u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 4u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 5u16, 0u16, 0u16, 0u16, 0u16, 6u16, 0u16, 0u16, 7u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 8u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 9u16, 0u16, 0u16,
        0u16, 10u16, 0u16, 0u16, 0u16, 0u16, 0u16, 11u16, 0u16, 0u16, 0u16, 12u16, 0u16,
        0u16, 13u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 14u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 15u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 16u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 17u16, 18u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 19u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 20u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 21u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 22u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        23u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 24u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 25u16, 26u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 27u16, 0u16, 28u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 29u16, 0u16, 30u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        31u16, 0u16, 32u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 33u16,
        0u16, 34u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 35u16, 0u16, 36u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 37u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 38u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 39u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 40u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        41u16, 0u16, 0u16, 0u16, 0u16, 42u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 43u16, 0u16, 44u16, 0u16, 45u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 46u16, 47u16, 0u16, 0u16, 0u16, 0u16, 0u16, 48u16, 49u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 50u16, 0u16, 0u16, 0u16, 0u16, 0u16, 51u16,
        52u16, 53u16, 0u16, 0u16, 54u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        55u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 56u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 57u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 58u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 59u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 60u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 61u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 62u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 63u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 64u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 65u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 66u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 67u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        68u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 69u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 70u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 71u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 72u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 73u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 74u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 75u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        76u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 77u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 78u16, 0u16, 0u16, 79u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 80u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 81u16, 0u16, 82u16,
        0u16, 83u16, 0u16, 84u16, 0u16, 85u16, 0u16, 86u16, 0u16, 87u16, 0u16, 88u16,
        0u16, 89u16, 0u16, 90u16, 0u16, 91u16, 0u16, 92u16, 0u16, 93u16, 0u16, 94u16,
        0u16, 95u16, 0u16, 96u16, 0u16, 97u16, 0u16, 98u16, 0u16, 99u16, 0u16, 100u16,
        101u16, 0u16, 0u16, 0u16, 0u16, 0u16, 102u16, 103u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 104u16, 0u16,
        0u16, 105u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 106u16,
    ];
    if pc < 2166088u32 || pc > 2171204u32 {
        return None;
    }
    let word_offset = ((pc - 2166088u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x00210d48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2166092u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2166096u32)?;
    emu.adi_no_count(12usize, 11usize, 0u32, 2166100u32);
    emu.apc_no_count(1usize, 2166100u32, 4294938624u32, 2166104u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2166108u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965680u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00210d5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2166112u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2166116u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2166120u32;
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
pub fn block_0x00210d68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 58u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2166124u32);
    emu.sw_no_count(8usize, 2usize, 12u32, 2166128u32)?;
    emu.sw_no_count(9usize, 2usize, 8u32, 2166132u32)?;
    emu.sw_no_count(18usize, 2usize, 4u32, 2166136u32)?;
    emu.sbr_no_count(13usize, 0usize, 13usize, 2166140u32);
    emu.lw_no_count(14usize, 11usize, 0u32, 2166144u32)?;
    emu.lw_no_count(15usize, 11usize, 4u32, 2166148u32)?;
    emu.lw_no_count(16usize, 11usize, 8u32, 2166152u32)?;
    emu.lw_no_count(17usize, 11usize, 12u32, 2166156u32)?;
    emu.lw_no_count(5usize, 12usize, 0u32, 2166160u32)?;
    emu.lw_no_count(6usize, 12usize, 4u32, 2166164u32)?;
    emu.lw_no_count(7usize, 12usize, 8u32, 2166168u32)?;
    emu.lw_no_count(28usize, 12usize, 12u32, 2166172u32)?;
    emu.lw_no_count(29usize, 11usize, 16u32, 2166176u32)?;
    emu.lw_no_count(30usize, 11usize, 20u32, 2166180u32)?;
    emu.lw_no_count(31usize, 11usize, 24u32, 2166184u32)?;
    emu.lw_no_count(11usize, 11usize, 28u32, 2166188u32)?;
    emu.lw_no_count(8usize, 12usize, 16u32, 2166192u32)?;
    emu.lw_no_count(9usize, 12usize, 20u32, 2166196u32)?;
    emu.lw_no_count(18usize, 12usize, 24u32, 2166200u32)?;
    emu.lw_no_count(12usize, 12usize, 28u32, 2166204u32)?;
    emu.xrr_no_count(5usize, 5usize, 14usize, 2166208u32);
    emu.xrr_no_count(6usize, 6usize, 15usize, 2166212u32);
    emu.xrr_no_count(7usize, 7usize, 16usize, 2166216u32);
    emu.xrr_no_count(28usize, 28usize, 17usize, 2166220u32);
    emu.xrr_no_count(8usize, 8usize, 29usize, 2166224u32);
    emu.xrr_no_count(9usize, 9usize, 30usize, 2166228u32);
    emu.xrr_no_count(18usize, 18usize, 31usize, 2166232u32);
    emu.xrr_no_count(12usize, 12usize, 11usize, 2166236u32);
    emu.anr_no_count(5usize, 5usize, 13usize, 2166240u32);
    emu.anr_no_count(6usize, 6usize, 13usize, 2166244u32);
    emu.anr_no_count(7usize, 7usize, 13usize, 2166248u32);
    emu.anr_no_count(28usize, 28usize, 13usize, 2166252u32);
    emu.anr_no_count(8usize, 8usize, 13usize, 2166256u32);
    emu.anr_no_count(9usize, 9usize, 13usize, 2166260u32);
    emu.anr_no_count(18usize, 18usize, 13usize, 2166264u32);
    emu.anr_no_count(12usize, 12usize, 13usize, 2166268u32);
    emu.xrr_no_count(13usize, 5usize, 14usize, 2166272u32);
    emu.xrr_no_count(14usize, 6usize, 15usize, 2166276u32);
    emu.xrr_no_count(15usize, 7usize, 16usize, 2166280u32);
    emu.xrr_no_count(16usize, 28usize, 17usize, 2166284u32);
    emu.xrr_no_count(17usize, 8usize, 29usize, 2166288u32);
    emu.xrr_no_count(5usize, 9usize, 30usize, 2166292u32);
    emu.xrr_no_count(6usize, 18usize, 31usize, 2166296u32);
    emu.xrr_no_count(11usize, 12usize, 11usize, 2166300u32);
    emu.sw_no_count(13usize, 10usize, 0u32, 2166304u32)?;
    emu.sw_no_count(14usize, 10usize, 4u32, 2166308u32)?;
    emu.sw_no_count(15usize, 10usize, 8u32, 2166312u32)?;
    emu.sw_no_count(16usize, 10usize, 12u32, 2166316u32)?;
    emu.sw_no_count(17usize, 10usize, 16u32, 2166320u32)?;
    emu.sw_no_count(5usize, 10usize, 20u32, 2166324u32)?;
    emu.sw_no_count(6usize, 10usize, 24u32, 2166328u32)?;
    emu.sw_no_count(11usize, 10usize, 28u32, 2166332u32)?;
    emu.lw_no_count(8usize, 2usize, 12u32, 2166336u32)?;
    emu.lw_no_count(9usize, 2usize, 8u32, 2166340u32)?;
    emu.lw_no_count(18usize, 2usize, 4u32, 2166344u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2166348u32);
    emu.add_memory_rw_events(58usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2166352u32;
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
pub fn block_0x00210e50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 39u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2166356u32);
    emu.sw_no_count(8usize, 2usize, 12u32, 2166360u32)?;
    emu.lw_no_count(12usize, 10usize, 0u32, 2166364u32)?;
    emu.lw_no_count(13usize, 10usize, 4u32, 2166368u32)?;
    emu.lw_no_count(14usize, 10usize, 8u32, 2166372u32)?;
    emu.lw_no_count(15usize, 10usize, 12u32, 2166376u32)?;
    emu.lw_no_count(16usize, 11usize, 0u32, 2166380u32)?;
    emu.lw_no_count(17usize, 11usize, 4u32, 2166384u32)?;
    emu.lw_no_count(5usize, 11usize, 8u32, 2166388u32)?;
    emu.lw_no_count(6usize, 11usize, 12u32, 2166392u32)?;
    emu.lw_no_count(7usize, 10usize, 16u32, 2166396u32)?;
    emu.lw_no_count(28usize, 10usize, 20u32, 2166400u32)?;
    emu.lw_no_count(29usize, 10usize, 24u32, 2166404u32)?;
    emu.lw_no_count(10usize, 10usize, 28u32, 2166408u32)?;
    emu.lw_no_count(30usize, 11usize, 16u32, 2166412u32)?;
    emu.lw_no_count(31usize, 11usize, 20u32, 2166416u32)?;
    emu.lw_no_count(8usize, 11usize, 24u32, 2166420u32)?;
    emu.lw_no_count(11usize, 11usize, 28u32, 2166424u32)?;
    emu.xrr_no_count(13usize, 17usize, 13usize, 2166428u32);
    emu.xrr_no_count(12usize, 16usize, 12usize, 2166432u32);
    emu.xrr_no_count(14usize, 5usize, 14usize, 2166436u32);
    emu.xrr_no_count(15usize, 6usize, 15usize, 2166440u32);
    emu.xrr_no_count(16usize, 30usize, 7usize, 2166444u32);
    emu.xrr_no_count(17usize, 31usize, 28usize, 2166448u32);
    emu.xrr_no_count(5usize, 8usize, 29usize, 2166452u32);
    emu.orr_no_count(12usize, 12usize, 13usize, 2166456u32);
    emu.orr_no_count(14usize, 14usize, 15usize, 2166460u32);
    emu.orr_no_count(13usize, 16usize, 17usize, 2166464u32);
    emu.orr_no_count(12usize, 12usize, 14usize, 2166468u32);
    emu.orr_no_count(13usize, 13usize, 5usize, 2166472u32);
    emu.orr_no_count(12usize, 12usize, 13usize, 2166476u32);
    emu.xrr_no_count(10usize, 11usize, 10usize, 2166480u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2166484u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2166488u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2166492u32);
    emu.lw_no_count(8usize, 2usize, 12u32, 2166496u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2166500u32);
    emu.apc_no_count(6usize, 2166500u32, 4096u32, 2166504u32);
    emu.add_memory_rw_events(39usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2166508u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(720u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00210eec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2166512u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2166516u32)?;
    emu.adi_no_count(12usize, 11usize, 0u32, 2166520u32);
    emu.apc_no_count(1usize, 2166520u32, 4294938624u32, 2166524u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2166528u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966552u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00210f00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2166532u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2166536u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2166540u32;
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
pub fn block_0x00210f0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2166544u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2166548u32)?;
    emu.sw_no_count(8usize, 2usize, 8u32, 2166552u32)?;
    emu.sw_no_count(9usize, 2usize, 4u32, 2166556u32)?;
    emu.adi_no_count(8usize, 11usize, 0u32, 2166560u32);
    emu.adi_no_count(9usize, 10usize, 0u32, 2166564u32);
    emu.apc_no_count(1usize, 2166564u32, 4294950912u32, 2166568u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2166572u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(260u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00210f2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 8usize, 0u32, 2166576u32)?;
    emu.lw_no_count(11usize, 8usize, 4u32, 2166580u32)?;
    emu.lw_no_count(12usize, 8usize, 8u32, 2166584u32)?;
    emu.lw_no_count(13usize, 8usize, 12u32, 2166588u32)?;
    emu.lw_no_count(14usize, 8usize, 16u32, 2166592u32)?;
    emu.lw_no_count(15usize, 8usize, 20u32, 2166596u32)?;
    emu.lw_no_count(16usize, 8usize, 24u32, 2166600u32)?;
    emu.lw_no_count(17usize, 8usize, 28u32, 2166604u32)?;
    emu.orr_no_count(10usize, 11usize, 10usize, 2166608u32);
    emu.orr_no_count(12usize, 12usize, 13usize, 2166612u32);
    emu.orr_no_count(14usize, 14usize, 15usize, 2166616u32);
    emu.orr_no_count(10usize, 10usize, 12usize, 2166620u32);
    emu.orr_no_count(11usize, 14usize, 16usize, 2166624u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2166628u32);
    emu.orr_no_count(10usize, 10usize, 17usize, 2166632u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2166636u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2166640u32);
    emu.apc_no_count(1usize, 2166640u32, 4096u32, 2166644u32);
    emu.add_memory_rw_events(19usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2166648u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(580u32);
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
    emu.xri_no_count(10usize, 10usize, 4294967295u32, 2166652u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2166656u32);
    emu.apc_no_count(1usize, 2166656u32, 4096u32, 2166660u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2166664u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x00210f88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(10usize, 9usize, 32u32, 2166668u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2166672u32)?;
    emu.lw_no_count(8usize, 2usize, 8u32, 2166676u32)?;
    emu.lw_no_count(9usize, 2usize, 4u32, 2166680u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2166684u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
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
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2166692u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2166696u32)?;
    emu.apc_no_count(1usize, 2166696u32, 4294959104u32, 2166700u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2166704u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965532u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00210fb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2166708u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2166712u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2166716u32;
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
pub fn block_0x00210fbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 44u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967168u32, 2166720u32);
    emu.sw_no_count(1usize, 2usize, 124u32, 2166724u32)?;
    emu.sw_no_count(8usize, 2usize, 120u32, 2166728u32)?;
    emu.sw_no_count(9usize, 2usize, 116u32, 2166732u32)?;
    emu.sw_no_count(18usize, 2usize, 112u32, 2166736u32)?;
    emu.sw_no_count(19usize, 2usize, 108u32, 2166740u32)?;
    emu.sw_no_count(20usize, 2usize, 104u32, 2166744u32)?;
    emu.sw_no_count(21usize, 2usize, 100u32, 2166748u32)?;
    emu.sw_no_count(22usize, 2usize, 96u32, 2166752u32)?;
    emu.sw_no_count(23usize, 2usize, 92u32, 2166756u32)?;
    emu.sw_no_count(24usize, 2usize, 88u32, 2166760u32)?;
    emu.sw_no_count(25usize, 2usize, 84u32, 2166764u32)?;
    emu.sw_no_count(26usize, 2usize, 80u32, 2166768u32)?;
    emu.sw_no_count(27usize, 2usize, 76u32, 2166772u32)?;
    emu.sw_no_count(10usize, 2usize, 4u32, 2166776u32)?;
    emu.adi_no_count(15usize, 0usize, 0u32, 2166780u32);
    emu.adi_no_count(16usize, 2usize, 16u32, 2166784u32);
    emu.sw_no_count(0usize, 2usize, 28u32, 2166788u32)?;
    emu.sw_no_count(0usize, 2usize, 32u32, 2166792u32)?;
    emu.sw_no_count(0usize, 2usize, 36u32, 2166796u32)?;
    emu.sw_no_count(0usize, 2usize, 40u32, 2166800u32)?;
    emu.sw_no_count(0usize, 2usize, 12u32, 2166804u32)?;
    emu.sw_no_count(0usize, 2usize, 16u32, 2166808u32)?;
    emu.sw_no_count(0usize, 2usize, 20u32, 2166812u32)?;
    emu.sw_no_count(0usize, 2usize, 24u32, 2166816u32)?;
    emu.adi_no_count(17usize, 2usize, 24u32, 2166820u32);
    emu.sw_no_count(0usize, 2usize, 44u32, 2166824u32)?;
    emu.sw_no_count(0usize, 2usize, 48u32, 2166828u32)?;
    emu.sw_no_count(0usize, 2usize, 52u32, 2166832u32)?;
    emu.sw_no_count(0usize, 2usize, 56u32, 2166836u32)?;
    emu.sw_no_count(0usize, 2usize, 60u32, 2166840u32)?;
    emu.sw_no_count(0usize, 2usize, 64u32, 2166844u32)?;
    emu.sw_no_count(0usize, 2usize, 68u32, 2166848u32)?;
    emu.sw_no_count(0usize, 2usize, 72u32, 2166852u32)?;
    emu.adi_no_count(5usize, 2usize, 12u32, 2166856u32);
    emu.lw_no_count(10usize, 12usize, 0u32, 2166860u32)?;
    emu.sw_no_count(10usize, 2usize, 8u32, 2166864u32)?;
    emu.lw_no_count(7usize, 12usize, 4u32, 2166868u32)?;
    emu.lw_no_count(28usize, 12usize, 8u32, 2166872u32)?;
    emu.lw_no_count(29usize, 12usize, 12u32, 2166876u32)?;
    emu.lw_no_count(30usize, 12usize, 16u32, 2166880u32)?;
    emu.lw_no_count(31usize, 12usize, 20u32, 2166884u32)?;
    emu.lw_no_count(8usize, 12usize, 24u32, 2166888u32)?;
    emu.lw_no_count(12usize, 12usize, 28u32, 2166892u32)?;
    emu.add_memory_rw_events(44usize);
    emu.pc = 2166892u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021106c));
}
#[inline(never)]
pub fn block_0x0021106c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 11usize, 0u32, 2166896u32)?;
    emu.lw_no_count(10usize, 17usize, 4294967284u32, 2166900u32)?;
    emu.lw_no_count(13usize, 2usize, 8u32, 2166904u32)?;
    emu.mulhu_no_count(14usize, 18usize, 13usize, 2166908u32);
    emu.mul_no_count(13usize, 18usize, 13usize, 2166912u32);
    emu.mulhu_no_count(19usize, 18usize, 7usize, 2166916u32);
    emu.mul_no_count(9usize, 18usize, 7usize, 2166920u32);
    emu.mulhu_no_count(24usize, 18usize, 12usize, 2166924u32);
    emu.mul_no_count(25usize, 18usize, 12usize, 2166928u32);
    emu.mulhu_no_count(26usize, 18usize, 8usize, 2166932u32);
    emu.mul_no_count(27usize, 18usize, 8usize, 2166936u32);
    emu.mulhu_no_count(1usize, 18usize, 31usize, 2166940u32);
    emu.mul_no_count(23usize, 18usize, 31usize, 2166944u32);
    emu.mulhu_no_count(22usize, 18usize, 30usize, 2166948u32);
    emu.mul_no_count(21usize, 18usize, 30usize, 2166952u32);
    emu.adr_no_count(10usize, 13usize, 10usize, 2166956u32);
    emu.sltru_no_count(20usize, 10usize, 13usize, 2166960u32);
    emu.sw_no_count(10usize, 17usize, 4294967284u32, 2166964u32)?;
    emu.mulhu_no_count(13usize, 18usize, 29usize, 2166968u32);
    emu.adr_no_count(20usize, 14usize, 20usize, 2166972u32);
    emu.mul_no_count(14usize, 18usize, 29usize, 2166976u32);
    emu.adr_no_count(9usize, 20usize, 9usize, 2166980u32);
    emu.sltru_no_count(10usize, 9usize, 20usize, 2166984u32);
    emu.adr_no_count(10usize, 19usize, 10usize, 2166988u32);
    emu.mulhu_no_count(20usize, 18usize, 28usize, 2166992u32);
    emu.mul_no_count(19usize, 18usize, 28usize, 2166996u32);
    emu.adi_no_count(6usize, 0usize, 7u32, 2167000u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a < b {
        emu.pc = 2167032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002110f8));
    } else {
        emu.pc = 2167004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002110dc));
    }
}
#[inline(always)]
pub fn block_0x002110dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(6usize, 16usize, 0u32, 2167008u32)?;
    emu.adi_no_count(18usize, 15usize, 2u32, 2167012u32);
    emu.adr_no_count(6usize, 9usize, 6usize, 2167016u32);
    emu.sltru_no_count(9usize, 6usize, 9usize, 2167020u32);
    emu.adr_no_count(9usize, 10usize, 9usize, 2167024u32);
    emu.sw_no_count(6usize, 16usize, 0u32, 2167028u32)?;
    emu.add_memory_rw_events(7usize);
    let return_addr = 2167032u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2167064u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211118));
}
#[inline(always)]
pub fn block_0x002110f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(6usize, 17usize, 4294967288u32, 2167036u32)?;
    emu.adr_no_count(6usize, 9usize, 6usize, 2167040u32);
    emu.sltru_no_count(9usize, 6usize, 9usize, 2167044u32);
    emu.adr_no_count(9usize, 10usize, 9usize, 2167048u32);
    emu.sw_no_count(6usize, 17usize, 4294967288u32, 2167052u32)?;
    emu.adi_no_count(10usize, 0usize, 6u32, 2167056u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a != b {
        emu.pc = 2167356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021123c));
    } else {
        emu.pc = 2167060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211114));
    }
}
#[inline(always)]
pub fn block_0x00211114(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 8u32, 2167064u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2167064u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211118));
}
#[inline]
pub fn block_0x00211118(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(18usize, 18usize, 2u32, 2167068u32);
    emu.adr_no_count(18usize, 5usize, 18usize, 2167072u32);
    emu.lw_no_count(10usize, 18usize, 0u32, 2167076u32)?;
    emu.adr_no_count(9usize, 19usize, 9usize, 2167080u32);
    emu.sltru_no_count(6usize, 9usize, 19usize, 2167084u32);
    emu.adr_no_count(6usize, 20usize, 6usize, 2167088u32);
    emu.adr_no_count(10usize, 9usize, 10usize, 2167092u32);
    emu.sltru_no_count(9usize, 10usize, 9usize, 2167096u32);
    emu.sw_no_count(10usize, 18usize, 0u32, 2167100u32)?;
    emu.adr_no_count(9usize, 6usize, 9usize, 2167104u32);
    emu.adi_no_count(10usize, 15usize, 3u32, 2167108u32);
    emu.add_memory_rw_events(11usize);
    emu.pc = 2167108u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211144));
}
#[inline]
pub fn block_0x00211144(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 10usize, 2u32, 2167112u32);
    emu.adr_no_count(10usize, 5usize, 10usize, 2167116u32);
    emu.lw_no_count(6usize, 10usize, 0u32, 2167120u32)?;
    emu.adr_no_count(9usize, 14usize, 9usize, 2167124u32);
    emu.sltru_no_count(14usize, 9usize, 14usize, 2167128u32);
    emu.adr_no_count(13usize, 13usize, 14usize, 2167132u32);
    emu.adr_no_count(6usize, 9usize, 6usize, 2167136u32);
    emu.sltru_no_count(14usize, 6usize, 9usize, 2167140u32);
    emu.sw_no_count(6usize, 10usize, 0u32, 2167144u32)?;
    emu.adr_no_count(13usize, 13usize, 14usize, 2167148u32);
    emu.adi_no_count(10usize, 15usize, 4u32, 2167152u32);
    emu.add_memory_rw_events(11usize);
    emu.pc = 2167152u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211170));
}
#[inline]
pub fn block_0x00211170(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 10usize, 2u32, 2167156u32);
    emu.adr_no_count(10usize, 5usize, 10usize, 2167160u32);
    emu.lw_no_count(14usize, 10usize, 0u32, 2167164u32)?;
    emu.adr_no_count(13usize, 21usize, 13usize, 2167168u32);
    emu.sltru_no_count(6usize, 13usize, 21usize, 2167172u32);
    emu.adr_no_count(6usize, 22usize, 6usize, 2167176u32);
    emu.adr_no_count(14usize, 13usize, 14usize, 2167180u32);
    emu.sltru_no_count(13usize, 14usize, 13usize, 2167184u32);
    emu.sw_no_count(14usize, 10usize, 0u32, 2167188u32)?;
    emu.adr_no_count(13usize, 6usize, 13usize, 2167192u32);
    emu.adi_no_count(10usize, 15usize, 5u32, 2167196u32);
    emu.add_memory_rw_events(11usize);
    emu.pc = 2167196u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021119c));
}
#[inline]
pub fn block_0x0021119c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 10usize, 2u32, 2167200u32);
    emu.adr_no_count(10usize, 5usize, 10usize, 2167204u32);
    emu.lw_no_count(14usize, 10usize, 0u32, 2167208u32)?;
    emu.adr_no_count(13usize, 23usize, 13usize, 2167212u32);
    emu.sltru_no_count(6usize, 13usize, 23usize, 2167216u32);
    emu.adr_no_count(6usize, 1usize, 6usize, 2167220u32);
    emu.adr_no_count(14usize, 13usize, 14usize, 2167224u32);
    emu.sltru_no_count(13usize, 14usize, 13usize, 2167228u32);
    emu.sw_no_count(14usize, 10usize, 0u32, 2167232u32)?;
    emu.adr_no_count(13usize, 6usize, 13usize, 2167236u32);
    emu.adi_no_count(10usize, 15usize, 6u32, 2167240u32);
    emu.add_memory_rw_events(11usize);
    emu.pc = 2167240u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002111c8));
}
#[inline]
pub fn block_0x002111c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 10usize, 2u32, 2167244u32);
    emu.adr_no_count(10usize, 5usize, 10usize, 2167248u32);
    emu.lw_no_count(14usize, 10usize, 0u32, 2167252u32)?;
    emu.adr_no_count(13usize, 27usize, 13usize, 2167256u32);
    emu.sltru_no_count(6usize, 13usize, 27usize, 2167260u32);
    emu.adr_no_count(6usize, 26usize, 6usize, 2167264u32);
    emu.adr_no_count(14usize, 13usize, 14usize, 2167268u32);
    emu.sltru_no_count(13usize, 14usize, 13usize, 2167272u32);
    emu.sw_no_count(14usize, 10usize, 0u32, 2167276u32)?;
    emu.adr_no_count(13usize, 6usize, 13usize, 2167280u32);
    emu.adi_no_count(10usize, 15usize, 7u32, 2167284u32);
    emu.add_memory_rw_events(11usize);
    emu.pc = 2167284u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002111f4));
}
#[inline]
pub fn block_0x002111f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sli_no_count(10usize, 10usize, 2u32, 2167288u32);
    emu.adr_no_count(14usize, 5usize, 10usize, 2167292u32);
    emu.lw_no_count(10usize, 14usize, 0u32, 2167296u32)?;
    emu.adr_no_count(13usize, 25usize, 13usize, 2167300u32);
    emu.sltru_no_count(6usize, 13usize, 25usize, 2167304u32);
    emu.adr_no_count(6usize, 24usize, 6usize, 2167308u32);
    emu.adr_no_count(9usize, 13usize, 10usize, 2167312u32);
    emu.sltru_no_count(10usize, 9usize, 13usize, 2167316u32);
    emu.adr_no_count(10usize, 6usize, 10usize, 2167320u32);
    emu.sw_no_count(9usize, 14usize, 0u32, 2167324u32)?;
    emu.add_memory_rw_events(10usize);
    emu.pc = 2167324u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021121c));
}
#[inline(always)]
pub fn block_0x0021121c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 15usize, 1u32, 2167328u32);
    emu.sw_no_count(10usize, 16usize, 28u32, 2167332u32)?;
    emu.adi_no_count(16usize, 16usize, 4u32, 2167336u32);
    emu.adi_no_count(11usize, 11usize, 4u32, 2167340u32);
    emu.adi_no_count(17usize, 17usize, 4u32, 2167344u32);
    emu.adi_no_count(10usize, 0usize, 8u32, 2167348u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a != b {
        emu.pc = 2166892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021106c));
    } else {
        emu.pc = 2167352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211238));
    }
}
#[inline(always)]
pub fn block_0x00211238(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2167356u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2167628u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021134c));
}
#[inline]
pub fn block_0x0021123c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 17usize, 4294967292u32, 2167360u32)?;
    emu.adr_no_count(19usize, 9usize, 19usize, 2167364u32);
    emu.sltru_no_count(6usize, 19usize, 9usize, 2167368u32);
    emu.adr_no_count(6usize, 20usize, 6usize, 2167372u32);
    emu.adr_no_count(10usize, 19usize, 10usize, 2167376u32);
    emu.sltru_no_count(9usize, 10usize, 19usize, 2167380u32);
    emu.adr_no_count(9usize, 6usize, 9usize, 2167384u32);
    emu.sw_no_count(10usize, 17usize, 4294967292u32, 2167388u32)?;
    emu.adi_no_count(10usize, 0usize, 5u32, 2167392u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a < b {
        emu.pc = 2167404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021126c));
    } else {
        emu.pc = 2167396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211264));
    }
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
    emu.adi_no_count(10usize, 0usize, 8u32, 2167400u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2167404u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2167108u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211144));
}
#[inline]
pub fn block_0x0021126c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 17usize, 0u32, 2167408u32)?;
    emu.adr_no_count(14usize, 9usize, 14usize, 2167412u32);
    emu.sltru_no_count(6usize, 14usize, 9usize, 2167416u32);
    emu.adr_no_count(13usize, 13usize, 6usize, 2167420u32);
    emu.adr_no_count(10usize, 14usize, 10usize, 2167424u32);
    emu.sltru_no_count(14usize, 10usize, 14usize, 2167428u32);
    emu.adr_no_count(13usize, 13usize, 14usize, 2167432u32);
    emu.sw_no_count(10usize, 17usize, 0u32, 2167436u32)?;
    emu.adi_no_count(10usize, 0usize, 4u32, 2167440u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a != b {
        emu.pc = 2167452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0021129c));
    } else {
        emu.pc = 2167444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211294));
    }
}
#[inline(always)]
pub fn block_0x00211294(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 8u32, 2167448u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2167452u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2167152u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211170));
}
#[inline]
pub fn block_0x0021129c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 17usize, 4u32, 2167456u32)?;
    emu.adr_no_count(21usize, 13usize, 21usize, 2167460u32);
    emu.sltru_no_count(13usize, 21usize, 13usize, 2167464u32);
    emu.adr_no_count(13usize, 22usize, 13usize, 2167468u32);
    emu.adr_no_count(10usize, 21usize, 10usize, 2167472u32);
    emu.sltru_no_count(14usize, 10usize, 21usize, 2167476u32);
    emu.adr_no_count(13usize, 13usize, 14usize, 2167480u32);
    emu.sw_no_count(10usize, 17usize, 4u32, 2167484u32)?;
    emu.adi_no_count(10usize, 0usize, 3u32, 2167488u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a < b {
        emu.pc = 2167500u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002112cc));
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
#[inline(always)]
pub fn block_0x002112c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 8u32, 2167496u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2167500u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2167196u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021119c));
}
#[inline]
pub fn block_0x002112cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 17usize, 8u32, 2167504u32)?;
    emu.adr_no_count(23usize, 13usize, 23usize, 2167508u32);
    emu.sltru_no_count(13usize, 23usize, 13usize, 2167512u32);
    emu.adr_no_count(13usize, 1usize, 13usize, 2167516u32);
    emu.adr_no_count(10usize, 23usize, 10usize, 2167520u32);
    emu.sltru_no_count(14usize, 10usize, 23usize, 2167524u32);
    emu.adr_no_count(13usize, 13usize, 14usize, 2167528u32);
    emu.sw_no_count(10usize, 17usize, 8u32, 2167532u32)?;
    emu.adi_no_count(10usize, 0usize, 2u32, 2167536u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a != b {
        emu.pc = 2167548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002112fc));
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
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 8u32, 2167544u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2167548u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2167240u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002111c8));
}
#[inline]
pub fn block_0x002112fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 17usize, 12u32, 2167552u32)?;
    emu.adr_no_count(27usize, 13usize, 27usize, 2167556u32);
    emu.sltru_no_count(13usize, 27usize, 13usize, 2167560u32);
    emu.adr_no_count(13usize, 26usize, 13usize, 2167564u32);
    emu.adr_no_count(10usize, 27usize, 10usize, 2167568u32);
    emu.sltru_no_count(14usize, 10usize, 27usize, 2167572u32);
    emu.adr_no_count(13usize, 13usize, 14usize, 2167576u32);
    emu.sw_no_count(10usize, 17usize, 12u32, 2167580u32)?;
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2167592u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211328));
    } else {
        emu.pc = 2167584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211320));
    }
}
#[inline(always)]
pub fn block_0x00211320(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 8u32, 2167588u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2167592u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2167284u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002111f4));
}
#[inline]
pub fn block_0x00211328(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 40u32, 2167596u32)?;
    emu.adr_no_count(25usize, 13usize, 25usize, 2167600u32);
    emu.sltru_no_count(13usize, 25usize, 13usize, 2167604u32);
    emu.adr_no_count(13usize, 24usize, 13usize, 2167608u32);
    emu.adr_no_count(14usize, 25usize, 10usize, 2167612u32);
    emu.sltru_no_count(10usize, 14usize, 25usize, 2167616u32);
    emu.adr_no_count(10usize, 13usize, 10usize, 2167620u32);
    emu.sw_no_count(14usize, 2usize, 40u32, 2167624u32)?;
    emu.add_memory_rw_events(9usize);
    let return_addr = 2167628u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2167324u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0021121c));
}
#[inline(never)]
pub fn block_0x0021134c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 50u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 2usize, 12u32, 2167632u32);
    emu.lw_no_count(10usize, 15usize, 16u32, 2167636u32)?;
    emu.lw_no_count(11usize, 15usize, 20u32, 2167640u32)?;
    emu.lw_no_count(12usize, 15usize, 24u32, 2167644u32)?;
    emu.lw_no_count(13usize, 15usize, 28u32, 2167648u32)?;
    emu.lw_no_count(14usize, 2usize, 4u32, 2167652u32)?;
    emu.sw_no_count(10usize, 14usize, 16u32, 2167656u32)?;
    emu.sw_no_count(11usize, 14usize, 20u32, 2167660u32)?;
    emu.sw_no_count(12usize, 14usize, 24u32, 2167664u32)?;
    emu.sw_no_count(13usize, 14usize, 28u32, 2167668u32)?;
    emu.lw_no_count(10usize, 15usize, 0u32, 2167672u32)?;
    emu.lw_no_count(11usize, 15usize, 4u32, 2167676u32)?;
    emu.lw_no_count(12usize, 15usize, 8u32, 2167680u32)?;
    emu.lw_no_count(13usize, 15usize, 12u32, 2167684u32)?;
    emu.sw_no_count(10usize, 14usize, 0u32, 2167688u32)?;
    emu.sw_no_count(11usize, 14usize, 4u32, 2167692u32)?;
    emu.sw_no_count(12usize, 14usize, 8u32, 2167696u32)?;
    emu.sw_no_count(13usize, 14usize, 12u32, 2167700u32)?;
    emu.adi_no_count(15usize, 2usize, 44u32, 2167704u32);
    emu.lw_no_count(10usize, 15usize, 0u32, 2167708u32)?;
    emu.lw_no_count(11usize, 15usize, 4u32, 2167712u32)?;
    emu.lw_no_count(12usize, 15usize, 8u32, 2167716u32)?;
    emu.lw_no_count(13usize, 15usize, 12u32, 2167720u32)?;
    emu.sw_no_count(10usize, 14usize, 32u32, 2167724u32)?;
    emu.sw_no_count(11usize, 14usize, 36u32, 2167728u32)?;
    emu.sw_no_count(12usize, 14usize, 40u32, 2167732u32)?;
    emu.sw_no_count(13usize, 14usize, 44u32, 2167736u32)?;
    emu.lw_no_count(10usize, 15usize, 16u32, 2167740u32)?;
    emu.lw_no_count(11usize, 15usize, 20u32, 2167744u32)?;
    emu.lw_no_count(12usize, 15usize, 24u32, 2167748u32)?;
    emu.lw_no_count(13usize, 15usize, 28u32, 2167752u32)?;
    emu.sw_no_count(10usize, 14usize, 48u32, 2167756u32)?;
    emu.sw_no_count(11usize, 14usize, 52u32, 2167760u32)?;
    emu.sw_no_count(12usize, 14usize, 56u32, 2167764u32)?;
    emu.sw_no_count(13usize, 14usize, 60u32, 2167768u32)?;
    emu.lw_no_count(1usize, 2usize, 124u32, 2167772u32)?;
    emu.lw_no_count(8usize, 2usize, 120u32, 2167776u32)?;
    emu.lw_no_count(9usize, 2usize, 116u32, 2167780u32)?;
    emu.lw_no_count(18usize, 2usize, 112u32, 2167784u32)?;
    emu.lw_no_count(19usize, 2usize, 108u32, 2167788u32)?;
    emu.lw_no_count(20usize, 2usize, 104u32, 2167792u32)?;
    emu.lw_no_count(21usize, 2usize, 100u32, 2167796u32)?;
    emu.lw_no_count(22usize, 2usize, 96u32, 2167800u32)?;
    emu.lw_no_count(23usize, 2usize, 92u32, 2167804u32)?;
    emu.lw_no_count(24usize, 2usize, 88u32, 2167808u32)?;
    emu.lw_no_count(25usize, 2usize, 84u32, 2167812u32)?;
    emu.lw_no_count(26usize, 2usize, 80u32, 2167816u32)?;
    emu.lw_no_count(27usize, 2usize, 76u32, 2167820u32)?;
    emu.adi_no_count(2usize, 2usize, 128u32, 2167824u32);
    emu.add_memory_rw_events(50usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2167828u32;
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
pub fn block_0x00211414(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 184u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2167832u32);
    emu.sw_no_count(8usize, 2usize, 28u32, 2167836u32)?;
    emu.sw_no_count(9usize, 2usize, 24u32, 2167840u32)?;
    emu.sw_no_count(18usize, 2usize, 20u32, 2167844u32)?;
    emu.sw_no_count(19usize, 2usize, 16u32, 2167848u32)?;
    emu.sw_no_count(20usize, 2usize, 12u32, 2167852u32)?;
    emu.sw_no_count(21usize, 2usize, 8u32, 2167856u32)?;
    emu.sw_no_count(22usize, 2usize, 4u32, 2167860u32)?;
    emu.sw_no_count(23usize, 2usize, 0u32, 2167864u32)?;
    emu.lw_no_count(29usize, 11usize, 0u32, 2167868u32)?;
    emu.lw_no_count(9usize, 11usize, 4u32, 2167872u32)?;
    emu.lw_no_count(6usize, 11usize, 8u32, 2167876u32)?;
    emu.lw_no_count(30usize, 11usize, 12u32, 2167880u32)?;
    emu.lw_no_count(20usize, 12usize, 0u32, 2167884u32)?;
    emu.lw_no_count(18usize, 12usize, 4u32, 2167888u32)?;
    emu.lw_no_count(31usize, 12usize, 8u32, 2167892u32)?;
    emu.lw_no_count(8usize, 12usize, 12u32, 2167896u32)?;
    emu.lw_no_count(19usize, 11usize, 16u32, 2167900u32)?;
    emu.lw_no_count(28usize, 11usize, 20u32, 2167904u32)?;
    emu.lw_no_count(5usize, 11usize, 24u32, 2167908u32)?;
    emu.lw_no_count(7usize, 11usize, 28u32, 2167912u32)?;
    emu.lw_no_count(14usize, 13usize, 0u32, 2167916u32)?;
    emu.lw_no_count(15usize, 13usize, 4u32, 2167920u32)?;
    emu.lw_no_count(16usize, 13usize, 8u32, 2167924u32)?;
    emu.lw_no_count(17usize, 13usize, 12u32, 2167928u32)?;
    emu.adr_no_count(29usize, 20usize, 29usize, 2167932u32);
    emu.sltru_no_count(11usize, 29usize, 20usize, 2167936u32);
    emu.sltru_no_count(20usize, 29usize, 14usize, 2167940u32);
    emu.adr_no_count(9usize, 11usize, 9usize, 2167944u32);
    emu.sltru_no_count(11usize, 9usize, 11usize, 2167948u32);
    emu.adr_no_count(18usize, 9usize, 18usize, 2167952u32);
    emu.sltru_no_count(9usize, 18usize, 9usize, 2167956u32);
    emu.sbr_no_count(21usize, 18usize, 15usize, 2167960u32);
    emu.sltru_no_count(18usize, 18usize, 15usize, 2167964u32);
    emu.adr_no_count(22usize, 11usize, 9usize, 2167968u32);
    emu.adr_no_count(18usize, 20usize, 18usize, 2167972u32);
    emu.sbr_no_count(11usize, 21usize, 20usize, 2167976u32);
    emu.sltru_no_count(9usize, 11usize, 21usize, 2167980u32);
    emu.sbr_no_count(21usize, 9usize, 18usize, 2167984u32);
    emu.lw_no_count(23usize, 12usize, 16u32, 2167988u32)?;
    emu.lw_no_count(20usize, 12usize, 20u32, 2167992u32)?;
    emu.lw_no_count(18usize, 12usize, 24u32, 2167996u32)?;
    emu.lw_no_count(9usize, 12usize, 28u32, 2168000u32)?;
    emu.adr_no_count(6usize, 31usize, 6usize, 2168004u32);
    emu.adr_no_count(30usize, 8usize, 30usize, 2168008u32);
    emu.adr_no_count(19usize, 23usize, 19usize, 2168012u32);
    emu.sltru_no_count(12usize, 6usize, 31usize, 2168016u32);
    emu.sltru_no_count(31usize, 30usize, 8usize, 2168020u32);
    emu.sltru_no_count(8usize, 19usize, 23usize, 2168024u32);
    emu.adr_no_count(22usize, 6usize, 22usize, 2168028u32);
    emu.sltru_no_count(6usize, 22usize, 6usize, 2168032u32);
    emu.adr_no_count(12usize, 12usize, 6usize, 2168036u32);
    emu.sbr_no_count(6usize, 22usize, 16usize, 2168040u32);
    emu.sltru_no_count(22usize, 22usize, 16usize, 2168044u32);
    emu.sai_no_count(21usize, 21usize, 1055u32, 2168048u32);
    emu.adr_no_count(23usize, 30usize, 12usize, 2168052u32);
    emu.sbr_no_count(22usize, 21usize, 22usize, 2168056u32);
    emu.adr_no_count(12usize, 6usize, 21usize, 2168060u32);
    emu.sltru_no_count(30usize, 23usize, 30usize, 2168064u32);
    emu.sltru_no_count(6usize, 12usize, 6usize, 2168068u32);
    emu.adr_no_count(30usize, 31usize, 30usize, 2168072u32);
    emu.sbr_no_count(31usize, 23usize, 17usize, 2168076u32);
    emu.sltru_no_count(21usize, 23usize, 17usize, 2168080u32);
    emu.adr_no_count(6usize, 22usize, 6usize, 2168084u32);
    emu.adr_no_count(22usize, 19usize, 30usize, 2168088u32);
    emu.sai_no_count(6usize, 6usize, 1055u32, 2168092u32);
    emu.sltru_no_count(30usize, 22usize, 19usize, 2168096u32);
    emu.sbr_no_count(19usize, 6usize, 21usize, 2168100u32);
    emu.adr_no_count(6usize, 31usize, 6usize, 2168104u32);
    emu.adr_no_count(21usize, 8usize, 30usize, 2168108u32);
    emu.sltru_no_count(30usize, 6usize, 31usize, 2168112u32);
    emu.adr_no_count(23usize, 19usize, 30usize, 2168116u32);
    emu.lw_no_count(30usize, 13usize, 16u32, 2168120u32)?;
    emu.lw_no_count(31usize, 13usize, 20u32, 2168124u32)?;
    emu.lw_no_count(8usize, 13usize, 24u32, 2168128u32)?;
    emu.lw_no_count(19usize, 13usize, 28u32, 2168132u32)?;
    emu.adr_no_count(28usize, 20usize, 28usize, 2168136u32);
    emu.sltru_no_count(13usize, 28usize, 20usize, 2168140u32);
    emu.adr_no_count(21usize, 28usize, 21usize, 2168144u32);
    emu.sltru_no_count(28usize, 21usize, 28usize, 2168148u32);
    emu.adr_no_count(28usize, 13usize, 28usize, 2168152u32);
    emu.sbr_no_count(20usize, 22usize, 30usize, 2168156u32);
    emu.sltru_no_count(13usize, 22usize, 30usize, 2168160u32);
    emu.sai_no_count(22usize, 23usize, 1055u32, 2168164u32);
    emu.sbr_no_count(23usize, 22usize, 13usize, 2168168u32);
    emu.adr_no_count(13usize, 20usize, 22usize, 2168172u32);
    emu.sltru_no_count(20usize, 13usize, 20usize, 2168176u32);
    emu.adr_no_count(20usize, 23usize, 20usize, 2168180u32);
    emu.adr_no_count(5usize, 18usize, 5usize, 2168184u32);
    emu.sltru_no_count(18usize, 5usize, 18usize, 2168188u32);
    emu.adr_no_count(28usize, 5usize, 28usize, 2168192u32);
    emu.sltru_no_count(5usize, 28usize, 5usize, 2168196u32);
    emu.adr_no_count(18usize, 18usize, 5usize, 2168200u32);
    emu.adr_no_count(7usize, 9usize, 7usize, 2168204u32);
    emu.sltru_no_count(9usize, 7usize, 9usize, 2168208u32);
    emu.sbr_no_count(5usize, 29usize, 14usize, 2168212u32);
    emu.adr_no_count(18usize, 7usize, 18usize, 2168216u32);
    emu.sltru_no_count(7usize, 18usize, 7usize, 2168220u32);
    emu.adr_no_count(9usize, 9usize, 7usize, 2168224u32);
    emu.sbr_no_count(29usize, 21usize, 31usize, 2168228u32);
    emu.sltru_no_count(7usize, 21usize, 31usize, 2168232u32);
    emu.sai_no_count(20usize, 20usize, 1055u32, 2168236u32);
    emu.sbr_no_count(21usize, 20usize, 7usize, 2168240u32);
    emu.adr_no_count(7usize, 29usize, 20usize, 2168244u32);
    emu.sltru_no_count(29usize, 7usize, 29usize, 2168248u32);
    emu.adr_no_count(29usize, 21usize, 29usize, 2168252u32);
    emu.sbr_no_count(20usize, 28usize, 8usize, 2168256u32);
    emu.sltru_no_count(28usize, 28usize, 8usize, 2168260u32);
    emu.sai_no_count(29usize, 29usize, 1055u32, 2168264u32);
    emu.sbr_no_count(21usize, 29usize, 28usize, 2168268u32);
    emu.adr_no_count(28usize, 20usize, 29usize, 2168272u32);
    emu.sltru_no_count(29usize, 28usize, 20usize, 2168276u32);
    emu.adr_no_count(29usize, 21usize, 29usize, 2168280u32);
    emu.sbr_no_count(20usize, 18usize, 19usize, 2168284u32);
    emu.sltru_no_count(18usize, 18usize, 19usize, 2168288u32);
    emu.sai_no_count(29usize, 29usize, 1055u32, 2168292u32);
    emu.sbr_no_count(18usize, 29usize, 18usize, 2168296u32);
    emu.adr_no_count(29usize, 20usize, 29usize, 2168300u32);
    emu.sltru_no_count(20usize, 29usize, 20usize, 2168304u32);
    emu.adr_no_count(18usize, 18usize, 20usize, 2168308u32);
    emu.sai_no_count(18usize, 18usize, 1055u32, 2168312u32);
    emu.adr_no_count(9usize, 18usize, 9usize, 2168316u32);
    emu.sltru_no_count(9usize, 9usize, 18usize, 2168320u32);
    emu.adr_no_count(9usize, 18usize, 9usize, 2168324u32);
    emu.anr_no_count(14usize, 14usize, 9usize, 2168328u32);
    emu.anr_no_count(15usize, 15usize, 9usize, 2168332u32);
    emu.anr_no_count(16usize, 16usize, 9usize, 2168336u32);
    emu.anr_no_count(17usize, 17usize, 9usize, 2168340u32);
    emu.anr_no_count(30usize, 30usize, 9usize, 2168344u32);
    emu.anr_no_count(31usize, 31usize, 9usize, 2168348u32);
    emu.anr_no_count(8usize, 8usize, 9usize, 2168352u32);
    emu.anr_no_count(9usize, 19usize, 9usize, 2168356u32);
    emu.adr_no_count(14usize, 5usize, 14usize, 2168360u32);
    emu.adr_no_count(15usize, 11usize, 15usize, 2168364u32);
    emu.adr_no_count(16usize, 12usize, 16usize, 2168368u32);
    emu.adr_no_count(17usize, 6usize, 17usize, 2168372u32);
    emu.adr_no_count(30usize, 13usize, 30usize, 2168376u32);
    emu.adr_no_count(31usize, 7usize, 31usize, 2168380u32);
    emu.adr_no_count(8usize, 28usize, 8usize, 2168384u32);
    emu.sltru_no_count(5usize, 14usize, 5usize, 2168388u32);
    emu.sltru_no_count(11usize, 15usize, 11usize, 2168392u32);
    emu.sltru_no_count(12usize, 16usize, 12usize, 2168396u32);
    emu.sltru_no_count(6usize, 17usize, 6usize, 2168400u32);
    emu.sltru_no_count(13usize, 30usize, 13usize, 2168404u32);
    emu.sltru_no_count(7usize, 31usize, 7usize, 2168408u32);
    emu.sltru_no_count(28usize, 8usize, 28usize, 2168412u32);
    emu.adr_no_count(5usize, 15usize, 5usize, 2168416u32);
    emu.adr_no_count(28usize, 28usize, 29usize, 2168420u32);
    emu.sltru_no_count(15usize, 5usize, 15usize, 2168424u32);
    emu.adr_no_count(28usize, 9usize, 28usize, 2168428u32);
    emu.adr_no_count(11usize, 11usize, 15usize, 2168432u32);
    emu.adr_no_count(11usize, 16usize, 11usize, 2168436u32);
    emu.sltru_no_count(15usize, 11usize, 16usize, 2168440u32);
    emu.adr_no_count(12usize, 12usize, 15usize, 2168444u32);
    emu.adr_no_count(12usize, 17usize, 12usize, 2168448u32);
    emu.sltru_no_count(15usize, 12usize, 17usize, 2168452u32);
    emu.sw_no_count(14usize, 10usize, 0u32, 2168456u32)?;
    emu.sw_no_count(5usize, 10usize, 4u32, 2168460u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2168464u32)?;
    emu.sw_no_count(12usize, 10usize, 12u32, 2168468u32)?;
    emu.adr_no_count(15usize, 6usize, 15usize, 2168472u32);
    emu.adr_no_count(15usize, 30usize, 15usize, 2168476u32);
    emu.sltru_no_count(11usize, 15usize, 30usize, 2168480u32);
    emu.adr_no_count(11usize, 13usize, 11usize, 2168484u32);
    emu.adr_no_count(11usize, 31usize, 11usize, 2168488u32);
    emu.sltru_no_count(12usize, 11usize, 31usize, 2168492u32);
    emu.adr_no_count(12usize, 7usize, 12usize, 2168496u32);
    emu.adr_no_count(12usize, 8usize, 12usize, 2168500u32);
    emu.sltru_no_count(13usize, 12usize, 8usize, 2168504u32);
    emu.adr_no_count(13usize, 28usize, 13usize, 2168508u32);
    emu.sw_no_count(15usize, 10usize, 16u32, 2168512u32)?;
    emu.sw_no_count(11usize, 10usize, 20u32, 2168516u32)?;
    emu.sw_no_count(12usize, 10usize, 24u32, 2168520u32)?;
    emu.sw_no_count(13usize, 10usize, 28u32, 2168524u32)?;
    emu.lw_no_count(8usize, 2usize, 28u32, 2168528u32)?;
    emu.lw_no_count(9usize, 2usize, 24u32, 2168532u32)?;
    emu.lw_no_count(18usize, 2usize, 20u32, 2168536u32)?;
    emu.lw_no_count(19usize, 2usize, 16u32, 2168540u32)?;
    emu.lw_no_count(20usize, 2usize, 12u32, 2168544u32)?;
    emu.lw_no_count(21usize, 2usize, 8u32, 2168548u32)?;
    emu.lw_no_count(22usize, 2usize, 4u32, 2168552u32)?;
    emu.lw_no_count(23usize, 2usize, 0u32, 2168556u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2168560u32);
    emu.add_memory_rw_events(184usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2168564u32;
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
pub fn block_0x002116f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 140u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2168568u32);
    emu.sw_no_count(8usize, 2usize, 28u32, 2168572u32)?;
    emu.sw_no_count(9usize, 2usize, 24u32, 2168576u32)?;
    emu.sw_no_count(18usize, 2usize, 20u32, 2168580u32)?;
    emu.sw_no_count(19usize, 2usize, 16u32, 2168584u32)?;
    emu.sw_no_count(20usize, 2usize, 12u32, 2168588u32)?;
    emu.sw_no_count(21usize, 2usize, 8u32, 2168592u32)?;
    emu.lw_no_count(15usize, 11usize, 0u32, 2168596u32)?;
    emu.lw_no_count(14usize, 11usize, 4u32, 2168600u32)?;
    emu.lw_no_count(17usize, 11usize, 8u32, 2168604u32)?;
    emu.lw_no_count(5usize, 11usize, 12u32, 2168608u32)?;
    emu.lw_no_count(6usize, 12usize, 0u32, 2168612u32)?;
    emu.lw_no_count(16usize, 12usize, 4u32, 2168616u32)?;
    emu.lw_no_count(7usize, 12usize, 8u32, 2168620u32)?;
    emu.lw_no_count(28usize, 12usize, 12u32, 2168624u32)?;
    emu.lw_no_count(29usize, 11usize, 16u32, 2168628u32)?;
    emu.lw_no_count(30usize, 11usize, 20u32, 2168632u32)?;
    emu.lw_no_count(31usize, 11usize, 24u32, 2168636u32)?;
    emu.lw_no_count(8usize, 11usize, 28u32, 2168640u32)?;
    emu.lw_no_count(9usize, 12usize, 16u32, 2168644u32)?;
    emu.lw_no_count(18usize, 12usize, 20u32, 2168648u32)?;
    emu.lw_no_count(19usize, 12usize, 24u32, 2168652u32)?;
    emu.lw_no_count(12usize, 12usize, 28u32, 2168656u32)?;
    emu.sbr_no_count(11usize, 15usize, 6usize, 2168660u32);
    emu.sltru_no_count(15usize, 15usize, 6usize, 2168664u32);
    emu.sbr_no_count(6usize, 17usize, 7usize, 2168668u32);
    emu.sltru_no_count(17usize, 17usize, 7usize, 2168672u32);
    emu.sbr_no_count(7usize, 5usize, 28usize, 2168676u32);
    emu.sltru_no_count(5usize, 5usize, 28usize, 2168680u32);
    emu.sbr_no_count(28usize, 29usize, 9usize, 2168684u32);
    emu.sltru_no_count(29usize, 29usize, 9usize, 2168688u32);
    emu.sbr_no_count(9usize, 30usize, 18usize, 2168692u32);
    emu.sltru_no_count(30usize, 30usize, 18usize, 2168696u32);
    emu.sbr_no_count(18usize, 31usize, 19usize, 2168700u32);
    emu.sltru_no_count(31usize, 31usize, 19usize, 2168704u32);
    emu.sbr_no_count(19usize, 8usize, 12usize, 2168708u32);
    emu.sltru_no_count(8usize, 8usize, 12usize, 2168712u32);
    emu.sbr_no_count(12usize, 0usize, 15usize, 2168716u32);
    emu.sbr_no_count(14usize, 14usize, 15usize, 2168720u32);
    emu.sltru_no_count(20usize, 14usize, 12usize, 2168724u32);
    emu.sltru_no_count(21usize, 14usize, 16usize, 2168728u32);
    emu.sbr_no_count(12usize, 14usize, 16usize, 2168732u32);
    emu.sbr_no_count(14usize, 20usize, 15usize, 2168736u32);
    emu.sbr_no_count(14usize, 14usize, 21usize, 2168740u32);
    emu.sai_no_count(14usize, 14usize, 1055u32, 2168744u32);
    emu.sbr_no_count(15usize, 14usize, 17usize, 2168748u32);
    emu.adr_no_count(14usize, 6usize, 14usize, 2168752u32);
    emu.sltru_no_count(16usize, 14usize, 6usize, 2168756u32);
    emu.adr_no_count(15usize, 15usize, 16usize, 2168760u32);
    emu.sai_no_count(15usize, 15usize, 1055u32, 2168764u32);
    emu.sbr_no_count(16usize, 15usize, 5usize, 2168768u32);
    emu.adr_no_count(15usize, 7usize, 15usize, 2168772u32);
    emu.sltru_no_count(17usize, 15usize, 7usize, 2168776u32);
    emu.adr_no_count(16usize, 16usize, 17usize, 2168780u32);
    emu.sai_no_count(16usize, 16usize, 1055u32, 2168784u32);
    emu.sbr_no_count(17usize, 16usize, 29usize, 2168788u32);
    emu.adr_no_count(16usize, 28usize, 16usize, 2168792u32);
    emu.sltru_no_count(5usize, 16usize, 28usize, 2168796u32);
    emu.adr_no_count(17usize, 17usize, 5usize, 2168800u32);
    emu.lw_no_count(7usize, 13usize, 0u32, 2168804u32)?;
    emu.lw_no_count(28usize, 13usize, 4u32, 2168808u32)?;
    emu.lw_no_count(29usize, 13usize, 8u32, 2168812u32)?;
    emu.lw_no_count(20usize, 13usize, 12u32, 2168816u32)?;
    emu.sai_no_count(17usize, 17usize, 1055u32, 2168820u32);
    emu.sbr_no_count(5usize, 17usize, 30usize, 2168824u32);
    emu.adr_no_count(17usize, 9usize, 17usize, 2168828u32);
    emu.sltru_no_count(6usize, 17usize, 9usize, 2168832u32);
    emu.adr_no_count(5usize, 5usize, 6usize, 2168836u32);
    emu.sai_no_count(5usize, 5usize, 1055u32, 2168840u32);
    emu.sbr_no_count(6usize, 5usize, 31usize, 2168844u32);
    emu.adr_no_count(5usize, 18usize, 5usize, 2168848u32);
    emu.sltru_no_count(30usize, 5usize, 18usize, 2168852u32);
    emu.adr_no_count(6usize, 6usize, 30usize, 2168856u32);
    emu.sai_no_count(6usize, 6usize, 1055u32, 2168860u32);
    emu.sbr_no_count(30usize, 6usize, 8usize, 2168864u32);
    emu.adr_no_count(6usize, 19usize, 6usize, 2168868u32);
    emu.sltru_no_count(31usize, 6usize, 19usize, 2168872u32);
    emu.adr_no_count(30usize, 30usize, 31usize, 2168876u32);
    emu.lw_no_count(31usize, 13usize, 16u32, 2168880u32)?;
    emu.lw_no_count(8usize, 13usize, 20u32, 2168884u32)?;
    emu.lw_no_count(9usize, 13usize, 24u32, 2168888u32)?;
    emu.lw_no_count(13usize, 13usize, 28u32, 2168892u32)?;
    emu.anr_no_count(7usize, 7usize, 30usize, 2168896u32);
    emu.anr_no_count(28usize, 28usize, 30usize, 2168900u32);
    emu.anr_no_count(29usize, 29usize, 30usize, 2168904u32);
    emu.anr_no_count(18usize, 20usize, 30usize, 2168908u32);
    emu.anr_no_count(31usize, 31usize, 30usize, 2168912u32);
    emu.anr_no_count(8usize, 8usize, 30usize, 2168916u32);
    emu.anr_no_count(9usize, 9usize, 30usize, 2168920u32);
    emu.anr_no_count(13usize, 13usize, 30usize, 2168924u32);
    emu.adr_no_count(7usize, 11usize, 7usize, 2168928u32);
    emu.adr_no_count(28usize, 12usize, 28usize, 2168932u32);
    emu.adr_no_count(29usize, 14usize, 29usize, 2168936u32);
    emu.adr_no_count(18usize, 15usize, 18usize, 2168940u32);
    emu.adr_no_count(31usize, 16usize, 31usize, 2168944u32);
    emu.adr_no_count(8usize, 17usize, 8usize, 2168948u32);
    emu.adr_no_count(9usize, 5usize, 9usize, 2168952u32);
    emu.sltru_no_count(11usize, 7usize, 11usize, 2168956u32);
    emu.sltru_no_count(12usize, 28usize, 12usize, 2168960u32);
    emu.sltru_no_count(14usize, 29usize, 14usize, 2168964u32);
    emu.sltru_no_count(15usize, 18usize, 15usize, 2168968u32);
    emu.sltru_no_count(16usize, 31usize, 16usize, 2168972u32);
    emu.sltru_no_count(17usize, 8usize, 17usize, 2168976u32);
    emu.sltru_no_count(5usize, 9usize, 5usize, 2168980u32);
    emu.adr_no_count(11usize, 28usize, 11usize, 2168984u32);
    emu.adr_no_count(5usize, 5usize, 6usize, 2168988u32);
    emu.sltru_no_count(6usize, 11usize, 28usize, 2168992u32);
    emu.adr_no_count(13usize, 13usize, 5usize, 2168996u32);
    emu.adr_no_count(12usize, 12usize, 6usize, 2169000u32);
    emu.adr_no_count(12usize, 29usize, 12usize, 2169004u32);
    emu.sltru_no_count(5usize, 12usize, 29usize, 2169008u32);
    emu.adr_no_count(14usize, 14usize, 5usize, 2169012u32);
    emu.adr_no_count(14usize, 18usize, 14usize, 2169016u32);
    emu.sltru_no_count(5usize, 14usize, 18usize, 2169020u32);
    emu.sw_no_count(7usize, 10usize, 0u32, 2169024u32)?;
    emu.sw_no_count(11usize, 10usize, 4u32, 2169028u32)?;
    emu.sw_no_count(12usize, 10usize, 8u32, 2169032u32)?;
    emu.sw_no_count(14usize, 10usize, 12u32, 2169036u32)?;
    emu.adr_no_count(15usize, 15usize, 5usize, 2169040u32);
    emu.adr_no_count(15usize, 31usize, 15usize, 2169044u32);
    emu.sltru_no_count(11usize, 15usize, 31usize, 2169048u32);
    emu.adr_no_count(11usize, 16usize, 11usize, 2169052u32);
    emu.adr_no_count(11usize, 8usize, 11usize, 2169056u32);
    emu.sltru_no_count(12usize, 11usize, 8usize, 2169060u32);
    emu.adr_no_count(12usize, 17usize, 12usize, 2169064u32);
    emu.adr_no_count(12usize, 9usize, 12usize, 2169068u32);
    emu.sltru_no_count(14usize, 12usize, 9usize, 2169072u32);
    emu.adr_no_count(13usize, 13usize, 14usize, 2169076u32);
    emu.sw_no_count(15usize, 10usize, 16u32, 2169080u32)?;
    emu.sw_no_count(11usize, 10usize, 20u32, 2169084u32)?;
    emu.sw_no_count(12usize, 10usize, 24u32, 2169088u32)?;
    emu.sw_no_count(13usize, 10usize, 28u32, 2169092u32)?;
    emu.lw_no_count(8usize, 2usize, 28u32, 2169096u32)?;
    emu.lw_no_count(9usize, 2usize, 24u32, 2169100u32)?;
    emu.lw_no_count(18usize, 2usize, 20u32, 2169104u32)?;
    emu.lw_no_count(19usize, 2usize, 16u32, 2169108u32)?;
    emu.lw_no_count(20usize, 2usize, 12u32, 2169112u32)?;
    emu.lw_no_count(21usize, 2usize, 8u32, 2169116u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2169120u32);
    emu.add_memory_rw_events(140usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2169124u32;
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
pub fn block_0x00211924(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 99u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2169128u32);
    emu.sw_no_count(8usize, 2usize, 12u32, 2169132u32)?;
    emu.sw_no_count(9usize, 2usize, 8u32, 2169136u32)?;
    emu.sw_no_count(18usize, 2usize, 4u32, 2169140u32)?;
    emu.sw_no_count(19usize, 2usize, 0u32, 2169144u32)?;
    emu.lbu_no_count(12usize, 11usize, 0u32, 2169148u32);
    emu.lbu_no_count(13usize, 11usize, 1u32, 2169152u32);
    emu.lbu_no_count(14usize, 11usize, 2u32, 2169156u32);
    emu.lbu_no_count(15usize, 11usize, 3u32, 2169160u32);
    emu.lbu_no_count(16usize, 11usize, 4u32, 2169164u32);
    emu.lbu_no_count(17usize, 11usize, 5u32, 2169168u32);
    emu.lbu_no_count(5usize, 11usize, 6u32, 2169172u32);
    emu.lbu_no_count(6usize, 11usize, 7u32, 2169176u32);
    emu.lbu_no_count(7usize, 11usize, 8u32, 2169180u32);
    emu.lbu_no_count(28usize, 11usize, 9u32, 2169184u32);
    emu.lbu_no_count(29usize, 11usize, 10u32, 2169188u32);
    emu.lbu_no_count(30usize, 11usize, 11u32, 2169192u32);
    emu.lbu_no_count(31usize, 11usize, 12u32, 2169196u32);
    emu.lbu_no_count(8usize, 11usize, 13u32, 2169200u32);
    emu.lbu_no_count(9usize, 11usize, 14u32, 2169204u32);
    emu.lbu_no_count(18usize, 11usize, 15u32, 2169208u32);
    emu.sli_no_count(14usize, 14usize, 8u32, 2169212u32);
    emu.sli_no_count(13usize, 13usize, 16u32, 2169216u32);
    emu.sli_no_count(19usize, 12usize, 24u32, 2169220u32);
    emu.sli_no_count(5usize, 5usize, 8u32, 2169224u32);
    emu.orr_no_count(12usize, 14usize, 15usize, 2169228u32);
    emu.orr_no_count(13usize, 19usize, 13usize, 2169232u32);
    emu.orr_no_count(14usize, 5usize, 6usize, 2169236u32);
    emu.lbu_no_count(15usize, 11usize, 16u32, 2169240u32);
    emu.lbu_no_count(5usize, 11usize, 17u32, 2169244u32);
    emu.lbu_no_count(6usize, 11usize, 18u32, 2169248u32);
    emu.lbu_no_count(19usize, 11usize, 19u32, 2169252u32);
    emu.sli_no_count(17usize, 17usize, 16u32, 2169256u32);
    emu.sli_no_count(16usize, 16usize, 24u32, 2169260u32);
    emu.sli_no_count(29usize, 29usize, 8u32, 2169264u32);
    emu.sli_no_count(28usize, 28usize, 16u32, 2169268u32);
    emu.sli_no_count(7usize, 7usize, 24u32, 2169272u32);
    emu.sli_no_count(9usize, 9usize, 8u32, 2169276u32);
    emu.orr_no_count(16usize, 16usize, 17usize, 2169280u32);
    emu.orr_no_count(17usize, 29usize, 30usize, 2169284u32);
    emu.orr_no_count(7usize, 7usize, 28usize, 2169288u32);
    emu.orr_no_count(28usize, 9usize, 18usize, 2169292u32);
    emu.lbu_no_count(29usize, 11usize, 20u32, 2169296u32);
    emu.lbu_no_count(30usize, 11usize, 21u32, 2169300u32);
    emu.lbu_no_count(9usize, 11usize, 22u32, 2169304u32);
    emu.lbu_no_count(18usize, 11usize, 23u32, 2169308u32);
    emu.sli_no_count(8usize, 8usize, 16u32, 2169312u32);
    emu.sli_no_count(31usize, 31usize, 24u32, 2169316u32);
    emu.sli_no_count(6usize, 6usize, 8u32, 2169320u32);
    emu.sli_no_count(5usize, 5usize, 16u32, 2169324u32);
    emu.sli_no_count(15usize, 15usize, 24u32, 2169328u32);
    emu.sli_no_count(9usize, 9usize, 8u32, 2169332u32);
    emu.orr_no_count(31usize, 31usize, 8usize, 2169336u32);
    emu.orr_no_count(6usize, 6usize, 19usize, 2169340u32);
    emu.orr_no_count(15usize, 15usize, 5usize, 2169344u32);
    emu.orr_no_count(5usize, 9usize, 18usize, 2169348u32);
    emu.lbu_no_count(8usize, 11usize, 24u32, 2169352u32);
    emu.lbu_no_count(9usize, 11usize, 25u32, 2169356u32);
    emu.lbu_no_count(18usize, 11usize, 26u32, 2169360u32);
    emu.lbu_no_count(19usize, 11usize, 27u32, 2169364u32);
    emu.sli_no_count(30usize, 30usize, 16u32, 2169368u32);
    emu.sli_no_count(29usize, 29usize, 24u32, 2169372u32);
    emu.sli_no_count(18usize, 18usize, 8u32, 2169376u32);
    emu.orr_no_count(29usize, 29usize, 30usize, 2169380u32);
    emu.orr_no_count(30usize, 18usize, 19usize, 2169384u32);
    emu.lbu_no_count(18usize, 11usize, 30u32, 2169388u32);
    emu.lbu_no_count(19usize, 11usize, 31u32, 2169392u32);
    emu.sli_no_count(9usize, 9usize, 16u32, 2169396u32);
    emu.sli_no_count(8usize, 8usize, 24u32, 2169400u32);
    emu.orr_no_count(8usize, 8usize, 9usize, 2169404u32);
    emu.lbu_no_count(9usize, 11usize, 29u32, 2169408u32);
    emu.lbu_no_count(11usize, 11usize, 28u32, 2169412u32);
    emu.sli_no_count(18usize, 18usize, 8u32, 2169416u32);
    emu.orr_no_count(18usize, 18usize, 19usize, 2169420u32);
    emu.sli_no_count(9usize, 9usize, 16u32, 2169424u32);
    emu.sli_no_count(11usize, 11usize, 24u32, 2169428u32);
    emu.orr_no_count(11usize, 11usize, 9usize, 2169432u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2169436u32);
    emu.orr_no_count(13usize, 16usize, 14usize, 2169440u32);
    emu.orr_no_count(14usize, 7usize, 17usize, 2169444u32);
    emu.orr_no_count(16usize, 31usize, 28usize, 2169448u32);
    emu.orr_no_count(15usize, 15usize, 6usize, 2169452u32);
    emu.orr_no_count(17usize, 29usize, 5usize, 2169456u32);
    emu.orr_no_count(5usize, 8usize, 30usize, 2169460u32);
    emu.orr_no_count(11usize, 11usize, 18usize, 2169464u32);
    emu.sw_no_count(11usize, 10usize, 0u32, 2169468u32)?;
    emu.sw_no_count(5usize, 10usize, 4u32, 2169472u32)?;
    emu.sw_no_count(17usize, 10usize, 8u32, 2169476u32)?;
    emu.sw_no_count(15usize, 10usize, 12u32, 2169480u32)?;
    emu.sw_no_count(16usize, 10usize, 16u32, 2169484u32)?;
    emu.sw_no_count(14usize, 10usize, 20u32, 2169488u32)?;
    emu.sw_no_count(13usize, 10usize, 24u32, 2169492u32)?;
    emu.sw_no_count(12usize, 10usize, 28u32, 2169496u32)?;
    emu.lw_no_count(8usize, 2usize, 12u32, 2169500u32)?;
    emu.lw_no_count(9usize, 2usize, 8u32, 2169504u32)?;
    emu.lw_no_count(18usize, 2usize, 4u32, 2169508u32)?;
    emu.lw_no_count(19usize, 2usize, 0u32, 2169512u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2169516u32);
    emu.add_memory_rw_events(99usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2169520u32;
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
pub fn block_0x00211ab0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2169524u32)?;
    emu.lw_no_count(12usize, 10usize, 4u32, 2169528u32)?;
    emu.lw_no_count(10usize, 10usize, 0u32, 2169532u32)?;
    emu.lw_no_count(6usize, 12usize, 16u32, 2169536u32)?;
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2169540u32;
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
pub fn block_0x00211ac4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2169544u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2169548u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2169552u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2169556u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2169560u32)?;
    emu.adi_no_count(8usize, 11usize, 0u32, 2169564u32);
    emu.adi_no_count(18usize, 10usize, 0u32, 2169568u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2169572u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1588u32, 2169576u32);
    emu.adi_no_count(12usize, 0usize, 27u32, 2169580u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2169584u32);
    emu.apc_no_count(1usize, 2169584u32, 49152u32, 2169588u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2169592u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965752u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00211af8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 0usize, 1u32, 2169596u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2169724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211b7c));
    } else {
        emu.pc = 2169600u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211b00));
    }
}
#[inline(always)]
pub fn block_0x00211b00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 0u32, 2169604u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2169752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211b98));
    } else {
        emu.pc = 2169608u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211b08));
    }
}
#[inline]
pub fn block_0x00211b08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 2usize, 12u32, 2169612u32)?;
    emu.adi_no_count(10usize, 2usize, 12u32, 2169616u32);
    let a = 0u32.wrapping_add(2170880u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2169620u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965936u32, 2169624u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2169628u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1624u32, 2169632u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2169636u32);
    emu.sw_no_count(0usize, 2usize, 32u32, 2169640u32)?;
    emu.adi_no_count(14usize, 2usize, 40u32, 2169644u32);
    emu.adi_no_count(9usize, 0usize, 1u32, 2169648u32);
    emu.sw_no_count(10usize, 2usize, 40u32, 2169652u32)?;
    emu.sw_no_count(11usize, 2usize, 44u32, 2169656u32)?;
    emu.lw_no_count(10usize, 8usize, 0u32, 2169660u32)?;
    emu.lw_no_count(11usize, 8usize, 4u32, 2169664u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2169668u32)?;
    emu.sw_no_count(13usize, 2usize, 20u32, 2169672u32)?;
    emu.sw_no_count(14usize, 2usize, 24u32, 2169676u32)?;
    emu.sw_no_count(9usize, 2usize, 28u32, 2169680u32)?;
    emu.adi_no_count(12usize, 2usize, 16u32, 2169684u32);
    emu.apc_no_count(1usize, 2169684u32, 45056u32, 2169688u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2169692u32;
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
pub fn block_0x00211b5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2169724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211b7c));
    } else {
        emu.pc = 2169696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211b60));
    }
}
#[inline(always)]
pub fn block_0x00211b60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2169700u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1640u32, 2169704u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2169708u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2169712u32);
    emu.apc_no_count(1usize, 2169712u32, 49152u32, 2169716u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2169720u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965624u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00211b78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 10usize, 0u32, 2169724u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2169724u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211b7c));
}
#[inline(always)]
pub fn block_0x00211b7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 9usize, 0u32, 2169728u32);
    emu.lw_no_count(1usize, 2usize, 60u32, 2169732u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2169736u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2169740u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2169744u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2169748u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2169752u32;
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
pub fn block_0x00211b98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2169756u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965808u32, 2169760u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2169764u32);
    emu.adi_no_count(10usize, 8usize, 0u32, 2169768u32);
    emu.apc_no_count(1usize, 2169768u32, 49152u32, 2169772u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2169776u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965568u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00211bb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2169696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211b60));
    } else {
        emu.pc = 2169780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00211bb4));
    }
}
#[inline(always)]
pub fn block_0x00211bb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2169784u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2169724u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211b7c));
}
#[inline(always)]
pub fn block_0x00211bb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2169788u32)?;
    emu.apc_no_count(6usize, 2169788u32, 0u32, 2169792u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2169796u32;
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
pub fn block_0x00211bc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2169800u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2169804u32)?;
    emu.lbu_no_count(12usize, 10usize, 0u32, 2169808u32);
    emu.sli_no_count(12usize, 12usize, 2u32, 2169812u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2169816u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1644u32, 2169820u32);
    emu.adr_no_count(12usize, 12usize, 13usize, 2169824u32);
    emu.lw_no_count(12usize, 12usize, 0u32, 2169828u32)?;
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(12usize);
    let return_addr = 2169832u32;
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
pub fn block_0x00211be8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2169836u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1829u32, 2169840u32);
    emu.adi_no_count(12usize, 0usize, 7u32, 2169844u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2169848u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2169852u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2169856u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2169860u32);
    emu.apc_no_count(6usize, 2169860u32, 49152u32, 2169864u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2169868u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965476u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00211c0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2169872u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1969u32, 2169876u32);
    emu.adi_no_count(12usize, 0usize, 7u32, 2169880u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2169884u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2169888u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2169892u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2169896u32);
    emu.apc_no_count(6usize, 2169896u32, 49152u32, 2169900u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2169904u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965440u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00211c30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2169908u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1946u32, 2169912u32);
    emu.adi_no_count(12usize, 0usize, 14u32, 2169916u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2169920u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2169924u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2169928u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2169932u32);
    emu.apc_no_count(6usize, 2169932u32, 49152u32, 2169936u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2169940u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965404u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00211c54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2169944u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1905u32, 2169948u32);
    emu.adi_no_count(12usize, 0usize, 13u32, 2169952u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2169956u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2169960u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2169964u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2169968u32);
    emu.apc_no_count(6usize, 2169968u32, 49152u32, 2169972u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2169976u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965368u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00211c78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2169980u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294965768u32, 2169984u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2169988u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2169992u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2169996u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2170000u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2170004u32);
    emu.apc_no_count(6usize, 2170004u32, 49152u32, 2170008u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2170012u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965332u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00211c9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2170016u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1918u32, 2170020u32);
    emu.adi_no_count(12usize, 0usize, 15u32, 2170024u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2170028u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2170032u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2170036u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2170040u32);
    emu.apc_no_count(6usize, 2170040u32, 49152u32, 2170044u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2170048u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965296u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00211cc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2170052u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294965828u32, 2170056u32);
    emu.adi_no_count(12usize, 0usize, 8u32, 2170060u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2170064u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2170068u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2170072u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2170076u32);
    emu.apc_no_count(6usize, 2170076u32, 49152u32, 2170080u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2170084u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965260u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00211ce4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2170088u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1843u32, 2170092u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2170096u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2170100u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2170104u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2170108u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2170112u32);
    emu.apc_no_count(6usize, 2170112u32, 45056u32, 2170116u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2170120u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(2024u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00211d08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2170124u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1960u32, 2170128u32);
    emu.adi_no_count(12usize, 0usize, 9u32, 2170132u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2170136u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2170140u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2170144u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2170148u32);
    emu.apc_no_count(6usize, 2170148u32, 45056u32, 2170152u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2170156u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1988u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00211d2c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2170160u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1853u32, 2170164u32);
    emu.adi_no_count(12usize, 0usize, 12u32, 2170168u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2170172u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2170176u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2170180u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2170184u32);
    emu.apc_no_count(6usize, 2170184u32, 45056u32, 2170188u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2170192u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1952u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00211d50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2170196u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1882u32, 2170200u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2170204u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2170208u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2170212u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2170216u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2170220u32);
    emu.apc_no_count(6usize, 2170220u32, 45056u32, 2170224u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2170228u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1916u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00211d74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2170232u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1836u32, 2170236u32);
    emu.adi_no_count(12usize, 0usize, 7u32, 2170240u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2170244u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2170248u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2170252u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2170256u32);
    emu.apc_no_count(6usize, 2170256u32, 45056u32, 2170260u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2170264u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1880u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00211d98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2170268u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1976u32, 2170272u32);
    emu.adi_no_count(12usize, 0usize, 15u32, 2170276u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2170280u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2170284u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2170288u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2170292u32);
    emu.apc_no_count(6usize, 2170292u32, 45056u32, 2170296u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2170300u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1844u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00211dbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(12usize, 10usize, 1u32, 2170304u32);
    emu.lbu_no_count(10usize, 10usize, 2u32, 2170308u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2170312u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 2036u32, 2170316u32);
    emu.adi_no_count(14usize, 2usize, 3u32, 2170320u32);
    let a = 0u32.wrapping_add(2170880u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2170324u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 324u32, 2170328u32);
    let a = 0u32.wrapping_add(2170880u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2170332u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 292u32, 2170336u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2170340u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 17usize, 4294965268u32, 2170344u32);
    emu.add_memory_rw_events(12usize);
    let return_addr = 2170348u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2170656u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211f20));
}
#[inline]
pub fn block_0x00211dec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2170352u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1865u32, 2170356u32);
    emu.adi_no_count(12usize, 0usize, 17u32, 2170360u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2170364u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2170368u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2170372u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2170376u32);
    emu.apc_no_count(6usize, 2170376u32, 45056u32, 2170380u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2170384u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1760u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00211e10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(12usize, 10usize, 1u32, 2170388u32);
    emu.lbu_no_count(10usize, 10usize, 2u32, 2170392u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2170396u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 2036u32, 2170400u32);
    emu.adi_no_count(14usize, 2usize, 3u32, 2170404u32);
    let a = 0u32.wrapping_add(2170880u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2170408u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 324u32, 2170412u32);
    let a = 0u32.wrapping_add(2170880u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2170416u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 292u32, 2170420u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2170424u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 17usize, 4294965312u32, 2170428u32);
    emu.add_memory_rw_events(12usize);
    let return_addr = 2170432u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2170656u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211f20));
}
#[inline]
pub fn block_0x00211e40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2170436u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1933u32, 2170440u32);
    emu.adi_no_count(12usize, 0usize, 13u32, 2170444u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2170448u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2170452u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2170456u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2170460u32);
    emu.apc_no_count(6usize, 2170460u32, 45056u32, 2170464u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2170468u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1676u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00211e64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2170472u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294965824u32, 2170476u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2170480u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2170484u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2170488u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2170492u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2170496u32);
    emu.apc_no_count(6usize, 2170496u32, 45056u32, 2170500u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2170504u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1640u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00211e88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2170508u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1991u32, 2170512u32);
    emu.adi_no_count(12usize, 0usize, 13u32, 2170516u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2170520u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2170524u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2170528u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2170532u32);
    emu.apc_no_count(6usize, 2170532u32, 45056u32, 2170536u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2170540u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1604u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00211eac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2170544u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1902u32, 2170548u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2170552u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2170556u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2170560u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2170564u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2170568u32);
    emu.apc_no_count(6usize, 2170568u32, 45056u32, 2170572u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2170576u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1568u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00211ed0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2170580u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 1892u32, 2170584u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2170588u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2170592u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2170596u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2170600u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2170604u32);
    emu.apc_no_count(6usize, 2170604u32, 45056u32, 2170608u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2170612u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1532u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00211ef4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(12usize, 10usize, 1u32, 2170616u32);
    emu.lbu_no_count(10usize, 10usize, 2u32, 2170620u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2170624u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 2036u32, 2170628u32);
    emu.adi_no_count(14usize, 2usize, 3u32, 2170632u32);
    let a = 0u32.wrapping_add(2170880u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2170636u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 324u32, 2170640u32);
    let a = 0u32.wrapping_add(2170880u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2170644u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 292u32, 2170648u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2170652u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 17usize, 4294965348u32, 2170656u32);
    emu.add_memory_rw_events(11usize);
    emu.pc = 2170656u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00211f20));
}
#[inline]
pub fn block_0x00211f20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(5usize, 0usize, 3u32, 2170660u32);
    emu.sw_no_count(0usize, 2usize, 20u32, 2170664u32)?;
    emu.sb_no_count(12usize, 2usize, 3u32, 2170668u32);
    emu.adi_no_count(12usize, 2usize, 28u32, 2170672u32);
    emu.sli_no_count(10usize, 10usize, 3u32, 2170676u32);
    emu.adr_no_count(10usize, 13usize, 10usize, 2170680u32);
    emu.adi_no_count(13usize, 0usize, 2u32, 2170684u32);
    emu.sw_no_count(14usize, 2usize, 28u32, 2170688u32)?;
    emu.sw_no_count(15usize, 2usize, 32u32, 2170692u32)?;
    emu.sw_no_count(10usize, 2usize, 36u32, 2170696u32)?;
    emu.sw_no_count(16usize, 2usize, 40u32, 2170700u32)?;
    emu.lw_no_count(10usize, 11usize, 0u32, 2170704u32)?;
    emu.lw_no_count(11usize, 11usize, 4u32, 2170708u32)?;
    emu.sw_no_count(17usize, 2usize, 4u32, 2170712u32)?;
    emu.sw_no_count(5usize, 2usize, 8u32, 2170716u32)?;
    emu.sw_no_count(12usize, 2usize, 12u32, 2170720u32)?;
    emu.sw_no_count(13usize, 2usize, 16u32, 2170724u32)?;
    emu.adi_no_count(12usize, 2usize, 4u32, 2170728u32);
    emu.apc_no_count(1usize, 2170728u32, 45056u32, 2170732u32);
    emu.add_memory_rw_events(20usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2170736u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966396u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00211f70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 44u32, 2170740u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2170744u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2170748u32;
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
pub fn block_0x00211f7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2170752u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 2004u32, 2170756u32);
    emu.adi_no_count(12usize, 0usize, 9u32, 2170760u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2170764u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2170768u32);
    emu.lw_no_count(1usize, 2usize, 44u32, 2170772u32)?;
    emu.adi_no_count(2usize, 2usize, 48u32, 2170776u32);
    emu.apc_no_count(6usize, 2170776u32, 45056u32, 2170780u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2170784u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1360u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00211fa0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2170788u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2170792u32)?;
    emu.lbu_no_count(12usize, 10usize, 1u32, 2170796u32);
    emu.lbu_no_count(13usize, 10usize, 2u32, 2170800u32);
    emu.lbu_no_count(14usize, 10usize, 0u32, 2170804u32);
    emu.sli_no_count(12usize, 12usize, 8u32, 2170808u32);
    emu.sli_no_count(13usize, 13usize, 16u32, 2170812u32);
    emu.orr_no_count(12usize, 12usize, 14usize, 2170816u32);
    emu.orr_no_count(12usize, 12usize, 13usize, 2170820u32);
    emu.ani_no_count(13usize, 12usize, 255u32, 2170824u32);
    emu.sli_no_count(13usize, 13usize, 2u32, 2170828u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2170832u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1736u32, 2170836u32);
    emu.adr_no_count(13usize, 13usize, 14usize, 2170840u32);
    emu.lw_no_count(14usize, 13usize, 0u32, 2170844u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2170848u32)?;
    emu.adi_no_count(13usize, 0usize, 64u32, 2170852u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2170856u32);
    emu.add_memory_rw_events(19usize);
    let base = emu.read_reg_b_tracked(14usize);
    let return_addr = 2170860u32;
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
pub fn block_0x00211fec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 2u32, 2170864u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2170868u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2171044u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002120a4));
}
#[inline(always)]
pub fn block_0x00211ff4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 23u32, 2170872u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2170876u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2171044u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002120a4));
}
#[inline(always)]
pub fn block_0x00211ffc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 21u32, 2170880u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2170884u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2171044u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002120a4));
}
#[inline(always)]
pub fn block_0x00212004(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 18u32, 2170888u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2170892u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2171044u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002120a4));
}
#[inline(always)]
pub fn block_0x0021200c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 5u32, 2170896u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2170900u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2171044u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002120a4));
}
#[inline(always)]
pub fn block_0x00212014(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 19u32, 2170904u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2170908u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2171044u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002120a4));
}
#[inline(always)]
pub fn block_0x0021201c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 48u32, 2170912u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2170916u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2171044u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002120a4));
}
#[inline(always)]
pub fn block_0x00212024(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 3u32, 2170920u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2170924u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2171044u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002120a4));
}
#[inline(always)]
pub fn block_0x0021202c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 22u32, 2170928u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2170932u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2171044u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002120a4));
}
#[inline(always)]
pub fn block_0x00212034(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2170936u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2170940u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2171044u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002120a4));
}
#[inline(always)]
pub fn block_0x0021203c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 10u32, 2170944u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2170948u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2171044u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002120a4));
}
#[inline(always)]
pub fn block_0x00212044(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 24u32, 2170952u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2170956u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2171044u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002120a4));
}
#[inline(always)]
pub fn block_0x0021204c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 6u32, 2170960u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2170964u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2171044u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002120a4));
}
#[inline(always)]
pub fn block_0x00212054(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 128u32, 2170968u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2170972u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2171016u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212088));
}
#[inline(always)]
pub fn block_0x0021205c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 20u32, 2170976u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2170980u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2171044u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002120a4));
}
#[inline(always)]
pub fn block_0x00212064(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 9u32, 2170984u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2170988u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2171044u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002120a4));
}
#[inline(always)]
pub fn block_0x0021206c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 26u32, 2170992u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2170996u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2171044u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002120a4));
}
#[inline(always)]
pub fn block_0x00212074(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 49u32, 2171000u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2171004u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2171044u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002120a4));
}
#[inline(always)]
pub fn block_0x0021207c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 12u32, 2171008u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2171012u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2171044u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002120a4));
}
#[inline(always)]
pub fn block_0x00212084(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 0usize, 192u32, 2171016u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2171016u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00212088));
}
#[inline(always)]
pub fn block_0x00212088(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 12usize, 8u32, 2171020u32);
    emu.sri_no_count(12usize, 12usize, 11u32, 2171024u32);
    emu.ani_no_count(12usize, 12usize, 32u32, 2171028u32);
    emu.orr_no_count(10usize, 10usize, 13usize, 2171032u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2171036u32);
    emu.add_memory_rw_events(6usize);
    let return_addr = 2171040u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2171044u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002120a4));
}
#[inline(always)]
pub fn block_0x002120a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 30u32, 2171044u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2171044u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002120a4));
}
#[inline(never)]
pub fn block_0x002120a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 29u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(10usize, 2usize, 43u32, 2171048u32);
    emu.adi_no_count(10usize, 2usize, 43u32, 2171052u32);
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2171056u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965716u32, 2171060u32);
    emu.adi_no_count(13usize, 2usize, 12u32, 2171064u32);
    let a = 0u32.wrapping_add(2170880u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2171068u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966200u32, 2171072u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2171076u32;
    emu.update_insn_clock();
    emu.adi_no_count(15usize, 15usize, 4294965380u32, 2171080u32);
    emu.adi_no_count(16usize, 0usize, 3u32, 2171084u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2171088u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 17usize, 4294965404u32, 2171092u32);
    emu.adi_no_count(5usize, 0usize, 2u32, 2171096u32);
    emu.adi_no_count(6usize, 2usize, 44u32, 2171100u32);
    emu.sw_no_count(10usize, 2usize, 44u32, 2171104u32)?;
    emu.sw_no_count(12usize, 2usize, 48u32, 2171108u32)?;
    emu.sw_no_count(13usize, 2usize, 52u32, 2171112u32)?;
    emu.sw_no_count(14usize, 2usize, 56u32, 2171116u32)?;
    emu.lw_no_count(10usize, 11usize, 0u32, 2171120u32)?;
    emu.lw_no_count(11usize, 11usize, 4u32, 2171124u32)?;
    emu.sw_no_count(17usize, 2usize, 32u32, 2171128u32)?;
    emu.sw_no_count(5usize, 2usize, 36u32, 2171132u32)?;
    emu.sw_no_count(15usize, 2usize, 16u32, 2171136u32)?;
    emu.sw_no_count(16usize, 2usize, 20u32, 2171140u32)?;
    emu.sw_no_count(6usize, 2usize, 24u32, 2171144u32)?;
    emu.sw_no_count(5usize, 2usize, 28u32, 2171148u32)?;
    emu.adi_no_count(12usize, 2usize, 16u32, 2171152u32);
    emu.apc_no_count(1usize, 2171152u32, 45056u32, 2171156u32);
    emu.add_memory_rw_events(29usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2171160u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965972u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00212118(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 60u32, 2171164u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2171168u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2171172u32;
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
pub fn block_0x00212124(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2171176u32)?;
    emu.lw_no_count(13usize, 10usize, 4u32, 2171180u32)?;
    emu.adi_no_count(14usize, 11usize, 0u32, 2171184u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2171188u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2171192u32);
    emu.adi_no_count(12usize, 14usize, 0u32, 2171196u32);
    emu.apc_no_count(6usize, 2171196u32, 49152u32, 2171200u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2171204u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966444u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00212144(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967248u32, 2171208u32);
    emu.sw_no_count(1usize, 2usize, 44u32, 2171212u32)?;
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2171216u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966312u32, 2171220u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2171224u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294965860u32, 2171228u32);
    emu.adi_no_count(14usize, 0usize, 1u32, 2171232u32);
    emu.sw_no_count(0usize, 2usize, 28u32, 2171236u32)?;
    emu.adi_no_count(15usize, 2usize, 36u32, 2171240u32);
    emu.sw_no_count(10usize, 2usize, 36u32, 2171244u32)?;
    emu.sw_no_count(12usize, 2usize, 40u32, 2171248u32)?;
    emu.lw_no_count(10usize, 11usize, 0u32, 2171252u32)?;
    emu.lw_no_count(11usize, 11usize, 4u32, 2171256u32)?;
    emu.sw_no_count(13usize, 2usize, 12u32, 2171260u32)?;
    emu.sw_no_count(14usize, 2usize, 16u32, 2171264u32)?;
    emu.sw_no_count(15usize, 2usize, 20u32, 2171268u32)?;
    emu.sw_no_count(14usize, 2usize, 24u32, 2171272u32)?;
    emu.adi_no_count(12usize, 2usize, 12u32, 2171276u32);
    emu.apc_no_count(1usize, 2171276u32, 45056u32, 2171280u32);
    emu.add_memory_rw_events(20usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2171284u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965848u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
