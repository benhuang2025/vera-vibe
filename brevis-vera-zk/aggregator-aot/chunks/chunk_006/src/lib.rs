pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2121472u32;
pub const PC_MAX: u32 = 2126348u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 132usize] = [
        block_0x00205f00,
        block_0x00205f3c,
        block_0x00205f4c,
        block_0x00205f60,
        block_0x00205f74,
        block_0x00205fdc,
        block_0x00205fe8,
        block_0x00205ff4,
        block_0x00206000,
        block_0x0020602c,
        block_0x00206054,
        block_0x00206060,
        block_0x0020607c,
        block_0x002060b0,
        block_0x002060bc,
        block_0x002060d8,
        block_0x002060f4,
        block_0x00206110,
        block_0x0020612c,
        block_0x00206148,
        block_0x00206164,
        block_0x00206180,
        block_0x002061b8,
        block_0x002061c4,
        block_0x002061d0,
        block_0x002061f0,
        block_0x0020620c,
        block_0x00206240,
        block_0x0020624c,
        block_0x00206278,
        block_0x00206294,
        block_0x002062b0,
        block_0x002062d8,
        block_0x002062e4,
        block_0x00206300,
        block_0x00206328,
        block_0x00206334,
        block_0x00206350,
        block_0x00206384,
        block_0x00206390,
        block_0x002063ac,
        block_0x002063c8,
        block_0x00206410,
        block_0x00206444,
        block_0x00206450,
        block_0x0020646c,
        block_0x00206488,
        block_0x002064bc,
        block_0x002064c8,
        block_0x002064e4,
        block_0x0020652c,
        block_0x00206548,
        block_0x0020657c,
        block_0x00206588,
        block_0x002065a4,
        block_0x002065c0,
        block_0x002065dc,
        block_0x00206604,
        block_0x00206610,
        block_0x00206654,
        block_0x00206668,
        block_0x00206674,
        block_0x002066a8,
        block_0x002066b4,
        block_0x00206864,
        block_0x00206880,
        block_0x00206894,
        block_0x002068ac,
        block_0x002068c4,
        block_0x002068d8,
        block_0x002068e4,
        block_0x002068fc,
        block_0x00206910,
        block_0x00206924,
        block_0x0020693c,
        block_0x00206948,
        block_0x00206974,
        block_0x0020697c,
        block_0x00206afc,
        block_0x00206b1c,
        block_0x00206b30,
        block_0x00206b48,
        block_0x00206b60,
        block_0x00206b74,
        block_0x00206b80,
        block_0x00206b90,
        block_0x00206ba8,
        block_0x00206bbc,
        block_0x00206bd0,
        block_0x00206be8,
        block_0x00206bf4,
        block_0x00206c24,
        block_0x00206c30,
        block_0x00206c68,
        block_0x00206c70,
        block_0x00206ca0,
        block_0x00206ca8,
        block_0x00206cac,
        block_0x00206cb8,
        block_0x00206cf4,
        block_0x00206d50,
        block_0x00206d84,
        block_0x00206df4,
        block_0x00206e04,
        block_0x00206e14,
        block_0x00206e84,
        block_0x00206e94,
        block_0x00206ea0,
        block_0x00206f30,
        block_0x00206f40,
        block_0x00206f94,
        block_0x00206fa8,
        block_0x00206fc0,
        block_0x00206fd4,
        block_0x00206fe8,
        block_0x00206ffc,
        block_0x00207010,
        block_0x00207020,
        block_0x00207058,
        block_0x002070ac,
        block_0x00207100,
        block_0x0020711c,
        block_0x00207138,
        block_0x00207150,
        block_0x00207168,
        block_0x00207194,
        block_0x002071a4,
        block_0x002071b4,
        block_0x002071bc,
        block_0x002071c4,
        block_0x002071e4,
        block_0x0020720c,
    ];
    const IDX: [u16; 1220usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 2u16, 0u16, 0u16, 0u16, 3u16, 0u16, 0u16, 0u16, 0u16, 4u16, 0u16,
        0u16, 0u16, 0u16, 5u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 6u16, 0u16, 0u16, 7u16, 0u16, 0u16, 8u16, 0u16, 0u16, 9u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 10u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 11u16, 0u16, 0u16, 12u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 13u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 14u16, 0u16, 0u16, 15u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 16u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 17u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 18u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 19u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 20u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 21u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 22u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 23u16, 0u16, 0u16, 24u16, 0u16, 0u16, 25u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 26u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        27u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        28u16, 0u16, 0u16, 29u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 30u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 31u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 32u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 33u16,
        0u16, 0u16, 34u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 35u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 36u16, 0u16, 0u16, 37u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 38u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 39u16, 0u16, 0u16, 40u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        41u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 42u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 43u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 44u16,
        0u16, 0u16, 45u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 46u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 47u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 48u16, 0u16, 0u16, 49u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        50u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 51u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 52u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 53u16,
        0u16, 0u16, 54u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 55u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 56u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 57u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 58u16, 0u16, 0u16, 59u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 60u16, 0u16, 0u16, 0u16, 0u16, 61u16, 0u16, 0u16, 62u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 63u16, 0u16, 0u16, 64u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 65u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 66u16, 0u16, 0u16,
        0u16, 0u16, 67u16, 0u16, 0u16, 0u16, 0u16, 0u16, 68u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 69u16, 0u16, 0u16, 0u16, 0u16, 70u16, 0u16, 0u16, 71u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 72u16, 0u16, 0u16, 0u16, 0u16, 73u16, 0u16, 0u16, 0u16, 0u16, 74u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 75u16, 0u16, 0u16, 76u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 77u16, 0u16, 78u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        79u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 80u16, 0u16, 0u16, 0u16, 0u16,
        81u16, 0u16, 0u16, 0u16, 0u16, 0u16, 82u16, 0u16, 0u16, 0u16, 0u16, 0u16, 83u16,
        0u16, 0u16, 0u16, 0u16, 84u16, 0u16, 0u16, 85u16, 0u16, 0u16, 0u16, 86u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 87u16, 0u16, 0u16, 0u16, 0u16, 88u16, 0u16, 0u16, 0u16,
        0u16, 89u16, 0u16, 0u16, 0u16, 0u16, 0u16, 90u16, 0u16, 0u16, 91u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 92u16, 0u16, 0u16, 93u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        94u16, 0u16, 95u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 96u16, 0u16, 97u16, 98u16, 0u16, 0u16, 99u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 100u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 101u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 102u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 103u16, 0u16, 0u16, 0u16, 104u16,
        0u16, 0u16, 0u16, 105u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 106u16, 0u16, 0u16, 0u16, 107u16, 0u16, 0u16,
        108u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 109u16, 0u16, 0u16,
        0u16, 110u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 111u16, 0u16, 0u16, 0u16,
        0u16, 112u16, 0u16, 0u16, 0u16, 0u16, 0u16, 113u16, 0u16, 0u16, 0u16, 0u16,
        114u16, 0u16, 0u16, 0u16, 0u16, 115u16, 0u16, 0u16, 0u16, 0u16, 116u16, 0u16,
        0u16, 0u16, 0u16, 117u16, 0u16, 0u16, 0u16, 118u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 119u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 120u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 121u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 122u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 123u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 124u16, 0u16, 0u16, 0u16, 0u16, 0u16, 125u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 126u16, 0u16, 0u16, 0u16,
        127u16, 0u16, 0u16, 0u16, 128u16, 0u16, 129u16, 0u16, 130u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 131u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 132u16,
    ];
    if pc < 2121472u32 || pc > 2126348u32 {
        return None;
    }
    let word_offset = ((pc - 2121472u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline]
pub fn block_0x00205f00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2121476u32)?;
    emu.lbu_no_count(10usize, 10usize, 0u32, 2121480u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2121484u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 1868u32, 2121488u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2121492u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 2036u32, 2121496u32);
    emu.sli_no_count(10usize, 10usize, 2u32, 2121500u32);
    emu.adr_no_count(12usize, 12usize, 10usize, 2121504u32);
    emu.adr_no_count(10usize, 13usize, 10usize, 2121508u32);
    emu.lw_no_count(12usize, 12usize, 0u32, 2121512u32)?;
    emu.lw_no_count(13usize, 10usize, 0u32, 2121516u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2121520u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2121524u32);
    emu.apc_no_count(6usize, 2121524u32, 98304u32, 2121528u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2121532u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966060u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205f3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2121536u32)?;
    emu.lbu_no_count(12usize, 10usize, 0u32, 2121540u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2121544u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a != b {
        emu.pc = 2121568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205f60));
    } else {
        emu.pc = 2121548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00205f4c));
    }
}
#[inline(always)]
pub fn block_0x00205f4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 15u32, 2121552u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2121556u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1029u32, 2121560u32);
    emu.apc_no_count(6usize, 2121560u32, 98304u32, 2121564u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2121568u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966024u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205f60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 13u32, 2121572u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2121576u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1044u32, 2121580u32);
    emu.apc_no_count(6usize, 2121580u32, 98304u32, 2121584u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2121588u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966004u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00205f74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 26u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2121592u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2121596u32)?;
    emu.adi_no_count(5usize, 11usize, 0u32, 2121600u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2121604u32)?;
    emu.adi_no_count(15usize, 10usize, 8u32, 2121608u32);
    emu.sw_no_count(10usize, 2usize, 24u32, 2121612u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2121616u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 340u32, 2121620u32);
    emu.adi_no_count(6usize, 2usize, 24u32, 2121624u32);
    emu.adi_no_count(7usize, 0usize, 8u32, 2121628u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2121632u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 356u32, 2121636u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2121640u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294967056u32, 2121644u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2121648u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 324u32, 2121652u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2121656u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 17usize, 420u32, 2121660u32);
    emu.adi_no_count(12usize, 0usize, 5u32, 2121664u32);
    emu.adi_no_count(14usize, 0usize, 4u32, 2121668u32);
    emu.sw_no_count(7usize, 2usize, 0u32, 2121672u32)?;
    emu.sw_no_count(6usize, 2usize, 4u32, 2121676u32)?;
    emu.sw_no_count(10usize, 2usize, 8u32, 2121680u32)?;
    emu.adi_no_count(10usize, 5usize, 0u32, 2121684u32);
    emu.apc_no_count(1usize, 2121684u32, 98304u32, 2121688u32);
    emu.add_memory_rw_events(26usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121692u32;
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
pub fn block_0x00205fdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2121696u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2121700u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121704u32;
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
pub fn block_0x00205fe8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2121708u32)?;
    emu.apc_no_count(6usize, 2121708u32, 69632u32, 2121712u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2121716u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(500u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00205ff4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2121720u32)?;
    emu.apc_no_count(6usize, 2121720u32, 81920u32, 2121724u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2121728u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1020u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00206000(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2121732u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2121736u32)?;
    emu.lw_no_count(12usize, 10usize, 0u32, 2121740u32)?;
    emu.lbu_no_count(10usize, 12usize, 0u32, 2121744u32);
    emu.sli_no_count(10usize, 10usize, 2u32, 2121748u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2121752u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 48u32, 2121756u32);
    emu.adr_no_count(10usize, 10usize, 13usize, 2121760u32);
    emu.lw_no_count(13usize, 10usize, 0u32, 2121764u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2121768u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2121772u32;
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
pub fn block_0x0020602c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 1u32, 2121776u32);
    emu.sw_no_count(12usize, 2usize, 4u32, 2121780u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2121784u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1076u32, 2121788u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2121792u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1060u32, 2121796u32);
    emu.adi_no_count(12usize, 0usize, 6u32, 2121800u32);
    emu.adi_no_count(13usize, 2usize, 4u32, 2121804u32);
    emu.apc_no_count(1usize, 2121804u32, 98304u32, 2121808u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121812u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966532u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206054(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2121816u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2121820u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121824u32;
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
pub fn block_0x00206060(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2121828u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1099u32, 2121832u32);
    emu.adi_no_count(12usize, 0usize, 5u32, 2121836u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2121840u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2121844u32);
    emu.apc_no_count(6usize, 2121844u32, 98304u32, 2121848u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2121852u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965740u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020607c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 4u32, 2121856u32);
    emu.sw_no_count(12usize, 2usize, 8u32, 2121860u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2121864u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1172u32, 2121868u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2121872u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 436u32, 2121876u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2121880u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 1156u32, 2121884u32);
    emu.adi_no_count(12usize, 0usize, 19u32, 2121888u32);
    emu.adi_no_count(14usize, 0usize, 8u32, 2121892u32);
    emu.adi_no_count(15usize, 2usize, 8u32, 2121896u32);
    emu.apc_no_count(1usize, 2121896u32, 98304u32, 2121900u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121904u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965788u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002060b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2121908u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2121912u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2121916u32;
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
pub fn block_0x002060bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2121920u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 750u32, 2121924u32);
    emu.adi_no_count(12usize, 0usize, 16u32, 2121928u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2121932u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2121936u32);
    emu.apc_no_count(6usize, 2121936u32, 98304u32, 2121940u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2121944u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965648u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002060d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2121948u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 686u32, 2121952u32);
    emu.adi_no_count(12usize, 0usize, 16u32, 2121956u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2121960u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2121964u32);
    emu.apc_no_count(6usize, 2121964u32, 98304u32, 2121968u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2121972u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965620u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002060f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2121976u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1104u32, 2121980u32);
    emu.adi_no_count(12usize, 0usize, 24u32, 2121984u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2121988u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2121992u32);
    emu.apc_no_count(6usize, 2121992u32, 98304u32, 2121996u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2122000u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965592u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206110(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2122004u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1082u32, 2122008u32);
    emu.adi_no_count(12usize, 0usize, 17u32, 2122012u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2122016u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2122020u32);
    emu.apc_no_count(6usize, 2122020u32, 98304u32, 2122024u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2122028u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965564u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020612c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2122032u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 468u32, 2122036u32);
    emu.adi_no_count(12usize, 0usize, 6u32, 2122040u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2122044u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2122048u32);
    emu.apc_no_count(6usize, 2122048u32, 98304u32, 2122052u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2122056u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965536u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206148(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2122060u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 444u32, 2122064u32);
    emu.adi_no_count(12usize, 0usize, 8u32, 2122068u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2122072u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2122076u32);
    emu.apc_no_count(6usize, 2122076u32, 98304u32, 2122080u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2122084u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965508u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206164(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2122088u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1128u32, 2122092u32);
    emu.adi_no_count(12usize, 0usize, 25u32, 2122096u32);
    emu.lw_no_count(1usize, 2usize, 12u32, 2122100u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2122104u32);
    emu.apc_no_count(6usize, 2122104u32, 98304u32, 2122108u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2122112u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965480u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00206180(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2122116u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2122120u32)?;
    emu.adi_no_count(15usize, 11usize, 0u32, 2122124u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2122128u32)?;
    emu.sw_no_count(10usize, 2usize, 8u32, 2122132u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2122136u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 468u32, 2122140u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2122144u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 452u32, 2122148u32);
    emu.adi_no_count(12usize, 0usize, 6u32, 2122152u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2122156u32);
    emu.adi_no_count(10usize, 15usize, 0u32, 2122160u32);
    emu.apc_no_count(1usize, 2122160u32, 98304u32, 2122164u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2122168u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966176u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002061b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2122172u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2122176u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2122180u32;
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
pub fn block_0x002061c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 10usize, 0u32, 2122184u32)?;
    emu.apc_no_count(6usize, 2122184u32, 69632u32, 2122188u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2122192u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294966140u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002061d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(12usize, 10usize, 0u32, 2122196u32)?;
    emu.lw_no_count(13usize, 10usize, 4u32, 2122200u32)?;
    emu.adi_no_count(14usize, 11usize, 0u32, 2122204u32);
    emu.adi_no_count(10usize, 12usize, 0u32, 2122208u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2122212u32);
    emu.adi_no_count(12usize, 14usize, 0u32, 2122216u32);
    emu.apc_no_count(6usize, 2122216u32, 69632u32, 2122220u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2122224u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1984u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002061f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2122228u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 204u32, 2122232u32);
    emu.adi_no_count(12usize, 0usize, 21u32, 2122236u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2122240u32);
    emu.adi_no_count(11usize, 13usize, 0u32, 2122244u32);
    emu.apc_no_count(6usize, 2122244u32, 98304u32, 2122248u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2122252u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(4294965340u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020620c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2122256u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2122260u32)?;
    emu.adi_no_count(15usize, 11usize, 0u32, 2122264u32);
    emu.sw_no_count(10usize, 2usize, 8u32, 2122268u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2122272u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 468u32, 2122276u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2122280u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 452u32, 2122284u32);
    emu.adi_no_count(12usize, 0usize, 6u32, 2122288u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2122292u32);
    emu.adi_no_count(10usize, 15usize, 0u32, 2122296u32);
    emu.apc_no_count(1usize, 2122296u32, 98304u32, 2122300u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2122304u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966040u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206240(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2122308u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2122312u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2122316u32;
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
pub fn block_0x0020624c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967264u32, 2122320u32);
    emu.sw_no_count(1usize, 2usize, 28u32, 2122324u32)?;
    emu.adi_no_count(12usize, 10usize, 0u32, 2122328u32);
    emu.lbu_no_count(10usize, 10usize, 0u32, 2122332u32);
    emu.sli_no_count(10usize, 10usize, 2u32, 2122336u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2122340u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 88u32, 2122344u32);
    emu.adr_no_count(10usize, 10usize, 13usize, 2122348u32);
    emu.lw_no_count(13usize, 10usize, 0u32, 2122352u32)?;
    emu.adi_no_count(10usize, 11usize, 0u32, 2122356u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(13usize);
    let return_addr = 2122360u32;
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
pub fn block_0x00206278(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2122364u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 428u32, 2122368u32);
    emu.adi_no_count(12usize, 0usize, 8u32, 2122372u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2122376u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2122380u32);
    emu.apc_no_count(6usize, 2122380u32, 94208u32, 2122384u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2122388u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(2004u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206294(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2122392u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 895u32, 2122396u32);
    emu.adi_no_count(12usize, 0usize, 6u32, 2122400u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2122404u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2122408u32);
    emu.apc_no_count(6usize, 2122408u32, 94208u32, 2122412u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2122416u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1976u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002062b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 4u32, 2122420u32);
    emu.sw_no_count(12usize, 2usize, 24u32, 2122424u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2122428u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 892u32, 2122432u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2122436u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 876u32, 2122440u32);
    emu.adi_no_count(12usize, 0usize, 3u32, 2122444u32);
    emu.adi_no_count(13usize, 2usize, 24u32, 2122448u32);
    emu.apc_no_count(1usize, 2122448u32, 98304u32, 2122452u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2122456u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965888u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002062d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2122460u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2122464u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2122468u32;
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
pub fn block_0x002062e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2122472u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 853u32, 2122476u32);
    emu.adi_no_count(12usize, 0usize, 11u32, 2122480u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2122484u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2122488u32);
    emu.apc_no_count(6usize, 2122488u32, 94208u32, 2122492u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2122496u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1896u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00206300(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 1u32, 2122500u32);
    emu.sw_no_count(12usize, 2usize, 24u32, 2122504u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2122508u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 572u32, 2122512u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2122516u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 556u32, 2122520u32);
    emu.adi_no_count(12usize, 0usize, 2u32, 2122524u32);
    emu.adi_no_count(13usize, 2usize, 24u32, 2122528u32);
    emu.apc_no_count(1usize, 2122528u32, 98304u32, 2122532u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2122536u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965808u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206328(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2122540u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2122544u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2122548u32;
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
pub fn block_0x00206334(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2122552u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 372u32, 2122556u32);
    emu.adi_no_count(12usize, 0usize, 8u32, 2122560u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2122564u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2122568u32);
    emu.apc_no_count(6usize, 2122568u32, 94208u32, 2122572u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2122576u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1816u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00206350(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 1u32, 2122580u32);
    emu.sw_no_count(12usize, 2usize, 24u32, 2122584u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2122588u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 828u32, 2122592u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2122596u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 838u32, 2122600u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2122604u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 812u32, 2122608u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2122612u32);
    emu.adi_no_count(14usize, 0usize, 3u32, 2122616u32);
    emu.adi_no_count(15usize, 2usize, 24u32, 2122620u32);
    emu.apc_no_count(1usize, 2122620u32, 94208u32, 2122624u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2122628u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1864u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206384(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2122632u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2122636u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2122640u32;
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
pub fn block_0x00206390(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2122644u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 480u32, 2122648u32);
    emu.adi_no_count(12usize, 0usize, 12u32, 2122652u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2122656u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2122660u32);
    emu.apc_no_count(6usize, 2122660u32, 94208u32, 2122664u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2122668u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1724u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002063ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2122672u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 654u32, 2122676u32);
    emu.adi_no_count(12usize, 0usize, 16u32, 2122680u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2122684u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2122688u32);
    emu.apc_no_count(6usize, 2122688u32, 94208u32, 2122692u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2122696u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1696u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002063c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 12usize, 4u32, 2122700u32);
    emu.adi_no_count(12usize, 12usize, 8u32, 2122704u32);
    emu.sw_no_count(12usize, 2usize, 24u32, 2122708u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2122712u32;
    emu.update_insn_clock();
    emu.adi_no_count(5usize, 5usize, 508u32, 2122716u32);
    emu.adi_no_count(6usize, 2usize, 24u32, 2122720u32);
    emu.adi_no_count(7usize, 0usize, 10u32, 2122724u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2122728u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 524u32, 2122732u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2122736u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 534u32, 2122740u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2122744u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 492u32, 2122748u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2122752u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 17usize, 546u32, 2122756u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2122760u32);
    emu.adi_no_count(14usize, 0usize, 12u32, 2122764u32);
    emu.add_memory_rw_events(18usize);
    let return_addr = 2122768u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2123348u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00206654));
}
#[inline]
pub fn block_0x00206410(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 1u32, 2122772u32);
    emu.sw_no_count(12usize, 2usize, 24u32, 2122776u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2122780u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 787u32, 2122784u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2122788u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 784u32, 2122792u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2122796u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 768u32, 2122800u32);
    emu.adi_no_count(12usize, 0usize, 12u32, 2122804u32);
    emu.adi_no_count(14usize, 0usize, 3u32, 2122808u32);
    emu.adi_no_count(15usize, 2usize, 24u32, 2122812u32);
    emu.apc_no_count(1usize, 2122812u32, 94208u32, 2122816u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2122820u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1672u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206444(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2122824u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2122828u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2122832u32;
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
pub fn block_0x00206450(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2122836u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 474u32, 2122840u32);
    emu.adi_no_count(12usize, 0usize, 6u32, 2122844u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2122848u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2122852u32);
    emu.apc_no_count(6usize, 2122852u32, 94208u32, 2122856u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2122860u32;
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
#[inline(always)]
pub fn block_0x0020646c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2122864u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 901u32, 2122868u32);
    emu.adi_no_count(12usize, 0usize, 14u32, 2122872u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2122876u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2122880u32);
    emu.apc_no_count(6usize, 2122880u32, 94208u32, 2122884u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2122888u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1504u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00206488(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 1u32, 2122892u32);
    emu.sw_no_count(12usize, 2usize, 24u32, 2122896u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2122900u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 968u32, 2122904u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2122908u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294967060u32, 2122912u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2122916u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 952u32, 2122920u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2122924u32);
    emu.adi_no_count(14usize, 0usize, 4u32, 2122928u32);
    emu.adi_no_count(15usize, 2usize, 24u32, 2122932u32);
    emu.apc_no_count(1usize, 2122932u32, 94208u32, 2122936u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2122940u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1552u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002064bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2122944u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2122948u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2122952u32;
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
pub fn block_0x002064c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2122956u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 670u32, 2122960u32);
    emu.adi_no_count(12usize, 0usize, 16u32, 2122964u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2122968u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2122972u32);
    emu.apc_no_count(6usize, 2122972u32, 94208u32, 2122976u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2122980u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1412u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002064e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 12usize, 4u32, 2122984u32);
    emu.adi_no_count(12usize, 12usize, 8u32, 2122988u32);
    emu.sw_no_count(12usize, 2usize, 24u32, 2122992u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2122996u32;
    emu.update_insn_clock();
    emu.adi_no_count(5usize, 5usize, 508u32, 2123000u32);
    emu.adi_no_count(6usize, 2usize, 24u32, 2123004u32);
    emu.adi_no_count(7usize, 0usize, 9u32, 2123008u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2123012u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 978u32, 2123016u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2123020u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 990u32, 2123024u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2123028u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 492u32, 2123032u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2123036u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 17usize, 997u32, 2123040u32);
    emu.adi_no_count(12usize, 0usize, 12u32, 2123044u32);
    emu.adi_no_count(14usize, 0usize, 7u32, 2123048u32);
    emu.add_memory_rw_events(18usize);
    let return_addr = 2123052u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2123348u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00206654));
}
#[inline(always)]
pub fn block_0x0020652c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2123056u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 864u32, 2123060u32);
    emu.adi_no_count(12usize, 0usize, 10u32, 2123064u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2123068u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2123072u32);
    emu.apc_no_count(6usize, 2123072u32, 94208u32, 2123076u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2123080u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1312u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00206548(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 1u32, 2123084u32);
    emu.sw_no_count(12usize, 2usize, 24u32, 2123088u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2123092u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 468u32, 2123096u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2123100u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 784u32, 2123104u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2123108u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 768u32, 2123112u32);
    emu.adi_no_count(12usize, 0usize, 6u32, 2123116u32);
    emu.adi_no_count(14usize, 0usize, 3u32, 2123120u32);
    emu.adi_no_count(15usize, 2usize, 24u32, 2123124u32);
    emu.apc_no_count(1usize, 2123124u32, 94208u32, 2123128u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2123132u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1360u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020657c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2123136u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2123140u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2123144u32;
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
pub fn block_0x00206588(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2123148u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 622u32, 2123152u32);
    emu.adi_no_count(12usize, 0usize, 16u32, 2123156u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2123160u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2123164u32);
    emu.apc_no_count(6usize, 2123164u32, 94208u32, 2123168u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2123172u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x002065a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2123176u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 841u32, 2123180u32);
    emu.adi_no_count(12usize, 0usize, 12u32, 2123184u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2123188u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2123192u32);
    emu.apc_no_count(6usize, 2123192u32, 94208u32, 2123196u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2123200u32;
    emu.write_reg_no_count(0usize, return_addr);
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
pub fn block_0x002065c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2123204u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 799u32, 2123208u32);
    emu.adi_no_count(12usize, 0usize, 12u32, 2123212u32);
    emu.lw_no_count(1usize, 2usize, 28u32, 2123216u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2123220u32);
    emu.apc_no_count(6usize, 2123220u32, 94208u32, 2123224u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2123228u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(1164u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002065dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 4u32, 2123232u32);
    emu.sw_no_count(12usize, 2usize, 24u32, 2123236u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2123240u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967092u32, 2123244u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2123248u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 1008u32, 2123252u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2123256u32);
    emu.adi_no_count(13usize, 2usize, 24u32, 2123260u32);
    emu.apc_no_count(1usize, 2123260u32, 94208u32, 2123264u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2123268u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1876u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206604(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2123272u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2123276u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2123280u32;
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
pub fn block_0x00206610(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(15usize, 12usize, 1u32, 2123284u32);
    emu.adi_no_count(12usize, 12usize, 4u32, 2123288u32);
    emu.sw_no_count(12usize, 2usize, 24u32, 2123292u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(5usize, a);
    emu.pc = 2123296u32;
    emu.update_insn_clock();
    emu.adi_no_count(5usize, 5usize, 768u32, 2123300u32);
    emu.adi_no_count(6usize, 2usize, 24u32, 2123304u32);
    emu.adi_no_count(7usize, 0usize, 6u32, 2123308u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2123312u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 932u32, 2123316u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2123320u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 436u32, 2123324u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2123328u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 916u32, 2123332u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2123336u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 17usize, 945u32, 2123340u32);
    emu.adi_no_count(12usize, 0usize, 13u32, 2123344u32);
    emu.adi_no_count(14usize, 0usize, 8u32, 2123348u32);
    emu.add_memory_rw_events(17usize);
    emu.pc = 2123348u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00206654));
}
#[inline(always)]
pub fn block_0x00206654(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(7usize, 2usize, 0u32, 2123352u32)?;
    emu.sw_no_count(6usize, 2usize, 4u32, 2123356u32)?;
    emu.sw_no_count(5usize, 2usize, 8u32, 2123360u32)?;
    emu.apc_no_count(1usize, 2123360u32, 94208u32, 2123364u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2123368u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1360u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206668(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2123372u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2123376u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2123380u32;
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
pub fn block_0x00206674(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 12usize, 1u32, 2123384u32);
    emu.sw_no_count(12usize, 2usize, 24u32, 2123388u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2123392u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1024u32, 2123396u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2123400u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 784u32, 2123404u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2123408u32;
    emu.update_insn_clock();
    emu.adi_no_count(16usize, 16usize, 768u32, 2123412u32);
    emu.adi_no_count(12usize, 0usize, 5u32, 2123416u32);
    emu.adi_no_count(14usize, 0usize, 3u32, 2123420u32);
    emu.adi_no_count(15usize, 2usize, 24u32, 2123424u32);
    emu.apc_no_count(1usize, 2123424u32, 94208u32, 2123428u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2123432u32;
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
pub fn block_0x002066a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 28u32, 2123436u32)?;
    emu.adi_no_count(2usize, 2usize, 32u32, 2123440u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2123444u32;
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
pub fn block_0x002066b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 108u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967152u32, 2123448u32);
    emu.sw_no_count(1usize, 2usize, 140u32, 2123452u32)?;
    emu.sw_no_count(8usize, 2usize, 136u32, 2123456u32)?;
    emu.sw_no_count(9usize, 2usize, 132u32, 2123460u32)?;
    emu.sw_no_count(18usize, 2usize, 128u32, 2123464u32)?;
    emu.sw_no_count(19usize, 2usize, 124u32, 2123468u32)?;
    emu.sw_no_count(20usize, 2usize, 120u32, 2123472u32)?;
    emu.sw_no_count(21usize, 2usize, 116u32, 2123476u32)?;
    emu.sw_no_count(22usize, 2usize, 112u32, 2123480u32)?;
    emu.sw_no_count(23usize, 2usize, 108u32, 2123484u32)?;
    emu.sw_no_count(24usize, 2usize, 104u32, 2123488u32)?;
    emu.sw_no_count(25usize, 2usize, 100u32, 2123492u32)?;
    emu.sw_no_count(26usize, 2usize, 96u32, 2123496u32)?;
    emu.sw_no_count(27usize, 2usize, 92u32, 2123500u32)?;
    emu.adi_no_count(9usize, 12usize, 0u32, 2123504u32);
    emu.adi_no_count(24usize, 10usize, 0u32, 2123508u32);
    emu.lbu_no_count(10usize, 11usize, 28u32, 2123512u32);
    emu.lbu_no_count(12usize, 11usize, 29u32, 2123516u32);
    emu.lbu_no_count(13usize, 11usize, 30u32, 2123520u32);
    emu.lbu_no_count(14usize, 11usize, 31u32, 2123524u32);
    emu.lbu_no_count(15usize, 11usize, 24u32, 2123528u32);
    emu.lbu_no_count(16usize, 11usize, 25u32, 2123532u32);
    emu.lbu_no_count(17usize, 11usize, 26u32, 2123536u32);
    emu.lbu_no_count(5usize, 11usize, 27u32, 2123540u32);
    emu.lbu_no_count(6usize, 11usize, 20u32, 2123544u32);
    emu.lbu_no_count(7usize, 11usize, 21u32, 2123548u32);
    emu.lbu_no_count(28usize, 11usize, 22u32, 2123552u32);
    emu.lbu_no_count(29usize, 11usize, 23u32, 2123556u32);
    emu.lbu_no_count(30usize, 11usize, 16u32, 2123560u32);
    emu.lbu_no_count(31usize, 11usize, 17u32, 2123564u32);
    emu.lbu_no_count(8usize, 11usize, 18u32, 2123568u32);
    emu.lbu_no_count(18usize, 11usize, 19u32, 2123572u32);
    emu.sli_no_count(12usize, 12usize, 8u32, 2123576u32);
    emu.sli_no_count(13usize, 13usize, 16u32, 2123580u32);
    emu.sli_no_count(14usize, 14usize, 24u32, 2123584u32);
    emu.sli_no_count(16usize, 16usize, 8u32, 2123588u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2123592u32);
    emu.orr_no_count(12usize, 14usize, 13usize, 2123596u32);
    emu.orr_no_count(13usize, 16usize, 15usize, 2123600u32);
    emu.lbu_no_count(19usize, 11usize, 12u32, 2123604u32);
    emu.lbu_no_count(20usize, 11usize, 13u32, 2123608u32);
    emu.lbu_no_count(21usize, 11usize, 14u32, 2123612u32);
    emu.lbu_no_count(22usize, 11usize, 15u32, 2123616u32);
    emu.sli_no_count(17usize, 17usize, 16u32, 2123620u32);
    emu.sli_no_count(5usize, 5usize, 24u32, 2123624u32);
    emu.sli_no_count(7usize, 7usize, 8u32, 2123628u32);
    emu.sli_no_count(28usize, 28usize, 16u32, 2123632u32);
    emu.sli_no_count(29usize, 29usize, 24u32, 2123636u32);
    emu.sli_no_count(31usize, 31usize, 8u32, 2123640u32);
    emu.orr_no_count(14usize, 5usize, 17usize, 2123644u32);
    emu.orr_no_count(15usize, 7usize, 6usize, 2123648u32);
    emu.orr_no_count(16usize, 29usize, 28usize, 2123652u32);
    emu.orr_no_count(17usize, 31usize, 30usize, 2123656u32);
    emu.lbu_no_count(5usize, 11usize, 8u32, 2123660u32);
    emu.lbu_no_count(6usize, 11usize, 9u32, 2123664u32);
    emu.lbu_no_count(7usize, 11usize, 10u32, 2123668u32);
    emu.lbu_no_count(28usize, 11usize, 11u32, 2123672u32);
    emu.sli_no_count(8usize, 8usize, 16u32, 2123676u32);
    emu.sli_no_count(18usize, 18usize, 24u32, 2123680u32);
    emu.sli_no_count(20usize, 20usize, 8u32, 2123684u32);
    emu.sli_no_count(21usize, 21usize, 16u32, 2123688u32);
    emu.sli_no_count(22usize, 22usize, 24u32, 2123692u32);
    emu.sli_no_count(6usize, 6usize, 8u32, 2123696u32);
    emu.orr_no_count(29usize, 18usize, 8usize, 2123700u32);
    emu.orr_no_count(30usize, 20usize, 19usize, 2123704u32);
    emu.orr_no_count(31usize, 22usize, 21usize, 2123708u32);
    emu.orr_no_count(5usize, 6usize, 5usize, 2123712u32);
    emu.lbu_no_count(6usize, 11usize, 4u32, 2123716u32);
    emu.lbu_no_count(8usize, 11usize, 5u32, 2123720u32);
    emu.lbu_no_count(18usize, 11usize, 6u32, 2123724u32);
    emu.lbu_no_count(19usize, 11usize, 7u32, 2123728u32);
    emu.sli_no_count(7usize, 7usize, 16u32, 2123732u32);
    emu.sli_no_count(28usize, 28usize, 24u32, 2123736u32);
    emu.sli_no_count(8usize, 8usize, 8u32, 2123740u32);
    emu.sli_no_count(18usize, 18usize, 16u32, 2123744u32);
    emu.sli_no_count(19usize, 19usize, 24u32, 2123748u32);
    emu.orr_no_count(7usize, 28usize, 7usize, 2123752u32);
    emu.orr_no_count(6usize, 8usize, 6usize, 2123756u32);
    emu.lbu_no_count(28usize, 11usize, 0u32, 2123760u32);
    emu.lbu_no_count(8usize, 11usize, 1u32, 2123764u32);
    emu.orr_no_count(18usize, 19usize, 18usize, 2123768u32);
    emu.lbu_no_count(19usize, 11usize, 2u32, 2123772u32);
    emu.lbu_no_count(11usize, 11usize, 3u32, 2123776u32);
    emu.sli_no_count(8usize, 8usize, 8u32, 2123780u32);
    emu.orr_no_count(28usize, 8usize, 28usize, 2123784u32);
    emu.sli_no_count(19usize, 19usize, 16u32, 2123788u32);
    emu.sli_no_count(11usize, 11usize, 24u32, 2123792u32);
    emu.orr_no_count(11usize, 11usize, 19usize, 2123796u32);
    emu.orr_no_count(10usize, 12usize, 10usize, 2123800u32);
    emu.orr_no_count(13usize, 14usize, 13usize, 2123804u32);
    emu.orr_no_count(12usize, 16usize, 15usize, 2123808u32);
    emu.orr_no_count(14usize, 29usize, 17usize, 2123812u32);
    emu.orr_no_count(15usize, 31usize, 30usize, 2123816u32);
    emu.orr_no_count(16usize, 7usize, 5usize, 2123820u32);
    emu.orr_no_count(17usize, 18usize, 6usize, 2123824u32);
    emu.orr_no_count(11usize, 11usize, 28usize, 2123828u32);
    emu.sw_no_count(14usize, 2usize, 44u32, 2123832u32)?;
    emu.sw_no_count(12usize, 2usize, 48u32, 2123836u32)?;
    emu.sw_no_count(13usize, 2usize, 52u32, 2123840u32)?;
    emu.sw_no_count(10usize, 2usize, 56u32, 2123844u32)?;
    emu.sw_no_count(11usize, 2usize, 28u32, 2123848u32)?;
    emu.sw_no_count(17usize, 2usize, 32u32, 2123852u32)?;
    emu.sw_no_count(16usize, 2usize, 36u32, 2123856u32)?;
    emu.sw_no_count(15usize, 2usize, 40u32, 2123860u32)?;
    emu.adi_no_count(10usize, 2usize, 60u32, 2123864u32);
    emu.adi_no_count(11usize, 2usize, 28u32, 2123868u32);
    emu.apc_no_count(1usize, 2123868u32, 49152u32, 2123872u32);
    emu.add_memory_rw_events(108usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2123876u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966284u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206864(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(4234354688u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2123880u32;
    emu.update_insn_clock();
    emu.lw_no_count(8usize, 2usize, 60u32, 2123884u32)?;
    emu.lw_no_count(18usize, 2usize, 64u32, 2123888u32)?;
    emu.lw_no_count(20usize, 2usize, 68u32, 2123892u32)?;
    emu.adi_no_count(10usize, 10usize, 1361u32, 2123896u32);
    emu.sltru_no_count(10usize, 8usize, 10usize, 2123900u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a >= b {
        emu.pc = 2124004u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002068e4));
    } else {
        emu.pc = 2123904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206880));
    }
}
#[inline(always)]
pub fn block_0x00206880(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 18usize, 10usize, 2123908u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2123912u32);
    emu.slti_no_count(10usize, 10usize, 0u32, 2123916u32);
    emu.lw_no_count(21usize, 2usize, 72u32, 2123920u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a < b {
        emu.pc = 2124028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002068fc));
    } else {
        emu.pc = 2123924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206894));
    }
}
#[inline(always)]
pub fn block_0x00206894(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 20usize, 10usize, 2123928u32);
    let a = 0u32.wrapping_add(2803343360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2123932u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966916u32, 2123936u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2123940u32);
    emu.lw_no_count(22usize, 2usize, 76u32, 2123944u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a < b {
        emu.pc = 2124048u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206910));
    } else {
        emu.pc = 2123948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002068ac));
    }
}
#[inline(always)]
pub fn block_0x002068ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 21usize, 10usize, 2123952u32);
    let a = 0u32.wrapping_add(3169255424u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2123956u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965933u32, 2123960u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2123964u32);
    emu.lw_no_count(23usize, 2usize, 80u32, 2123968u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a < b {
        emu.pc = 2124068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206924));
    } else {
        emu.pc = 2123972u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002068c4));
    }
}
#[inline(always)]
pub fn block_0x002068c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 22usize, 10usize, 2123976u32);
    emu.sltiu_no_count(10usize, 10usize, 4294967295u32, 2123980u32);
    emu.lw_no_count(19usize, 2usize, 84u32, 2123984u32)?;
    emu.adi_no_count(26usize, 0usize, 1u32, 2123988u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a < b {
        emu.pc = 2124092u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020693c));
    } else {
        emu.pc = 2123992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002068d8));
    }
}
#[inline(always)]
pub fn block_0x002068d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 23usize, 10usize, 2123996u32);
    emu.sltiu_no_count(10usize, 10usize, 4294967295u32, 2124000u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2124004u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2124104u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00206948));
}
#[inline(always)]
pub fn block_0x002068e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 18usize, 10usize, 2124008u32);
    let a = 0u32.wrapping_add(4089040896u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2124012u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965954u32, 2124016u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2124020u32);
    emu.lw_no_count(21usize, 2usize, 72u32, 2124024u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a >= b {
        emu.pc = 2123924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206894));
    } else {
        emu.pc = 2124028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002068fc));
    }
}
#[inline(always)]
pub fn block_0x002068fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 20usize, 10usize, 2124032u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2124036u32);
    emu.slti_no_count(10usize, 10usize, 0u32, 2124040u32);
    emu.lw_no_count(22usize, 2usize, 76u32, 2124044u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a >= b {
        emu.pc = 2123948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002068ac));
    } else {
        emu.pc = 2124048u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206910));
    }
}
#[inline(always)]
pub fn block_0x00206910(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 21usize, 10usize, 2124052u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2124056u32);
    emu.slti_no_count(10usize, 10usize, 0u32, 2124060u32);
    emu.lw_no_count(23usize, 2usize, 80u32, 2124064u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a >= b {
        emu.pc = 2123972u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002068c4));
    } else {
        emu.pc = 2124068u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206924));
    }
}
#[inline(always)]
pub fn block_0x00206924(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 22usize, 10usize, 2124072u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2124076u32);
    emu.slti_no_count(10usize, 10usize, 0u32, 2124080u32);
    emu.lw_no_count(19usize, 2usize, 84u32, 2124084u32)?;
    emu.adi_no_count(26usize, 0usize, 1u32, 2124088u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a >= b {
        emu.pc = 2123992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002068d8));
    } else {
        emu.pc = 2124092u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020693c));
    }
}
#[inline(always)]
pub fn block_0x0020693c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 23usize, 10usize, 2124096u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2124100u32);
    emu.slti_no_count(10usize, 10usize, 0u32, 2124104u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2124104u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00206948));
}
#[inline]
pub fn block_0x00206948(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(25usize, 2usize, 88u32, 2124108u32)?;
    emu.sltru_no_count(10usize, 19usize, 10usize, 2124112u32);
    emu.sbr_no_count(11usize, 25usize, 10usize, 2124116u32);
    emu.sltru_no_count(12usize, 11usize, 25usize, 2124120u32);
    emu.adi_no_count(11usize, 11usize, 1u32, 2124124u32);
    emu.sbr_no_count(12usize, 12usize, 10usize, 2124128u32);
    emu.sltiu_no_count(10usize, 11usize, 1u32, 2124132u32);
    emu.adr_no_count(10usize, 12usize, 10usize, 2124136u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2124140u32);
    emu.apc_no_count(1usize, 2124140u32, 69632u32, 2124144u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2124148u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966304u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206974(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 255u32, 2124152u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(26usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2124672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206b80));
    } else {
        emu.pc = 2124156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020697c));
    }
}
#[inline(never)]
pub fn block_0x0020697c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 96u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(19usize, 2usize, 12u32, 2124160u32)?;
    emu.sw_no_count(18usize, 2usize, 16u32, 2124164u32)?;
    emu.sw_no_count(8usize, 2usize, 20u32, 2124168u32)?;
    emu.sw_no_count(24usize, 2usize, 24u32, 2124172u32)?;
    emu.lbu_no_count(10usize, 9usize, 28u32, 2124176u32);
    emu.lbu_no_count(11usize, 9usize, 29u32, 2124180u32);
    emu.lbu_no_count(12usize, 9usize, 30u32, 2124184u32);
    emu.lbu_no_count(13usize, 9usize, 31u32, 2124188u32);
    emu.lbu_no_count(14usize, 9usize, 24u32, 2124192u32);
    emu.lbu_no_count(15usize, 9usize, 25u32, 2124196u32);
    emu.lbu_no_count(16usize, 9usize, 26u32, 2124200u32);
    emu.lbu_no_count(17usize, 9usize, 27u32, 2124204u32);
    emu.lbu_no_count(5usize, 9usize, 20u32, 2124208u32);
    emu.lbu_no_count(6usize, 9usize, 21u32, 2124212u32);
    emu.lbu_no_count(7usize, 9usize, 22u32, 2124216u32);
    emu.lbu_no_count(28usize, 9usize, 23u32, 2124220u32);
    emu.lbu_no_count(29usize, 9usize, 16u32, 2124224u32);
    emu.lbu_no_count(30usize, 9usize, 17u32, 2124228u32);
    emu.lbu_no_count(31usize, 9usize, 18u32, 2124232u32);
    emu.lbu_no_count(8usize, 9usize, 19u32, 2124236u32);
    emu.sli_no_count(11usize, 11usize, 8u32, 2124240u32);
    emu.sli_no_count(12usize, 12usize, 16u32, 2124244u32);
    emu.sli_no_count(13usize, 13usize, 24u32, 2124248u32);
    emu.sli_no_count(15usize, 15usize, 8u32, 2124252u32);
    emu.orr_no_count(10usize, 11usize, 10usize, 2124256u32);
    emu.orr_no_count(11usize, 13usize, 12usize, 2124260u32);
    emu.orr_no_count(12usize, 15usize, 14usize, 2124264u32);
    emu.lbu_no_count(18usize, 9usize, 12u32, 2124268u32);
    emu.lbu_no_count(19usize, 9usize, 13u32, 2124272u32);
    emu.lbu_no_count(24usize, 9usize, 14u32, 2124276u32);
    emu.lbu_no_count(26usize, 9usize, 15u32, 2124280u32);
    emu.sli_no_count(16usize, 16usize, 16u32, 2124284u32);
    emu.sli_no_count(17usize, 17usize, 24u32, 2124288u32);
    emu.sli_no_count(6usize, 6usize, 8u32, 2124292u32);
    emu.sli_no_count(7usize, 7usize, 16u32, 2124296u32);
    emu.sli_no_count(28usize, 28usize, 24u32, 2124300u32);
    emu.sli_no_count(30usize, 30usize, 8u32, 2124304u32);
    emu.orr_no_count(13usize, 17usize, 16usize, 2124308u32);
    emu.orr_no_count(14usize, 6usize, 5usize, 2124312u32);
    emu.orr_no_count(15usize, 28usize, 7usize, 2124316u32);
    emu.orr_no_count(16usize, 30usize, 29usize, 2124320u32);
    emu.lbu_no_count(17usize, 9usize, 8u32, 2124324u32);
    emu.lbu_no_count(5usize, 9usize, 9u32, 2124328u32);
    emu.lbu_no_count(6usize, 9usize, 10u32, 2124332u32);
    emu.lbu_no_count(7usize, 9usize, 11u32, 2124336u32);
    emu.sli_no_count(31usize, 31usize, 16u32, 2124340u32);
    emu.sli_no_count(8usize, 8usize, 24u32, 2124344u32);
    emu.sli_no_count(19usize, 19usize, 8u32, 2124348u32);
    emu.sli_no_count(24usize, 24usize, 16u32, 2124352u32);
    emu.sli_no_count(26usize, 26usize, 24u32, 2124356u32);
    emu.sli_no_count(5usize, 5usize, 8u32, 2124360u32);
    emu.orr_no_count(28usize, 8usize, 31usize, 2124364u32);
    emu.orr_no_count(29usize, 19usize, 18usize, 2124368u32);
    emu.orr_no_count(30usize, 26usize, 24usize, 2124372u32);
    emu.orr_no_count(17usize, 5usize, 17usize, 2124376u32);
    emu.lbu_no_count(5usize, 9usize, 4u32, 2124380u32);
    emu.lbu_no_count(31usize, 9usize, 5u32, 2124384u32);
    emu.lbu_no_count(8usize, 9usize, 6u32, 2124388u32);
    emu.lbu_no_count(18usize, 9usize, 7u32, 2124392u32);
    emu.sli_no_count(6usize, 6usize, 16u32, 2124396u32);
    emu.sli_no_count(7usize, 7usize, 24u32, 2124400u32);
    emu.sli_no_count(31usize, 31usize, 8u32, 2124404u32);
    emu.sli_no_count(8usize, 8usize, 16u32, 2124408u32);
    emu.sli_no_count(18usize, 18usize, 24u32, 2124412u32);
    emu.orr_no_count(6usize, 7usize, 6usize, 2124416u32);
    emu.orr_no_count(5usize, 31usize, 5usize, 2124420u32);
    emu.lbu_no_count(7usize, 9usize, 0u32, 2124424u32);
    emu.lbu_no_count(31usize, 9usize, 1u32, 2124428u32);
    emu.orr_no_count(8usize, 18usize, 8usize, 2124432u32);
    emu.lbu_no_count(18usize, 9usize, 2u32, 2124436u32);
    emu.lbu_no_count(9usize, 9usize, 3u32, 2124440u32);
    emu.sli_no_count(31usize, 31usize, 8u32, 2124444u32);
    emu.orr_no_count(7usize, 31usize, 7usize, 2124448u32);
    emu.sli_no_count(18usize, 18usize, 16u32, 2124452u32);
    emu.sli_no_count(9usize, 9usize, 24u32, 2124456u32);
    emu.orr_no_count(31usize, 9usize, 18usize, 2124460u32);
    emu.orr_no_count(10usize, 11usize, 10usize, 2124464u32);
    emu.orr_no_count(12usize, 13usize, 12usize, 2124468u32);
    emu.orr_no_count(14usize, 15usize, 14usize, 2124472u32);
    emu.orr_no_count(11usize, 28usize, 16usize, 2124476u32);
    emu.orr_no_count(13usize, 30usize, 29usize, 2124480u32);
    emu.orr_no_count(15usize, 6usize, 17usize, 2124484u32);
    emu.orr_no_count(16usize, 8usize, 5usize, 2124488u32);
    emu.orr_no_count(17usize, 31usize, 7usize, 2124492u32);
    emu.sw_no_count(11usize, 2usize, 44u32, 2124496u32)?;
    emu.sw_no_count(14usize, 2usize, 48u32, 2124500u32)?;
    emu.sw_no_count(12usize, 2usize, 52u32, 2124504u32)?;
    emu.sw_no_count(10usize, 2usize, 56u32, 2124508u32)?;
    emu.sw_no_count(17usize, 2usize, 28u32, 2124512u32)?;
    emu.sw_no_count(16usize, 2usize, 32u32, 2124516u32)?;
    emu.sw_no_count(15usize, 2usize, 36u32, 2124520u32)?;
    emu.sw_no_count(13usize, 2usize, 40u32, 2124524u32)?;
    emu.adi_no_count(10usize, 2usize, 60u32, 2124528u32);
    emu.adi_no_count(11usize, 2usize, 28u32, 2124532u32);
    emu.apc_no_count(1usize, 2124532u32, 49152u32, 2124536u32);
    emu.add_memory_rw_events(96usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2124540u32;
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
#[inline(always)]
pub fn block_0x00206afc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(4234354688u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2124544u32;
    emu.update_insn_clock();
    emu.lw_no_count(11usize, 2usize, 60u32, 2124548u32)?;
    emu.lw_no_count(27usize, 2usize, 64u32, 2124552u32)?;
    emu.lw_no_count(26usize, 2usize, 68u32, 2124556u32)?;
    emu.adi_no_count(10usize, 10usize, 1361u32, 2124560u32);
    emu.sw_no_count(11usize, 2usize, 8u32, 2124564u32)?;
    emu.sltru_no_count(10usize, 11usize, 10usize, 2124568u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(27usize);
    if a >= b {
        emu.pc = 2124688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206b90));
    } else {
        emu.pc = 2124572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206b1c));
    }
}
#[inline(always)]
pub fn block_0x00206b1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 27usize, 10usize, 2124576u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2124580u32);
    emu.slti_no_count(10usize, 10usize, 0u32, 2124584u32);
    emu.lw_no_count(18usize, 2usize, 72u32, 2124588u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a < b {
        emu.pc = 2124712u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206ba8));
    } else {
        emu.pc = 2124592u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206b30));
    }
}
#[inline(always)]
pub fn block_0x00206b30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 26usize, 10usize, 2124596u32);
    let a = 0u32.wrapping_add(2803343360u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2124600u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966916u32, 2124604u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2124608u32);
    emu.lw_no_count(8usize, 2usize, 76u32, 2124612u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a < b {
        emu.pc = 2124732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206bbc));
    } else {
        emu.pc = 2124616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206b48));
    }
}
#[inline(always)]
pub fn block_0x00206b48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 18usize, 10usize, 2124620u32);
    let a = 0u32.wrapping_add(3169255424u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2124624u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965933u32, 2124628u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2124632u32);
    emu.lw_no_count(9usize, 2usize, 80u32, 2124636u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a < b {
        emu.pc = 2124752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206bd0));
    } else {
        emu.pc = 2124640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206b60));
    }
}
#[inline(always)]
pub fn block_0x00206b60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 8usize, 10usize, 2124644u32);
    emu.sltiu_no_count(11usize, 10usize, 4294967295u32, 2124648u32);
    emu.lw_no_count(24usize, 2usize, 84u32, 2124652u32)?;
    emu.adi_no_count(10usize, 0usize, 4294967295u32, 2124656u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a < b {
        emu.pc = 2124776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206be8));
    } else {
        emu.pc = 2124660u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206b74));
    }
}
#[inline(always)]
pub fn block_0x00206b74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 9usize, 11usize, 2124664u32);
    emu.sltiu_no_count(11usize, 11usize, 4294967295u32, 2124668u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2124672u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2124788u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00206bf4));
}
#[inline(always)]
pub fn block_0x00206b80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2124676u32);
    emu.sw_no_count(10usize, 24usize, 0u32, 2124680u32)?;
    emu.sw_no_count(0usize, 24usize, 4u32, 2124684u32)?;
    emu.add_memory_rw_events(4usize);
    let return_addr = 2124688u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2124984u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00206cb8));
}
#[inline(always)]
pub fn block_0x00206b90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 27usize, 10usize, 2124692u32);
    let a = 0u32.wrapping_add(4089040896u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2124696u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965954u32, 2124700u32);
    emu.sltru_no_count(10usize, 10usize, 11usize, 2124704u32);
    emu.lw_no_count(18usize, 2usize, 72u32, 2124708u32)?;
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a >= b {
        emu.pc = 2124592u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206b30));
    } else {
        emu.pc = 2124712u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206ba8));
    }
}
#[inline(always)]
pub fn block_0x00206ba8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 26usize, 10usize, 2124716u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2124720u32);
    emu.slti_no_count(10usize, 10usize, 0u32, 2124724u32);
    emu.lw_no_count(8usize, 2usize, 76u32, 2124728u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a >= b {
        emu.pc = 2124616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206b48));
    } else {
        emu.pc = 2124732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206bbc));
    }
}
#[inline(always)]
pub fn block_0x00206bbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 18usize, 10usize, 2124736u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2124740u32);
    emu.slti_no_count(10usize, 10usize, 0u32, 2124744u32);
    emu.lw_no_count(9usize, 2usize, 80u32, 2124748u32)?;
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a >= b {
        emu.pc = 2124640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206b60));
    } else {
        emu.pc = 2124752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206bd0));
    }
}
#[inline(always)]
pub fn block_0x00206bd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 8usize, 10usize, 2124756u32);
    emu.sbr_no_count(10usize, 0usize, 10usize, 2124760u32);
    emu.slti_no_count(11usize, 10usize, 0u32, 2124764u32);
    emu.lw_no_count(24usize, 2usize, 84u32, 2124768u32)?;
    emu.adi_no_count(10usize, 0usize, 4294967295u32, 2124772u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a >= b {
        emu.pc = 2124660u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206b74));
    } else {
        emu.pc = 2124776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206be8));
    }
}
#[inline(always)]
pub fn block_0x00206be8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(11usize, 9usize, 11usize, 2124780u32);
    emu.sbr_no_count(11usize, 0usize, 11usize, 2124784u32);
    emu.slti_no_count(11usize, 11usize, 0u32, 2124788u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2124788u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00206bf4));
}
#[inline]
pub fn block_0x00206bf4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(19usize, 2usize, 88u32, 2124792u32)?;
    emu.sltru_no_count(11usize, 24usize, 11usize, 2124796u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2124800u32);
    emu.adr_no_count(12usize, 19usize, 12usize, 2124804u32);
    emu.sltru_no_count(13usize, 12usize, 19usize, 2124808u32);
    emu.adr_no_count(10usize, 10usize, 13usize, 2124812u32);
    emu.sbr_no_count(13usize, 12usize, 11usize, 2124816u32);
    emu.sbr_no_count(10usize, 10usize, 11usize, 2124820u32);
    emu.sltru_no_count(11usize, 13usize, 12usize, 2124824u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2124828u32);
    emu.apc_no_count(1usize, 2124828u32, 69632u32, 2124832u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2124836u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965616u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206c24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 10usize, 255u32, 2124840u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2124844u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2124972u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206cac));
    } else {
        emu.pc = 2124848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206c30));
    }
}
#[inline]
pub fn block_0x00206c30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 20u32, 2124852u32)?;
    emu.lw_no_count(11usize, 2usize, 16u32, 2124856u32)?;
    emu.orr_no_count(10usize, 11usize, 10usize, 2124860u32);
    emu.orr_no_count(11usize, 20usize, 21usize, 2124864u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2124868u32);
    emu.orr_no_count(11usize, 22usize, 23usize, 2124872u32);
    emu.lw_no_count(12usize, 2usize, 12u32, 2124876u32)?;
    emu.orr_no_count(11usize, 11usize, 12usize, 2124880u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2124884u32);
    emu.orr_no_count(10usize, 10usize, 25usize, 2124888u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2124892u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2124896u32);
    emu.apc_no_count(1usize, 2124896u32, 69632u32, 2124900u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2124904u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965548u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206c68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 255u32, 2124908u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2124968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206ca8));
    } else {
        emu.pc = 2124912u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206c70));
    }
}
#[inline]
pub fn block_0x00206c70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 8u32, 2124916u32)?;
    emu.orr_no_count(10usize, 27usize, 10usize, 2124920u32);
    emu.orr_no_count(11usize, 26usize, 18usize, 2124924u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2124928u32);
    emu.orr_no_count(11usize, 8usize, 9usize, 2124932u32);
    emu.orr_no_count(11usize, 11usize, 24usize, 2124936u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2124940u32);
    emu.orr_no_count(10usize, 10usize, 19usize, 2124944u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2124948u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2124952u32);
    emu.apc_no_count(1usize, 2124952u32, 69632u32, 2124956u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2124960u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965492u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206ca0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 255u32, 2124964u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2125044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206cf4));
    } else {
        emu.pc = 2124968u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206ca8));
    }
}
#[inline(always)]
pub fn block_0x00206ca8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2124972u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2124972u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00206cac));
}
#[inline(always)]
pub fn block_0x00206cac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 24u32, 2124976u32)?;
    emu.sw_no_count(10usize, 11usize, 0u32, 2124980u32)?;
    emu.sw_no_count(0usize, 11usize, 4u32, 2124984u32)?;
    emu.add_memory_rw_events(3usize);
    emu.pc = 2124984u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00206cb8));
}
#[inline]
pub fn block_0x00206cb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 140u32, 2124988u32)?;
    emu.lw_no_count(8usize, 2usize, 136u32, 2124992u32)?;
    emu.lw_no_count(9usize, 2usize, 132u32, 2124996u32)?;
    emu.lw_no_count(18usize, 2usize, 128u32, 2125000u32)?;
    emu.lw_no_count(19usize, 2usize, 124u32, 2125004u32)?;
    emu.lw_no_count(20usize, 2usize, 120u32, 2125008u32)?;
    emu.lw_no_count(21usize, 2usize, 116u32, 2125012u32)?;
    emu.lw_no_count(22usize, 2usize, 112u32, 2125016u32)?;
    emu.lw_no_count(23usize, 2usize, 108u32, 2125020u32)?;
    emu.lw_no_count(24usize, 2usize, 104u32, 2125024u32)?;
    emu.lw_no_count(25usize, 2usize, 100u32, 2125028u32)?;
    emu.lw_no_count(26usize, 2usize, 96u32, 2125032u32)?;
    emu.lw_no_count(27usize, 2usize, 92u32, 2125036u32)?;
    emu.adi_no_count(2usize, 2usize, 144u32, 2125040u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2125044u32;
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
pub fn block_0x00206cf4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 23u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 24u32, 2125048u32)?;
    emu.sw_no_count(0usize, 10usize, 0u32, 2125052u32)?;
    emu.lw_no_count(11usize, 2usize, 20u32, 2125056u32)?;
    emu.sw_no_count(11usize, 10usize, 4u32, 2125060u32)?;
    emu.lw_no_count(11usize, 2usize, 16u32, 2125064u32)?;
    emu.sw_no_count(11usize, 10usize, 8u32, 2125068u32)?;
    emu.sw_no_count(20usize, 10usize, 12u32, 2125072u32)?;
    emu.sw_no_count(21usize, 10usize, 16u32, 2125076u32)?;
    emu.sw_no_count(22usize, 10usize, 20u32, 2125080u32)?;
    emu.sw_no_count(23usize, 10usize, 24u32, 2125084u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2125088u32)?;
    emu.sw_no_count(11usize, 10usize, 28u32, 2125092u32)?;
    emu.sw_no_count(25usize, 10usize, 32u32, 2125096u32)?;
    emu.lw_no_count(11usize, 2usize, 8u32, 2125100u32)?;
    emu.sw_no_count(11usize, 10usize, 36u32, 2125104u32)?;
    emu.sw_no_count(27usize, 10usize, 40u32, 2125108u32)?;
    emu.sw_no_count(26usize, 10usize, 44u32, 2125112u32)?;
    emu.sw_no_count(18usize, 10usize, 48u32, 2125116u32)?;
    emu.sw_no_count(8usize, 10usize, 52u32, 2125120u32)?;
    emu.sw_no_count(9usize, 10usize, 56u32, 2125124u32)?;
    emu.sw_no_count(24usize, 10usize, 60u32, 2125128u32)?;
    emu.sw_no_count(19usize, 10usize, 64u32, 2125132u32)?;
    emu.add_memory_rw_events(23usize);
    let return_addr = 2125136u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2124984u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00206cb8));
}
#[inline]
pub fn block_0x00206d50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294966656u32, 2125140u32);
    emu.sw_no_count(1usize, 2usize, 636u32, 2125144u32)?;
    emu.sw_no_count(8usize, 2usize, 632u32, 2125148u32)?;
    emu.sw_no_count(9usize, 2usize, 628u32, 2125152u32)?;
    emu.sw_no_count(18usize, 2usize, 624u32, 2125156u32)?;
    emu.sw_no_count(19usize, 2usize, 620u32, 2125160u32)?;
    emu.adi_no_count(18usize, 13usize, 0u32, 2125164u32);
    emu.adi_no_count(9usize, 11usize, 0u32, 2125168u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2125172u32);
    emu.adi_no_count(10usize, 2usize, 8u32, 2125176u32);
    emu.adi_no_count(11usize, 12usize, 0u32, 2125180u32);
    emu.apc_no_count(1usize, 2125180u32, 28672u32, 2125184u32);
    emu.add_memory_rw_events(13usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2125188u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965720u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00206d84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 16u32, 2125192u32)?;
    emu.lw_no_count(11usize, 18usize, 20u32, 2125196u32)?;
    emu.lw_no_count(12usize, 18usize, 24u32, 2125200u32)?;
    emu.lw_no_count(13usize, 18usize, 28u32, 2125204u32)?;
    emu.sw_no_count(10usize, 2usize, 380u32, 2125208u32)?;
    emu.sw_no_count(11usize, 2usize, 384u32, 2125212u32)?;
    emu.sw_no_count(12usize, 2usize, 388u32, 2125216u32)?;
    emu.sw_no_count(13usize, 2usize, 392u32, 2125220u32)?;
    emu.lw_no_count(10usize, 18usize, 0u32, 2125224u32)?;
    emu.lw_no_count(11usize, 18usize, 4u32, 2125228u32)?;
    emu.lw_no_count(12usize, 18usize, 8u32, 2125232u32)?;
    emu.lw_no_count(13usize, 18usize, 12u32, 2125236u32)?;
    emu.sw_no_count(0usize, 2usize, 476u32, 2125240u32)?;
    emu.sw_no_count(0usize, 2usize, 480u32, 2125244u32)?;
    emu.sw_no_count(0usize, 2usize, 484u32, 2125248u32)?;
    emu.sw_no_count(0usize, 2usize, 488u32, 2125252u32)?;
    emu.sw_no_count(10usize, 2usize, 364u32, 2125256u32)?;
    emu.sw_no_count(11usize, 2usize, 368u32, 2125260u32)?;
    emu.sw_no_count(12usize, 2usize, 372u32, 2125264u32)?;
    emu.sw_no_count(13usize, 2usize, 376u32, 2125268u32)?;
    emu.sw_no_count(0usize, 2usize, 460u32, 2125272u32)?;
    emu.sw_no_count(0usize, 2usize, 464u32, 2125276u32)?;
    emu.sw_no_count(0usize, 2usize, 468u32, 2125280u32)?;
    emu.sw_no_count(0usize, 2usize, 472u32, 2125284u32)?;
    emu.adi_no_count(10usize, 2usize, 364u32, 2125288u32);
    emu.adi_no_count(11usize, 2usize, 460u32, 2125292u32);
    emu.apc_no_count(1usize, 2125292u32, 28672u32, 2125296u32);
    emu.add_memory_rw_events(28usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2125300u32;
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
pub fn block_0x00206df4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(10usize, 10usize, 4294967295u32, 2125304u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2125308u32);
    emu.apc_no_count(1usize, 2125308u32, 65536u32, 2125312u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2125316u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1916u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206e04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 10usize, 255u32, 2125320u32);
    emu.adi_no_count(19usize, 0usize, 1u32, 2125324u32);
    emu.sb_no_count(10usize, 2usize, 364u32, 2125328u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2126308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002071e4));
    } else {
        emu.pc = 2125332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206e14));
    }
}
#[inline(never)]
pub fn block_0x00206e14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 48u32, 2125336u32)?;
    emu.lw_no_count(11usize, 18usize, 52u32, 2125340u32)?;
    emu.lw_no_count(12usize, 18usize, 56u32, 2125344u32)?;
    emu.lw_no_count(13usize, 18usize, 60u32, 2125348u32)?;
    emu.sw_no_count(10usize, 2usize, 380u32, 2125352u32)?;
    emu.sw_no_count(11usize, 2usize, 384u32, 2125356u32)?;
    emu.sw_no_count(12usize, 2usize, 388u32, 2125360u32)?;
    emu.sw_no_count(13usize, 2usize, 392u32, 2125364u32)?;
    emu.lw_no_count(10usize, 18usize, 32u32, 2125368u32)?;
    emu.lw_no_count(11usize, 18usize, 36u32, 2125372u32)?;
    emu.lw_no_count(12usize, 18usize, 40u32, 2125376u32)?;
    emu.lw_no_count(13usize, 18usize, 44u32, 2125380u32)?;
    emu.sw_no_count(0usize, 2usize, 476u32, 2125384u32)?;
    emu.sw_no_count(0usize, 2usize, 480u32, 2125388u32)?;
    emu.sw_no_count(0usize, 2usize, 484u32, 2125392u32)?;
    emu.sw_no_count(0usize, 2usize, 488u32, 2125396u32)?;
    emu.sw_no_count(10usize, 2usize, 364u32, 2125400u32)?;
    emu.sw_no_count(11usize, 2usize, 368u32, 2125404u32)?;
    emu.sw_no_count(12usize, 2usize, 372u32, 2125408u32)?;
    emu.sw_no_count(13usize, 2usize, 376u32, 2125412u32)?;
    emu.sw_no_count(0usize, 2usize, 460u32, 2125416u32)?;
    emu.sw_no_count(0usize, 2usize, 464u32, 2125420u32)?;
    emu.sw_no_count(0usize, 2usize, 468u32, 2125424u32)?;
    emu.sw_no_count(0usize, 2usize, 472u32, 2125428u32)?;
    emu.adi_no_count(10usize, 2usize, 364u32, 2125432u32);
    emu.adi_no_count(11usize, 2usize, 460u32, 2125436u32);
    emu.apc_no_count(1usize, 2125436u32, 28672u32, 2125440u32);
    emu.add_memory_rw_events(28usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2125444u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x00206e84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(10usize, 10usize, 4294967295u32, 2125448u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2125452u32);
    emu.apc_no_count(1usize, 2125452u32, 65536u32, 2125456u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2125460u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1772u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206e94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 10usize, 255u32, 2125464u32);
    emu.sb_no_count(10usize, 2usize, 364u32, 2125468u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a != b {
        emu.pc = 2126308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002071e4));
    } else {
        emu.pc = 2125472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206ea0));
    }
}
#[inline(never)]
pub fn block_0x00206ea0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 36u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 18usize, 48u32, 2125476u32)?;
    emu.lw_no_count(11usize, 18usize, 52u32, 2125480u32)?;
    emu.lw_no_count(12usize, 18usize, 56u32, 2125484u32)?;
    emu.lw_no_count(13usize, 18usize, 60u32, 2125488u32)?;
    emu.sw_no_count(10usize, 2usize, 88u32, 2125492u32)?;
    emu.sw_no_count(11usize, 2usize, 92u32, 2125496u32)?;
    emu.sw_no_count(12usize, 2usize, 96u32, 2125500u32)?;
    emu.sw_no_count(13usize, 2usize, 100u32, 2125504u32)?;
    emu.lw_no_count(10usize, 18usize, 32u32, 2125508u32)?;
    emu.lw_no_count(11usize, 18usize, 36u32, 2125512u32)?;
    emu.lw_no_count(12usize, 18usize, 40u32, 2125516u32)?;
    emu.lw_no_count(13usize, 18usize, 44u32, 2125520u32)?;
    emu.sw_no_count(10usize, 2usize, 72u32, 2125524u32)?;
    emu.sw_no_count(11usize, 2usize, 76u32, 2125528u32)?;
    emu.sw_no_count(12usize, 2usize, 80u32, 2125532u32)?;
    emu.sw_no_count(13usize, 2usize, 84u32, 2125536u32)?;
    emu.lw_no_count(10usize, 18usize, 16u32, 2125540u32)?;
    emu.lw_no_count(11usize, 18usize, 20u32, 2125544u32)?;
    emu.lw_no_count(12usize, 18usize, 24u32, 2125548u32)?;
    emu.lw_no_count(13usize, 18usize, 28u32, 2125552u32)?;
    emu.sw_no_count(10usize, 2usize, 56u32, 2125556u32)?;
    emu.sw_no_count(11usize, 2usize, 60u32, 2125560u32)?;
    emu.sw_no_count(12usize, 2usize, 64u32, 2125564u32)?;
    emu.sw_no_count(13usize, 2usize, 68u32, 2125568u32)?;
    emu.lw_no_count(10usize, 18usize, 0u32, 2125572u32)?;
    emu.lw_no_count(11usize, 18usize, 4u32, 2125576u32)?;
    emu.lw_no_count(12usize, 18usize, 8u32, 2125580u32)?;
    emu.lw_no_count(13usize, 18usize, 12u32, 2125584u32)?;
    emu.sw_no_count(10usize, 2usize, 40u32, 2125588u32)?;
    emu.sw_no_count(11usize, 2usize, 44u32, 2125592u32)?;
    emu.sw_no_count(12usize, 2usize, 48u32, 2125596u32)?;
    emu.sw_no_count(13usize, 2usize, 52u32, 2125600u32)?;
    emu.adi_no_count(10usize, 2usize, 460u32, 2125604u32);
    emu.adi_no_count(11usize, 2usize, 72u32, 2125608u32);
    emu.apc_no_count(1usize, 2125608u32, 24576u32, 2125612u32);
    emu.add_memory_rw_events(36usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2125616u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966864u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206f30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 492u32, 2125620u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2125624u32);
    emu.sb_no_count(10usize, 2usize, 268u32, 2125628u32);
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2126348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020720c));
    } else {
        emu.pc = 2125632u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00206f40));
    }
}
#[inline]
pub fn block_0x00206f40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 476u32, 2125636u32)?;
    emu.lw_no_count(11usize, 2usize, 480u32, 2125640u32)?;
    emu.lw_no_count(12usize, 2usize, 484u32, 2125644u32)?;
    emu.lw_no_count(13usize, 2usize, 488u32, 2125648u32)?;
    emu.sw_no_count(10usize, 2usize, 120u32, 2125652u32)?;
    emu.sw_no_count(11usize, 2usize, 124u32, 2125656u32)?;
    emu.sw_no_count(12usize, 2usize, 128u32, 2125660u32)?;
    emu.sw_no_count(13usize, 2usize, 132u32, 2125664u32)?;
    emu.lw_no_count(10usize, 2usize, 460u32, 2125668u32)?;
    emu.lw_no_count(11usize, 2usize, 464u32, 2125672u32)?;
    emu.lw_no_count(12usize, 2usize, 468u32, 2125676u32)?;
    emu.lw_no_count(13usize, 2usize, 472u32, 2125680u32)?;
    emu.sw_no_count(10usize, 2usize, 104u32, 2125684u32)?;
    emu.sw_no_count(11usize, 2usize, 108u32, 2125688u32)?;
    emu.sw_no_count(12usize, 2usize, 112u32, 2125692u32)?;
    emu.sw_no_count(13usize, 2usize, 116u32, 2125696u32)?;
    emu.adi_no_count(10usize, 2usize, 136u32, 2125700u32);
    emu.adi_no_count(11usize, 2usize, 8u32, 2125704u32);
    emu.adi_no_count(12usize, 2usize, 104u32, 2125708u32);
    emu.apc_no_count(1usize, 2125708u32, 24576u32, 2125712u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2125716u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1964u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206f94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 168u32, 2125720u32);
    emu.adi_no_count(12usize, 2usize, 104u32, 2125724u32);
    emu.adi_no_count(11usize, 18usize, 0u32, 2125728u32);
    emu.apc_no_count(1usize, 2125728u32, 24576u32, 2125732u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2125736u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1944u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206fa8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2125740u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1680u32, 2125744u32);
    emu.adi_no_count(10usize, 2usize, 460u32, 2125748u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2125752u32);
    emu.apc_no_count(1usize, 2125752u32, 12288u32, 2125756u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2125760u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965420u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206fc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 268u32, 2125764u32);
    emu.adi_no_count(11usize, 2usize, 460u32, 2125768u32);
    emu.adi_no_count(12usize, 2usize, 136u32, 2125772u32);
    emu.apc_no_count(1usize, 2125772u32, 4294950912u32, 2125776u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2125780u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(796u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206fd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 460u32, 2125784u32);
    emu.adi_no_count(12usize, 0usize, 96u32, 2125788u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2125792u32);
    emu.apc_no_count(1usize, 2125792u32, 12288u32, 2125796u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2125800u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965380u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206fe8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 364u32, 2125804u32);
    emu.adi_no_count(11usize, 2usize, 460u32, 2125808u32);
    emu.adi_no_count(12usize, 2usize, 168u32, 2125812u32);
    emu.apc_no_count(1usize, 2125812u32, 4294950912u32, 2125816u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2125820u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(756u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00206ffc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 460u32, 2125824u32);
    emu.adi_no_count(11usize, 2usize, 268u32, 2125828u32);
    emu.adi_no_count(12usize, 2usize, 364u32, 2125832u32);
    emu.apc_no_count(1usize, 2125832u32, 4294950912u32, 2125836u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2125840u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1776u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207010(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 2usize, 524u32, 2125844u32);
    emu.adi_no_count(10usize, 2usize, 200u32, 2125848u32);
    emu.apc_no_count(1usize, 2125848u32, 45056u32, 2125852u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2125856u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965816u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00207020(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 2usize, 604u32, 2125860u32)?;
    emu.sw_no_count(0usize, 2usize, 608u32, 2125864u32)?;
    emu.sw_no_count(0usize, 2usize, 612u32, 2125868u32)?;
    emu.sw_no_count(0usize, 2usize, 616u32, 2125872u32)?;
    emu.lbu_no_count(13usize, 2usize, 232u32, 2125876u32);
    emu.sw_no_count(0usize, 2usize, 588u32, 2125880u32)?;
    emu.sw_no_count(0usize, 2usize, 592u32, 2125884u32)?;
    emu.sw_no_count(0usize, 2usize, 596u32, 2125888u32)?;
    emu.sw_no_count(0usize, 2usize, 600u32, 2125892u32)?;
    emu.adi_no_count(10usize, 2usize, 556u32, 2125896u32);
    emu.adi_no_count(11usize, 2usize, 588u32, 2125900u32);
    emu.adi_no_count(12usize, 2usize, 200u32, 2125904u32);
    emu.apc_no_count(1usize, 2125904u32, 45056u32, 2125908u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2125912u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965340u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00207058(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 476u32, 2125916u32)?;
    emu.lw_no_count(11usize, 2usize, 480u32, 2125920u32)?;
    emu.lw_no_count(12usize, 2usize, 484u32, 2125924u32)?;
    emu.lw_no_count(13usize, 2usize, 488u32, 2125928u32)?;
    emu.sw_no_count(10usize, 2usize, 380u32, 2125932u32)?;
    emu.sw_no_count(11usize, 2usize, 384u32, 2125936u32)?;
    emu.sw_no_count(12usize, 2usize, 388u32, 2125940u32)?;
    emu.sw_no_count(13usize, 2usize, 392u32, 2125944u32)?;
    emu.lw_no_count(10usize, 2usize, 460u32, 2125948u32)?;
    emu.lw_no_count(11usize, 2usize, 464u32, 2125952u32)?;
    emu.lw_no_count(12usize, 2usize, 468u32, 2125956u32)?;
    emu.lw_no_count(13usize, 2usize, 472u32, 2125960u32)?;
    emu.sw_no_count(10usize, 2usize, 364u32, 2125964u32)?;
    emu.sw_no_count(11usize, 2usize, 368u32, 2125968u32)?;
    emu.sw_no_count(12usize, 2usize, 372u32, 2125972u32)?;
    emu.sw_no_count(13usize, 2usize, 376u32, 2125976u32)?;
    emu.adi_no_count(10usize, 2usize, 268u32, 2125980u32);
    emu.adi_no_count(11usize, 2usize, 364u32, 2125984u32);
    emu.adi_no_count(12usize, 2usize, 556u32, 2125988u32);
    emu.apc_no_count(1usize, 2125988u32, 12288u32, 2125992u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2125996u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1712u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002070ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 508u32, 2126000u32)?;
    emu.lw_no_count(11usize, 2usize, 512u32, 2126004u32)?;
    emu.lw_no_count(12usize, 2usize, 516u32, 2126008u32)?;
    emu.lw_no_count(13usize, 2usize, 520u32, 2126012u32)?;
    emu.sw_no_count(10usize, 2usize, 380u32, 2126016u32)?;
    emu.sw_no_count(11usize, 2usize, 384u32, 2126020u32)?;
    emu.sw_no_count(12usize, 2usize, 388u32, 2126024u32)?;
    emu.sw_no_count(13usize, 2usize, 392u32, 2126028u32)?;
    emu.lw_no_count(10usize, 2usize, 492u32, 2126032u32)?;
    emu.lw_no_count(11usize, 2usize, 496u32, 2126036u32)?;
    emu.lw_no_count(12usize, 2usize, 500u32, 2126040u32)?;
    emu.lw_no_count(13usize, 2usize, 504u32, 2126044u32)?;
    emu.sw_no_count(10usize, 2usize, 364u32, 2126048u32)?;
    emu.sw_no_count(11usize, 2usize, 368u32, 2126052u32)?;
    emu.sw_no_count(12usize, 2usize, 372u32, 2126056u32)?;
    emu.sw_no_count(13usize, 2usize, 376u32, 2126060u32)?;
    emu.adi_no_count(10usize, 2usize, 300u32, 2126064u32);
    emu.adi_no_count(11usize, 2usize, 364u32, 2126068u32);
    emu.adi_no_count(12usize, 2usize, 556u32, 2126072u32);
    emu.apc_no_count(1usize, 2126072u32, 12288u32, 2126076u32);
    emu.add_memory_rw_events(21usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126080u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1628u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207100(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(9usize, 2usize, 232u32, 2126084u32);
    emu.sb_no_count(0usize, 2usize, 332u32, 2126088u32);
    emu.adi_no_count(10usize, 2usize, 364u32, 2126092u32);
    emu.adi_no_count(11usize, 2usize, 268u32, 2126096u32);
    emu.adi_no_count(12usize, 0usize, 68u32, 2126100u32);
    emu.apc_no_count(1usize, 2126100u32, 8192u32, 2126104u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126108u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1872u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020711c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(9usize, 2usize, 432u32, 2126112u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2126116u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966612u32, 2126120u32);
    emu.adi_no_count(10usize, 2usize, 268u32, 2126124u32);
    emu.adi_no_count(12usize, 0usize, 68u32, 2126128u32);
    emu.apc_no_count(1usize, 2126128u32, 8192u32, 2126132u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126136u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1844u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207138(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 200u32, 2126140u32);
    emu.adi_no_count(11usize, 2usize, 268u32, 2126144u32);
    emu.adi_no_count(12usize, 2usize, 364u32, 2126148u32);
    emu.adi_no_count(13usize, 9usize, 0u32, 2126152u32);
    emu.apc_no_count(1usize, 2126152u32, 40960u32, 2126156u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126160u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1892u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207150(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 2usize, 300u32, 2126164u32);
    emu.adi_no_count(12usize, 2usize, 396u32, 2126168u32);
    emu.adi_no_count(10usize, 2usize, 232u32, 2126172u32);
    emu.adi_no_count(13usize, 9usize, 0u32, 2126176u32);
    emu.apc_no_count(1usize, 2126176u32, 40960u32, 2126180u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126184u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1868u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00207168(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(10usize, 2usize, 332u32, 2126188u32);
    emu.lbu_no_count(11usize, 2usize, 428u32, 2126192u32);
    emu.sbr_no_count(12usize, 0usize, 9usize, 2126196u32);
    emu.xrr_no_count(11usize, 11usize, 10usize, 2126200u32);
    emu.anr_no_count(11usize, 11usize, 12usize, 2126204u32);
    emu.xrr_no_count(10usize, 11usize, 10usize, 2126208u32);
    emu.sb_no_count(10usize, 2usize, 264u32, 2126212u32);
    emu.adi_no_count(10usize, 2usize, 364u32, 2126216u32);
    emu.adi_no_count(11usize, 2usize, 200u32, 2126220u32);
    emu.apc_no_count(1usize, 2126220u32, 40960u32, 2126224u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126228u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966848u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207194(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 460u32, 2126232u32);
    emu.adi_no_count(11usize, 2usize, 364u32, 2126236u32);
    emu.apc_no_count(1usize, 2126236u32, 24576u32, 2126240u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126244u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1464u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002071a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 40u32, 2126248u32);
    emu.adi_no_count(11usize, 2usize, 460u32, 2126252u32);
    emu.apc_no_count(1usize, 2126252u32, 24576u32, 2126256u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126260u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1244u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002071b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 0usize, 0u32, 2126264u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2126276u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002071c4));
    } else {
        emu.pc = 2126268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002071bc));
    }
}
#[inline(always)]
pub fn block_0x002071bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 8usize, 4u32, 2126272u32)?;
    emu.adi_no_count(11usize, 0usize, 1u32, 2126276u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2126276u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002071c4));
}
#[inline(always)]
pub fn block_0x002071c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(11usize, 8usize, 0u32, 2126280u32)?;
    emu.lw_no_count(1usize, 2usize, 636u32, 2126284u32)?;
    emu.lw_no_count(8usize, 2usize, 632u32, 2126288u32)?;
    emu.lw_no_count(9usize, 2usize, 628u32, 2126292u32)?;
    emu.lw_no_count(18usize, 2usize, 624u32, 2126296u32)?;
    emu.lw_no_count(19usize, 2usize, 620u32, 2126300u32)?;
    emu.adi_no_count(2usize, 2usize, 640u32, 2126304u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126308u32;
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
pub fn block_0x002071e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 2usize, 460u32, 2126312u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2126316u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 184u32, 2126320u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2126324u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 188u32, 2126328u32);
    emu.adi_no_count(11usize, 2usize, 364u32, 2126332u32);
    emu.adi_no_count(13usize, 2usize, 460u32, 2126336u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2126340u32);
    emu.apc_no_count(1usize, 2126340u32, 12288u32, 2126344u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126348u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967268u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020720c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 2usize, 364u32, 2126352u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2126356u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 184u32, 2126360u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2126364u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 188u32, 2126368u32);
    emu.adi_no_count(11usize, 2usize, 268u32, 2126372u32);
    emu.adi_no_count(13usize, 2usize, 364u32, 2126376u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2126380u32);
    emu.apc_no_count(1usize, 2126380u32, 12288u32, 2126384u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126388u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967228u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
