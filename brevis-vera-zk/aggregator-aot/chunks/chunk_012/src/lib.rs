pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2143692u32;
pub const PC_MAX: u32 = 2149020u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 109usize] = [
        block_0x0020b5cc,
        block_0x0020b614,
        block_0x0020b6a8,
        block_0x0020b788,
        block_0x0020b78c,
        block_0x0020b7f4,
        block_0x0020b7f8,
        block_0x0020b878,
        block_0x0020b87c,
        block_0x0020b8f4,
        block_0x0020b8f8,
        block_0x0020b91c,
        block_0x0020b920,
        block_0x0020b970,
        block_0x0020b974,
        block_0x0020b99c,
        block_0x0020b9a0,
        block_0x0020ba24,
        block_0x0020ba28,
        block_0x0020ba9c,
        block_0x0020baa0,
        block_0x0020bad4,
        block_0x0020bad8,
        block_0x0020bb50,
        block_0x0020bb54,
        block_0x0020bbd0,
        block_0x0020bbdc,
        block_0x0020bc34,
        block_0x0020bc38,
        block_0x0020bca0,
        block_0x0020bca4,
        block_0x0020bce4,
        block_0x0020bce8,
        block_0x0020bd34,
        block_0x0020bd38,
        block_0x0020bd6c,
        block_0x0020bd70,
        block_0x0020bdbc,
        block_0x0020bdc0,
        block_0x0020bdf8,
        block_0x0020bdfc,
        block_0x0020be24,
        block_0x0020be28,
        block_0x0020be50,
        block_0x0020be54,
        block_0x0020be8c,
        block_0x0020be90,
        block_0x0020bee8,
        block_0x0020beec,
        block_0x0020bf30,
        block_0x0020bf34,
        block_0x0020bf60,
        block_0x0020bf64,
        block_0x0020bf80,
        block_0x0020bf84,
        block_0x0020bfb4,
        block_0x0020bfb8,
        block_0x0020bfcc,
        block_0x0020bfd0,
        block_0x0020bff0,
        block_0x0020bff4,
        block_0x0020c014,
        block_0x0020c018,
        block_0x0020c034,
        block_0x0020c038,
        block_0x0020c060,
        block_0x0020c064,
        block_0x0020c09c,
        block_0x0020c0a0,
        block_0x0020c0f4,
        block_0x0020c0f8,
        block_0x0020c13c,
        block_0x0020c140,
        block_0x0020c16c,
        block_0x0020c170,
        block_0x0020c18c,
        block_0x0020c190,
        block_0x0020c1c0,
        block_0x0020c1c4,
        block_0x0020c1d8,
        block_0x0020c1dc,
        block_0x0020c258,
        block_0x0020c36c,
        block_0x0020c384,
        block_0x0020c3dc,
        block_0x0020c41c,
        block_0x0020c468,
        block_0x0020c470,
        block_0x0020c480,
        block_0x0020c490,
        block_0x0020c498,
        block_0x0020c540,
        block_0x0020c5e0,
        block_0x0020c5f8,
        block_0x0020c650,
        block_0x0020c694,
        block_0x0020c73c,
        block_0x0020c7dc,
        block_0x0020c7f4,
        block_0x0020c84c,
        block_0x0020c88c,
        block_0x0020c89c,
        block_0x0020c8ac,
        block_0x0020c8b4,
        block_0x0020c9c0,
        block_0x0020c9e8,
        block_0x0020ca00,
        block_0x0020ca58,
        block_0x0020ca9c,
    ];
    const IDX: [u16; 1333usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 3u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 4u16, 5u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 6u16, 7u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 8u16, 9u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 10u16, 11u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 12u16, 13u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 14u16,
        15u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 16u16, 17u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 18u16, 19u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 20u16, 21u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 22u16, 23u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 24u16, 25u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 26u16, 0u16, 0u16, 27u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 28u16, 29u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 30u16, 31u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 32u16,
        33u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 34u16, 35u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 36u16, 37u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 38u16, 39u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 40u16, 41u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 42u16, 43u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 44u16,
        45u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 46u16, 47u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 48u16, 49u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 50u16, 51u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 52u16, 53u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 54u16, 55u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 56u16, 57u16, 0u16,
        0u16, 0u16, 0u16, 58u16, 59u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 60u16,
        61u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 62u16, 63u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 64u16, 65u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 66u16, 67u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 68u16, 69u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 70u16,
        71u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 72u16, 73u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 74u16, 75u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 76u16, 77u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 78u16, 79u16,
        0u16, 0u16, 0u16, 0u16, 80u16, 81u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 82u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 83u16, 0u16, 0u16, 0u16, 0u16, 0u16, 84u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 85u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 86u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 87u16,
        0u16, 88u16, 0u16, 0u16, 0u16, 89u16, 0u16, 0u16, 0u16, 90u16, 0u16, 91u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 92u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 93u16, 0u16, 0u16, 0u16, 0u16, 0u16, 94u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 95u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 96u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 97u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 98u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 99u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 100u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 101u16, 0u16, 0u16, 0u16, 102u16, 0u16, 0u16, 0u16,
        103u16, 0u16, 104u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 105u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 106u16, 0u16, 0u16, 0u16, 0u16, 0u16, 107u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 108u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 109u16,
    ];
    if pc < 2143692u32 || pc > 2149020u32 {
        return None;
    }
    let word_offset = ((pc - 2143692u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline]
pub fn block_0x0020b5cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967040u32, 2143696u32);
    emu.sw_no_count(1usize, 2usize, 252u32, 2143700u32)?;
    emu.sw_no_count(8usize, 2usize, 248u32, 2143704u32)?;
    emu.sw_no_count(9usize, 2usize, 244u32, 2143708u32)?;
    emu.sw_no_count(18usize, 2usize, 240u32, 2143712u32)?;
    emu.sw_no_count(19usize, 2usize, 236u32, 2143716u32)?;
    emu.sw_no_count(20usize, 2usize, 232u32, 2143720u32)?;
    emu.sw_no_count(21usize, 2usize, 228u32, 2143724u32)?;
    emu.sw_no_count(22usize, 2usize, 224u32, 2143728u32)?;
    emu.sw_no_count(23usize, 2usize, 220u32, 2143732u32)?;
    emu.sw_no_count(24usize, 2usize, 216u32, 2143736u32)?;
    emu.sw_no_count(25usize, 2usize, 212u32, 2143740u32)?;
    emu.sw_no_count(26usize, 2usize, 208u32, 2143744u32)?;
    emu.sw_no_count(27usize, 2usize, 204u32, 2143748u32)?;
    emu.sw_no_count(10usize, 2usize, 64u32, 2143752u32)?;
    emu.adi_no_count(10usize, 2usize, 96u32, 2143756u32);
    emu.apc_no_count(1usize, 2143756u32, 24576u32, 2143760u32);
    emu.add_memory_rw_events(18usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2143764u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965716u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0020b614(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 96u32, 2143768u32)?;
    emu.sw_no_count(10usize, 2usize, 72u32, 2143772u32)?;
    emu.lw_no_count(10usize, 2usize, 100u32, 2143776u32)?;
    emu.sw_no_count(10usize, 2usize, 80u32, 2143780u32)?;
    emu.lw_no_count(10usize, 2usize, 104u32, 2143784u32)?;
    emu.sw_no_count(10usize, 2usize, 28u32, 2143788u32)?;
    emu.lw_no_count(10usize, 2usize, 108u32, 2143792u32)?;
    emu.sw_no_count(10usize, 2usize, 24u32, 2143796u32)?;
    emu.lw_no_count(9usize, 2usize, 112u32, 2143800u32)?;
    emu.lw_no_count(10usize, 2usize, 116u32, 2143804u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2143808u32)?;
    emu.lw_no_count(8usize, 2usize, 120u32, 2143812u32)?;
    emu.lw_no_count(20usize, 2usize, 124u32, 2143816u32)?;
    emu.lw_no_count(16usize, 2usize, 128u32, 2143820u32)?;
    emu.lw_no_count(17usize, 2usize, 132u32, 2143824u32)?;
    emu.lw_no_count(10usize, 2usize, 136u32, 2143828u32)?;
    emu.lw_no_count(11usize, 2usize, 140u32, 2143832u32)?;
    emu.lw_no_count(12usize, 2usize, 144u32, 2143836u32)?;
    emu.lw_no_count(13usize, 2usize, 148u32, 2143840u32)?;
    emu.lw_no_count(14usize, 2usize, 152u32, 2143844u32)?;
    emu.lw_no_count(15usize, 2usize, 156u32, 2143848u32)?;
    emu.sw_no_count(8usize, 2usize, 160u32, 2143852u32)?;
    emu.sw_no_count(20usize, 2usize, 164u32, 2143856u32)?;
    emu.sw_no_count(16usize, 2usize, 52u32, 2143860u32)?;
    emu.sw_no_count(16usize, 2usize, 168u32, 2143864u32)?;
    emu.sw_no_count(17usize, 2usize, 48u32, 2143868u32)?;
    emu.sw_no_count(17usize, 2usize, 172u32, 2143872u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2143876u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2143880u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2143884u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2143888u32)?;
    emu.sw_no_count(14usize, 2usize, 192u32, 2143892u32)?;
    emu.sw_no_count(15usize, 2usize, 196u32, 2143896u32)?;
    emu.adi_no_count(10usize, 2usize, 96u32, 2143900u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2143904u32);
    emu.apc_no_count(1usize, 2143904u32, 4294963200u32, 2143908u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2143912u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1052u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0020b6a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 56u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 2usize, 96u32, 2143916u32)?;
    emu.lw_no_count(5usize, 2usize, 100u32, 2143920u32)?;
    let a = 0u32.wrapping_add(4234354688u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2143924u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4089040896u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2143928u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2803343360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2143932u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(3169255424u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2143936u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 10usize, 1361u32, 2143940u32);
    emu.adi_no_count(28usize, 11usize, 4294965954u32, 2143944u32);
    emu.adi_no_count(24usize, 12usize, 4294966916u32, 2143948u32);
    emu.adi_no_count(26usize, 14usize, 4294965933u32, 2143952u32);
    emu.mul_no_count(10usize, 5usize, 13usize, 2143956u32);
    emu.mulhu_no_count(11usize, 15usize, 13usize, 2143960u32);
    emu.mulhu_no_count(12usize, 5usize, 13usize, 2143964u32);
    emu.mul_no_count(14usize, 15usize, 28usize, 2143968u32);
    emu.mulhu_no_count(16usize, 15usize, 28usize, 2143972u32);
    emu.mul_no_count(17usize, 5usize, 28usize, 2143976u32);
    emu.mul_no_count(6usize, 5usize, 24usize, 2143980u32);
    emu.mulhu_no_count(7usize, 15usize, 24usize, 2143984u32);
    emu.adr_no_count(11usize, 10usize, 11usize, 2143988u32);
    emu.sltru_no_count(10usize, 11usize, 10usize, 2143992u32);
    emu.adr_no_count(10usize, 12usize, 10usize, 2143996u32);
    emu.mulhu_no_count(12usize, 5usize, 24usize, 2144000u32);
    emu.adr_no_count(7usize, 6usize, 7usize, 2144004u32);
    emu.sltru_no_count(6usize, 7usize, 6usize, 2144008u32);
    emu.adr_no_count(12usize, 12usize, 6usize, 2144012u32);
    emu.mul_no_count(6usize, 15usize, 26usize, 2144016u32);
    emu.adr_no_count(11usize, 14usize, 11usize, 2144020u32);
    emu.sw_no_count(11usize, 2usize, 76u32, 2144024u32)?;
    emu.sltru_no_count(11usize, 11usize, 14usize, 2144028u32);
    emu.adr_no_count(11usize, 16usize, 11usize, 2144032u32);
    emu.mulhu_no_count(14usize, 15usize, 26usize, 2144036u32);
    emu.adr_no_count(7usize, 6usize, 7usize, 2144040u32);
    emu.sltru_no_count(16usize, 7usize, 6usize, 2144044u32);
    emu.adr_no_count(14usize, 14usize, 16usize, 2144048u32);
    emu.sw_no_count(28usize, 2usize, 92u32, 2144052u32)?;
    emu.mulhu_no_count(16usize, 5usize, 28usize, 2144056u32);
    emu.adr_no_count(11usize, 10usize, 11usize, 2144060u32);
    emu.sltru_no_count(10usize, 11usize, 10usize, 2144064u32);
    emu.adr_no_count(10usize, 16usize, 10usize, 2144068u32);
    emu.mulhu_no_count(16usize, 5usize, 26usize, 2144072u32);
    emu.adr_no_count(14usize, 12usize, 14usize, 2144076u32);
    emu.sltru_no_count(12usize, 14usize, 12usize, 2144080u32);
    emu.adr_no_count(16usize, 16usize, 12usize, 2144084u32);
    emu.mul_no_count(12usize, 5usize, 26usize, 2144088u32);
    emu.adr_no_count(6usize, 17usize, 11usize, 2144092u32);
    emu.sltru_no_count(11usize, 6usize, 17usize, 2144096u32);
    emu.adr_no_count(11usize, 10usize, 11usize, 2144100u32);
    emu.mul_no_count(28usize, 15usize, 24usize, 2144104u32);
    emu.adr_no_count(10usize, 12usize, 14usize, 2144108u32);
    emu.sltru_no_count(14usize, 10usize, 12usize, 2144112u32);
    emu.adr_no_count(28usize, 6usize, 28usize, 2144116u32);
    emu.sltru_no_count(12usize, 28usize, 6usize, 2144120u32);
    emu.adr_no_count(7usize, 11usize, 7usize, 2144124u32);
    emu.adr_no_count(23usize, 7usize, 12usize, 2144128u32);
    emu.adr_no_count(14usize, 16usize, 14usize, 2144132u32);
    emu.add_memory_rw_events(55usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a == b {
        emu.pc = 2144140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b78c));
    } else {
        emu.pc = 2144136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b788));
    }
}
#[inline(always)]
pub fn block_0x0020b788(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 23usize, 11usize, 2144140u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2144140u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b78c));
}
#[inline(never)]
pub fn block_0x0020b78c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 26u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(9usize, 2usize, 36u32, 2144144u32)?;
    emu.sw_no_count(8usize, 2usize, 44u32, 2144148u32)?;
    emu.adr_no_count(12usize, 10usize, 12usize, 2144152u32);
    emu.sbr_no_count(11usize, 0usize, 5usize, 2144156u32);
    emu.adi_no_count(7usize, 0usize, 4294967295u32, 2144160u32);
    emu.sbr_no_count(29usize, 0usize, 15usize, 2144164u32);
    emu.sltru_no_count(10usize, 12usize, 10usize, 2144168u32);
    emu.mulhu_no_count(16usize, 15usize, 7usize, 2144172u32);
    emu.mulhu_no_count(6usize, 5usize, 7usize, 2144176u32);
    emu.sbr_no_count(18usize, 12usize, 15usize, 2144180u32);
    emu.adr_no_count(14usize, 14usize, 10usize, 2144184u32);
    emu.sbr_no_count(17usize, 16usize, 5usize, 2144188u32);
    emu.sltru_no_count(31usize, 18usize, 12usize, 2144192u32);
    emu.sbr_no_count(10usize, 17usize, 15usize, 2144196u32);
    emu.sltru_no_count(12usize, 10usize, 29usize, 2144200u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2144204u32);
    emu.sltru_no_count(16usize, 17usize, 11usize, 2144208u32);
    emu.adr_no_count(16usize, 6usize, 16usize, 2144212u32);
    emu.adr_no_count(10usize, 14usize, 10usize, 2144216u32);
    emu.adr_no_count(19usize, 10usize, 31usize, 2144220u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2144224u32);
    emu.sbr_no_count(10usize, 12usize, 5usize, 2144228u32);
    emu.sltru_no_count(12usize, 12usize, 16usize, 2144232u32);
    emu.sltru_no_count(27usize, 10usize, 11usize, 2144236u32);
    emu.adr_no_count(11usize, 6usize, 12usize, 2144240u32);
    emu.add_memory_rw_events(25usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2144248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b7f8));
    } else {
        emu.pc = 2144244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b7f4));
    }
}
#[inline(always)]
pub fn block_0x0020b7f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(31usize, 19usize, 14usize, 2144248u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2144248u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b7f8));
}
#[inline(never)]
pub fn block_0x0020b7f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 32u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(27usize, 11usize, 27usize, 2144252u32);
    emu.lw_no_count(5usize, 2usize, 104u32, 2144256u32)?;
    emu.lw_no_count(30usize, 2usize, 108u32, 2144260u32)?;
    emu.adr_no_count(31usize, 10usize, 31usize, 2144264u32);
    emu.sltru_no_count(14usize, 31usize, 10usize, 2144268u32);
    emu.mulhu_no_count(10usize, 5usize, 13usize, 2144272u32);
    emu.mul_no_count(11usize, 30usize, 13usize, 2144276u32);
    emu.mulhu_no_count(12usize, 30usize, 13usize, 2144280u32);
    emu.lw_no_count(9usize, 2usize, 92u32, 2144284u32)?;
    emu.mul_no_count(6usize, 5usize, 9usize, 2144288u32);
    emu.mulhu_no_count(8usize, 5usize, 9usize, 2144292u32);
    emu.adr_no_count(10usize, 11usize, 10usize, 2144296u32);
    emu.sltru_no_count(11usize, 10usize, 11usize, 2144300u32);
    emu.adr_no_count(11usize, 12usize, 11usize, 2144304u32);
    emu.mul_no_count(12usize, 30usize, 9usize, 2144308u32);
    emu.adr_no_count(10usize, 6usize, 10usize, 2144312u32);
    emu.sltru_no_count(6usize, 10usize, 6usize, 2144316u32);
    emu.adr_no_count(6usize, 8usize, 6usize, 2144320u32);
    emu.mulhu_no_count(8usize, 30usize, 9usize, 2144324u32);
    emu.adr_no_count(6usize, 11usize, 6usize, 2144328u32);
    emu.sltru_no_count(11usize, 6usize, 11usize, 2144332u32);
    emu.adr_no_count(8usize, 8usize, 11usize, 2144336u32);
    emu.mul_no_count(11usize, 5usize, 13usize, 2144340u32);
    emu.adr_no_count(11usize, 28usize, 11usize, 2144344u32);
    emu.sw_no_count(11usize, 2usize, 8u32, 2144348u32)?;
    emu.sltru_no_count(11usize, 11usize, 28usize, 2144352u32);
    emu.adr_no_count(28usize, 10usize, 11usize, 2144356u32);
    emu.adr_no_count(10usize, 12usize, 6usize, 2144360u32);
    emu.sltru_no_count(6usize, 10usize, 12usize, 2144364u32);
    emu.adr_no_count(28usize, 23usize, 28usize, 2144368u32);
    emu.adr_no_count(6usize, 8usize, 6usize, 2144372u32);
    emu.add_memory_rw_events(31usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2144380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b87c));
    } else {
        emu.pc = 2144376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b878));
    }
}
#[inline(always)]
pub fn block_0x0020b878(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(11usize, 28usize, 23usize, 2144380u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2144380u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b87c));
}
#[inline(never)]
pub fn block_0x0020b87c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 30u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(28usize, 2usize, 4u32, 2144384u32)?;
    emu.adr_no_count(27usize, 27usize, 14usize, 2144388u32);
    emu.adr_no_count(23usize, 10usize, 11usize, 2144392u32);
    emu.mulhu_no_count(11usize, 5usize, 24usize, 2144396u32);
    emu.mul_no_count(12usize, 30usize, 24usize, 2144400u32);
    emu.mulhu_no_count(14usize, 30usize, 24usize, 2144404u32);
    emu.mul_no_count(28usize, 5usize, 26usize, 2144408u32);
    emu.sltru_no_count(10usize, 23usize, 10usize, 2144412u32);
    emu.adr_no_count(6usize, 6usize, 10usize, 2144416u32);
    emu.mulhu_no_count(10usize, 5usize, 26usize, 2144420u32);
    emu.adr_no_count(11usize, 12usize, 11usize, 2144424u32);
    emu.sltru_no_count(12usize, 11usize, 12usize, 2144428u32);
    emu.adr_no_count(12usize, 14usize, 12usize, 2144432u32);
    emu.mul_no_count(14usize, 30usize, 26usize, 2144436u32);
    emu.adr_no_count(11usize, 28usize, 11usize, 2144440u32);
    emu.sltru_no_count(28usize, 11usize, 28usize, 2144444u32);
    emu.adr_no_count(10usize, 10usize, 28usize, 2144448u32);
    emu.mulhu_no_count(28usize, 30usize, 26usize, 2144452u32);
    emu.adr_no_count(8usize, 12usize, 10usize, 2144456u32);
    emu.sltru_no_count(10usize, 8usize, 12usize, 2144460u32);
    emu.adr_no_count(9usize, 28usize, 10usize, 2144464u32);
    emu.mul_no_count(28usize, 5usize, 24usize, 2144468u32);
    emu.adr_no_count(28usize, 18usize, 28usize, 2144472u32);
    emu.sltru_no_count(10usize, 28usize, 18usize, 2144476u32);
    emu.adr_no_count(11usize, 11usize, 10usize, 2144480u32);
    emu.adr_no_count(12usize, 14usize, 8usize, 2144484u32);
    emu.sltru_no_count(8usize, 12usize, 14usize, 2144488u32);
    emu.adr_no_count(11usize, 19usize, 11usize, 2144492u32);
    emu.adr_no_count(8usize, 9usize, 8usize, 2144496u32);
    emu.add_memory_rw_events(29usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2144504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b8f8));
    } else {
        emu.pc = 2144500u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b8f4));
    }
}
#[inline(always)]
pub fn block_0x0020b8f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 11usize, 19usize, 2144504u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2144504u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b8f8));
}
#[inline]
pub fn block_0x0020b8f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(14usize, 27usize, 15usize, 2144508u32);
    emu.adr_no_count(10usize, 12usize, 10usize, 2144512u32);
    emu.adr_no_count(6usize, 11usize, 6usize, 2144516u32);
    emu.adr_no_count(23usize, 28usize, 23usize, 2144520u32);
    emu.sltru_no_count(12usize, 10usize, 12usize, 2144524u32);
    emu.sltru_no_count(25usize, 23usize, 28usize, 2144528u32);
    emu.adr_no_count(18usize, 6usize, 25usize, 2144532u32);
    emu.adr_no_count(8usize, 8usize, 12usize, 2144536u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2144544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b920));
    } else {
        emu.pc = 2144540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b91c));
    }
}
#[inline(always)]
pub fn block_0x0020b91c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(25usize, 18usize, 11usize, 2144544u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2144544u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b920));
}
#[inline]
pub fn block_0x0020b920(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(28usize, 29usize, 1u32, 2144548u32);
    emu.adr_no_count(25usize, 10usize, 25usize, 2144552u32);
    emu.sbr_no_count(29usize, 0usize, 30usize, 2144556u32);
    emu.mulhu_no_count(11usize, 5usize, 7usize, 2144560u32);
    emu.mulhu_no_count(6usize, 30usize, 7usize, 2144564u32);
    emu.sbr_no_count(12usize, 0usize, 5usize, 2144568u32);
    emu.sltru_no_count(10usize, 25usize, 10usize, 2144572u32);
    emu.sbr_no_count(9usize, 11usize, 30usize, 2144576u32);
    emu.adr_no_count(10usize, 8usize, 10usize, 2144580u32);
    emu.sbr_no_count(8usize, 9usize, 5usize, 2144584u32);
    emu.sltru_no_count(12usize, 8usize, 12usize, 2144588u32);
    emu.adr_no_count(12usize, 11usize, 12usize, 2144592u32);
    emu.sbr_no_count(7usize, 31usize, 5usize, 2144596u32);
    emu.sltru_no_count(1usize, 7usize, 31usize, 2144600u32);
    emu.sltru_no_count(31usize, 9usize, 29usize, 2144604u32);
    emu.adr_no_count(31usize, 6usize, 31usize, 2144608u32);
    emu.adr_no_count(11usize, 8usize, 1usize, 2144612u32);
    emu.adr_no_count(11usize, 14usize, 11usize, 2144616u32);
    emu.adr_no_count(8usize, 31usize, 12usize, 2144620u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2144628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b974));
    } else {
        emu.pc = 2144624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b970));
    }
}
#[inline(always)]
pub fn block_0x0020b970(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(1usize, 11usize, 14usize, 2144628u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2144628u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b974));
}
#[inline]
pub fn block_0x0020b974(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(20usize, 2usize, 40u32, 2144632u32)?;
    emu.sltru_no_count(12usize, 14usize, 27usize, 2144636u32);
    emu.adi_no_count(28usize, 28usize, 4294967295u32, 2144640u32);
    emu.sbr_no_count(9usize, 8usize, 30usize, 2144644u32);
    emu.adr_no_count(10usize, 11usize, 10usize, 2144648u32);
    emu.adr_no_count(25usize, 7usize, 25usize, 2144652u32);
    emu.sltru_no_count(22usize, 25usize, 7usize, 2144656u32);
    emu.adr_no_count(19usize, 10usize, 22usize, 2144660u32);
    emu.sltru_no_count(14usize, 8usize, 31usize, 2144664u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2144672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b9a0));
    } else {
        emu.pc = 2144668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b99c));
    }
}
#[inline(always)]
pub fn block_0x0020b99c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(22usize, 19usize, 11usize, 2144672u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2144672u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b9a0));
}
#[inline(never)]
pub fn block_0x0020b9a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 33u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(7usize, 28usize, 12usize, 2144676u32);
    emu.sltru_no_count(20usize, 9usize, 29usize, 2144680u32);
    emu.lw_no_count(12usize, 2usize, 112u32, 2144684u32)?;
    emu.lw_no_count(27usize, 2usize, 116u32, 2144688u32)?;
    emu.adr_no_count(14usize, 6usize, 14usize, 2144692u32);
    emu.adr_no_count(1usize, 9usize, 1usize, 2144696u32);
    emu.mulhu_no_count(10usize, 12usize, 13usize, 2144700u32);
    emu.mul_no_count(11usize, 27usize, 13usize, 2144704u32);
    emu.mulhu_no_count(6usize, 27usize, 13usize, 2144708u32);
    emu.lw_no_count(30usize, 2usize, 92u32, 2144712u32)?;
    emu.mul_no_count(28usize, 12usize, 30usize, 2144716u32);
    emu.mulhu_no_count(29usize, 12usize, 30usize, 2144720u32);
    emu.adr_no_count(10usize, 11usize, 10usize, 2144724u32);
    emu.sltru_no_count(11usize, 10usize, 11usize, 2144728u32);
    emu.adr_no_count(11usize, 6usize, 11usize, 2144732u32);
    emu.mul_no_count(6usize, 27usize, 30usize, 2144736u32);
    emu.adr_no_count(10usize, 28usize, 10usize, 2144740u32);
    emu.sltru_no_count(28usize, 10usize, 28usize, 2144744u32);
    emu.adr_no_count(28usize, 29usize, 28usize, 2144748u32);
    emu.mulhu_no_count(29usize, 27usize, 30usize, 2144752u32);
    emu.adr_no_count(28usize, 11usize, 28usize, 2144756u32);
    emu.sltru_no_count(11usize, 28usize, 11usize, 2144760u32);
    emu.adr_no_count(11usize, 29usize, 11usize, 2144764u32);
    emu.mul_no_count(29usize, 12usize, 13usize, 2144768u32);
    emu.adr_no_count(29usize, 23usize, 29usize, 2144772u32);
    emu.sw_no_count(29usize, 2usize, 0u32, 2144776u32)?;
    emu.sltru_no_count(8usize, 29usize, 23usize, 2144780u32);
    emu.adr_no_count(30usize, 10usize, 8usize, 2144784u32);
    emu.adr_no_count(10usize, 6usize, 28usize, 2144788u32);
    emu.sltru_no_count(6usize, 10usize, 6usize, 2144792u32);
    emu.adr_no_count(30usize, 18usize, 30usize, 2144796u32);
    emu.adr_no_count(11usize, 11usize, 6usize, 2144800u32);
    emu.add_memory_rw_events(32usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(30usize);
    if a == b {
        emu.pc = 2144808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ba28));
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
#[inline(always)]
pub fn block_0x0020ba24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(8usize, 30usize, 18usize, 2144808u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2144808u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ba28));
}
#[inline(never)]
pub fn block_0x0020ba28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 29u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(6usize, 17usize, 7usize, 2144812u32);
    emu.adr_no_count(20usize, 14usize, 20usize, 2144816u32);
    emu.sltru_no_count(14usize, 1usize, 9usize, 2144820u32);
    emu.adr_no_count(23usize, 1usize, 22usize, 2144824u32);
    emu.adr_no_count(8usize, 10usize, 8usize, 2144828u32);
    emu.mulhu_no_count(28usize, 12usize, 24usize, 2144832u32);
    emu.mul_no_count(29usize, 27usize, 24usize, 2144836u32);
    emu.mulhu_no_count(31usize, 27usize, 24usize, 2144840u32);
    emu.mul_no_count(9usize, 12usize, 26usize, 2144844u32);
    emu.mulhu_no_count(21usize, 12usize, 26usize, 2144848u32);
    emu.sltru_no_count(7usize, 8usize, 10usize, 2144852u32);
    emu.adr_no_count(7usize, 11usize, 7usize, 2144856u32);
    emu.mul_no_count(11usize, 27usize, 26usize, 2144860u32);
    emu.adr_no_count(10usize, 29usize, 28usize, 2144864u32);
    emu.sltru_no_count(28usize, 10usize, 29usize, 2144868u32);
    emu.adr_no_count(31usize, 31usize, 28usize, 2144872u32);
    emu.mul_no_count(28usize, 12usize, 24usize, 2144876u32);
    emu.adr_no_count(28usize, 25usize, 28usize, 2144880u32);
    emu.adr_no_count(10usize, 9usize, 10usize, 2144884u32);
    emu.sltru_no_count(18usize, 28usize, 25usize, 2144888u32);
    emu.sltru_no_count(29usize, 10usize, 9usize, 2144892u32);
    emu.adr_no_count(10usize, 10usize, 18usize, 2144896u32);
    emu.adr_no_count(29usize, 21usize, 29usize, 2144900u32);
    emu.adr_no_count(9usize, 19usize, 10usize, 2144904u32);
    emu.adr_no_count(29usize, 31usize, 29usize, 2144908u32);
    emu.adr_no_count(10usize, 11usize, 29usize, 2144912u32);
    emu.sltru_no_count(31usize, 29usize, 31usize, 2144916u32);
    emu.mulhu_no_count(29usize, 27usize, 26usize, 2144920u32);
    emu.add_memory_rw_events(28usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2144928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020baa0));
    } else {
        emu.pc = 2144924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ba9c));
    }
}
#[inline(always)]
pub fn block_0x0020ba9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(18usize, 9usize, 19usize, 2144928u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2144928u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020baa0));
}
#[inline]
pub fn block_0x0020baa0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(26usize, 2usize, 56u32, 2144932u32)?;
    emu.sw_no_count(24usize, 2usize, 60u32, 2144936u32)?;
    emu.sltru_no_count(21usize, 10usize, 11usize, 2144940u32);
    emu.adr_no_count(31usize, 29usize, 31usize, 2144944u32);
    emu.sltru_no_count(19usize, 6usize, 17usize, 2144948u32);
    emu.adr_no_count(11usize, 20usize, 14usize, 2144952u32);
    emu.sltru_no_count(29usize, 23usize, 1usize, 2144956u32);
    emu.adr_no_count(7usize, 9usize, 7usize, 2144960u32);
    emu.adr_no_count(14usize, 28usize, 8usize, 2144964u32);
    emu.sltru_no_count(25usize, 14usize, 28usize, 2144968u32);
    emu.adr_no_count(7usize, 7usize, 25usize, 2144972u32);
    emu.adr_no_count(17usize, 10usize, 18usize, 2144976u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a == b {
        emu.pc = 2144984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bad8));
    } else {
        emu.pc = 2144980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bad4));
    }
}
#[inline(always)]
pub fn block_0x0020bad4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(25usize, 7usize, 9usize, 2144984u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2144984u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bad8));
}
#[inline(never)]
pub fn block_0x0020bad8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 30u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 31usize, 21usize, 2144988u32);
    emu.sltru_no_count(20usize, 17usize, 10usize, 2144992u32);
    emu.mul_no_count(10usize, 15usize, 13usize, 2144996u32);
    emu.sw_no_count(10usize, 2usize, 68u32, 2145000u32)?;
    emu.adr_no_count(16usize, 16usize, 19usize, 2145004u32);
    emu.lw_no_count(24usize, 2usize, 120u32, 2145008u32)?;
    emu.lw_no_count(26usize, 2usize, 124u32, 2145012u32)?;
    emu.adr_no_count(9usize, 11usize, 29usize, 2145016u32);
    emu.adr_no_count(25usize, 17usize, 25usize, 2145020u32);
    emu.mulhu_no_count(10usize, 24usize, 13usize, 2145024u32);
    emu.mul_no_count(11usize, 26usize, 13usize, 2145028u32);
    emu.mulhu_no_count(15usize, 26usize, 13usize, 2145032u32);
    emu.lw_no_count(22usize, 2usize, 92u32, 2145036u32)?;
    emu.mul_no_count(28usize, 24usize, 22usize, 2145040u32);
    emu.mulhu_no_count(29usize, 24usize, 22usize, 2145044u32);
    emu.adr_no_count(31usize, 11usize, 10usize, 2145048u32);
    emu.sltru_no_count(10usize, 31usize, 11usize, 2145052u32);
    emu.adr_no_count(10usize, 15usize, 10usize, 2145056u32);
    emu.sw_no_count(13usize, 2usize, 12u32, 2145060u32)?;
    emu.mul_no_count(18usize, 24usize, 13usize, 2145064u32);
    emu.adr_no_count(18usize, 14usize, 18usize, 2145068u32);
    emu.adr_no_count(31usize, 28usize, 31usize, 2145072u32);
    emu.sltru_no_count(21usize, 18usize, 14usize, 2145076u32);
    emu.sltru_no_count(11usize, 31usize, 28usize, 2145080u32);
    emu.adr_no_count(31usize, 31usize, 21usize, 2145084u32);
    emu.adr_no_count(11usize, 29usize, 11usize, 2145088u32);
    emu.adr_no_count(19usize, 7usize, 31usize, 2145092u32);
    emu.adr_no_count(11usize, 10usize, 11usize, 2145096u32);
    emu.mul_no_count(14usize, 26usize, 22usize, 2145100u32);
    emu.add_memory_rw_events(29usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2145108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bb54));
    } else {
        emu.pc = 2145104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bb50));
    }
}
#[inline(always)]
pub fn block_0x0020bb50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(21usize, 19usize, 7usize, 2145108u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2145108u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bb54));
}
#[inline(never)]
pub fn block_0x0020bb54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 31u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(15usize, 8usize, 20usize, 2145112u32);
    emu.sltru_no_count(28usize, 25usize, 17usize, 2145116u32);
    emu.adr_no_count(20usize, 14usize, 11usize, 2145120u32);
    emu.sltru_no_count(1usize, 11usize, 10usize, 2145124u32);
    emu.lw_no_count(10usize, 2usize, 92u32, 2145128u32)?;
    emu.mulhu_no_count(22usize, 26usize, 10usize, 2145132u32);
    let a = 0u32.wrapping_add(1491623936u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2145136u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1125711872u32);
    emu.write_reg_no_count(29usize, a);
    emu.pc = 2145140u32;
    emu.update_insn_clock();
    emu.lw_no_count(8usize, 2usize, 128u32, 2145144u32)?;
    emu.lw_no_count(10usize, 2usize, 132u32, 2145148u32)?;
    let a = 0u32.wrapping_add(60612608u32);
    emu.write_reg_no_count(31usize, a);
    emu.pc = 2145152u32;
    emu.update_insn_clock();
    emu.adr_no_count(7usize, 6usize, 23usize, 2145156u32);
    emu.lw_no_count(23usize, 2usize, 72u32, 2145160u32)?;
    emu.lw_no_count(11usize, 2usize, 68u32, 2145164u32)?;
    emu.sltru_no_count(23usize, 23usize, 11usize, 2145168u32);
    emu.adi_no_count(11usize, 17usize, 380u32, 2145172u32);
    emu.adi_no_count(17usize, 29usize, 1362u32, 2145176u32);
    emu.adi_no_count(13usize, 31usize, 4294965935u32, 2145180u32);
    emu.sltru_no_count(29usize, 7usize, 6usize, 2145184u32);
    emu.mulhu_no_count(31usize, 24usize, 11usize, 2145188u32);
    emu.mul_no_count(6usize, 26usize, 11usize, 2145192u32);
    emu.adr_no_count(31usize, 6usize, 31usize, 2145196u32);
    emu.sw_no_count(17usize, 2usize, 20u32, 2145200u32)?;
    emu.mul_no_count(26usize, 24usize, 17usize, 2145204u32);
    emu.sw_no_count(11usize, 2usize, 16u32, 2145208u32)?;
    emu.mul_no_count(24usize, 24usize, 11usize, 2145212u32);
    emu.adr_no_count(16usize, 16usize, 9usize, 2145216u32);
    emu.adi_no_count(6usize, 23usize, 0u32, 2145220u32);
    emu.lw_no_count(9usize, 2usize, 80u32, 2145224u32)?;
    emu.lw_no_count(17usize, 2usize, 76u32, 2145228u32)?;
    emu.add_memory_rw_events(30usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2145244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bbdc));
    } else {
        emu.pc = 2145232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bbd0));
    }
}
#[inline(always)]
pub fn block_0x0020bbd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(6usize, 2usize, 80u32, 2145236u32)?;
    emu.lw_no_count(9usize, 2usize, 76u32, 2145240u32)?;
    emu.sltru_no_count(6usize, 6usize, 9usize, 2145244u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2145244u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bbdc));
}
#[inline]
pub fn block_0x0020bbdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 22u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(28usize, 15usize, 28usize, 2145248u32);
    emu.sltru_no_count(14usize, 20usize, 14usize, 2145252u32);
    emu.adr_no_count(1usize, 22usize, 1usize, 2145256u32);
    emu.adr_no_count(21usize, 20usize, 21usize, 2145260u32);
    emu.adr_no_count(9usize, 26usize, 31usize, 2145264u32);
    emu.mulhu_no_count(26usize, 8usize, 13usize, 2145268u32);
    emu.sw_no_count(13usize, 2usize, 84u32, 2145272u32)?;
    emu.mul_no_count(15usize, 10usize, 13usize, 2145276u32);
    let a = 0u32.wrapping_add(205926400u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2145280u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1341u32, 2145284u32);
    emu.sw_no_count(10usize, 2usize, 88u32, 2145288u32)?;
    emu.adr_no_count(27usize, 5usize, 27usize, 2145292u32);
    emu.adr_no_count(31usize, 12usize, 24usize, 2145296u32);
    emu.adr_no_count(16usize, 16usize, 29usize, 2145300u32);
    emu.lw_no_count(24usize, 2usize, 28u32, 2145304u32)?;
    emu.lw_no_count(13usize, 2usize, 8u32, 2145308u32)?;
    emu.sltru_no_count(22usize, 24usize, 13usize, 2145312u32);
    emu.adr_no_count(25usize, 7usize, 25usize, 2145316u32);
    emu.adi_no_count(29usize, 22usize, 0u32, 2145320u32);
    emu.lw_no_count(10usize, 2usize, 24u32, 2145324u32)?;
    emu.lw_no_count(17usize, 2usize, 4u32, 2145328u32)?;
    emu.add_memory_rw_events(21usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2145336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bc38));
    } else {
        emu.pc = 2145332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bc34));
    }
}
#[inline(always)]
pub fn block_0x0020bc34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(29usize, 10usize, 17usize, 2145336u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2145336u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bc38));
}
#[inline(never)]
pub fn block_0x0020bc38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 26u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(14usize, 1usize, 14usize, 2145340u32);
    emu.adi_no_count(1usize, 10usize, 0u32, 2145344u32);
    emu.sltru_no_count(10usize, 21usize, 20usize, 2145348u32);
    emu.adr_no_count(15usize, 15usize, 26usize, 2145352u32);
    emu.lw_no_count(11usize, 2usize, 88u32, 2145356u32)?;
    emu.mul_no_count(11usize, 8usize, 11usize, 2145360u32);
    emu.lw_no_count(5usize, 2usize, 84u32, 2145364u32)?;
    emu.mul_no_count(5usize, 8usize, 5usize, 2145368u32);
    emu.adr_no_count(8usize, 27usize, 9usize, 2145372u32);
    emu.sltru_no_count(20usize, 31usize, 12usize, 2145376u32);
    emu.adr_no_count(9usize, 16usize, 28usize, 2145380u32);
    emu.sltru_no_count(7usize, 25usize, 7usize, 2145384u32);
    emu.sbr_no_count(12usize, 0usize, 29usize, 2145388u32);
    emu.sbr_no_count(16usize, 1usize, 17usize, 2145392u32);
    emu.sbr_no_count(26usize, 24usize, 13usize, 2145396u32);
    emu.adr_no_count(29usize, 6usize, 29usize, 2145400u32);
    emu.sbr_no_count(24usize, 16usize, 22usize, 2145404u32);
    emu.sbr_no_count(22usize, 0usize, 29usize, 2145408u32);
    emu.sbr_no_count(28usize, 26usize, 6usize, 2145412u32);
    emu.sltru_no_count(16usize, 22usize, 12usize, 2145416u32);
    emu.sltru_no_count(26usize, 28usize, 26usize, 2145420u32);
    emu.sbr_no_count(12usize, 6usize, 26usize, 2145424u32);
    emu.sbr_no_count(12usize, 24usize, 12usize, 2145428u32);
    emu.sbr_no_count(16usize, 16usize, 29usize, 2145432u32);
    emu.lw_no_count(13usize, 2usize, 12u32, 2145436u32)?;
    emu.add_memory_rw_events(25usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2145444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bca4));
    } else {
        emu.pc = 2145440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bca0));
    }
}
#[inline(always)]
pub fn block_0x0020bca0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(26usize, 12usize, 24usize, 2145444u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2145444u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bca4));
}
#[inline]
pub fn block_0x0020bca4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(24usize, 14usize, 10usize, 2145448u32);
    emu.adr_no_count(10usize, 11usize, 15usize, 2145452u32);
    emu.adr_no_count(8usize, 8usize, 20usize, 2145456u32);
    emu.adr_no_count(5usize, 31usize, 5usize, 2145460u32);
    emu.adr_no_count(15usize, 9usize, 7usize, 2145464u32);
    emu.adr_no_count(6usize, 25usize, 21usize, 2145468u32);
    emu.sbr_no_count(11usize, 26usize, 29usize, 2145472u32);
    emu.sltru_no_count(11usize, 11usize, 22usize, 2145476u32);
    emu.adr_no_count(11usize, 16usize, 11usize, 2145480u32);
    emu.lw_no_count(9usize, 2usize, 36u32, 2145484u32)?;
    emu.lw_no_count(17usize, 2usize, 0u32, 2145488u32)?;
    emu.sltru_no_count(29usize, 9usize, 17usize, 2145492u32);
    emu.sai_no_count(7usize, 11usize, 1055u32, 2145496u32);
    emu.adi_no_count(16usize, 29usize, 0u32, 2145500u32);
    emu.lw_no_count(20usize, 2usize, 32u32, 2145504u32)?;
    emu.add_memory_rw_events(15usize);
    let b = emu.read_reg_b_tracked(30usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2145512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bce8));
    } else {
        emu.pc = 2145508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bce4));
    }
}
#[inline(always)]
pub fn block_0x0020bce4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 20usize, 30usize, 2145512u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2145512u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bce8));
}
#[inline]
pub fn block_0x0020bce8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 8usize, 10usize, 2145516u32);
    emu.sltru_no_count(11usize, 5usize, 31usize, 2145520u32);
    emu.sltru_no_count(14usize, 6usize, 25usize, 2145524u32);
    emu.adr_no_count(15usize, 15usize, 24usize, 2145528u32);
    emu.sbr_no_count(8usize, 0usize, 16usize, 2145532u32);
    emu.sbr_no_count(30usize, 20usize, 30usize, 2145536u32);
    emu.sbr_no_count(9usize, 9usize, 17usize, 2145540u32);
    emu.sbr_no_count(16usize, 7usize, 16usize, 2145544u32);
    emu.sbr_no_count(31usize, 30usize, 29usize, 2145548u32);
    emu.sltru_no_count(8usize, 16usize, 8usize, 2145552u32);
    emu.adr_no_count(30usize, 9usize, 7usize, 2145556u32);
    emu.sltru_no_count(29usize, 30usize, 9usize, 2145560u32);
    emu.adr_no_count(7usize, 7usize, 29usize, 2145564u32);
    emu.adr_no_count(24usize, 31usize, 7usize, 2145568u32);
    emu.adr_no_count(7usize, 16usize, 8usize, 2145572u32);
    emu.lw_no_count(25usize, 2usize, 56u32, 2145576u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2145580u32)?;
    emu.lw_no_count(26usize, 2usize, 16u32, 2145584u32)?;
    emu.add_memory_rw_events(18usize);
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(24usize);
    if a == b {
        emu.pc = 2145592u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bd38));
    } else {
        emu.pc = 2145588u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bd34));
    }
}
#[inline(always)]
pub fn block_0x0020bd34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(29usize, 24usize, 31usize, 2145592u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2145592u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bd38));
}
#[inline]
pub fn block_0x0020bd38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(11usize, 10usize, 11usize, 2145596u32);
    emu.adr_no_count(15usize, 15usize, 14usize, 2145600u32);
    emu.lw_no_count(10usize, 2usize, 80u32, 2145604u32)?;
    emu.lw_no_count(14usize, 2usize, 76u32, 2145608u32)?;
    emu.sbr_no_count(10usize, 10usize, 14usize, 2145612u32);
    emu.adr_no_count(29usize, 16usize, 29usize, 2145616u32);
    emu.sltru_no_count(14usize, 29usize, 16usize, 2145620u32);
    emu.adr_no_count(7usize, 7usize, 14usize, 2145624u32);
    emu.lw_no_count(9usize, 2usize, 44u32, 2145628u32)?;
    emu.sltru_no_count(14usize, 9usize, 18usize, 2145632u32);
    emu.sai_no_count(29usize, 7usize, 1055u32, 2145636u32);
    emu.adi_no_count(31usize, 14usize, 0u32, 2145640u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2145648u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bd70));
    } else {
        emu.pc = 2145644u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bd6c));
    }
}
#[inline(always)]
pub fn block_0x0020bd6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(31usize, 20usize, 19usize, 2145648u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2145648u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bd70));
}
#[inline]
pub fn block_0x0020bd70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 11usize, 15usize, 2145652u32);
    emu.sltru_no_count(7usize, 5usize, 6usize, 2145656u32);
    emu.sbr_no_count(16usize, 5usize, 6usize, 2145660u32);
    emu.sbr_no_count(15usize, 10usize, 23usize, 2145664u32);
    emu.lw_no_count(10usize, 2usize, 72u32, 2145668u32)?;
    emu.lw_no_count(17usize, 2usize, 68u32, 2145672u32)?;
    emu.sbr_no_count(10usize, 10usize, 17usize, 2145676u32);
    emu.sbr_no_count(8usize, 0usize, 31usize, 2145680u32);
    emu.sbr_no_count(5usize, 20usize, 19usize, 2145684u32);
    emu.sbr_no_count(9usize, 9usize, 18usize, 2145688u32);
    emu.sbr_no_count(6usize, 29usize, 31usize, 2145692u32);
    emu.sbr_no_count(5usize, 5usize, 14usize, 2145696u32);
    emu.sltru_no_count(8usize, 6usize, 8usize, 2145700u32);
    emu.adr_no_count(14usize, 9usize, 29usize, 2145704u32);
    emu.sltru_no_count(31usize, 14usize, 9usize, 2145708u32);
    emu.adr_no_count(29usize, 29usize, 31usize, 2145712u32);
    emu.adr_no_count(18usize, 5usize, 29usize, 2145716u32);
    emu.adr_no_count(29usize, 6usize, 8usize, 2145720u32);
    emu.add_memory_rw_events(18usize);
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2145728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bdc0));
    } else {
        emu.pc = 2145724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bdbc));
    }
}
#[inline(always)]
pub fn block_0x0020bdbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(31usize, 18usize, 5usize, 2145728u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2145728u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bdc0));
}
#[inline]
pub fn block_0x0020bdc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 11usize, 7usize, 2145732u32);
    emu.adr_no_count(31usize, 6usize, 31usize, 2145736u32);
    emu.lw_no_count(5usize, 2usize, 84u32, 2145740u32)?;
    emu.adr_no_count(5usize, 10usize, 5usize, 2145744u32);
    emu.sltru_no_count(6usize, 31usize, 6usize, 2145748u32);
    emu.sltru_no_count(10usize, 5usize, 10usize, 2145752u32);
    emu.adr_no_count(29usize, 29usize, 6usize, 2145756u32);
    emu.lw_no_count(6usize, 2usize, 88u32, 2145760u32)?;
    emu.adr_no_count(6usize, 10usize, 6usize, 2145764u32);
    emu.adr_no_count(6usize, 15usize, 6usize, 2145768u32);
    emu.lw_no_count(31usize, 2usize, 52u32, 2145772u32)?;
    emu.adr_no_count(16usize, 31usize, 16usize, 2145776u32);
    emu.lw_no_count(23usize, 2usize, 20u32, 2145780u32)?;
    emu.add_memory_rw_events(13usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2145788u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bdfc));
    } else {
        emu.pc = 2145784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bdf8));
    }
}
#[inline(always)]
pub fn block_0x0020bdf8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 6usize, 15usize, 2145788u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2145788u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bdfc));
}
#[inline]
pub fn block_0x0020bdfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sai_no_count(7usize, 29usize, 1055u32, 2145792u32);
    emu.lw_no_count(15usize, 2usize, 48u32, 2145796u32)?;
    emu.adr_no_count(11usize, 15usize, 11usize, 2145800u32);
    emu.adi_no_count(15usize, 10usize, 4294967295u32, 2145804u32);
    emu.adr_no_count(28usize, 15usize, 28usize, 2145808u32);
    emu.sltru_no_count(29usize, 28usize, 15usize, 2145812u32);
    emu.adr_no_count(10usize, 15usize, 29usize, 2145816u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2145820u32);
    emu.sltru_no_count(8usize, 16usize, 31usize, 2145824u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2145832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020be28));
    } else {
        emu.pc = 2145828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020be24));
    }
}
#[inline(always)]
pub fn block_0x0020be24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(29usize, 10usize, 15usize, 2145832u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2145832u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020be28));
}
#[inline]
pub fn block_0x0020be28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 11usize, 8usize, 2145836u32);
    emu.adr_no_count(11usize, 15usize, 29usize, 2145840u32);
    emu.adr_no_count(29usize, 28usize, 26usize, 2145844u32);
    emu.sltru_no_count(31usize, 11usize, 15usize, 2145848u32);
    emu.sltru_no_count(12usize, 29usize, 28usize, 2145852u32);
    emu.adr_no_count(15usize, 15usize, 31usize, 2145856u32);
    emu.adr_no_count(31usize, 12usize, 23usize, 2145860u32);
    emu.adr_no_count(31usize, 10usize, 31usize, 2145864u32);
    emu.adr_no_count(28usize, 16usize, 7usize, 2145868u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(31usize);
    if a == b {
        emu.pc = 2145876u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020be54));
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
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 31usize, 10usize, 2145876u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2145876u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020be54));
}
#[inline]
pub fn block_0x0020be54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(7usize, 8usize, 7usize, 2145880u32);
    emu.adi_no_count(10usize, 11usize, 4294967295u32, 2145884u32);
    emu.sltiu_no_count(11usize, 11usize, 1u32, 2145888u32);
    emu.adr_no_count(12usize, 10usize, 12usize, 2145892u32);
    emu.sbr_no_count(15usize, 15usize, 11usize, 2145896u32);
    emu.sltru_no_count(10usize, 12usize, 10usize, 2145900u32);
    emu.adr_no_count(10usize, 15usize, 10usize, 2145904u32);
    emu.sai_no_count(10usize, 10usize, 1055u32, 2145908u32);
    emu.adr_no_count(30usize, 10usize, 30usize, 2145912u32);
    emu.sltru_no_count(11usize, 30usize, 10usize, 2145916u32);
    emu.adr_no_count(19usize, 10usize, 24usize, 2145920u32);
    emu.adr_no_count(19usize, 19usize, 11usize, 2145924u32);
    emu.sltru_no_count(15usize, 28usize, 16usize, 2145928u32);
    emu.add_memory_rw_events(13usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2145936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020be90));
    } else {
        emu.pc = 2145932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020be8c));
    }
}
#[inline(always)]
pub fn block_0x0020be8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(11usize, 19usize, 10usize, 2145936u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2145936u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020be90));
}
#[inline]
pub fn block_0x0020be90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 22u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(11usize, 10usize, 11usize, 2145940u32);
    emu.adi_no_count(9usize, 30usize, 1u32, 2145944u32);
    emu.sltru_no_count(12usize, 11usize, 10usize, 2145948u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2145952u32);
    emu.sltiu_no_count(12usize, 9usize, 1u32, 2145956u32);
    emu.adr_no_count(19usize, 19usize, 12usize, 2145960u32);
    emu.adi_no_count(12usize, 11usize, 4294967295u32, 2145964u32);
    emu.sltiu_no_count(11usize, 11usize, 1u32, 2145968u32);
    emu.sbr_no_count(10usize, 10usize, 11usize, 2145972u32);
    emu.orr_no_count(11usize, 9usize, 19usize, 2145976u32);
    emu.sltiu_no_count(11usize, 11usize, 1u32, 2145980u32);
    emu.adr_no_count(11usize, 12usize, 11usize, 2145984u32);
    emu.sltru_no_count(11usize, 11usize, 12usize, 2145988u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2145992u32);
    emu.sai_no_count(10usize, 10usize, 1055u32, 2145996u32);
    emu.adr_no_count(30usize, 10usize, 14usize, 2146000u32);
    emu.sltru_no_count(12usize, 30usize, 10usize, 2146004u32);
    emu.adr_no_count(11usize, 10usize, 18usize, 2146008u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2146012u32);
    emu.adr_no_count(15usize, 7usize, 15usize, 2146016u32);
    emu.lw_no_count(22usize, 2usize, 60u32, 2146020u32)?;
    emu.add_memory_rw_events(21usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2146028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020beec));
    } else {
        emu.pc = 2146024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bee8));
    }
}
#[inline(always)]
pub fn block_0x0020bee8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 11usize, 10usize, 2146028u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2146028u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020beec));
}
#[inline]
pub fn block_0x0020beec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(12usize, 10usize, 12usize, 2146032u32);
    emu.adi_no_count(18usize, 11usize, 1u32, 2146036u32);
    emu.sltru_no_count(14usize, 12usize, 10usize, 2146040u32);
    emu.adr_no_count(10usize, 10usize, 14usize, 2146044u32);
    emu.adi_no_count(14usize, 12usize, 4294967295u32, 2146048u32);
    emu.sltru_no_count(11usize, 18usize, 11usize, 2146052u32);
    emu.sltiu_no_count(12usize, 12usize, 1u32, 2146056u32);
    emu.adr_no_count(11usize, 14usize, 11usize, 2146060u32);
    emu.sltru_no_count(11usize, 11usize, 14usize, 2146064u32);
    emu.sbr_no_count(10usize, 10usize, 12usize, 2146068u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2146072u32);
    emu.sai_no_count(20usize, 10usize, 1055u32, 2146076u32);
    emu.adr_no_count(28usize, 20usize, 28usize, 2146080u32);
    emu.sltru_no_count(7usize, 28usize, 20usize, 2146084u32);
    emu.adr_no_count(14usize, 20usize, 15usize, 2146088u32);
    emu.adr_no_count(14usize, 14usize, 7usize, 2146092u32);
    emu.add_memory_rw_events(16usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2146100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bf34));
    } else {
        emu.pc = 2146096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bf30));
    }
}
#[inline(always)]
pub fn block_0x0020bf30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(7usize, 14usize, 20usize, 2146100u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2146100u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bf34));
}
#[inline]
pub fn block_0x0020bf34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(7usize, 20usize, 7usize, 2146104u32);
    emu.sltru_no_count(10usize, 7usize, 20usize, 2146108u32);
    emu.anr_no_count(11usize, 7usize, 13usize, 2146112u32);
    emu.adr_no_count(20usize, 20usize, 10usize, 2146116u32);
    emu.adr_no_count(5usize, 11usize, 5usize, 2146120u32);
    emu.lw_no_count(10usize, 2usize, 92u32, 2146124u32)?;
    emu.anr_no_count(10usize, 20usize, 10usize, 2146128u32);
    emu.sltru_no_count(16usize, 5usize, 11usize, 2146132u32);
    emu.adr_no_count(6usize, 10usize, 6usize, 2146136u32);
    emu.adr_no_count(21usize, 6usize, 16usize, 2146140u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a == b {
        emu.pc = 2146148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bf64));
    } else {
        emu.pc = 2146144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bf60));
    }
}
#[inline(always)]
pub fn block_0x0020bf60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 21usize, 10usize, 2146148u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2146148u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bf64));
}
#[inline(always)]
pub fn block_0x0020bf64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(11usize, 20usize, 25usize, 2146152u32);
    emu.anr_no_count(10usize, 7usize, 22usize, 2146156u32);
    emu.adr_no_count(29usize, 10usize, 29usize, 2146160u32);
    emu.sltru_no_count(10usize, 29usize, 10usize, 2146164u32);
    emu.adr_no_count(15usize, 11usize, 31usize, 2146168u32);
    emu.adr_no_count(15usize, 15usize, 10usize, 2146172u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2146180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bf84));
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
#[inline(always)]
pub fn block_0x0020bf80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 15usize, 11usize, 2146180u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2146180u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bf84));
}
#[inline]
pub fn block_0x0020bf84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(16usize, 29usize, 16usize, 2146184u32);
    emu.adr_no_count(6usize, 7usize, 9usize, 2146188u32);
    emu.adr_no_count(12usize, 20usize, 19usize, 2146192u32);
    emu.sltru_no_count(29usize, 16usize, 29usize, 2146196u32);
    emu.sltru_no_count(11usize, 6usize, 7usize, 2146200u32);
    emu.adr_no_count(19usize, 15usize, 29usize, 2146204u32);
    emu.sltru_no_count(15usize, 19usize, 15usize, 2146208u32);
    emu.anr_no_count(9usize, 29usize, 15usize, 2146212u32);
    emu.adr_no_count(9usize, 10usize, 9usize, 2146216u32);
    emu.adr_no_count(12usize, 12usize, 11usize, 2146220u32);
    emu.sltru_no_count(10usize, 9usize, 10usize, 2146224u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2146232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bfb8));
    } else {
        emu.pc = 2146228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bfb4));
    }
}
#[inline(always)]
pub fn block_0x0020bfb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(11usize, 12usize, 20usize, 2146232u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2146232u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bfb8));
}
#[inline(always)]
pub fn block_0x0020bfb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(9usize, 6usize, 9usize, 2146236u32);
    emu.sltru_no_count(15usize, 9usize, 6usize, 2146240u32);
    emu.adr_no_count(10usize, 12usize, 10usize, 2146244u32);
    emu.adr_no_count(31usize, 10usize, 15usize, 2146248u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(31usize);
    if a == b {
        emu.pc = 2146256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bfd0));
    } else {
        emu.pc = 2146252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bfcc));
    }
}
#[inline(always)]
pub fn block_0x0020bfcc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(15usize, 31usize, 12usize, 2146256u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2146256u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bfd0));
}
#[inline(always)]
pub fn block_0x0020bfd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(15usize, 11usize, 15usize, 2146260u32);
    emu.adr_no_count(18usize, 20usize, 18usize, 2146264u32);
    emu.sltru_no_count(10usize, 15usize, 11usize, 2146268u32);
    emu.adr_no_count(29usize, 30usize, 15usize, 2146272u32);
    emu.sltru_no_count(8usize, 29usize, 30usize, 2146276u32);
    emu.adr_no_count(10usize, 18usize, 10usize, 2146280u32);
    emu.adr_no_count(6usize, 10usize, 8usize, 2146284u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2146292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bff4));
    } else {
        emu.pc = 2146288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bff0));
    }
}
#[inline(always)]
pub fn block_0x0020bff0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(8usize, 6usize, 18usize, 2146292u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2146292u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bff4));
}
#[inline(always)]
pub fn block_0x0020bff4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(17usize, 2usize, 84u32, 2146296u32)?;
    emu.adr_no_count(17usize, 5usize, 17usize, 2146300u32);
    emu.sltru_no_count(11usize, 17usize, 5usize, 2146304u32);
    emu.lw_no_count(5usize, 2usize, 88u32, 2146308u32)?;
    emu.adr_no_count(5usize, 11usize, 5usize, 2146312u32);
    emu.adr_no_count(5usize, 21usize, 5usize, 2146316u32);
    emu.sltru_no_count(10usize, 18usize, 20usize, 2146320u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2146328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c018));
    } else {
        emu.pc = 2146324u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c014));
    }
}
#[inline(always)]
pub fn block_0x0020c014(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(11usize, 5usize, 21usize, 2146328u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2146328u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c018));
}
#[inline(always)]
pub fn block_0x0020c018(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2146332u32);
    emu.adr_no_count(12usize, 16usize, 11usize, 2146336u32);
    emu.sltru_no_count(15usize, 12usize, 16usize, 2146340u32);
    emu.adr_no_count(18usize, 19usize, 11usize, 2146344u32);
    emu.adr_no_count(18usize, 18usize, 15usize, 2146348u32);
    emu.adr_no_count(8usize, 10usize, 8usize, 2146352u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2146360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c038));
    } else {
        emu.pc = 2146356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c034));
    }
}
#[inline(always)]
pub fn block_0x0020c034(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(15usize, 18usize, 19usize, 2146360u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2146360u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c038));
}
#[inline]
pub fn block_0x0020c038(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(7usize, 8usize, 10usize, 2146364u32);
    emu.adr_no_count(30usize, 8usize, 28usize, 2146368u32);
    emu.adr_no_count(10usize, 11usize, 15usize, 2146372u32);
    emu.adr_no_count(16usize, 12usize, 26usize, 2146376u32);
    emu.sltru_no_count(28usize, 16usize, 12usize, 2146380u32);
    emu.adr_no_count(15usize, 18usize, 28usize, 2146384u32);
    emu.adr_no_count(15usize, 15usize, 23usize, 2146388u32);
    emu.sltru_no_count(12usize, 10usize, 11usize, 2146392u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2146396u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2146404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c064));
    } else {
        emu.pc = 2146400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c060));
    }
}
#[inline(always)]
pub fn block_0x0020c060(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 15usize, 18usize, 2146404u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2146404u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c064));
}
#[inline]
pub fn block_0x0020c064(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(12usize, 10usize, 1u32, 2146408u32);
    emu.sbr_no_count(11usize, 11usize, 12usize, 2146412u32);
    emu.sltru_no_count(8usize, 30usize, 8usize, 2146416u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2146420u32);
    emu.adr_no_count(28usize, 10usize, 28usize, 2146424u32);
    emu.sltru_no_count(10usize, 28usize, 10usize, 2146428u32);
    emu.adr_no_count(10usize, 11usize, 10usize, 2146432u32);
    emu.sai_no_count(10usize, 10usize, 1055u32, 2146436u32);
    emu.adr_no_count(28usize, 31usize, 10usize, 2146440u32);
    emu.adr_no_count(11usize, 9usize, 10usize, 2146444u32);
    emu.sltru_no_count(12usize, 11usize, 9usize, 2146448u32);
    emu.adr_no_count(28usize, 28usize, 12usize, 2146452u32);
    emu.adr_no_count(9usize, 7usize, 14usize, 2146456u32);
    emu.add_memory_rw_events(13usize);
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2146464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c0a0));
    } else {
        emu.pc = 2146460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c09c));
    }
}
#[inline(always)]
pub fn block_0x0020c09c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 28usize, 31usize, 2146464u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2146464u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c0a0));
}
#[inline]
pub fn block_0x0020c0a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(12usize, 10usize, 12usize, 2146468u32);
    emu.adi_no_count(7usize, 11usize, 1u32, 2146472u32);
    emu.sltru_no_count(11usize, 12usize, 10usize, 2146476u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2146480u32);
    emu.sltiu_no_count(11usize, 7usize, 1u32, 2146484u32);
    emu.adr_no_count(28usize, 28usize, 11usize, 2146488u32);
    emu.adi_no_count(11usize, 12usize, 4294967295u32, 2146492u32);
    emu.sltiu_no_count(12usize, 12usize, 1u32, 2146496u32);
    emu.sbr_no_count(10usize, 10usize, 12usize, 2146500u32);
    emu.orr_no_count(12usize, 7usize, 28usize, 2146504u32);
    emu.sltiu_no_count(12usize, 12usize, 1u32, 2146508u32);
    emu.adr_no_count(12usize, 11usize, 12usize, 2146512u32);
    emu.sltru_no_count(11usize, 12usize, 11usize, 2146516u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2146520u32);
    emu.sai_no_count(10usize, 10usize, 1055u32, 2146524u32);
    emu.adr_no_count(11usize, 6usize, 10usize, 2146528u32);
    emu.adr_no_count(14usize, 29usize, 10usize, 2146532u32);
    emu.sltru_no_count(12usize, 14usize, 29usize, 2146536u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2146540u32);
    emu.adr_no_count(8usize, 9usize, 8usize, 2146544u32);
    emu.add_memory_rw_events(20usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2146552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c0f8));
    } else {
        emu.pc = 2146548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c0f4));
    }
}
#[inline(always)]
pub fn block_0x0020c0f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 11usize, 6usize, 2146552u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2146552u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c0f8));
}
#[inline]
pub fn block_0x0020c0f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(12usize, 10usize, 12usize, 2146556u32);
    emu.adi_no_count(6usize, 11usize, 1u32, 2146560u32);
    emu.sltru_no_count(29usize, 12usize, 10usize, 2146564u32);
    emu.sltru_no_count(11usize, 6usize, 11usize, 2146568u32);
    emu.adr_no_count(10usize, 10usize, 29usize, 2146572u32);
    emu.adi_no_count(29usize, 12usize, 4294967295u32, 2146576u32);
    emu.sltiu_no_count(12usize, 12usize, 1u32, 2146580u32);
    emu.adr_no_count(11usize, 29usize, 11usize, 2146584u32);
    emu.sltru_no_count(11usize, 11usize, 29usize, 2146588u32);
    emu.sbr_no_count(10usize, 10usize, 12usize, 2146592u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2146596u32);
    emu.sai_no_count(10usize, 10usize, 1055u32, 2146600u32);
    emu.adr_no_count(30usize, 10usize, 30usize, 2146604u32);
    emu.sltru_no_count(29usize, 30usize, 10usize, 2146608u32);
    emu.adr_no_count(11usize, 10usize, 8usize, 2146612u32);
    emu.adr_no_count(11usize, 11usize, 29usize, 2146616u32);
    emu.add_memory_rw_events(16usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2146624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c140));
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
    emu.sltru_no_count(29usize, 11usize, 10usize, 2146624u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2146624u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c140));
}
#[inline]
pub fn block_0x0020c140(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(29usize, 10usize, 29usize, 2146628u32);
    emu.sltru_no_count(11usize, 29usize, 10usize, 2146632u32);
    emu.anr_no_count(12usize, 29usize, 13usize, 2146636u32);
    emu.adr_no_count(13usize, 10usize, 11usize, 2146640u32);
    emu.adr_no_count(17usize, 12usize, 17usize, 2146644u32);
    emu.lw_no_count(11usize, 2usize, 92u32, 2146648u32)?;
    emu.anr_no_count(11usize, 13usize, 11usize, 2146652u32);
    emu.sltru_no_count(10usize, 17usize, 12usize, 2146656u32);
    emu.adr_no_count(12usize, 11usize, 5usize, 2146660u32);
    emu.adr_no_count(12usize, 12usize, 10usize, 2146664u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(11usize);
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
        emu.pc = 2146668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c16c));
    }
}
#[inline(always)]
pub fn block_0x0020c16c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 12usize, 11usize, 2146672u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2146672u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c170));
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
    emu.anr_no_count(30usize, 13usize, 25usize, 2146676u32);
    emu.anr_no_count(5usize, 29usize, 22usize, 2146680u32);
    emu.adr_no_count(11usize, 5usize, 16usize, 2146684u32);
    emu.sltru_no_count(16usize, 11usize, 5usize, 2146688u32);
    emu.adr_no_count(15usize, 30usize, 15usize, 2146692u32);
    emu.adr_no_count(5usize, 15usize, 16usize, 2146696u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(30usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2146704u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c190));
    } else {
        emu.pc = 2146700u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c18c));
    }
}
#[inline(always)]
pub fn block_0x0020c18c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 5usize, 30usize, 2146704u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2146704u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c190));
}
#[inline]
pub fn block_0x0020c190(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 11usize, 10usize, 2146708u32);
    emu.adr_no_count(7usize, 29usize, 7usize, 2146712u32);
    emu.adr_no_count(28usize, 13usize, 28usize, 2146716u32);
    emu.sltru_no_count(30usize, 10usize, 11usize, 2146720u32);
    emu.sltru_no_count(11usize, 7usize, 29usize, 2146724u32);
    emu.adr_no_count(15usize, 5usize, 30usize, 2146728u32);
    emu.sltru_no_count(5usize, 15usize, 5usize, 2146732u32);
    emu.anr_no_count(5usize, 30usize, 5usize, 2146736u32);
    emu.adr_no_count(5usize, 16usize, 5usize, 2146740u32);
    emu.adr_no_count(28usize, 28usize, 11usize, 2146744u32);
    emu.sltru_no_count(29usize, 5usize, 16usize, 2146748u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2146756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c1c4));
    } else {
        emu.pc = 2146752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c1c0));
    }
}
#[inline(always)]
pub fn block_0x0020c1c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(11usize, 28usize, 13usize, 2146756u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2146756u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c1c4));
}
#[inline(always)]
pub fn block_0x0020c1c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(16usize, 7usize, 5usize, 2146760u32);
    emu.sltru_no_count(7usize, 16usize, 7usize, 2146764u32);
    emu.adr_no_count(5usize, 28usize, 29usize, 2146768u32);
    emu.adr_no_count(5usize, 5usize, 7usize, 2146772u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2146780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c1dc));
    } else {
        emu.pc = 2146776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c1d8));
    }
}
#[inline(always)]
pub fn block_0x0020c1d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(7usize, 5usize, 28usize, 2146780u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2146780u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c1dc));
}
#[inline(never)]
pub fn block_0x0020c1dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 31u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(7usize, 11usize, 7usize, 2146784u32);
    emu.adr_no_count(13usize, 13usize, 6usize, 2146788u32);
    emu.lw_no_count(6usize, 2usize, 64u32, 2146792u32)?;
    emu.sw_no_count(17usize, 6usize, 0u32, 2146796u32)?;
    emu.sw_no_count(12usize, 6usize, 4u32, 2146800u32)?;
    emu.sw_no_count(10usize, 6usize, 8u32, 2146804u32)?;
    emu.sw_no_count(15usize, 6usize, 12u32, 2146808u32)?;
    emu.sltru_no_count(10usize, 7usize, 11usize, 2146812u32);
    emu.adr_no_count(7usize, 14usize, 7usize, 2146816u32);
    emu.sltru_no_count(11usize, 7usize, 14usize, 2146820u32);
    emu.adr_no_count(10usize, 13usize, 10usize, 2146824u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2146828u32);
    emu.sw_no_count(16usize, 6usize, 16u32, 2146832u32)?;
    emu.sw_no_count(5usize, 6usize, 20u32, 2146836u32)?;
    emu.sw_no_count(7usize, 6usize, 24u32, 2146840u32)?;
    emu.sw_no_count(10usize, 6usize, 28u32, 2146844u32)?;
    emu.lw_no_count(1usize, 2usize, 252u32, 2146848u32)?;
    emu.lw_no_count(8usize, 2usize, 248u32, 2146852u32)?;
    emu.lw_no_count(9usize, 2usize, 244u32, 2146856u32)?;
    emu.lw_no_count(18usize, 2usize, 240u32, 2146860u32)?;
    emu.lw_no_count(19usize, 2usize, 236u32, 2146864u32)?;
    emu.lw_no_count(20usize, 2usize, 232u32, 2146868u32)?;
    emu.lw_no_count(21usize, 2usize, 228u32, 2146872u32)?;
    emu.lw_no_count(22usize, 2usize, 224u32, 2146876u32)?;
    emu.lw_no_count(23usize, 2usize, 220u32, 2146880u32)?;
    emu.lw_no_count(24usize, 2usize, 216u32, 2146884u32)?;
    emu.lw_no_count(25usize, 2usize, 212u32, 2146888u32)?;
    emu.lw_no_count(26usize, 2usize, 208u32, 2146892u32)?;
    emu.lw_no_count(27usize, 2usize, 204u32, 2146896u32)?;
    emu.adi_no_count(2usize, 2usize, 256u32, 2146900u32);
    emu.add_memory_rw_events(31usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2146904u32;
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
pub fn block_0x0020c258(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 69u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967088u32, 2146908u32);
    emu.sw_no_count(1usize, 2usize, 204u32, 2146912u32)?;
    emu.sw_no_count(8usize, 2usize, 200u32, 2146916u32)?;
    emu.sw_no_count(9usize, 2usize, 196u32, 2146920u32)?;
    emu.sw_no_count(18usize, 2usize, 192u32, 2146924u32)?;
    emu.sw_no_count(19usize, 2usize, 188u32, 2146928u32)?;
    emu.sw_no_count(20usize, 2usize, 184u32, 2146932u32)?;
    emu.sw_no_count(21usize, 2usize, 180u32, 2146936u32)?;
    emu.sw_no_count(22usize, 2usize, 176u32, 2146940u32)?;
    emu.sw_no_count(23usize, 2usize, 172u32, 2146944u32)?;
    emu.adi_no_count(9usize, 11usize, 0u32, 2146948u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2146952u32);
    emu.lw_no_count(17usize, 11usize, 16u32, 2146956u32)?;
    emu.lw_no_count(16usize, 11usize, 20u32, 2146960u32)?;
    emu.lw_no_count(15usize, 11usize, 24u32, 2146964u32)?;
    emu.lw_no_count(14usize, 11usize, 28u32, 2146968u32)?;
    emu.lw_no_count(13usize, 11usize, 0u32, 2146972u32)?;
    emu.lw_no_count(12usize, 11usize, 4u32, 2146976u32)?;
    emu.lw_no_count(11usize, 11usize, 8u32, 2146980u32)?;
    emu.lw_no_count(10usize, 9usize, 12u32, 2146984u32)?;
    emu.adi_no_count(5usize, 0usize, 4294967295u32, 2146988u32);
    let a = 0u32.wrapping_add(3169255424u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2146992u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2803343360u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2146996u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4089040896u32);
    emu.write_reg_no_count(28usize, a);
    emu.pc = 2147000u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4234354688u32);
    emu.write_reg_no_count(29usize, a);
    emu.pc = 2147004u32;
    emu.update_insn_clock();
    emu.sw_no_count(0usize, 2usize, 92u32, 2147008u32)?;
    emu.sw_no_count(0usize, 2usize, 96u32, 2147012u32)?;
    emu.sw_no_count(0usize, 2usize, 100u32, 2147016u32)?;
    emu.sw_no_count(0usize, 2usize, 104u32, 2147020u32)?;
    emu.sw_no_count(5usize, 2usize, 60u32, 2147024u32)?;
    emu.sw_no_count(5usize, 2usize, 64u32, 2147028u32)?;
    emu.sw_no_count(0usize, 2usize, 68u32, 2147032u32)?;
    emu.sw_no_count(5usize, 2usize, 72u32, 2147036u32)?;
    emu.adi_no_count(5usize, 0usize, 1u32, 2147040u32);
    emu.sw_no_count(0usize, 2usize, 124u32, 2147044u32)?;
    emu.sw_no_count(0usize, 2usize, 128u32, 2147048u32)?;
    emu.sw_no_count(0usize, 2usize, 132u32, 2147052u32)?;
    emu.sw_no_count(0usize, 2usize, 136u32, 2147056u32)?;
    emu.sw_no_count(0usize, 2usize, 108u32, 2147060u32)?;
    emu.sw_no_count(0usize, 2usize, 112u32, 2147064u32)?;
    emu.sw_no_count(0usize, 2usize, 116u32, 2147068u32)?;
    emu.sw_no_count(0usize, 2usize, 120u32, 2147072u32)?;
    emu.adi_no_count(6usize, 6usize, 4294965933u32, 2147076u32);
    emu.adi_no_count(7usize, 7usize, 4294966916u32, 2147080u32);
    emu.adi_no_count(28usize, 28usize, 4294965954u32, 2147084u32);
    emu.adi_no_count(29usize, 29usize, 1361u32, 2147088u32);
    emu.sw_no_count(29usize, 2usize, 44u32, 2147092u32)?;
    emu.sw_no_count(28usize, 2usize, 48u32, 2147096u32)?;
    emu.sw_no_count(7usize, 2usize, 52u32, 2147100u32)?;
    emu.sw_no_count(6usize, 2usize, 56u32, 2147104u32)?;
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(18usize, a);
    emu.pc = 2147108u32;
    emu.update_insn_clock();
    emu.adi_no_count(18usize, 18usize, 4294966600u32, 2147112u32);
    emu.sw_no_count(17usize, 2usize, 28u32, 2147116u32)?;
    emu.sw_no_count(16usize, 2usize, 32u32, 2147120u32)?;
    emu.sw_no_count(15usize, 2usize, 36u32, 2147124u32)?;
    emu.sw_no_count(14usize, 2usize, 40u32, 2147128u32)?;
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2147132u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 19usize, 4294966632u32, 2147136u32);
    emu.sw_no_count(5usize, 2usize, 76u32, 2147140u32)?;
    emu.sw_no_count(0usize, 2usize, 80u32, 2147144u32)?;
    emu.sw_no_count(0usize, 2usize, 84u32, 2147148u32)?;
    emu.sw_no_count(0usize, 2usize, 88u32, 2147152u32)?;
    emu.sw_no_count(13usize, 2usize, 12u32, 2147156u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2147160u32)?;
    emu.sw_no_count(11usize, 2usize, 20u32, 2147164u32)?;
    emu.sw_no_count(10usize, 2usize, 24u32, 2147168u32)?;
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(20usize, a);
    emu.pc = 2147172u32;
    emu.update_insn_clock();
    emu.adi_no_count(20usize, 20usize, 4294966696u32, 2147176u32);
    emu.add_memory_rw_events(69usize);
    let return_addr = 2147180u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2147356u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c41c));
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
    emu.adi_no_count(10usize, 2usize, 140u32, 2147184u32);
    emu.adi_no_count(11usize, 2usize, 12u32, 2147188u32);
    emu.adi_no_count(12usize, 2usize, 44u32, 2147192u32);
    emu.adi_no_count(13usize, 18usize, 0u32, 2147196u32);
    emu.apc_no_count(1usize, 2147196u32, 20480u32, 2147200u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2147204u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(924u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020c384(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 22u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 156u32, 2147208u32)?;
    emu.lw_no_count(11usize, 2usize, 160u32, 2147212u32)?;
    emu.lw_no_count(12usize, 2usize, 164u32, 2147216u32)?;
    emu.lw_no_count(13usize, 2usize, 168u32, 2147220u32)?;
    emu.sw_no_count(10usize, 2usize, 28u32, 2147224u32)?;
    emu.sw_no_count(11usize, 2usize, 32u32, 2147228u32)?;
    emu.sw_no_count(12usize, 2usize, 36u32, 2147232u32)?;
    emu.sw_no_count(13usize, 2usize, 40u32, 2147236u32)?;
    emu.lw_no_count(10usize, 2usize, 140u32, 2147240u32)?;
    emu.lw_no_count(11usize, 2usize, 144u32, 2147244u32)?;
    emu.lw_no_count(12usize, 2usize, 148u32, 2147248u32)?;
    emu.lw_no_count(13usize, 2usize, 152u32, 2147252u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2147256u32)?;
    emu.sw_no_count(11usize, 2usize, 16u32, 2147260u32)?;
    emu.sw_no_count(12usize, 2usize, 20u32, 2147264u32)?;
    emu.sw_no_count(13usize, 2usize, 24u32, 2147268u32)?;
    emu.adi_no_count(10usize, 2usize, 140u32, 2147272u32);
    emu.adi_no_count(11usize, 2usize, 76u32, 2147276u32);
    emu.adi_no_count(12usize, 2usize, 108u32, 2147280u32);
    emu.adi_no_count(13usize, 18usize, 0u32, 2147284u32);
    emu.apc_no_count(1usize, 2147284u32, 20480u32, 2147288u32);
    emu.add_memory_rw_events(22usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2147292u32;
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
#[inline]
pub fn block_0x0020c3dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 156u32, 2147296u32)?;
    emu.lw_no_count(11usize, 2usize, 160u32, 2147300u32)?;
    emu.lw_no_count(12usize, 2usize, 164u32, 2147304u32)?;
    emu.lw_no_count(13usize, 2usize, 168u32, 2147308u32)?;
    emu.sw_no_count(10usize, 2usize, 92u32, 2147312u32)?;
    emu.sw_no_count(11usize, 2usize, 96u32, 2147316u32)?;
    emu.sw_no_count(12usize, 2usize, 100u32, 2147320u32)?;
    emu.sw_no_count(13usize, 2usize, 104u32, 2147324u32)?;
    emu.lw_no_count(10usize, 2usize, 140u32, 2147328u32)?;
    emu.lw_no_count(11usize, 2usize, 144u32, 2147332u32)?;
    emu.lw_no_count(12usize, 2usize, 148u32, 2147336u32)?;
    emu.lw_no_count(13usize, 2usize, 152u32, 2147340u32)?;
    emu.sw_no_count(10usize, 2usize, 76u32, 2147344u32)?;
    emu.sw_no_count(11usize, 2usize, 80u32, 2147348u32)?;
    emu.sw_no_count(12usize, 2usize, 84u32, 2147352u32)?;
    emu.sw_no_count(13usize, 2usize, 88u32, 2147356u32)?;
    emu.add_memory_rw_events(16usize);
    emu.pc = 2147356u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c41c));
}
#[inline]
pub fn block_0x0020c41c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2147360u32)?;
    emu.lw_no_count(11usize, 2usize, 16u32, 2147364u32)?;
    emu.lw_no_count(12usize, 2usize, 20u32, 2147368u32)?;
    emu.lw_no_count(13usize, 2usize, 24u32, 2147372u32)?;
    emu.lw_no_count(14usize, 2usize, 28u32, 2147376u32)?;
    emu.lw_no_count(15usize, 2usize, 32u32, 2147380u32)?;
    emu.lw_no_count(16usize, 2usize, 36u32, 2147384u32)?;
    emu.lw_no_count(17usize, 2usize, 40u32, 2147388u32)?;
    emu.orr_no_count(10usize, 11usize, 10usize, 2147392u32);
    emu.orr_no_count(12usize, 12usize, 13usize, 2147396u32);
    emu.orr_no_count(14usize, 14usize, 15usize, 2147400u32);
    emu.orr_no_count(10usize, 10usize, 12usize, 2147404u32);
    emu.orr_no_count(11usize, 14usize, 16usize, 2147408u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2147412u32);
    emu.orr_no_count(10usize, 10usize, 17usize, 2147416u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2147420u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2147424u32);
    emu.apc_no_count(1usize, 2147424u32, 40960u32, 2147428u32);
    emu.add_memory_rw_events(19usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2147432u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967256u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
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
    emu.ani_no_count(10usize, 10usize, 255u32, 2147436u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2149020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ca9c));
    } else {
        emu.pc = 2147440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c470));
    }
}
#[inline(always)]
pub fn block_0x0020c470(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2147444u32)?;
    emu.ani_no_count(10usize, 10usize, 1u32, 2147448u32);
    emu.apc_no_count(1usize, 2147448u32, 40960u32, 2147452u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2147456u32;
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
pub fn block_0x0020c480(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(10usize, 10usize, 4294967295u32, 2147460u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2147464u32);
    emu.apc_no_count(1usize, 2147464u32, 40960u32, 2147468u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2147472u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967196u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020c490(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 255u32, 2147476u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2148492u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c88c));
    } else {
        emu.pc = 2147480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c498));
    }
}
#[inline(never)]
pub fn block_0x0020c498(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 42u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2147484u32)?;
    emu.lw_no_count(11usize, 2usize, 16u32, 2147488u32)?;
    emu.lw_no_count(12usize, 2usize, 20u32, 2147492u32)?;
    emu.lw_no_count(13usize, 2usize, 24u32, 2147496u32)?;
    emu.lw_no_count(14usize, 2usize, 28u32, 2147500u32)?;
    emu.lw_no_count(15usize, 2usize, 32u32, 2147504u32)?;
    emu.lw_no_count(16usize, 2usize, 36u32, 2147508u32)?;
    emu.lw_no_count(17usize, 2usize, 40u32, 2147512u32)?;
    emu.sri_no_count(10usize, 10usize, 1u32, 2147516u32);
    emu.sli_no_count(5usize, 11usize, 31u32, 2147520u32);
    emu.sri_no_count(11usize, 11usize, 1u32, 2147524u32);
    emu.orr_no_count(10usize, 5usize, 10usize, 2147528u32);
    emu.sli_no_count(5usize, 12usize, 31u32, 2147532u32);
    emu.sri_no_count(12usize, 12usize, 1u32, 2147536u32);
    emu.orr_no_count(11usize, 5usize, 11usize, 2147540u32);
    emu.sli_no_count(5usize, 13usize, 31u32, 2147544u32);
    emu.sri_no_count(13usize, 13usize, 1u32, 2147548u32);
    emu.orr_no_count(12usize, 5usize, 12usize, 2147552u32);
    emu.sli_no_count(5usize, 14usize, 31u32, 2147556u32);
    emu.sri_no_count(14usize, 14usize, 1u32, 2147560u32);
    emu.orr_no_count(13usize, 5usize, 13usize, 2147564u32);
    emu.sli_no_count(5usize, 15usize, 31u32, 2147568u32);
    emu.sri_no_count(15usize, 15usize, 1u32, 2147572u32);
    emu.orr_no_count(14usize, 5usize, 14usize, 2147576u32);
    emu.sli_no_count(5usize, 16usize, 31u32, 2147580u32);
    emu.sri_no_count(16usize, 16usize, 1u32, 2147584u32);
    emu.orr_no_count(15usize, 5usize, 15usize, 2147588u32);
    emu.sli_no_count(5usize, 17usize, 31u32, 2147592u32);
    emu.orr_no_count(16usize, 5usize, 16usize, 2147596u32);
    emu.lw_no_count(5usize, 2usize, 76u32, 2147600u32)?;
    emu.sri_no_count(17usize, 17usize, 1u32, 2147604u32);
    emu.sw_no_count(10usize, 2usize, 12u32, 2147608u32)?;
    emu.sw_no_count(11usize, 2usize, 16u32, 2147612u32)?;
    emu.sw_no_count(12usize, 2usize, 20u32, 2147616u32)?;
    emu.sw_no_count(13usize, 2usize, 24u32, 2147620u32)?;
    emu.sw_no_count(14usize, 2usize, 28u32, 2147624u32)?;
    emu.sw_no_count(15usize, 2usize, 32u32, 2147628u32)?;
    emu.sw_no_count(16usize, 2usize, 36u32, 2147632u32)?;
    emu.sw_no_count(17usize, 2usize, 40u32, 2147636u32)?;
    emu.ani_no_count(10usize, 5usize, 1u32, 2147640u32);
    emu.apc_no_count(1usize, 2147640u32, 40960u32, 2147644u32);
    emu.add_memory_rw_events(42usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2147648u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967020u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0020c540(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 40u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 255u32, 2147652u32);
    emu.lw_no_count(11usize, 2usize, 76u32, 2147656u32)?;
    emu.lw_no_count(12usize, 2usize, 80u32, 2147660u32)?;
    emu.lw_no_count(13usize, 2usize, 84u32, 2147664u32)?;
    emu.lw_no_count(14usize, 2usize, 88u32, 2147668u32)?;
    emu.lw_no_count(15usize, 2usize, 92u32, 2147672u32)?;
    emu.lw_no_count(16usize, 2usize, 96u32, 2147676u32)?;
    emu.lw_no_count(17usize, 2usize, 100u32, 2147680u32)?;
    emu.lw_no_count(5usize, 2usize, 104u32, 2147684u32)?;
    emu.sri_no_count(11usize, 11usize, 1u32, 2147688u32);
    emu.sli_no_count(6usize, 12usize, 31u32, 2147692u32);
    emu.sri_no_count(12usize, 12usize, 1u32, 2147696u32);
    emu.orr_no_count(11usize, 6usize, 11usize, 2147700u32);
    emu.sli_no_count(6usize, 13usize, 31u32, 2147704u32);
    emu.sri_no_count(13usize, 13usize, 1u32, 2147708u32);
    emu.orr_no_count(12usize, 6usize, 12usize, 2147712u32);
    emu.sli_no_count(6usize, 14usize, 31u32, 2147716u32);
    emu.sri_no_count(14usize, 14usize, 1u32, 2147720u32);
    emu.orr_no_count(13usize, 6usize, 13usize, 2147724u32);
    emu.sli_no_count(6usize, 15usize, 31u32, 2147728u32);
    emu.sri_no_count(15usize, 15usize, 1u32, 2147732u32);
    emu.orr_no_count(14usize, 6usize, 14usize, 2147736u32);
    emu.sli_no_count(6usize, 16usize, 31u32, 2147740u32);
    emu.sri_no_count(16usize, 16usize, 1u32, 2147744u32);
    emu.orr_no_count(15usize, 6usize, 15usize, 2147748u32);
    emu.sli_no_count(6usize, 17usize, 31u32, 2147752u32);
    emu.sri_no_count(17usize, 17usize, 1u32, 2147756u32);
    emu.orr_no_count(16usize, 6usize, 16usize, 2147760u32);
    emu.sli_no_count(6usize, 5usize, 31u32, 2147764u32);
    emu.sri_no_count(5usize, 5usize, 1u32, 2147768u32);
    emu.orr_no_count(17usize, 6usize, 17usize, 2147772u32);
    emu.sw_no_count(11usize, 2usize, 76u32, 2147776u32)?;
    emu.sw_no_count(12usize, 2usize, 80u32, 2147780u32)?;
    emu.sw_no_count(13usize, 2usize, 84u32, 2147784u32)?;
    emu.sw_no_count(14usize, 2usize, 88u32, 2147788u32)?;
    emu.sw_no_count(15usize, 2usize, 92u32, 2147792u32)?;
    emu.sw_no_count(16usize, 2usize, 96u32, 2147796u32)?;
    emu.sw_no_count(17usize, 2usize, 100u32, 2147800u32)?;
    emu.sw_no_count(5usize, 2usize, 104u32, 2147804u32)?;
    emu.add_memory_rw_events(39usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2147440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c470));
    } else {
        emu.pc = 2147808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c5e0));
    }
}
#[inline(always)]
pub fn block_0x0020c5e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 140u32, 2147812u32);
    emu.adi_no_count(11usize, 2usize, 76u32, 2147816u32);
    emu.adi_no_count(12usize, 19usize, 0u32, 2147820u32);
    emu.adi_no_count(13usize, 18usize, 0u32, 2147824u32);
    emu.apc_no_count(1usize, 2147824u32, 20480u32, 2147828u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2147832u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966856u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020c5f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 22u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 156u32, 2147836u32)?;
    emu.lw_no_count(11usize, 2usize, 160u32, 2147840u32)?;
    emu.lw_no_count(12usize, 2usize, 164u32, 2147844u32)?;
    emu.lw_no_count(13usize, 2usize, 168u32, 2147848u32)?;
    emu.sw_no_count(10usize, 2usize, 92u32, 2147852u32)?;
    emu.sw_no_count(11usize, 2usize, 96u32, 2147856u32)?;
    emu.sw_no_count(12usize, 2usize, 100u32, 2147860u32)?;
    emu.sw_no_count(13usize, 2usize, 104u32, 2147864u32)?;
    emu.lw_no_count(10usize, 2usize, 140u32, 2147868u32)?;
    emu.lw_no_count(11usize, 2usize, 144u32, 2147872u32)?;
    emu.lw_no_count(12usize, 2usize, 148u32, 2147876u32)?;
    emu.lw_no_count(13usize, 2usize, 152u32, 2147880u32)?;
    emu.sw_no_count(10usize, 2usize, 76u32, 2147884u32)?;
    emu.sw_no_count(11usize, 2usize, 80u32, 2147888u32)?;
    emu.sw_no_count(12usize, 2usize, 84u32, 2147892u32)?;
    emu.sw_no_count(13usize, 2usize, 88u32, 2147896u32)?;
    emu.adi_no_count(10usize, 2usize, 140u32, 2147900u32);
    emu.adi_no_count(11usize, 2usize, 76u32, 2147904u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2147908u32);
    emu.adi_no_count(13usize, 18usize, 0u32, 2147912u32);
    emu.apc_no_count(1usize, 2147912u32, 20480u32, 2147916u32);
    emu.add_memory_rw_events(22usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2147920u32;
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
#[inline]
pub fn block_0x0020c650(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 156u32, 2147924u32)?;
    emu.lw_no_count(11usize, 2usize, 160u32, 2147928u32)?;
    emu.lw_no_count(12usize, 2usize, 164u32, 2147932u32)?;
    emu.lw_no_count(13usize, 2usize, 168u32, 2147936u32)?;
    emu.sw_no_count(10usize, 2usize, 92u32, 2147940u32)?;
    emu.sw_no_count(11usize, 2usize, 96u32, 2147944u32)?;
    emu.sw_no_count(12usize, 2usize, 100u32, 2147948u32)?;
    emu.sw_no_count(13usize, 2usize, 104u32, 2147952u32)?;
    emu.lw_no_count(10usize, 2usize, 140u32, 2147956u32)?;
    emu.lw_no_count(11usize, 2usize, 144u32, 2147960u32)?;
    emu.lw_no_count(12usize, 2usize, 148u32, 2147964u32)?;
    emu.lw_no_count(13usize, 2usize, 152u32, 2147968u32)?;
    emu.sw_no_count(10usize, 2usize, 76u32, 2147972u32)?;
    emu.sw_no_count(11usize, 2usize, 80u32, 2147976u32)?;
    emu.sw_no_count(12usize, 2usize, 84u32, 2147980u32)?;
    emu.sw_no_count(13usize, 2usize, 88u32, 2147984u32)?;
    emu.add_memory_rw_events(17usize);
    let return_addr = 2147988u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2147440u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c470));
}
#[inline(never)]
pub fn block_0x0020c694(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 42u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 44u32, 2147992u32)?;
    emu.lw_no_count(11usize, 2usize, 48u32, 2147996u32)?;
    emu.lw_no_count(12usize, 2usize, 52u32, 2148000u32)?;
    emu.lw_no_count(13usize, 2usize, 56u32, 2148004u32)?;
    emu.lw_no_count(14usize, 2usize, 60u32, 2148008u32)?;
    emu.lw_no_count(15usize, 2usize, 64u32, 2148012u32)?;
    emu.lw_no_count(16usize, 2usize, 68u32, 2148016u32)?;
    emu.lw_no_count(17usize, 2usize, 72u32, 2148020u32)?;
    emu.sri_no_count(10usize, 10usize, 1u32, 2148024u32);
    emu.sli_no_count(5usize, 11usize, 31u32, 2148028u32);
    emu.sri_no_count(11usize, 11usize, 1u32, 2148032u32);
    emu.orr_no_count(10usize, 5usize, 10usize, 2148036u32);
    emu.sli_no_count(5usize, 12usize, 31u32, 2148040u32);
    emu.sri_no_count(12usize, 12usize, 1u32, 2148044u32);
    emu.orr_no_count(11usize, 5usize, 11usize, 2148048u32);
    emu.sli_no_count(5usize, 13usize, 31u32, 2148052u32);
    emu.sri_no_count(13usize, 13usize, 1u32, 2148056u32);
    emu.orr_no_count(12usize, 5usize, 12usize, 2148060u32);
    emu.sli_no_count(5usize, 14usize, 31u32, 2148064u32);
    emu.sri_no_count(14usize, 14usize, 1u32, 2148068u32);
    emu.orr_no_count(13usize, 5usize, 13usize, 2148072u32);
    emu.sli_no_count(5usize, 15usize, 31u32, 2148076u32);
    emu.sri_no_count(15usize, 15usize, 1u32, 2148080u32);
    emu.orr_no_count(14usize, 5usize, 14usize, 2148084u32);
    emu.sli_no_count(5usize, 16usize, 31u32, 2148088u32);
    emu.sri_no_count(16usize, 16usize, 1u32, 2148092u32);
    emu.orr_no_count(15usize, 5usize, 15usize, 2148096u32);
    emu.sli_no_count(5usize, 17usize, 31u32, 2148100u32);
    emu.orr_no_count(16usize, 5usize, 16usize, 2148104u32);
    emu.lw_no_count(5usize, 2usize, 108u32, 2148108u32)?;
    emu.sri_no_count(17usize, 17usize, 1u32, 2148112u32);
    emu.sw_no_count(10usize, 2usize, 44u32, 2148116u32)?;
    emu.sw_no_count(11usize, 2usize, 48u32, 2148120u32)?;
    emu.sw_no_count(12usize, 2usize, 52u32, 2148124u32)?;
    emu.sw_no_count(13usize, 2usize, 56u32, 2148128u32)?;
    emu.sw_no_count(14usize, 2usize, 60u32, 2148132u32)?;
    emu.sw_no_count(15usize, 2usize, 64u32, 2148136u32)?;
    emu.sw_no_count(16usize, 2usize, 68u32, 2148140u32)?;
    emu.sw_no_count(17usize, 2usize, 72u32, 2148144u32)?;
    emu.ani_no_count(10usize, 5usize, 1u32, 2148148u32);
    emu.apc_no_count(1usize, 2148148u32, 40960u32, 2148152u32);
    emu.add_memory_rw_events(42usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2148156u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966512u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0020c73c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 40u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 255u32, 2148160u32);
    emu.lw_no_count(11usize, 2usize, 108u32, 2148164u32)?;
    emu.lw_no_count(12usize, 2usize, 112u32, 2148168u32)?;
    emu.lw_no_count(13usize, 2usize, 116u32, 2148172u32)?;
    emu.lw_no_count(14usize, 2usize, 120u32, 2148176u32)?;
    emu.lw_no_count(15usize, 2usize, 124u32, 2148180u32)?;
    emu.lw_no_count(16usize, 2usize, 128u32, 2148184u32)?;
    emu.lw_no_count(17usize, 2usize, 132u32, 2148188u32)?;
    emu.lw_no_count(5usize, 2usize, 136u32, 2148192u32)?;
    emu.sri_no_count(11usize, 11usize, 1u32, 2148196u32);
    emu.sli_no_count(6usize, 12usize, 31u32, 2148200u32);
    emu.sri_no_count(12usize, 12usize, 1u32, 2148204u32);
    emu.orr_no_count(11usize, 6usize, 11usize, 2148208u32);
    emu.sli_no_count(6usize, 13usize, 31u32, 2148212u32);
    emu.sri_no_count(13usize, 13usize, 1u32, 2148216u32);
    emu.orr_no_count(12usize, 6usize, 12usize, 2148220u32);
    emu.sli_no_count(6usize, 14usize, 31u32, 2148224u32);
    emu.sri_no_count(14usize, 14usize, 1u32, 2148228u32);
    emu.orr_no_count(13usize, 6usize, 13usize, 2148232u32);
    emu.sli_no_count(6usize, 15usize, 31u32, 2148236u32);
    emu.sri_no_count(15usize, 15usize, 1u32, 2148240u32);
    emu.orr_no_count(14usize, 6usize, 14usize, 2148244u32);
    emu.sli_no_count(6usize, 16usize, 31u32, 2148248u32);
    emu.sri_no_count(16usize, 16usize, 1u32, 2148252u32);
    emu.orr_no_count(15usize, 6usize, 15usize, 2148256u32);
    emu.sli_no_count(6usize, 17usize, 31u32, 2148260u32);
    emu.sri_no_count(17usize, 17usize, 1u32, 2148264u32);
    emu.orr_no_count(16usize, 6usize, 16usize, 2148268u32);
    emu.sli_no_count(6usize, 5usize, 31u32, 2148272u32);
    emu.sri_no_count(5usize, 5usize, 1u32, 2148276u32);
    emu.orr_no_count(17usize, 6usize, 17usize, 2148280u32);
    emu.sw_no_count(11usize, 2usize, 108u32, 2148284u32)?;
    emu.sw_no_count(12usize, 2usize, 112u32, 2148288u32)?;
    emu.sw_no_count(13usize, 2usize, 116u32, 2148292u32)?;
    emu.sw_no_count(14usize, 2usize, 120u32, 2148296u32)?;
    emu.sw_no_count(15usize, 2usize, 124u32, 2148300u32)?;
    emu.sw_no_count(16usize, 2usize, 128u32, 2148304u32)?;
    emu.sw_no_count(17usize, 2usize, 132u32, 2148308u32)?;
    emu.sw_no_count(5usize, 2usize, 136u32, 2148312u32)?;
    emu.add_memory_rw_events(39usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2148492u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c88c));
    } else {
        emu.pc = 2148316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c7dc));
    }
}
#[inline(always)]
pub fn block_0x0020c7dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 140u32, 2148320u32);
    emu.adi_no_count(11usize, 2usize, 108u32, 2148324u32);
    emu.adi_no_count(12usize, 19usize, 0u32, 2148328u32);
    emu.adi_no_count(13usize, 18usize, 0u32, 2148332u32);
    emu.apc_no_count(1usize, 2148332u32, 20480u32, 2148336u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2148340u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966348u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020c7f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 22u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 156u32, 2148344u32)?;
    emu.lw_no_count(11usize, 2usize, 160u32, 2148348u32)?;
    emu.lw_no_count(12usize, 2usize, 164u32, 2148352u32)?;
    emu.lw_no_count(13usize, 2usize, 168u32, 2148356u32)?;
    emu.sw_no_count(10usize, 2usize, 124u32, 2148360u32)?;
    emu.sw_no_count(11usize, 2usize, 128u32, 2148364u32)?;
    emu.sw_no_count(12usize, 2usize, 132u32, 2148368u32)?;
    emu.sw_no_count(13usize, 2usize, 136u32, 2148372u32)?;
    emu.lw_no_count(10usize, 2usize, 140u32, 2148376u32)?;
    emu.lw_no_count(11usize, 2usize, 144u32, 2148380u32)?;
    emu.lw_no_count(12usize, 2usize, 148u32, 2148384u32)?;
    emu.lw_no_count(13usize, 2usize, 152u32, 2148388u32)?;
    emu.sw_no_count(10usize, 2usize, 108u32, 2148392u32)?;
    emu.sw_no_count(11usize, 2usize, 112u32, 2148396u32)?;
    emu.sw_no_count(12usize, 2usize, 116u32, 2148400u32)?;
    emu.sw_no_count(13usize, 2usize, 120u32, 2148404u32)?;
    emu.adi_no_count(10usize, 2usize, 140u32, 2148408u32);
    emu.adi_no_count(11usize, 2usize, 108u32, 2148412u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2148416u32);
    emu.adi_no_count(13usize, 18usize, 0u32, 2148420u32);
    emu.apc_no_count(1usize, 2148420u32, 20480u32, 2148424u32);
    emu.add_memory_rw_events(22usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2148428u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966260u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020c84c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 156u32, 2148432u32)?;
    emu.lw_no_count(11usize, 2usize, 160u32, 2148436u32)?;
    emu.lw_no_count(12usize, 2usize, 164u32, 2148440u32)?;
    emu.lw_no_count(13usize, 2usize, 168u32, 2148444u32)?;
    emu.sw_no_count(10usize, 2usize, 124u32, 2148448u32)?;
    emu.sw_no_count(11usize, 2usize, 128u32, 2148452u32)?;
    emu.sw_no_count(12usize, 2usize, 132u32, 2148456u32)?;
    emu.sw_no_count(13usize, 2usize, 136u32, 2148460u32)?;
    emu.lw_no_count(10usize, 2usize, 140u32, 2148464u32)?;
    emu.lw_no_count(11usize, 2usize, 144u32, 2148468u32)?;
    emu.lw_no_count(12usize, 2usize, 148u32, 2148472u32)?;
    emu.lw_no_count(13usize, 2usize, 152u32, 2148476u32)?;
    emu.sw_no_count(10usize, 2usize, 108u32, 2148480u32)?;
    emu.sw_no_count(11usize, 2usize, 112u32, 2148484u32)?;
    emu.sw_no_count(12usize, 2usize, 116u32, 2148488u32)?;
    emu.sw_no_count(13usize, 2usize, 120u32, 2148492u32)?;
    emu.add_memory_rw_events(16usize);
    emu.pc = 2148492u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c88c));
}
#[inline(always)]
pub fn block_0x0020c88c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 44u32, 2148496u32)?;
    emu.ani_no_count(10usize, 10usize, 1u32, 2148500u32);
    emu.apc_no_count(1usize, 2148500u32, 40960u32, 2148504u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2148508u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966160u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020c89c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(10usize, 10usize, 4294967295u32, 2148512u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2148516u32);
    emu.apc_no_count(1usize, 2148516u32, 40960u32, 2148520u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2148524u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966144u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020c8ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 255u32, 2148528u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2147988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c694));
    } else {
        emu.pc = 2148532u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c8b4));
    }
}
#[inline(never)]
pub fn block_0x0020c8b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 67u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 44u32, 2148536u32)?;
    emu.lw_no_count(13usize, 2usize, 48u32, 2148540u32)?;
    emu.lw_no_count(14usize, 2usize, 52u32, 2148544u32)?;
    emu.lw_no_count(15usize, 2usize, 56u32, 2148548u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2148552u32)?;
    emu.lw_no_count(12usize, 2usize, 16u32, 2148556u32)?;
    emu.lw_no_count(16usize, 2usize, 20u32, 2148560u32)?;
    emu.lw_no_count(17usize, 2usize, 24u32, 2148564u32)?;
    emu.lw_no_count(5usize, 2usize, 60u32, 2148568u32)?;
    emu.lw_no_count(6usize, 2usize, 64u32, 2148572u32)?;
    emu.lw_no_count(7usize, 2usize, 68u32, 2148576u32)?;
    emu.lw_no_count(28usize, 2usize, 72u32, 2148580u32)?;
    emu.lw_no_count(29usize, 2usize, 28u32, 2148584u32)?;
    emu.lw_no_count(30usize, 2usize, 32u32, 2148588u32)?;
    emu.lw_no_count(31usize, 2usize, 36u32, 2148592u32)?;
    emu.lw_no_count(21usize, 2usize, 40u32, 2148596u32)?;
    emu.sltru_no_count(22usize, 10usize, 11usize, 2148600u32);
    emu.sbr_no_count(23usize, 14usize, 16usize, 2148604u32);
    emu.sltru_no_count(14usize, 14usize, 16usize, 2148608u32);
    emu.sbr_no_count(16usize, 15usize, 17usize, 2148612u32);
    emu.sltru_no_count(15usize, 15usize, 17usize, 2148616u32);
    emu.sbr_no_count(17usize, 5usize, 29usize, 2148620u32);
    emu.sltru_no_count(5usize, 5usize, 29usize, 2148624u32);
    emu.sbr_no_count(29usize, 6usize, 30usize, 2148628u32);
    emu.sltru_no_count(6usize, 6usize, 30usize, 2148632u32);
    emu.sbr_no_count(30usize, 7usize, 31usize, 2148636u32);
    emu.sltru_no_count(7usize, 7usize, 31usize, 2148640u32);
    emu.sbr_no_count(31usize, 28usize, 21usize, 2148644u32);
    emu.sltru_no_count(28usize, 28usize, 21usize, 2148648u32);
    emu.sbr_no_count(21usize, 0usize, 22usize, 2148652u32);
    emu.sbr_no_count(13usize, 13usize, 22usize, 2148656u32);
    emu.sltru_no_count(21usize, 13usize, 21usize, 2148660u32);
    emu.sbr_no_count(21usize, 21usize, 22usize, 2148664u32);
    emu.sltru_no_count(22usize, 13usize, 12usize, 2148668u32);
    emu.sbr_no_count(21usize, 21usize, 22usize, 2148672u32);
    emu.sai_no_count(21usize, 21usize, 1055u32, 2148676u32);
    emu.sbr_no_count(22usize, 21usize, 14usize, 2148680u32);
    emu.adr_no_count(14usize, 23usize, 21usize, 2148684u32);
    emu.sltru_no_count(21usize, 14usize, 23usize, 2148688u32);
    emu.adr_no_count(21usize, 22usize, 21usize, 2148692u32);
    emu.sai_no_count(21usize, 21usize, 1055u32, 2148696u32);
    emu.sbr_no_count(22usize, 21usize, 15usize, 2148700u32);
    emu.adr_no_count(15usize, 16usize, 21usize, 2148704u32);
    emu.sltru_no_count(16usize, 15usize, 16usize, 2148708u32);
    emu.adr_no_count(16usize, 22usize, 16usize, 2148712u32);
    emu.sai_no_count(16usize, 16usize, 1055u32, 2148716u32);
    emu.sbr_no_count(5usize, 16usize, 5usize, 2148720u32);
    emu.adr_no_count(16usize, 17usize, 16usize, 2148724u32);
    emu.sltru_no_count(17usize, 16usize, 17usize, 2148728u32);
    emu.adr_no_count(17usize, 5usize, 17usize, 2148732u32);
    emu.sai_no_count(17usize, 17usize, 1055u32, 2148736u32);
    emu.sbr_no_count(5usize, 17usize, 6usize, 2148740u32);
    emu.adr_no_count(17usize, 29usize, 17usize, 2148744u32);
    emu.sltru_no_count(6usize, 17usize, 29usize, 2148748u32);
    emu.adr_no_count(5usize, 5usize, 6usize, 2148752u32);
    emu.sai_no_count(5usize, 5usize, 1055u32, 2148756u32);
    emu.sbr_no_count(6usize, 5usize, 7usize, 2148760u32);
    emu.adr_no_count(5usize, 30usize, 5usize, 2148764u32);
    emu.sltru_no_count(7usize, 5usize, 30usize, 2148768u32);
    emu.adr_no_count(6usize, 6usize, 7usize, 2148772u32);
    emu.sai_no_count(6usize, 6usize, 1055u32, 2148776u32);
    emu.sbr_no_count(7usize, 6usize, 28usize, 2148780u32);
    emu.adr_no_count(6usize, 31usize, 6usize, 2148784u32);
    emu.sltru_no_count(28usize, 6usize, 31usize, 2148788u32);
    emu.adr_no_count(7usize, 7usize, 28usize, 2148792u32);
    emu.ani_no_count(7usize, 7usize, 2u32, 2148796u32);
    emu.add_memory_rw_events(66usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a != b {
        emu.pc = 2147180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c36c));
    } else {
        emu.pc = 2148800u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c9c0));
    }
}
#[inline]
pub fn block_0x0020c9c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 10usize, 11usize, 2148804u32);
    emu.sbr_no_count(13usize, 13usize, 12usize, 2148808u32);
    emu.orr_no_count(10usize, 13usize, 10usize, 2148812u32);
    emu.orr_no_count(10usize, 10usize, 14usize, 2148816u32);
    emu.orr_no_count(10usize, 10usize, 15usize, 2148820u32);
    emu.orr_no_count(10usize, 10usize, 16usize, 2148824u32);
    emu.orr_no_count(10usize, 10usize, 17usize, 2148828u32);
    emu.orr_no_count(10usize, 10usize, 5usize, 2148832u32);
    emu.orr_no_count(10usize, 10usize, 6usize, 2148836u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2147180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c36c));
    } else {
        emu.pc = 2148840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c9e8));
    }
}
#[inline(always)]
pub fn block_0x0020c9e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 140u32, 2148844u32);
    emu.adi_no_count(11usize, 2usize, 44u32, 2148848u32);
    emu.adi_no_count(12usize, 2usize, 12u32, 2148852u32);
    emu.adi_no_count(13usize, 18usize, 0u32, 2148856u32);
    emu.apc_no_count(1usize, 2148856u32, 20480u32, 2148860u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2148864u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966560u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020ca00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 22u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 156u32, 2148868u32)?;
    emu.lw_no_count(11usize, 2usize, 160u32, 2148872u32)?;
    emu.lw_no_count(12usize, 2usize, 164u32, 2148876u32)?;
    emu.lw_no_count(13usize, 2usize, 168u32, 2148880u32)?;
    emu.sw_no_count(10usize, 2usize, 60u32, 2148884u32)?;
    emu.sw_no_count(11usize, 2usize, 64u32, 2148888u32)?;
    emu.sw_no_count(12usize, 2usize, 68u32, 2148892u32)?;
    emu.sw_no_count(13usize, 2usize, 72u32, 2148896u32)?;
    emu.lw_no_count(10usize, 2usize, 140u32, 2148900u32)?;
    emu.lw_no_count(11usize, 2usize, 144u32, 2148904u32)?;
    emu.lw_no_count(12usize, 2usize, 148u32, 2148908u32)?;
    emu.lw_no_count(13usize, 2usize, 152u32, 2148912u32)?;
    emu.sw_no_count(10usize, 2usize, 44u32, 2148916u32)?;
    emu.sw_no_count(11usize, 2usize, 48u32, 2148920u32)?;
    emu.sw_no_count(12usize, 2usize, 52u32, 2148924u32)?;
    emu.sw_no_count(13usize, 2usize, 56u32, 2148928u32)?;
    emu.adi_no_count(10usize, 2usize, 140u32, 2148932u32);
    emu.adi_no_count(11usize, 2usize, 108u32, 2148936u32);
    emu.adi_no_count(12usize, 2usize, 76u32, 2148940u32);
    emu.adi_no_count(13usize, 18usize, 0u32, 2148944u32);
    emu.apc_no_count(1usize, 2148944u32, 20480u32, 2148948u32);
    emu.add_memory_rw_events(22usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2148952u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966472u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x0020ca58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 156u32, 2148956u32)?;
    emu.lw_no_count(11usize, 2usize, 160u32, 2148960u32)?;
    emu.lw_no_count(12usize, 2usize, 164u32, 2148964u32)?;
    emu.lw_no_count(13usize, 2usize, 168u32, 2148968u32)?;
    emu.sw_no_count(10usize, 2usize, 124u32, 2148972u32)?;
    emu.sw_no_count(11usize, 2usize, 128u32, 2148976u32)?;
    emu.sw_no_count(12usize, 2usize, 132u32, 2148980u32)?;
    emu.sw_no_count(13usize, 2usize, 136u32, 2148984u32)?;
    emu.lw_no_count(10usize, 2usize, 140u32, 2148988u32)?;
    emu.lw_no_count(11usize, 2usize, 144u32, 2148992u32)?;
    emu.lw_no_count(12usize, 2usize, 148u32, 2148996u32)?;
    emu.lw_no_count(13usize, 2usize, 152u32, 2149000u32)?;
    emu.sw_no_count(10usize, 2usize, 108u32, 2149004u32)?;
    emu.sw_no_count(11usize, 2usize, 112u32, 2149008u32)?;
    emu.sw_no_count(12usize, 2usize, 116u32, 2149012u32)?;
    emu.sw_no_count(13usize, 2usize, 120u32, 2149016u32)?;
    emu.add_memory_rw_events(17usize);
    let return_addr = 2149020u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2147356u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c41c));
}
#[inline(never)]
pub fn block_0x0020ca9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 35u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 124u32, 2149024u32)?;
    emu.lw_no_count(11usize, 2usize, 128u32, 2149028u32)?;
    emu.lw_no_count(12usize, 2usize, 132u32, 2149032u32)?;
    emu.lw_no_count(13usize, 2usize, 136u32, 2149036u32)?;
    emu.lw_no_count(14usize, 2usize, 108u32, 2149040u32)?;
    emu.lw_no_count(15usize, 2usize, 112u32, 2149044u32)?;
    emu.lw_no_count(16usize, 2usize, 116u32, 2149048u32)?;
    emu.lw_no_count(17usize, 2usize, 120u32, 2149052u32)?;
    emu.sw_no_count(10usize, 8usize, 16u32, 2149056u32)?;
    emu.sw_no_count(11usize, 8usize, 20u32, 2149060u32)?;
    emu.sw_no_count(12usize, 8usize, 24u32, 2149064u32)?;
    emu.sw_no_count(13usize, 8usize, 28u32, 2149068u32)?;
    emu.lw_no_count(10usize, 9usize, 0u32, 2149072u32)?;
    emu.lw_no_count(11usize, 9usize, 4u32, 2149076u32)?;
    emu.lw_no_count(12usize, 9usize, 8u32, 2149080u32)?;
    emu.lw_no_count(13usize, 9usize, 12u32, 2149084u32)?;
    emu.sw_no_count(14usize, 8usize, 0u32, 2149088u32)?;
    emu.sw_no_count(15usize, 8usize, 4u32, 2149092u32)?;
    emu.sw_no_count(16usize, 8usize, 8u32, 2149096u32)?;
    emu.sw_no_count(17usize, 8usize, 12u32, 2149100u32)?;
    emu.lw_no_count(14usize, 9usize, 16u32, 2149104u32)?;
    emu.lw_no_count(15usize, 9usize, 20u32, 2149108u32)?;
    emu.lw_no_count(16usize, 9usize, 24u32, 2149112u32)?;
    emu.lw_no_count(17usize, 9usize, 28u32, 2149116u32)?;
    emu.orr_no_count(10usize, 11usize, 10usize, 2149120u32);
    emu.orr_no_count(12usize, 12usize, 13usize, 2149124u32);
    emu.orr_no_count(14usize, 14usize, 15usize, 2149128u32);
    emu.orr_no_count(10usize, 10usize, 12usize, 2149132u32);
    emu.orr_no_count(11usize, 14usize, 16usize, 2149136u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2149140u32);
    emu.orr_no_count(10usize, 10usize, 17usize, 2149144u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2149148u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2149152u32);
    emu.apc_no_count(1usize, 2149152u32, 40960u32, 2149156u32);
    emu.add_memory_rw_events(35usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2149160u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965528u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
