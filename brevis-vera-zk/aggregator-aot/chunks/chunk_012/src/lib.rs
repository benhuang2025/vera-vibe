pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2146540u32;
pub const PC_MAX: u32 = 2151868u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 109usize] = [
        block_0x0020c0ec,
        block_0x0020c134,
        block_0x0020c1c8,
        block_0x0020c2a8,
        block_0x0020c2ac,
        block_0x0020c314,
        block_0x0020c318,
        block_0x0020c398,
        block_0x0020c39c,
        block_0x0020c414,
        block_0x0020c418,
        block_0x0020c43c,
        block_0x0020c440,
        block_0x0020c490,
        block_0x0020c494,
        block_0x0020c4bc,
        block_0x0020c4c0,
        block_0x0020c544,
        block_0x0020c548,
        block_0x0020c5bc,
        block_0x0020c5c0,
        block_0x0020c5f4,
        block_0x0020c5f8,
        block_0x0020c670,
        block_0x0020c674,
        block_0x0020c6f0,
        block_0x0020c6fc,
        block_0x0020c754,
        block_0x0020c758,
        block_0x0020c7c0,
        block_0x0020c7c4,
        block_0x0020c804,
        block_0x0020c808,
        block_0x0020c854,
        block_0x0020c858,
        block_0x0020c88c,
        block_0x0020c890,
        block_0x0020c8dc,
        block_0x0020c8e0,
        block_0x0020c918,
        block_0x0020c91c,
        block_0x0020c944,
        block_0x0020c948,
        block_0x0020c970,
        block_0x0020c974,
        block_0x0020c9ac,
        block_0x0020c9b0,
        block_0x0020ca08,
        block_0x0020ca0c,
        block_0x0020ca50,
        block_0x0020ca54,
        block_0x0020ca80,
        block_0x0020ca84,
        block_0x0020caa0,
        block_0x0020caa4,
        block_0x0020cad4,
        block_0x0020cad8,
        block_0x0020caec,
        block_0x0020caf0,
        block_0x0020cb10,
        block_0x0020cb14,
        block_0x0020cb34,
        block_0x0020cb38,
        block_0x0020cb54,
        block_0x0020cb58,
        block_0x0020cb80,
        block_0x0020cb84,
        block_0x0020cbbc,
        block_0x0020cbc0,
        block_0x0020cc14,
        block_0x0020cc18,
        block_0x0020cc5c,
        block_0x0020cc60,
        block_0x0020cc8c,
        block_0x0020cc90,
        block_0x0020ccac,
        block_0x0020ccb0,
        block_0x0020cce0,
        block_0x0020cce4,
        block_0x0020ccf8,
        block_0x0020ccfc,
        block_0x0020cd78,
        block_0x0020ce8c,
        block_0x0020cea4,
        block_0x0020cefc,
        block_0x0020cf3c,
        block_0x0020cf88,
        block_0x0020cf90,
        block_0x0020cfa0,
        block_0x0020cfb0,
        block_0x0020cfb8,
        block_0x0020d060,
        block_0x0020d100,
        block_0x0020d118,
        block_0x0020d170,
        block_0x0020d1b4,
        block_0x0020d25c,
        block_0x0020d2fc,
        block_0x0020d314,
        block_0x0020d36c,
        block_0x0020d3ac,
        block_0x0020d3bc,
        block_0x0020d3cc,
        block_0x0020d3d4,
        block_0x0020d4e0,
        block_0x0020d508,
        block_0x0020d520,
        block_0x0020d578,
        block_0x0020d5bc,
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
    if pc < 2146540u32 || pc > 2151868u32 {
        return None;
    }
    let word_offset = ((pc - 2146540u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline]
pub fn block_0x0020c0ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967040u32, 2146544u32);
    emu.sw_no_count(1usize, 2usize, 252u32, 2146548u32)?;
    emu.sw_no_count(8usize, 2usize, 248u32, 2146552u32)?;
    emu.sw_no_count(9usize, 2usize, 244u32, 2146556u32)?;
    emu.sw_no_count(18usize, 2usize, 240u32, 2146560u32)?;
    emu.sw_no_count(19usize, 2usize, 236u32, 2146564u32)?;
    emu.sw_no_count(20usize, 2usize, 232u32, 2146568u32)?;
    emu.sw_no_count(21usize, 2usize, 228u32, 2146572u32)?;
    emu.sw_no_count(22usize, 2usize, 224u32, 2146576u32)?;
    emu.sw_no_count(23usize, 2usize, 220u32, 2146580u32)?;
    emu.sw_no_count(24usize, 2usize, 216u32, 2146584u32)?;
    emu.sw_no_count(25usize, 2usize, 212u32, 2146588u32)?;
    emu.sw_no_count(26usize, 2usize, 208u32, 2146592u32)?;
    emu.sw_no_count(27usize, 2usize, 204u32, 2146596u32)?;
    emu.sw_no_count(10usize, 2usize, 64u32, 2146600u32)?;
    emu.adi_no_count(10usize, 2usize, 96u32, 2146604u32);
    emu.apc_no_count(1usize, 2146604u32, 24576u32, 2146608u32);
    emu.add_memory_rw_events(18usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2146612u32;
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
pub fn block_0x0020c134(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 37u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 96u32, 2146616u32)?;
    emu.sw_no_count(10usize, 2usize, 72u32, 2146620u32)?;
    emu.lw_no_count(10usize, 2usize, 100u32, 2146624u32)?;
    emu.sw_no_count(10usize, 2usize, 80u32, 2146628u32)?;
    emu.lw_no_count(10usize, 2usize, 104u32, 2146632u32)?;
    emu.sw_no_count(10usize, 2usize, 28u32, 2146636u32)?;
    emu.lw_no_count(10usize, 2usize, 108u32, 2146640u32)?;
    emu.sw_no_count(10usize, 2usize, 24u32, 2146644u32)?;
    emu.lw_no_count(9usize, 2usize, 112u32, 2146648u32)?;
    emu.lw_no_count(10usize, 2usize, 116u32, 2146652u32)?;
    emu.sw_no_count(10usize, 2usize, 32u32, 2146656u32)?;
    emu.lw_no_count(8usize, 2usize, 120u32, 2146660u32)?;
    emu.lw_no_count(20usize, 2usize, 124u32, 2146664u32)?;
    emu.lw_no_count(16usize, 2usize, 128u32, 2146668u32)?;
    emu.lw_no_count(17usize, 2usize, 132u32, 2146672u32)?;
    emu.lw_no_count(10usize, 2usize, 136u32, 2146676u32)?;
    emu.lw_no_count(11usize, 2usize, 140u32, 2146680u32)?;
    emu.lw_no_count(12usize, 2usize, 144u32, 2146684u32)?;
    emu.lw_no_count(13usize, 2usize, 148u32, 2146688u32)?;
    emu.lw_no_count(14usize, 2usize, 152u32, 2146692u32)?;
    emu.lw_no_count(15usize, 2usize, 156u32, 2146696u32)?;
    emu.sw_no_count(8usize, 2usize, 160u32, 2146700u32)?;
    emu.sw_no_count(20usize, 2usize, 164u32, 2146704u32)?;
    emu.sw_no_count(16usize, 2usize, 52u32, 2146708u32)?;
    emu.sw_no_count(16usize, 2usize, 168u32, 2146712u32)?;
    emu.sw_no_count(17usize, 2usize, 48u32, 2146716u32)?;
    emu.sw_no_count(17usize, 2usize, 172u32, 2146720u32)?;
    emu.sw_no_count(10usize, 2usize, 176u32, 2146724u32)?;
    emu.sw_no_count(11usize, 2usize, 180u32, 2146728u32)?;
    emu.sw_no_count(12usize, 2usize, 184u32, 2146732u32)?;
    emu.sw_no_count(13usize, 2usize, 188u32, 2146736u32)?;
    emu.sw_no_count(14usize, 2usize, 192u32, 2146740u32)?;
    emu.sw_no_count(15usize, 2usize, 196u32, 2146744u32)?;
    emu.adi_no_count(10usize, 2usize, 96u32, 2146748u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2146752u32);
    emu.apc_no_count(1usize, 2146752u32, 4294963200u32, 2146756u32);
    emu.add_memory_rw_events(37usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2146760u32;
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
pub fn block_0x0020c1c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 56u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(15usize, 2usize, 96u32, 2146764u32)?;
    emu.lw_no_count(5usize, 2usize, 100u32, 2146768u32)?;
    let a = 0u32.wrapping_add(4234354688u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2146772u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4089040896u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2146776u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2803343360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2146780u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(3169255424u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2146784u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 10usize, 1361u32, 2146788u32);
    emu.adi_no_count(28usize, 11usize, 4294965954u32, 2146792u32);
    emu.adi_no_count(24usize, 12usize, 4294966916u32, 2146796u32);
    emu.adi_no_count(26usize, 14usize, 4294965933u32, 2146800u32);
    emu.mul_no_count(10usize, 5usize, 13usize, 2146804u32);
    emu.mulhu_no_count(11usize, 15usize, 13usize, 2146808u32);
    emu.mulhu_no_count(12usize, 5usize, 13usize, 2146812u32);
    emu.mul_no_count(14usize, 15usize, 28usize, 2146816u32);
    emu.mulhu_no_count(16usize, 15usize, 28usize, 2146820u32);
    emu.mul_no_count(17usize, 5usize, 28usize, 2146824u32);
    emu.mul_no_count(6usize, 5usize, 24usize, 2146828u32);
    emu.mulhu_no_count(7usize, 15usize, 24usize, 2146832u32);
    emu.adr_no_count(11usize, 10usize, 11usize, 2146836u32);
    emu.sltru_no_count(10usize, 11usize, 10usize, 2146840u32);
    emu.adr_no_count(10usize, 12usize, 10usize, 2146844u32);
    emu.mulhu_no_count(12usize, 5usize, 24usize, 2146848u32);
    emu.adr_no_count(7usize, 6usize, 7usize, 2146852u32);
    emu.sltru_no_count(6usize, 7usize, 6usize, 2146856u32);
    emu.adr_no_count(12usize, 12usize, 6usize, 2146860u32);
    emu.mul_no_count(6usize, 15usize, 26usize, 2146864u32);
    emu.adr_no_count(11usize, 14usize, 11usize, 2146868u32);
    emu.sw_no_count(11usize, 2usize, 76u32, 2146872u32)?;
    emu.sltru_no_count(11usize, 11usize, 14usize, 2146876u32);
    emu.adr_no_count(11usize, 16usize, 11usize, 2146880u32);
    emu.mulhu_no_count(14usize, 15usize, 26usize, 2146884u32);
    emu.adr_no_count(7usize, 6usize, 7usize, 2146888u32);
    emu.sltru_no_count(16usize, 7usize, 6usize, 2146892u32);
    emu.adr_no_count(14usize, 14usize, 16usize, 2146896u32);
    emu.sw_no_count(28usize, 2usize, 92u32, 2146900u32)?;
    emu.mulhu_no_count(16usize, 5usize, 28usize, 2146904u32);
    emu.adr_no_count(11usize, 10usize, 11usize, 2146908u32);
    emu.sltru_no_count(10usize, 11usize, 10usize, 2146912u32);
    emu.adr_no_count(10usize, 16usize, 10usize, 2146916u32);
    emu.mulhu_no_count(16usize, 5usize, 26usize, 2146920u32);
    emu.adr_no_count(14usize, 12usize, 14usize, 2146924u32);
    emu.sltru_no_count(12usize, 14usize, 12usize, 2146928u32);
    emu.adr_no_count(16usize, 16usize, 12usize, 2146932u32);
    emu.mul_no_count(12usize, 5usize, 26usize, 2146936u32);
    emu.adr_no_count(6usize, 17usize, 11usize, 2146940u32);
    emu.sltru_no_count(11usize, 6usize, 17usize, 2146944u32);
    emu.adr_no_count(11usize, 10usize, 11usize, 2146948u32);
    emu.mul_no_count(28usize, 15usize, 24usize, 2146952u32);
    emu.adr_no_count(10usize, 12usize, 14usize, 2146956u32);
    emu.sltru_no_count(14usize, 10usize, 12usize, 2146960u32);
    emu.adr_no_count(28usize, 6usize, 28usize, 2146964u32);
    emu.sltru_no_count(12usize, 28usize, 6usize, 2146968u32);
    emu.adr_no_count(7usize, 11usize, 7usize, 2146972u32);
    emu.adr_no_count(23usize, 7usize, 12usize, 2146976u32);
    emu.adr_no_count(14usize, 16usize, 14usize, 2146980u32);
    emu.add_memory_rw_events(55usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a == b {
        emu.pc = 2146988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c2ac));
    } else {
        emu.pc = 2146984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c2a8));
    }
}
#[inline(always)]
pub fn block_0x0020c2a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 23usize, 11usize, 2146988u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2146988u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c2ac));
}
#[inline(never)]
pub fn block_0x0020c2ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 26u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(9usize, 2usize, 36u32, 2146992u32)?;
    emu.sw_no_count(8usize, 2usize, 44u32, 2146996u32)?;
    emu.adr_no_count(12usize, 10usize, 12usize, 2147000u32);
    emu.sbr_no_count(11usize, 0usize, 5usize, 2147004u32);
    emu.adi_no_count(7usize, 0usize, 4294967295u32, 2147008u32);
    emu.sbr_no_count(29usize, 0usize, 15usize, 2147012u32);
    emu.sltru_no_count(10usize, 12usize, 10usize, 2147016u32);
    emu.mulhu_no_count(16usize, 15usize, 7usize, 2147020u32);
    emu.mulhu_no_count(6usize, 5usize, 7usize, 2147024u32);
    emu.sbr_no_count(18usize, 12usize, 15usize, 2147028u32);
    emu.adr_no_count(14usize, 14usize, 10usize, 2147032u32);
    emu.sbr_no_count(17usize, 16usize, 5usize, 2147036u32);
    emu.sltru_no_count(31usize, 18usize, 12usize, 2147040u32);
    emu.sbr_no_count(10usize, 17usize, 15usize, 2147044u32);
    emu.sltru_no_count(12usize, 10usize, 29usize, 2147048u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2147052u32);
    emu.sltru_no_count(16usize, 17usize, 11usize, 2147056u32);
    emu.adr_no_count(16usize, 6usize, 16usize, 2147060u32);
    emu.adr_no_count(10usize, 14usize, 10usize, 2147064u32);
    emu.adr_no_count(19usize, 10usize, 31usize, 2147068u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2147072u32);
    emu.sbr_no_count(10usize, 12usize, 5usize, 2147076u32);
    emu.sltru_no_count(12usize, 12usize, 16usize, 2147080u32);
    emu.sltru_no_count(27usize, 10usize, 11usize, 2147084u32);
    emu.adr_no_count(11usize, 6usize, 12usize, 2147088u32);
    emu.add_memory_rw_events(25usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2147096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c318));
    } else {
        emu.pc = 2147092u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c314));
    }
}
#[inline(always)]
pub fn block_0x0020c314(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(31usize, 19usize, 14usize, 2147096u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2147096u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c318));
}
#[inline(never)]
pub fn block_0x0020c318(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 32u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(27usize, 11usize, 27usize, 2147100u32);
    emu.lw_no_count(5usize, 2usize, 104u32, 2147104u32)?;
    emu.lw_no_count(30usize, 2usize, 108u32, 2147108u32)?;
    emu.adr_no_count(31usize, 10usize, 31usize, 2147112u32);
    emu.sltru_no_count(14usize, 31usize, 10usize, 2147116u32);
    emu.mulhu_no_count(10usize, 5usize, 13usize, 2147120u32);
    emu.mul_no_count(11usize, 30usize, 13usize, 2147124u32);
    emu.mulhu_no_count(12usize, 30usize, 13usize, 2147128u32);
    emu.lw_no_count(9usize, 2usize, 92u32, 2147132u32)?;
    emu.mul_no_count(6usize, 5usize, 9usize, 2147136u32);
    emu.mulhu_no_count(8usize, 5usize, 9usize, 2147140u32);
    emu.adr_no_count(10usize, 11usize, 10usize, 2147144u32);
    emu.sltru_no_count(11usize, 10usize, 11usize, 2147148u32);
    emu.adr_no_count(11usize, 12usize, 11usize, 2147152u32);
    emu.mul_no_count(12usize, 30usize, 9usize, 2147156u32);
    emu.adr_no_count(10usize, 6usize, 10usize, 2147160u32);
    emu.sltru_no_count(6usize, 10usize, 6usize, 2147164u32);
    emu.adr_no_count(6usize, 8usize, 6usize, 2147168u32);
    emu.mulhu_no_count(8usize, 30usize, 9usize, 2147172u32);
    emu.adr_no_count(6usize, 11usize, 6usize, 2147176u32);
    emu.sltru_no_count(11usize, 6usize, 11usize, 2147180u32);
    emu.adr_no_count(8usize, 8usize, 11usize, 2147184u32);
    emu.mul_no_count(11usize, 5usize, 13usize, 2147188u32);
    emu.adr_no_count(11usize, 28usize, 11usize, 2147192u32);
    emu.sw_no_count(11usize, 2usize, 8u32, 2147196u32)?;
    emu.sltru_no_count(11usize, 11usize, 28usize, 2147200u32);
    emu.adr_no_count(28usize, 10usize, 11usize, 2147204u32);
    emu.adr_no_count(10usize, 12usize, 6usize, 2147208u32);
    emu.sltru_no_count(6usize, 10usize, 12usize, 2147212u32);
    emu.adr_no_count(28usize, 23usize, 28usize, 2147216u32);
    emu.adr_no_count(6usize, 8usize, 6usize, 2147220u32);
    emu.add_memory_rw_events(31usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2147228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c39c));
    } else {
        emu.pc = 2147224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c398));
    }
}
#[inline(always)]
pub fn block_0x0020c398(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(11usize, 28usize, 23usize, 2147228u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2147228u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c39c));
}
#[inline(never)]
pub fn block_0x0020c39c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 30u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(28usize, 2usize, 4u32, 2147232u32)?;
    emu.adr_no_count(27usize, 27usize, 14usize, 2147236u32);
    emu.adr_no_count(23usize, 10usize, 11usize, 2147240u32);
    emu.mulhu_no_count(11usize, 5usize, 24usize, 2147244u32);
    emu.mul_no_count(12usize, 30usize, 24usize, 2147248u32);
    emu.mulhu_no_count(14usize, 30usize, 24usize, 2147252u32);
    emu.mul_no_count(28usize, 5usize, 26usize, 2147256u32);
    emu.sltru_no_count(10usize, 23usize, 10usize, 2147260u32);
    emu.adr_no_count(6usize, 6usize, 10usize, 2147264u32);
    emu.mulhu_no_count(10usize, 5usize, 26usize, 2147268u32);
    emu.adr_no_count(11usize, 12usize, 11usize, 2147272u32);
    emu.sltru_no_count(12usize, 11usize, 12usize, 2147276u32);
    emu.adr_no_count(12usize, 14usize, 12usize, 2147280u32);
    emu.mul_no_count(14usize, 30usize, 26usize, 2147284u32);
    emu.adr_no_count(11usize, 28usize, 11usize, 2147288u32);
    emu.sltru_no_count(28usize, 11usize, 28usize, 2147292u32);
    emu.adr_no_count(10usize, 10usize, 28usize, 2147296u32);
    emu.mulhu_no_count(28usize, 30usize, 26usize, 2147300u32);
    emu.adr_no_count(8usize, 12usize, 10usize, 2147304u32);
    emu.sltru_no_count(10usize, 8usize, 12usize, 2147308u32);
    emu.adr_no_count(9usize, 28usize, 10usize, 2147312u32);
    emu.mul_no_count(28usize, 5usize, 24usize, 2147316u32);
    emu.adr_no_count(28usize, 18usize, 28usize, 2147320u32);
    emu.sltru_no_count(10usize, 28usize, 18usize, 2147324u32);
    emu.adr_no_count(11usize, 11usize, 10usize, 2147328u32);
    emu.adr_no_count(12usize, 14usize, 8usize, 2147332u32);
    emu.sltru_no_count(8usize, 12usize, 14usize, 2147336u32);
    emu.adr_no_count(11usize, 19usize, 11usize, 2147340u32);
    emu.adr_no_count(8usize, 9usize, 8usize, 2147344u32);
    emu.add_memory_rw_events(29usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2147352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c418));
    } else {
        emu.pc = 2147348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c414));
    }
}
#[inline(always)]
pub fn block_0x0020c414(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 11usize, 19usize, 2147352u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2147352u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c418));
}
#[inline]
pub fn block_0x0020c418(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(14usize, 27usize, 15usize, 2147356u32);
    emu.adr_no_count(10usize, 12usize, 10usize, 2147360u32);
    emu.adr_no_count(6usize, 11usize, 6usize, 2147364u32);
    emu.adr_no_count(23usize, 28usize, 23usize, 2147368u32);
    emu.sltru_no_count(12usize, 10usize, 12usize, 2147372u32);
    emu.sltru_no_count(25usize, 23usize, 28usize, 2147376u32);
    emu.adr_no_count(18usize, 6usize, 25usize, 2147380u32);
    emu.adr_no_count(8usize, 8usize, 12usize, 2147384u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2147392u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c440));
    } else {
        emu.pc = 2147388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c43c));
    }
}
#[inline(always)]
pub fn block_0x0020c43c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(25usize, 18usize, 11usize, 2147392u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2147392u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c440));
}
#[inline]
pub fn block_0x0020c440(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(28usize, 29usize, 1u32, 2147396u32);
    emu.adr_no_count(25usize, 10usize, 25usize, 2147400u32);
    emu.sbr_no_count(29usize, 0usize, 30usize, 2147404u32);
    emu.mulhu_no_count(11usize, 5usize, 7usize, 2147408u32);
    emu.mulhu_no_count(6usize, 30usize, 7usize, 2147412u32);
    emu.sbr_no_count(12usize, 0usize, 5usize, 2147416u32);
    emu.sltru_no_count(10usize, 25usize, 10usize, 2147420u32);
    emu.sbr_no_count(9usize, 11usize, 30usize, 2147424u32);
    emu.adr_no_count(10usize, 8usize, 10usize, 2147428u32);
    emu.sbr_no_count(8usize, 9usize, 5usize, 2147432u32);
    emu.sltru_no_count(12usize, 8usize, 12usize, 2147436u32);
    emu.adr_no_count(12usize, 11usize, 12usize, 2147440u32);
    emu.sbr_no_count(7usize, 31usize, 5usize, 2147444u32);
    emu.sltru_no_count(1usize, 7usize, 31usize, 2147448u32);
    emu.sltru_no_count(31usize, 9usize, 29usize, 2147452u32);
    emu.adr_no_count(31usize, 6usize, 31usize, 2147456u32);
    emu.adr_no_count(11usize, 8usize, 1usize, 2147460u32);
    emu.adr_no_count(11usize, 14usize, 11usize, 2147464u32);
    emu.adr_no_count(8usize, 31usize, 12usize, 2147468u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2147476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c494));
    } else {
        emu.pc = 2147472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c490));
    }
}
#[inline(always)]
pub fn block_0x0020c490(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(1usize, 11usize, 14usize, 2147476u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2147476u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c494));
}
#[inline]
pub fn block_0x0020c494(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(20usize, 2usize, 40u32, 2147480u32)?;
    emu.sltru_no_count(12usize, 14usize, 27usize, 2147484u32);
    emu.adi_no_count(28usize, 28usize, 4294967295u32, 2147488u32);
    emu.sbr_no_count(9usize, 8usize, 30usize, 2147492u32);
    emu.adr_no_count(10usize, 11usize, 10usize, 2147496u32);
    emu.adr_no_count(25usize, 7usize, 25usize, 2147500u32);
    emu.sltru_no_count(22usize, 25usize, 7usize, 2147504u32);
    emu.adr_no_count(19usize, 10usize, 22usize, 2147508u32);
    emu.sltru_no_count(14usize, 8usize, 31usize, 2147512u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2147520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c4c0));
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
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(22usize, 19usize, 11usize, 2147520u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2147520u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c4c0));
}
#[inline(never)]
pub fn block_0x0020c4c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 33u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(7usize, 28usize, 12usize, 2147524u32);
    emu.sltru_no_count(20usize, 9usize, 29usize, 2147528u32);
    emu.lw_no_count(12usize, 2usize, 112u32, 2147532u32)?;
    emu.lw_no_count(27usize, 2usize, 116u32, 2147536u32)?;
    emu.adr_no_count(14usize, 6usize, 14usize, 2147540u32);
    emu.adr_no_count(1usize, 9usize, 1usize, 2147544u32);
    emu.mulhu_no_count(10usize, 12usize, 13usize, 2147548u32);
    emu.mul_no_count(11usize, 27usize, 13usize, 2147552u32);
    emu.mulhu_no_count(6usize, 27usize, 13usize, 2147556u32);
    emu.lw_no_count(30usize, 2usize, 92u32, 2147560u32)?;
    emu.mul_no_count(28usize, 12usize, 30usize, 2147564u32);
    emu.mulhu_no_count(29usize, 12usize, 30usize, 2147568u32);
    emu.adr_no_count(10usize, 11usize, 10usize, 2147572u32);
    emu.sltru_no_count(11usize, 10usize, 11usize, 2147576u32);
    emu.adr_no_count(11usize, 6usize, 11usize, 2147580u32);
    emu.mul_no_count(6usize, 27usize, 30usize, 2147584u32);
    emu.adr_no_count(10usize, 28usize, 10usize, 2147588u32);
    emu.sltru_no_count(28usize, 10usize, 28usize, 2147592u32);
    emu.adr_no_count(28usize, 29usize, 28usize, 2147596u32);
    emu.mulhu_no_count(29usize, 27usize, 30usize, 2147600u32);
    emu.adr_no_count(28usize, 11usize, 28usize, 2147604u32);
    emu.sltru_no_count(11usize, 28usize, 11usize, 2147608u32);
    emu.adr_no_count(11usize, 29usize, 11usize, 2147612u32);
    emu.mul_no_count(29usize, 12usize, 13usize, 2147616u32);
    emu.adr_no_count(29usize, 23usize, 29usize, 2147620u32);
    emu.sw_no_count(29usize, 2usize, 0u32, 2147624u32)?;
    emu.sltru_no_count(8usize, 29usize, 23usize, 2147628u32);
    emu.adr_no_count(30usize, 10usize, 8usize, 2147632u32);
    emu.adr_no_count(10usize, 6usize, 28usize, 2147636u32);
    emu.sltru_no_count(6usize, 10usize, 6usize, 2147640u32);
    emu.adr_no_count(30usize, 18usize, 30usize, 2147644u32);
    emu.adr_no_count(11usize, 11usize, 6usize, 2147648u32);
    emu.add_memory_rw_events(32usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(30usize);
    if a == b {
        emu.pc = 2147656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c548));
    } else {
        emu.pc = 2147652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c544));
    }
}
#[inline(always)]
pub fn block_0x0020c544(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(8usize, 30usize, 18usize, 2147656u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2147656u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c548));
}
#[inline(never)]
pub fn block_0x0020c548(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 29u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(6usize, 17usize, 7usize, 2147660u32);
    emu.adr_no_count(20usize, 14usize, 20usize, 2147664u32);
    emu.sltru_no_count(14usize, 1usize, 9usize, 2147668u32);
    emu.adr_no_count(23usize, 1usize, 22usize, 2147672u32);
    emu.adr_no_count(8usize, 10usize, 8usize, 2147676u32);
    emu.mulhu_no_count(28usize, 12usize, 24usize, 2147680u32);
    emu.mul_no_count(29usize, 27usize, 24usize, 2147684u32);
    emu.mulhu_no_count(31usize, 27usize, 24usize, 2147688u32);
    emu.mul_no_count(9usize, 12usize, 26usize, 2147692u32);
    emu.mulhu_no_count(21usize, 12usize, 26usize, 2147696u32);
    emu.sltru_no_count(7usize, 8usize, 10usize, 2147700u32);
    emu.adr_no_count(7usize, 11usize, 7usize, 2147704u32);
    emu.mul_no_count(11usize, 27usize, 26usize, 2147708u32);
    emu.adr_no_count(10usize, 29usize, 28usize, 2147712u32);
    emu.sltru_no_count(28usize, 10usize, 29usize, 2147716u32);
    emu.adr_no_count(31usize, 31usize, 28usize, 2147720u32);
    emu.mul_no_count(28usize, 12usize, 24usize, 2147724u32);
    emu.adr_no_count(28usize, 25usize, 28usize, 2147728u32);
    emu.adr_no_count(10usize, 9usize, 10usize, 2147732u32);
    emu.sltru_no_count(18usize, 28usize, 25usize, 2147736u32);
    emu.sltru_no_count(29usize, 10usize, 9usize, 2147740u32);
    emu.adr_no_count(10usize, 10usize, 18usize, 2147744u32);
    emu.adr_no_count(29usize, 21usize, 29usize, 2147748u32);
    emu.adr_no_count(9usize, 19usize, 10usize, 2147752u32);
    emu.adr_no_count(29usize, 31usize, 29usize, 2147756u32);
    emu.adr_no_count(10usize, 11usize, 29usize, 2147760u32);
    emu.sltru_no_count(31usize, 29usize, 31usize, 2147764u32);
    emu.mulhu_no_count(29usize, 27usize, 26usize, 2147768u32);
    emu.add_memory_rw_events(28usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2147776u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c5c0));
    } else {
        emu.pc = 2147772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c5bc));
    }
}
#[inline(always)]
pub fn block_0x0020c5bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(18usize, 9usize, 19usize, 2147776u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2147776u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c5c0));
}
#[inline]
pub fn block_0x0020c5c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(26usize, 2usize, 56u32, 2147780u32)?;
    emu.sw_no_count(24usize, 2usize, 60u32, 2147784u32)?;
    emu.sltru_no_count(21usize, 10usize, 11usize, 2147788u32);
    emu.adr_no_count(31usize, 29usize, 31usize, 2147792u32);
    emu.sltru_no_count(19usize, 6usize, 17usize, 2147796u32);
    emu.adr_no_count(11usize, 20usize, 14usize, 2147800u32);
    emu.sltru_no_count(29usize, 23usize, 1usize, 2147804u32);
    emu.adr_no_count(7usize, 9usize, 7usize, 2147808u32);
    emu.adr_no_count(14usize, 28usize, 8usize, 2147812u32);
    emu.sltru_no_count(25usize, 14usize, 28usize, 2147816u32);
    emu.adr_no_count(7usize, 7usize, 25usize, 2147820u32);
    emu.adr_no_count(17usize, 10usize, 18usize, 2147824u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a == b {
        emu.pc = 2147832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c5f8));
    } else {
        emu.pc = 2147828u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c5f4));
    }
}
#[inline(always)]
pub fn block_0x0020c5f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(25usize, 7usize, 9usize, 2147832u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2147832u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c5f8));
}
#[inline(never)]
pub fn block_0x0020c5f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 30u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 31usize, 21usize, 2147836u32);
    emu.sltru_no_count(20usize, 17usize, 10usize, 2147840u32);
    emu.mul_no_count(10usize, 15usize, 13usize, 2147844u32);
    emu.sw_no_count(10usize, 2usize, 68u32, 2147848u32)?;
    emu.adr_no_count(16usize, 16usize, 19usize, 2147852u32);
    emu.lw_no_count(24usize, 2usize, 120u32, 2147856u32)?;
    emu.lw_no_count(26usize, 2usize, 124u32, 2147860u32)?;
    emu.adr_no_count(9usize, 11usize, 29usize, 2147864u32);
    emu.adr_no_count(25usize, 17usize, 25usize, 2147868u32);
    emu.mulhu_no_count(10usize, 24usize, 13usize, 2147872u32);
    emu.mul_no_count(11usize, 26usize, 13usize, 2147876u32);
    emu.mulhu_no_count(15usize, 26usize, 13usize, 2147880u32);
    emu.lw_no_count(22usize, 2usize, 92u32, 2147884u32)?;
    emu.mul_no_count(28usize, 24usize, 22usize, 2147888u32);
    emu.mulhu_no_count(29usize, 24usize, 22usize, 2147892u32);
    emu.adr_no_count(31usize, 11usize, 10usize, 2147896u32);
    emu.sltru_no_count(10usize, 31usize, 11usize, 2147900u32);
    emu.adr_no_count(10usize, 15usize, 10usize, 2147904u32);
    emu.sw_no_count(13usize, 2usize, 12u32, 2147908u32)?;
    emu.mul_no_count(18usize, 24usize, 13usize, 2147912u32);
    emu.adr_no_count(18usize, 14usize, 18usize, 2147916u32);
    emu.adr_no_count(31usize, 28usize, 31usize, 2147920u32);
    emu.sltru_no_count(21usize, 18usize, 14usize, 2147924u32);
    emu.sltru_no_count(11usize, 31usize, 28usize, 2147928u32);
    emu.adr_no_count(31usize, 31usize, 21usize, 2147932u32);
    emu.adr_no_count(11usize, 29usize, 11usize, 2147936u32);
    emu.adr_no_count(19usize, 7usize, 31usize, 2147940u32);
    emu.adr_no_count(11usize, 10usize, 11usize, 2147944u32);
    emu.mul_no_count(14usize, 26usize, 22usize, 2147948u32);
    emu.add_memory_rw_events(29usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2147956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c674));
    } else {
        emu.pc = 2147952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c670));
    }
}
#[inline(always)]
pub fn block_0x0020c670(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(21usize, 19usize, 7usize, 2147956u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2147956u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c674));
}
#[inline(never)]
pub fn block_0x0020c674(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 31u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(15usize, 8usize, 20usize, 2147960u32);
    emu.sltru_no_count(28usize, 25usize, 17usize, 2147964u32);
    emu.adr_no_count(20usize, 14usize, 11usize, 2147968u32);
    emu.sltru_no_count(1usize, 11usize, 10usize, 2147972u32);
    emu.lw_no_count(10usize, 2usize, 92u32, 2147976u32)?;
    emu.mulhu_no_count(22usize, 26usize, 10usize, 2147980u32);
    let a = 0u32.wrapping_add(1491623936u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2147984u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1125711872u32);
    emu.write_reg_no_count(29usize, a);
    emu.pc = 2147988u32;
    emu.update_insn_clock();
    emu.lw_no_count(8usize, 2usize, 128u32, 2147992u32)?;
    emu.lw_no_count(10usize, 2usize, 132u32, 2147996u32)?;
    let a = 0u32.wrapping_add(60612608u32);
    emu.write_reg_no_count(31usize, a);
    emu.pc = 2148000u32;
    emu.update_insn_clock();
    emu.adr_no_count(7usize, 6usize, 23usize, 2148004u32);
    emu.lw_no_count(23usize, 2usize, 72u32, 2148008u32)?;
    emu.lw_no_count(11usize, 2usize, 68u32, 2148012u32)?;
    emu.sltru_no_count(23usize, 23usize, 11usize, 2148016u32);
    emu.adi_no_count(11usize, 17usize, 380u32, 2148020u32);
    emu.adi_no_count(17usize, 29usize, 1362u32, 2148024u32);
    emu.adi_no_count(13usize, 31usize, 4294965935u32, 2148028u32);
    emu.sltru_no_count(29usize, 7usize, 6usize, 2148032u32);
    emu.mulhu_no_count(31usize, 24usize, 11usize, 2148036u32);
    emu.mul_no_count(6usize, 26usize, 11usize, 2148040u32);
    emu.adr_no_count(31usize, 6usize, 31usize, 2148044u32);
    emu.sw_no_count(17usize, 2usize, 20u32, 2148048u32)?;
    emu.mul_no_count(26usize, 24usize, 17usize, 2148052u32);
    emu.sw_no_count(11usize, 2usize, 16u32, 2148056u32)?;
    emu.mul_no_count(24usize, 24usize, 11usize, 2148060u32);
    emu.adr_no_count(16usize, 16usize, 9usize, 2148064u32);
    emu.adi_no_count(6usize, 23usize, 0u32, 2148068u32);
    emu.lw_no_count(9usize, 2usize, 80u32, 2148072u32)?;
    emu.lw_no_count(17usize, 2usize, 76u32, 2148076u32)?;
    emu.add_memory_rw_events(30usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2148092u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c6fc));
    } else {
        emu.pc = 2148080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c6f0));
    }
}
#[inline(always)]
pub fn block_0x0020c6f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(6usize, 2usize, 80u32, 2148084u32)?;
    emu.lw_no_count(9usize, 2usize, 76u32, 2148088u32)?;
    emu.sltru_no_count(6usize, 6usize, 9usize, 2148092u32);
    emu.add_memory_rw_events(3usize);
    emu.pc = 2148092u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c6fc));
}
#[inline]
pub fn block_0x0020c6fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 22u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(28usize, 15usize, 28usize, 2148096u32);
    emu.sltru_no_count(14usize, 20usize, 14usize, 2148100u32);
    emu.adr_no_count(1usize, 22usize, 1usize, 2148104u32);
    emu.adr_no_count(21usize, 20usize, 21usize, 2148108u32);
    emu.adr_no_count(9usize, 26usize, 31usize, 2148112u32);
    emu.mulhu_no_count(26usize, 8usize, 13usize, 2148116u32);
    emu.sw_no_count(13usize, 2usize, 84u32, 2148120u32)?;
    emu.mul_no_count(15usize, 10usize, 13usize, 2148124u32);
    let a = 0u32.wrapping_add(205926400u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2148128u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1341u32, 2148132u32);
    emu.sw_no_count(10usize, 2usize, 88u32, 2148136u32)?;
    emu.adr_no_count(27usize, 5usize, 27usize, 2148140u32);
    emu.adr_no_count(31usize, 12usize, 24usize, 2148144u32);
    emu.adr_no_count(16usize, 16usize, 29usize, 2148148u32);
    emu.lw_no_count(24usize, 2usize, 28u32, 2148152u32)?;
    emu.lw_no_count(13usize, 2usize, 8u32, 2148156u32)?;
    emu.sltru_no_count(22usize, 24usize, 13usize, 2148160u32);
    emu.adr_no_count(25usize, 7usize, 25usize, 2148164u32);
    emu.adi_no_count(29usize, 22usize, 0u32, 2148168u32);
    emu.lw_no_count(10usize, 2usize, 24u32, 2148172u32)?;
    emu.lw_no_count(17usize, 2usize, 4u32, 2148176u32)?;
    emu.add_memory_rw_events(21usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2148184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c758));
    } else {
        emu.pc = 2148180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c754));
    }
}
#[inline(always)]
pub fn block_0x0020c754(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(29usize, 10usize, 17usize, 2148184u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2148184u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c758));
}
#[inline(never)]
pub fn block_0x0020c758(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 26u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(14usize, 1usize, 14usize, 2148188u32);
    emu.adi_no_count(1usize, 10usize, 0u32, 2148192u32);
    emu.sltru_no_count(10usize, 21usize, 20usize, 2148196u32);
    emu.adr_no_count(15usize, 15usize, 26usize, 2148200u32);
    emu.lw_no_count(11usize, 2usize, 88u32, 2148204u32)?;
    emu.mul_no_count(11usize, 8usize, 11usize, 2148208u32);
    emu.lw_no_count(5usize, 2usize, 84u32, 2148212u32)?;
    emu.mul_no_count(5usize, 8usize, 5usize, 2148216u32);
    emu.adr_no_count(8usize, 27usize, 9usize, 2148220u32);
    emu.sltru_no_count(20usize, 31usize, 12usize, 2148224u32);
    emu.adr_no_count(9usize, 16usize, 28usize, 2148228u32);
    emu.sltru_no_count(7usize, 25usize, 7usize, 2148232u32);
    emu.sbr_no_count(12usize, 0usize, 29usize, 2148236u32);
    emu.sbr_no_count(16usize, 1usize, 17usize, 2148240u32);
    emu.sbr_no_count(26usize, 24usize, 13usize, 2148244u32);
    emu.adr_no_count(29usize, 6usize, 29usize, 2148248u32);
    emu.sbr_no_count(24usize, 16usize, 22usize, 2148252u32);
    emu.sbr_no_count(22usize, 0usize, 29usize, 2148256u32);
    emu.sbr_no_count(28usize, 26usize, 6usize, 2148260u32);
    emu.sltru_no_count(16usize, 22usize, 12usize, 2148264u32);
    emu.sltru_no_count(26usize, 28usize, 26usize, 2148268u32);
    emu.sbr_no_count(12usize, 6usize, 26usize, 2148272u32);
    emu.sbr_no_count(12usize, 24usize, 12usize, 2148276u32);
    emu.sbr_no_count(16usize, 16usize, 29usize, 2148280u32);
    emu.lw_no_count(13usize, 2usize, 12u32, 2148284u32)?;
    emu.add_memory_rw_events(25usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2148292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c7c4));
    } else {
        emu.pc = 2148288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c7c0));
    }
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
    emu.sltru_no_count(26usize, 12usize, 24usize, 2148292u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2148292u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c7c4));
}
#[inline]
pub fn block_0x0020c7c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(24usize, 14usize, 10usize, 2148296u32);
    emu.adr_no_count(10usize, 11usize, 15usize, 2148300u32);
    emu.adr_no_count(8usize, 8usize, 20usize, 2148304u32);
    emu.adr_no_count(5usize, 31usize, 5usize, 2148308u32);
    emu.adr_no_count(15usize, 9usize, 7usize, 2148312u32);
    emu.adr_no_count(6usize, 25usize, 21usize, 2148316u32);
    emu.sbr_no_count(11usize, 26usize, 29usize, 2148320u32);
    emu.sltru_no_count(11usize, 11usize, 22usize, 2148324u32);
    emu.adr_no_count(11usize, 16usize, 11usize, 2148328u32);
    emu.lw_no_count(9usize, 2usize, 36u32, 2148332u32)?;
    emu.lw_no_count(17usize, 2usize, 0u32, 2148336u32)?;
    emu.sltru_no_count(29usize, 9usize, 17usize, 2148340u32);
    emu.sai_no_count(7usize, 11usize, 1055u32, 2148344u32);
    emu.adi_no_count(16usize, 29usize, 0u32, 2148348u32);
    emu.lw_no_count(20usize, 2usize, 32u32, 2148352u32)?;
    emu.add_memory_rw_events(15usize);
    let b = emu.read_reg_b_tracked(30usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2148360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c808));
    } else {
        emu.pc = 2148356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c804));
    }
}
#[inline(always)]
pub fn block_0x0020c804(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 20usize, 30usize, 2148360u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2148360u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c808));
}
#[inline]
pub fn block_0x0020c808(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 8usize, 10usize, 2148364u32);
    emu.sltru_no_count(11usize, 5usize, 31usize, 2148368u32);
    emu.sltru_no_count(14usize, 6usize, 25usize, 2148372u32);
    emu.adr_no_count(15usize, 15usize, 24usize, 2148376u32);
    emu.sbr_no_count(8usize, 0usize, 16usize, 2148380u32);
    emu.sbr_no_count(30usize, 20usize, 30usize, 2148384u32);
    emu.sbr_no_count(9usize, 9usize, 17usize, 2148388u32);
    emu.sbr_no_count(16usize, 7usize, 16usize, 2148392u32);
    emu.sbr_no_count(31usize, 30usize, 29usize, 2148396u32);
    emu.sltru_no_count(8usize, 16usize, 8usize, 2148400u32);
    emu.adr_no_count(30usize, 9usize, 7usize, 2148404u32);
    emu.sltru_no_count(29usize, 30usize, 9usize, 2148408u32);
    emu.adr_no_count(7usize, 7usize, 29usize, 2148412u32);
    emu.adr_no_count(24usize, 31usize, 7usize, 2148416u32);
    emu.adr_no_count(7usize, 16usize, 8usize, 2148420u32);
    emu.lw_no_count(25usize, 2usize, 56u32, 2148424u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2148428u32)?;
    emu.lw_no_count(26usize, 2usize, 16u32, 2148432u32)?;
    emu.add_memory_rw_events(18usize);
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(24usize);
    if a == b {
        emu.pc = 2148440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c858));
    } else {
        emu.pc = 2148436u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c854));
    }
}
#[inline(always)]
pub fn block_0x0020c854(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(29usize, 24usize, 31usize, 2148440u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2148440u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c858));
}
#[inline]
pub fn block_0x0020c858(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(11usize, 10usize, 11usize, 2148444u32);
    emu.adr_no_count(15usize, 15usize, 14usize, 2148448u32);
    emu.lw_no_count(10usize, 2usize, 80u32, 2148452u32)?;
    emu.lw_no_count(14usize, 2usize, 76u32, 2148456u32)?;
    emu.sbr_no_count(10usize, 10usize, 14usize, 2148460u32);
    emu.adr_no_count(29usize, 16usize, 29usize, 2148464u32);
    emu.sltru_no_count(14usize, 29usize, 16usize, 2148468u32);
    emu.adr_no_count(7usize, 7usize, 14usize, 2148472u32);
    emu.lw_no_count(9usize, 2usize, 44u32, 2148476u32)?;
    emu.sltru_no_count(14usize, 9usize, 18usize, 2148480u32);
    emu.sai_no_count(29usize, 7usize, 1055u32, 2148484u32);
    emu.adi_no_count(31usize, 14usize, 0u32, 2148488u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2148496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c890));
    } else {
        emu.pc = 2148492u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c88c));
    }
}
#[inline(always)]
pub fn block_0x0020c88c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(31usize, 20usize, 19usize, 2148496u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2148496u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c890));
}
#[inline]
pub fn block_0x0020c890(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 11usize, 15usize, 2148500u32);
    emu.sltru_no_count(7usize, 5usize, 6usize, 2148504u32);
    emu.sbr_no_count(16usize, 5usize, 6usize, 2148508u32);
    emu.sbr_no_count(15usize, 10usize, 23usize, 2148512u32);
    emu.lw_no_count(10usize, 2usize, 72u32, 2148516u32)?;
    emu.lw_no_count(17usize, 2usize, 68u32, 2148520u32)?;
    emu.sbr_no_count(10usize, 10usize, 17usize, 2148524u32);
    emu.sbr_no_count(8usize, 0usize, 31usize, 2148528u32);
    emu.sbr_no_count(5usize, 20usize, 19usize, 2148532u32);
    emu.sbr_no_count(9usize, 9usize, 18usize, 2148536u32);
    emu.sbr_no_count(6usize, 29usize, 31usize, 2148540u32);
    emu.sbr_no_count(5usize, 5usize, 14usize, 2148544u32);
    emu.sltru_no_count(8usize, 6usize, 8usize, 2148548u32);
    emu.adr_no_count(14usize, 9usize, 29usize, 2148552u32);
    emu.sltru_no_count(31usize, 14usize, 9usize, 2148556u32);
    emu.adr_no_count(29usize, 29usize, 31usize, 2148560u32);
    emu.adr_no_count(18usize, 5usize, 29usize, 2148564u32);
    emu.adr_no_count(29usize, 6usize, 8usize, 2148568u32);
    emu.add_memory_rw_events(18usize);
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2148576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c8e0));
    } else {
        emu.pc = 2148572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c8dc));
    }
}
#[inline(always)]
pub fn block_0x0020c8dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(31usize, 18usize, 5usize, 2148576u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2148576u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c8e0));
}
#[inline]
pub fn block_0x0020c8e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(11usize, 11usize, 7usize, 2148580u32);
    emu.adr_no_count(31usize, 6usize, 31usize, 2148584u32);
    emu.lw_no_count(5usize, 2usize, 84u32, 2148588u32)?;
    emu.adr_no_count(5usize, 10usize, 5usize, 2148592u32);
    emu.sltru_no_count(6usize, 31usize, 6usize, 2148596u32);
    emu.sltru_no_count(10usize, 5usize, 10usize, 2148600u32);
    emu.adr_no_count(29usize, 29usize, 6usize, 2148604u32);
    emu.lw_no_count(6usize, 2usize, 88u32, 2148608u32)?;
    emu.adr_no_count(6usize, 10usize, 6usize, 2148612u32);
    emu.adr_no_count(6usize, 15usize, 6usize, 2148616u32);
    emu.lw_no_count(31usize, 2usize, 52u32, 2148620u32)?;
    emu.adr_no_count(16usize, 31usize, 16usize, 2148624u32);
    emu.lw_no_count(23usize, 2usize, 20u32, 2148628u32)?;
    emu.add_memory_rw_events(13usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2148636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c91c));
    } else {
        emu.pc = 2148632u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c918));
    }
}
#[inline(always)]
pub fn block_0x0020c918(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 6usize, 15usize, 2148636u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2148636u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c91c));
}
#[inline]
pub fn block_0x0020c91c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sai_no_count(7usize, 29usize, 1055u32, 2148640u32);
    emu.lw_no_count(15usize, 2usize, 48u32, 2148644u32)?;
    emu.adr_no_count(11usize, 15usize, 11usize, 2148648u32);
    emu.adi_no_count(15usize, 10usize, 4294967295u32, 2148652u32);
    emu.adr_no_count(28usize, 15usize, 28usize, 2148656u32);
    emu.sltru_no_count(29usize, 28usize, 15usize, 2148660u32);
    emu.adr_no_count(10usize, 15usize, 29usize, 2148664u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2148668u32);
    emu.sltru_no_count(8usize, 16usize, 31usize, 2148672u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(15usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2148680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c948));
    } else {
        emu.pc = 2148676u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c944));
    }
}
#[inline(always)]
pub fn block_0x0020c944(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(29usize, 10usize, 15usize, 2148680u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2148680u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c948));
}
#[inline]
pub fn block_0x0020c948(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 11usize, 8usize, 2148684u32);
    emu.adr_no_count(11usize, 15usize, 29usize, 2148688u32);
    emu.adr_no_count(29usize, 28usize, 26usize, 2148692u32);
    emu.sltru_no_count(31usize, 11usize, 15usize, 2148696u32);
    emu.sltru_no_count(12usize, 29usize, 28usize, 2148700u32);
    emu.adr_no_count(15usize, 15usize, 31usize, 2148704u32);
    emu.adr_no_count(31usize, 12usize, 23usize, 2148708u32);
    emu.adr_no_count(31usize, 10usize, 31usize, 2148712u32);
    emu.adr_no_count(28usize, 16usize, 7usize, 2148716u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(31usize);
    if a == b {
        emu.pc = 2148724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c974));
    } else {
        emu.pc = 2148720u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c970));
    }
}
#[inline(always)]
pub fn block_0x0020c970(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 31usize, 10usize, 2148724u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2148724u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c974));
}
#[inline]
pub fn block_0x0020c974(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(7usize, 8usize, 7usize, 2148728u32);
    emu.adi_no_count(10usize, 11usize, 4294967295u32, 2148732u32);
    emu.sltiu_no_count(11usize, 11usize, 1u32, 2148736u32);
    emu.adr_no_count(12usize, 10usize, 12usize, 2148740u32);
    emu.sbr_no_count(15usize, 15usize, 11usize, 2148744u32);
    emu.sltru_no_count(10usize, 12usize, 10usize, 2148748u32);
    emu.adr_no_count(10usize, 15usize, 10usize, 2148752u32);
    emu.sai_no_count(10usize, 10usize, 1055u32, 2148756u32);
    emu.adr_no_count(30usize, 10usize, 30usize, 2148760u32);
    emu.sltru_no_count(11usize, 30usize, 10usize, 2148764u32);
    emu.adr_no_count(19usize, 10usize, 24usize, 2148768u32);
    emu.adr_no_count(19usize, 19usize, 11usize, 2148772u32);
    emu.sltru_no_count(15usize, 28usize, 16usize, 2148776u32);
    emu.add_memory_rw_events(13usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2148784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c9b0));
    } else {
        emu.pc = 2148780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c9ac));
    }
}
#[inline(always)]
pub fn block_0x0020c9ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(11usize, 19usize, 10usize, 2148784u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2148784u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c9b0));
}
#[inline]
pub fn block_0x0020c9b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 22u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(11usize, 10usize, 11usize, 2148788u32);
    emu.adi_no_count(9usize, 30usize, 1u32, 2148792u32);
    emu.sltru_no_count(12usize, 11usize, 10usize, 2148796u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2148800u32);
    emu.sltiu_no_count(12usize, 9usize, 1u32, 2148804u32);
    emu.adr_no_count(19usize, 19usize, 12usize, 2148808u32);
    emu.adi_no_count(12usize, 11usize, 4294967295u32, 2148812u32);
    emu.sltiu_no_count(11usize, 11usize, 1u32, 2148816u32);
    emu.sbr_no_count(10usize, 10usize, 11usize, 2148820u32);
    emu.orr_no_count(11usize, 9usize, 19usize, 2148824u32);
    emu.sltiu_no_count(11usize, 11usize, 1u32, 2148828u32);
    emu.adr_no_count(11usize, 12usize, 11usize, 2148832u32);
    emu.sltru_no_count(11usize, 11usize, 12usize, 2148836u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2148840u32);
    emu.sai_no_count(10usize, 10usize, 1055u32, 2148844u32);
    emu.adr_no_count(30usize, 10usize, 14usize, 2148848u32);
    emu.sltru_no_count(12usize, 30usize, 10usize, 2148852u32);
    emu.adr_no_count(11usize, 10usize, 18usize, 2148856u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2148860u32);
    emu.adr_no_count(15usize, 7usize, 15usize, 2148864u32);
    emu.lw_no_count(22usize, 2usize, 60u32, 2148868u32)?;
    emu.add_memory_rw_events(21usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2148876u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ca0c));
    } else {
        emu.pc = 2148872u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ca08));
    }
}
#[inline(always)]
pub fn block_0x0020ca08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 11usize, 10usize, 2148876u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2148876u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ca0c));
}
#[inline]
pub fn block_0x0020ca0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(12usize, 10usize, 12usize, 2148880u32);
    emu.adi_no_count(18usize, 11usize, 1u32, 2148884u32);
    emu.sltru_no_count(14usize, 12usize, 10usize, 2148888u32);
    emu.adr_no_count(10usize, 10usize, 14usize, 2148892u32);
    emu.adi_no_count(14usize, 12usize, 4294967295u32, 2148896u32);
    emu.sltru_no_count(11usize, 18usize, 11usize, 2148900u32);
    emu.sltiu_no_count(12usize, 12usize, 1u32, 2148904u32);
    emu.adr_no_count(11usize, 14usize, 11usize, 2148908u32);
    emu.sltru_no_count(11usize, 11usize, 14usize, 2148912u32);
    emu.sbr_no_count(10usize, 10usize, 12usize, 2148916u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2148920u32);
    emu.sai_no_count(20usize, 10usize, 1055u32, 2148924u32);
    emu.adr_no_count(28usize, 20usize, 28usize, 2148928u32);
    emu.sltru_no_count(7usize, 28usize, 20usize, 2148932u32);
    emu.adr_no_count(14usize, 20usize, 15usize, 2148936u32);
    emu.adr_no_count(14usize, 14usize, 7usize, 2148940u32);
    emu.add_memory_rw_events(16usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2148948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ca54));
    } else {
        emu.pc = 2148944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ca50));
    }
}
#[inline(always)]
pub fn block_0x0020ca50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(7usize, 14usize, 20usize, 2148948u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2148948u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ca54));
}
#[inline]
pub fn block_0x0020ca54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(7usize, 20usize, 7usize, 2148952u32);
    emu.sltru_no_count(10usize, 7usize, 20usize, 2148956u32);
    emu.anr_no_count(11usize, 7usize, 13usize, 2148960u32);
    emu.adr_no_count(20usize, 20usize, 10usize, 2148964u32);
    emu.adr_no_count(5usize, 11usize, 5usize, 2148968u32);
    emu.lw_no_count(10usize, 2usize, 92u32, 2148972u32)?;
    emu.anr_no_count(10usize, 20usize, 10usize, 2148976u32);
    emu.sltru_no_count(16usize, 5usize, 11usize, 2148980u32);
    emu.adr_no_count(6usize, 10usize, 6usize, 2148984u32);
    emu.adr_no_count(21usize, 6usize, 16usize, 2148988u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a == b {
        emu.pc = 2148996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ca84));
    } else {
        emu.pc = 2148992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ca80));
    }
}
#[inline(always)]
pub fn block_0x0020ca80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 21usize, 10usize, 2148996u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2148996u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ca84));
}
#[inline(always)]
pub fn block_0x0020ca84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(11usize, 20usize, 25usize, 2149000u32);
    emu.anr_no_count(10usize, 7usize, 22usize, 2149004u32);
    emu.adr_no_count(29usize, 10usize, 29usize, 2149008u32);
    emu.sltru_no_count(10usize, 29usize, 10usize, 2149012u32);
    emu.adr_no_count(15usize, 11usize, 31usize, 2149016u32);
    emu.adr_no_count(15usize, 15usize, 10usize, 2149020u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2149028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020caa4));
    } else {
        emu.pc = 2149024u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020caa0));
    }
}
#[inline(always)]
pub fn block_0x0020caa0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 15usize, 11usize, 2149028u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2149028u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020caa4));
}
#[inline]
pub fn block_0x0020caa4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(16usize, 29usize, 16usize, 2149032u32);
    emu.adr_no_count(6usize, 7usize, 9usize, 2149036u32);
    emu.adr_no_count(12usize, 20usize, 19usize, 2149040u32);
    emu.sltru_no_count(29usize, 16usize, 29usize, 2149044u32);
    emu.sltru_no_count(11usize, 6usize, 7usize, 2149048u32);
    emu.adr_no_count(19usize, 15usize, 29usize, 2149052u32);
    emu.sltru_no_count(15usize, 19usize, 15usize, 2149056u32);
    emu.anr_no_count(9usize, 29usize, 15usize, 2149060u32);
    emu.adr_no_count(9usize, 10usize, 9usize, 2149064u32);
    emu.adr_no_count(12usize, 12usize, 11usize, 2149068u32);
    emu.sltru_no_count(10usize, 9usize, 10usize, 2149072u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2149080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cad8));
    } else {
        emu.pc = 2149076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cad4));
    }
}
#[inline(always)]
pub fn block_0x0020cad4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(11usize, 12usize, 20usize, 2149080u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2149080u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cad8));
}
#[inline(always)]
pub fn block_0x0020cad8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(9usize, 6usize, 9usize, 2149084u32);
    emu.sltru_no_count(15usize, 9usize, 6usize, 2149088u32);
    emu.adr_no_count(10usize, 12usize, 10usize, 2149092u32);
    emu.adr_no_count(31usize, 10usize, 15usize, 2149096u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(31usize);
    if a == b {
        emu.pc = 2149104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020caf0));
    } else {
        emu.pc = 2149100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020caec));
    }
}
#[inline(always)]
pub fn block_0x0020caec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(15usize, 31usize, 12usize, 2149104u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2149104u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020caf0));
}
#[inline(always)]
pub fn block_0x0020caf0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(15usize, 11usize, 15usize, 2149108u32);
    emu.adr_no_count(18usize, 20usize, 18usize, 2149112u32);
    emu.sltru_no_count(10usize, 15usize, 11usize, 2149116u32);
    emu.adr_no_count(29usize, 30usize, 15usize, 2149120u32);
    emu.sltru_no_count(8usize, 29usize, 30usize, 2149124u32);
    emu.adr_no_count(10usize, 18usize, 10usize, 2149128u32);
    emu.adr_no_count(6usize, 10usize, 8usize, 2149132u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2149140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cb14));
    } else {
        emu.pc = 2149136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cb10));
    }
}
#[inline(always)]
pub fn block_0x0020cb10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(8usize, 6usize, 18usize, 2149140u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2149140u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cb14));
}
#[inline(always)]
pub fn block_0x0020cb14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(17usize, 2usize, 84u32, 2149144u32)?;
    emu.adr_no_count(17usize, 5usize, 17usize, 2149148u32);
    emu.sltru_no_count(11usize, 17usize, 5usize, 2149152u32);
    emu.lw_no_count(5usize, 2usize, 88u32, 2149156u32)?;
    emu.adr_no_count(5usize, 11usize, 5usize, 2149160u32);
    emu.adr_no_count(5usize, 21usize, 5usize, 2149164u32);
    emu.sltru_no_count(10usize, 18usize, 20usize, 2149168u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2149176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cb38));
    } else {
        emu.pc = 2149172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cb34));
    }
}
#[inline(always)]
pub fn block_0x0020cb34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(11usize, 5usize, 21usize, 2149176u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2149176u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cb38));
}
#[inline(always)]
pub fn block_0x0020cb38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 11usize, 4294967295u32, 2149180u32);
    emu.adr_no_count(12usize, 16usize, 11usize, 2149184u32);
    emu.sltru_no_count(15usize, 12usize, 16usize, 2149188u32);
    emu.adr_no_count(18usize, 19usize, 11usize, 2149192u32);
    emu.adr_no_count(18usize, 18usize, 15usize, 2149196u32);
    emu.adr_no_count(8usize, 10usize, 8usize, 2149200u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2149208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cb58));
    } else {
        emu.pc = 2149204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cb54));
    }
}
#[inline(always)]
pub fn block_0x0020cb54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(15usize, 18usize, 19usize, 2149208u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2149208u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cb58));
}
#[inline]
pub fn block_0x0020cb58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(7usize, 8usize, 10usize, 2149212u32);
    emu.adr_no_count(30usize, 8usize, 28usize, 2149216u32);
    emu.adr_no_count(10usize, 11usize, 15usize, 2149220u32);
    emu.adr_no_count(16usize, 12usize, 26usize, 2149224u32);
    emu.sltru_no_count(28usize, 16usize, 12usize, 2149228u32);
    emu.adr_no_count(15usize, 18usize, 28usize, 2149232u32);
    emu.adr_no_count(15usize, 15usize, 23usize, 2149236u32);
    emu.sltru_no_count(12usize, 10usize, 11usize, 2149240u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2149244u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2149252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cb84));
    } else {
        emu.pc = 2149248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cb80));
    }
}
#[inline(always)]
pub fn block_0x0020cb80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 15usize, 18usize, 2149252u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2149252u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cb84));
}
#[inline]
pub fn block_0x0020cb84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltiu_no_count(12usize, 10usize, 1u32, 2149256u32);
    emu.sbr_no_count(11usize, 11usize, 12usize, 2149260u32);
    emu.sltru_no_count(8usize, 30usize, 8usize, 2149264u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2149268u32);
    emu.adr_no_count(28usize, 10usize, 28usize, 2149272u32);
    emu.sltru_no_count(10usize, 28usize, 10usize, 2149276u32);
    emu.adr_no_count(10usize, 11usize, 10usize, 2149280u32);
    emu.sai_no_count(10usize, 10usize, 1055u32, 2149284u32);
    emu.adr_no_count(28usize, 31usize, 10usize, 2149288u32);
    emu.adr_no_count(11usize, 9usize, 10usize, 2149292u32);
    emu.sltru_no_count(12usize, 11usize, 9usize, 2149296u32);
    emu.adr_no_count(28usize, 28usize, 12usize, 2149300u32);
    emu.adr_no_count(9usize, 7usize, 14usize, 2149304u32);
    emu.add_memory_rw_events(13usize);
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2149312u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cbc0));
    } else {
        emu.pc = 2149308u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cbbc));
    }
}
#[inline(always)]
pub fn block_0x0020cbbc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 28usize, 31usize, 2149312u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2149312u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cbc0));
}
#[inline]
pub fn block_0x0020cbc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(12usize, 10usize, 12usize, 2149316u32);
    emu.adi_no_count(7usize, 11usize, 1u32, 2149320u32);
    emu.sltru_no_count(11usize, 12usize, 10usize, 2149324u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2149328u32);
    emu.sltiu_no_count(11usize, 7usize, 1u32, 2149332u32);
    emu.adr_no_count(28usize, 28usize, 11usize, 2149336u32);
    emu.adi_no_count(11usize, 12usize, 4294967295u32, 2149340u32);
    emu.sltiu_no_count(12usize, 12usize, 1u32, 2149344u32);
    emu.sbr_no_count(10usize, 10usize, 12usize, 2149348u32);
    emu.orr_no_count(12usize, 7usize, 28usize, 2149352u32);
    emu.sltiu_no_count(12usize, 12usize, 1u32, 2149356u32);
    emu.adr_no_count(12usize, 11usize, 12usize, 2149360u32);
    emu.sltru_no_count(11usize, 12usize, 11usize, 2149364u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2149368u32);
    emu.sai_no_count(10usize, 10usize, 1055u32, 2149372u32);
    emu.adr_no_count(11usize, 6usize, 10usize, 2149376u32);
    emu.adr_no_count(14usize, 29usize, 10usize, 2149380u32);
    emu.sltru_no_count(12usize, 14usize, 29usize, 2149384u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2149388u32);
    emu.adr_no_count(8usize, 9usize, 8usize, 2149392u32);
    emu.add_memory_rw_events(20usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2149400u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cc18));
    } else {
        emu.pc = 2149396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cc14));
    }
}
#[inline(always)]
pub fn block_0x0020cc14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 11usize, 6usize, 2149400u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2149400u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cc18));
}
#[inline]
pub fn block_0x0020cc18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(12usize, 10usize, 12usize, 2149404u32);
    emu.adi_no_count(6usize, 11usize, 1u32, 2149408u32);
    emu.sltru_no_count(29usize, 12usize, 10usize, 2149412u32);
    emu.sltru_no_count(11usize, 6usize, 11usize, 2149416u32);
    emu.adr_no_count(10usize, 10usize, 29usize, 2149420u32);
    emu.adi_no_count(29usize, 12usize, 4294967295u32, 2149424u32);
    emu.sltiu_no_count(12usize, 12usize, 1u32, 2149428u32);
    emu.adr_no_count(11usize, 29usize, 11usize, 2149432u32);
    emu.sltru_no_count(11usize, 11usize, 29usize, 2149436u32);
    emu.sbr_no_count(10usize, 10usize, 12usize, 2149440u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2149444u32);
    emu.sai_no_count(10usize, 10usize, 1055u32, 2149448u32);
    emu.adr_no_count(30usize, 10usize, 30usize, 2149452u32);
    emu.sltru_no_count(29usize, 30usize, 10usize, 2149456u32);
    emu.adr_no_count(11usize, 10usize, 8usize, 2149460u32);
    emu.adr_no_count(11usize, 11usize, 29usize, 2149464u32);
    emu.add_memory_rw_events(16usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a == b {
        emu.pc = 2149472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cc60));
    } else {
        emu.pc = 2149468u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cc5c));
    }
}
#[inline(always)]
pub fn block_0x0020cc5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(29usize, 11usize, 10usize, 2149472u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2149472u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cc60));
}
#[inline]
pub fn block_0x0020cc60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(29usize, 10usize, 29usize, 2149476u32);
    emu.sltru_no_count(11usize, 29usize, 10usize, 2149480u32);
    emu.anr_no_count(12usize, 29usize, 13usize, 2149484u32);
    emu.adr_no_count(13usize, 10usize, 11usize, 2149488u32);
    emu.adr_no_count(17usize, 12usize, 17usize, 2149492u32);
    emu.lw_no_count(11usize, 2usize, 92u32, 2149496u32)?;
    emu.anr_no_count(11usize, 13usize, 11usize, 2149500u32);
    emu.sltru_no_count(10usize, 17usize, 12usize, 2149504u32);
    emu.adr_no_count(12usize, 11usize, 5usize, 2149508u32);
    emu.adr_no_count(12usize, 12usize, 10usize, 2149512u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2149520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cc90));
    } else {
        emu.pc = 2149516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cc8c));
    }
}
#[inline(always)]
pub fn block_0x0020cc8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 12usize, 11usize, 2149520u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2149520u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cc90));
}
#[inline(always)]
pub fn block_0x0020cc90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(30usize, 13usize, 25usize, 2149524u32);
    emu.anr_no_count(5usize, 29usize, 22usize, 2149528u32);
    emu.adr_no_count(11usize, 5usize, 16usize, 2149532u32);
    emu.sltru_no_count(16usize, 11usize, 5usize, 2149536u32);
    emu.adr_no_count(15usize, 30usize, 15usize, 2149540u32);
    emu.adr_no_count(5usize, 15usize, 16usize, 2149544u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(30usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2149552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ccb0));
    } else {
        emu.pc = 2149548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ccac));
    }
}
#[inline(always)]
pub fn block_0x0020ccac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 5usize, 30usize, 2149552u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2149552u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ccb0));
}
#[inline]
pub fn block_0x0020ccb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 11usize, 10usize, 2149556u32);
    emu.adr_no_count(7usize, 29usize, 7usize, 2149560u32);
    emu.adr_no_count(28usize, 13usize, 28usize, 2149564u32);
    emu.sltru_no_count(30usize, 10usize, 11usize, 2149568u32);
    emu.sltru_no_count(11usize, 7usize, 29usize, 2149572u32);
    emu.adr_no_count(15usize, 5usize, 30usize, 2149576u32);
    emu.sltru_no_count(5usize, 15usize, 5usize, 2149580u32);
    emu.anr_no_count(5usize, 30usize, 5usize, 2149584u32);
    emu.adr_no_count(5usize, 16usize, 5usize, 2149588u32);
    emu.adr_no_count(28usize, 28usize, 11usize, 2149592u32);
    emu.sltru_no_count(29usize, 5usize, 16usize, 2149596u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(13usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2149604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cce4));
    } else {
        emu.pc = 2149600u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cce0));
    }
}
#[inline(always)]
pub fn block_0x0020cce0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(11usize, 28usize, 13usize, 2149604u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2149604u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cce4));
}
#[inline(always)]
pub fn block_0x0020cce4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(16usize, 7usize, 5usize, 2149608u32);
    emu.sltru_no_count(7usize, 16usize, 7usize, 2149612u32);
    emu.adr_no_count(5usize, 28usize, 29usize, 2149616u32);
    emu.adr_no_count(5usize, 5usize, 7usize, 2149620u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2149628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ccfc));
    } else {
        emu.pc = 2149624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ccf8));
    }
}
#[inline(always)]
pub fn block_0x0020ccf8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(7usize, 5usize, 28usize, 2149628u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2149628u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ccfc));
}
#[inline(never)]
pub fn block_0x0020ccfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 31u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(7usize, 11usize, 7usize, 2149632u32);
    emu.adr_no_count(13usize, 13usize, 6usize, 2149636u32);
    emu.lw_no_count(6usize, 2usize, 64u32, 2149640u32)?;
    emu.sw_no_count(17usize, 6usize, 0u32, 2149644u32)?;
    emu.sw_no_count(12usize, 6usize, 4u32, 2149648u32)?;
    emu.sw_no_count(10usize, 6usize, 8u32, 2149652u32)?;
    emu.sw_no_count(15usize, 6usize, 12u32, 2149656u32)?;
    emu.sltru_no_count(10usize, 7usize, 11usize, 2149660u32);
    emu.adr_no_count(7usize, 14usize, 7usize, 2149664u32);
    emu.sltru_no_count(11usize, 7usize, 14usize, 2149668u32);
    emu.adr_no_count(10usize, 13usize, 10usize, 2149672u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2149676u32);
    emu.sw_no_count(16usize, 6usize, 16u32, 2149680u32)?;
    emu.sw_no_count(5usize, 6usize, 20u32, 2149684u32)?;
    emu.sw_no_count(7usize, 6usize, 24u32, 2149688u32)?;
    emu.sw_no_count(10usize, 6usize, 28u32, 2149692u32)?;
    emu.lw_no_count(1usize, 2usize, 252u32, 2149696u32)?;
    emu.lw_no_count(8usize, 2usize, 248u32, 2149700u32)?;
    emu.lw_no_count(9usize, 2usize, 244u32, 2149704u32)?;
    emu.lw_no_count(18usize, 2usize, 240u32, 2149708u32)?;
    emu.lw_no_count(19usize, 2usize, 236u32, 2149712u32)?;
    emu.lw_no_count(20usize, 2usize, 232u32, 2149716u32)?;
    emu.lw_no_count(21usize, 2usize, 228u32, 2149720u32)?;
    emu.lw_no_count(22usize, 2usize, 224u32, 2149724u32)?;
    emu.lw_no_count(23usize, 2usize, 220u32, 2149728u32)?;
    emu.lw_no_count(24usize, 2usize, 216u32, 2149732u32)?;
    emu.lw_no_count(25usize, 2usize, 212u32, 2149736u32)?;
    emu.lw_no_count(26usize, 2usize, 208u32, 2149740u32)?;
    emu.lw_no_count(27usize, 2usize, 204u32, 2149744u32)?;
    emu.adi_no_count(2usize, 2usize, 256u32, 2149748u32);
    emu.add_memory_rw_events(31usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2149752u32;
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
pub fn block_0x0020cd78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 69u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967088u32, 2149756u32);
    emu.sw_no_count(1usize, 2usize, 204u32, 2149760u32)?;
    emu.sw_no_count(8usize, 2usize, 200u32, 2149764u32)?;
    emu.sw_no_count(9usize, 2usize, 196u32, 2149768u32)?;
    emu.sw_no_count(18usize, 2usize, 192u32, 2149772u32)?;
    emu.sw_no_count(19usize, 2usize, 188u32, 2149776u32)?;
    emu.sw_no_count(20usize, 2usize, 184u32, 2149780u32)?;
    emu.sw_no_count(21usize, 2usize, 180u32, 2149784u32)?;
    emu.sw_no_count(22usize, 2usize, 176u32, 2149788u32)?;
    emu.sw_no_count(23usize, 2usize, 172u32, 2149792u32)?;
    emu.adi_no_count(9usize, 11usize, 0u32, 2149796u32);
    emu.adi_no_count(8usize, 10usize, 0u32, 2149800u32);
    emu.lw_no_count(17usize, 11usize, 16u32, 2149804u32)?;
    emu.lw_no_count(16usize, 11usize, 20u32, 2149808u32)?;
    emu.lw_no_count(15usize, 11usize, 24u32, 2149812u32)?;
    emu.lw_no_count(14usize, 11usize, 28u32, 2149816u32)?;
    emu.lw_no_count(13usize, 11usize, 0u32, 2149820u32)?;
    emu.lw_no_count(12usize, 11usize, 4u32, 2149824u32)?;
    emu.lw_no_count(11usize, 11usize, 8u32, 2149828u32)?;
    emu.lw_no_count(10usize, 9usize, 12u32, 2149832u32)?;
    emu.adi_no_count(5usize, 0usize, 4294967295u32, 2149836u32);
    let a = 0u32.wrapping_add(3169255424u32);
    emu.write_reg_no_count(6usize, a);
    emu.pc = 2149840u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2803343360u32);
    emu.write_reg_no_count(7usize, a);
    emu.pc = 2149844u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4089040896u32);
    emu.write_reg_no_count(28usize, a);
    emu.pc = 2149848u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(4234354688u32);
    emu.write_reg_no_count(29usize, a);
    emu.pc = 2149852u32;
    emu.update_insn_clock();
    emu.sw_no_count(0usize, 2usize, 92u32, 2149856u32)?;
    emu.sw_no_count(0usize, 2usize, 96u32, 2149860u32)?;
    emu.sw_no_count(0usize, 2usize, 100u32, 2149864u32)?;
    emu.sw_no_count(0usize, 2usize, 104u32, 2149868u32)?;
    emu.sw_no_count(5usize, 2usize, 60u32, 2149872u32)?;
    emu.sw_no_count(5usize, 2usize, 64u32, 2149876u32)?;
    emu.sw_no_count(0usize, 2usize, 68u32, 2149880u32)?;
    emu.sw_no_count(5usize, 2usize, 72u32, 2149884u32)?;
    emu.adi_no_count(5usize, 0usize, 1u32, 2149888u32);
    emu.sw_no_count(0usize, 2usize, 124u32, 2149892u32)?;
    emu.sw_no_count(0usize, 2usize, 128u32, 2149896u32)?;
    emu.sw_no_count(0usize, 2usize, 132u32, 2149900u32)?;
    emu.sw_no_count(0usize, 2usize, 136u32, 2149904u32)?;
    emu.sw_no_count(0usize, 2usize, 108u32, 2149908u32)?;
    emu.sw_no_count(0usize, 2usize, 112u32, 2149912u32)?;
    emu.sw_no_count(0usize, 2usize, 116u32, 2149916u32)?;
    emu.sw_no_count(0usize, 2usize, 120u32, 2149920u32)?;
    emu.adi_no_count(6usize, 6usize, 4294965933u32, 2149924u32);
    emu.adi_no_count(7usize, 7usize, 4294966916u32, 2149928u32);
    emu.adi_no_count(28usize, 28usize, 4294965954u32, 2149932u32);
    emu.adi_no_count(29usize, 29usize, 1361u32, 2149936u32);
    emu.sw_no_count(29usize, 2usize, 44u32, 2149940u32)?;
    emu.sw_no_count(28usize, 2usize, 48u32, 2149944u32)?;
    emu.sw_no_count(7usize, 2usize, 52u32, 2149948u32)?;
    emu.sw_no_count(6usize, 2usize, 56u32, 2149952u32)?;
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(18usize, a);
    emu.pc = 2149956u32;
    emu.update_insn_clock();
    emu.adi_no_count(18usize, 18usize, 696u32, 2149960u32);
    emu.sw_no_count(17usize, 2usize, 28u32, 2149964u32)?;
    emu.sw_no_count(16usize, 2usize, 32u32, 2149968u32)?;
    emu.sw_no_count(15usize, 2usize, 36u32, 2149972u32)?;
    emu.sw_no_count(14usize, 2usize, 40u32, 2149976u32)?;
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(19usize, a);
    emu.pc = 2149980u32;
    emu.update_insn_clock();
    emu.adi_no_count(19usize, 19usize, 728u32, 2149984u32);
    emu.sw_no_count(5usize, 2usize, 76u32, 2149988u32)?;
    emu.sw_no_count(0usize, 2usize, 80u32, 2149992u32)?;
    emu.sw_no_count(0usize, 2usize, 84u32, 2149996u32)?;
    emu.sw_no_count(0usize, 2usize, 88u32, 2150000u32)?;
    emu.sw_no_count(13usize, 2usize, 12u32, 2150004u32)?;
    emu.sw_no_count(12usize, 2usize, 16u32, 2150008u32)?;
    emu.sw_no_count(11usize, 2usize, 20u32, 2150012u32)?;
    emu.sw_no_count(10usize, 2usize, 24u32, 2150016u32)?;
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(20usize, a);
    emu.pc = 2150020u32;
    emu.update_insn_clock();
    emu.adi_no_count(20usize, 20usize, 792u32, 2150024u32);
    emu.add_memory_rw_events(69usize);
    let return_addr = 2150028u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2150204u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cf3c));
}
#[inline(always)]
pub fn block_0x0020ce8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 140u32, 2150032u32);
    emu.adi_no_count(11usize, 2usize, 12u32, 2150036u32);
    emu.adi_no_count(12usize, 2usize, 44u32, 2150040u32);
    emu.adi_no_count(13usize, 18usize, 0u32, 2150044u32);
    emu.apc_no_count(1usize, 2150044u32, 20480u32, 2150048u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2150052u32;
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
pub fn block_0x0020cea4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 22u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 156u32, 2150056u32)?;
    emu.lw_no_count(11usize, 2usize, 160u32, 2150060u32)?;
    emu.lw_no_count(12usize, 2usize, 164u32, 2150064u32)?;
    emu.lw_no_count(13usize, 2usize, 168u32, 2150068u32)?;
    emu.sw_no_count(10usize, 2usize, 28u32, 2150072u32)?;
    emu.sw_no_count(11usize, 2usize, 32u32, 2150076u32)?;
    emu.sw_no_count(12usize, 2usize, 36u32, 2150080u32)?;
    emu.sw_no_count(13usize, 2usize, 40u32, 2150084u32)?;
    emu.lw_no_count(10usize, 2usize, 140u32, 2150088u32)?;
    emu.lw_no_count(11usize, 2usize, 144u32, 2150092u32)?;
    emu.lw_no_count(12usize, 2usize, 148u32, 2150096u32)?;
    emu.lw_no_count(13usize, 2usize, 152u32, 2150100u32)?;
    emu.sw_no_count(10usize, 2usize, 12u32, 2150104u32)?;
    emu.sw_no_count(11usize, 2usize, 16u32, 2150108u32)?;
    emu.sw_no_count(12usize, 2usize, 20u32, 2150112u32)?;
    emu.sw_no_count(13usize, 2usize, 24u32, 2150116u32)?;
    emu.adi_no_count(10usize, 2usize, 140u32, 2150120u32);
    emu.adi_no_count(11usize, 2usize, 76u32, 2150124u32);
    emu.adi_no_count(12usize, 2usize, 108u32, 2150128u32);
    emu.adi_no_count(13usize, 18usize, 0u32, 2150132u32);
    emu.apc_no_count(1usize, 2150132u32, 20480u32, 2150136u32);
    emu.add_memory_rw_events(22usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2150140u32;
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
pub fn block_0x0020cefc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 156u32, 2150144u32)?;
    emu.lw_no_count(11usize, 2usize, 160u32, 2150148u32)?;
    emu.lw_no_count(12usize, 2usize, 164u32, 2150152u32)?;
    emu.lw_no_count(13usize, 2usize, 168u32, 2150156u32)?;
    emu.sw_no_count(10usize, 2usize, 92u32, 2150160u32)?;
    emu.sw_no_count(11usize, 2usize, 96u32, 2150164u32)?;
    emu.sw_no_count(12usize, 2usize, 100u32, 2150168u32)?;
    emu.sw_no_count(13usize, 2usize, 104u32, 2150172u32)?;
    emu.lw_no_count(10usize, 2usize, 140u32, 2150176u32)?;
    emu.lw_no_count(11usize, 2usize, 144u32, 2150180u32)?;
    emu.lw_no_count(12usize, 2usize, 148u32, 2150184u32)?;
    emu.lw_no_count(13usize, 2usize, 152u32, 2150188u32)?;
    emu.sw_no_count(10usize, 2usize, 76u32, 2150192u32)?;
    emu.sw_no_count(11usize, 2usize, 80u32, 2150196u32)?;
    emu.sw_no_count(12usize, 2usize, 84u32, 2150200u32)?;
    emu.sw_no_count(13usize, 2usize, 88u32, 2150204u32)?;
    emu.add_memory_rw_events(16usize);
    emu.pc = 2150204u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cf3c));
}
#[inline]
pub fn block_0x0020cf3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2150208u32)?;
    emu.lw_no_count(11usize, 2usize, 16u32, 2150212u32)?;
    emu.lw_no_count(12usize, 2usize, 20u32, 2150216u32)?;
    emu.lw_no_count(13usize, 2usize, 24u32, 2150220u32)?;
    emu.lw_no_count(14usize, 2usize, 28u32, 2150224u32)?;
    emu.lw_no_count(15usize, 2usize, 32u32, 2150228u32)?;
    emu.lw_no_count(16usize, 2usize, 36u32, 2150232u32)?;
    emu.lw_no_count(17usize, 2usize, 40u32, 2150236u32)?;
    emu.orr_no_count(10usize, 11usize, 10usize, 2150240u32);
    emu.orr_no_count(12usize, 12usize, 13usize, 2150244u32);
    emu.orr_no_count(14usize, 14usize, 15usize, 2150248u32);
    emu.orr_no_count(10usize, 10usize, 12usize, 2150252u32);
    emu.orr_no_count(11usize, 14usize, 16usize, 2150256u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2150260u32);
    emu.orr_no_count(10usize, 10usize, 17usize, 2150264u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2150268u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2150272u32);
    emu.apc_no_count(1usize, 2150272u32, 40960u32, 2150276u32);
    emu.add_memory_rw_events(19usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2150280u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1548u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020cf88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 255u32, 2150284u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2151868u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d5bc));
    } else {
        emu.pc = 2150288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cf90));
    }
}
#[inline(always)]
pub fn block_0x0020cf90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2150292u32)?;
    emu.ani_no_count(10usize, 10usize, 1u32, 2150296u32);
    emu.apc_no_count(1usize, 2150296u32, 40960u32, 2150300u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2150304u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1504u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020cfa0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(10usize, 10usize, 4294967295u32, 2150308u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2150312u32);
    emu.apc_no_count(1usize, 2150312u32, 40960u32, 2150316u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2150320u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1488u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020cfb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 255u32, 2150324u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2151340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d3ac));
    } else {
        emu.pc = 2150328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cfb8));
    }
}
#[inline(never)]
pub fn block_0x0020cfb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 42u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 12u32, 2150332u32)?;
    emu.lw_no_count(11usize, 2usize, 16u32, 2150336u32)?;
    emu.lw_no_count(12usize, 2usize, 20u32, 2150340u32)?;
    emu.lw_no_count(13usize, 2usize, 24u32, 2150344u32)?;
    emu.lw_no_count(14usize, 2usize, 28u32, 2150348u32)?;
    emu.lw_no_count(15usize, 2usize, 32u32, 2150352u32)?;
    emu.lw_no_count(16usize, 2usize, 36u32, 2150356u32)?;
    emu.lw_no_count(17usize, 2usize, 40u32, 2150360u32)?;
    emu.sri_no_count(10usize, 10usize, 1u32, 2150364u32);
    emu.sli_no_count(5usize, 11usize, 31u32, 2150368u32);
    emu.sri_no_count(11usize, 11usize, 1u32, 2150372u32);
    emu.orr_no_count(10usize, 5usize, 10usize, 2150376u32);
    emu.sli_no_count(5usize, 12usize, 31u32, 2150380u32);
    emu.sri_no_count(12usize, 12usize, 1u32, 2150384u32);
    emu.orr_no_count(11usize, 5usize, 11usize, 2150388u32);
    emu.sli_no_count(5usize, 13usize, 31u32, 2150392u32);
    emu.sri_no_count(13usize, 13usize, 1u32, 2150396u32);
    emu.orr_no_count(12usize, 5usize, 12usize, 2150400u32);
    emu.sli_no_count(5usize, 14usize, 31u32, 2150404u32);
    emu.sri_no_count(14usize, 14usize, 1u32, 2150408u32);
    emu.orr_no_count(13usize, 5usize, 13usize, 2150412u32);
    emu.sli_no_count(5usize, 15usize, 31u32, 2150416u32);
    emu.sri_no_count(15usize, 15usize, 1u32, 2150420u32);
    emu.orr_no_count(14usize, 5usize, 14usize, 2150424u32);
    emu.sli_no_count(5usize, 16usize, 31u32, 2150428u32);
    emu.sri_no_count(16usize, 16usize, 1u32, 2150432u32);
    emu.orr_no_count(15usize, 5usize, 15usize, 2150436u32);
    emu.sli_no_count(5usize, 17usize, 31u32, 2150440u32);
    emu.orr_no_count(16usize, 5usize, 16usize, 2150444u32);
    emu.lw_no_count(5usize, 2usize, 76u32, 2150448u32)?;
    emu.sri_no_count(17usize, 17usize, 1u32, 2150452u32);
    emu.sw_no_count(10usize, 2usize, 12u32, 2150456u32)?;
    emu.sw_no_count(11usize, 2usize, 16u32, 2150460u32)?;
    emu.sw_no_count(12usize, 2usize, 20u32, 2150464u32)?;
    emu.sw_no_count(13usize, 2usize, 24u32, 2150468u32)?;
    emu.sw_no_count(14usize, 2usize, 28u32, 2150472u32)?;
    emu.sw_no_count(15usize, 2usize, 32u32, 2150476u32)?;
    emu.sw_no_count(16usize, 2usize, 36u32, 2150480u32)?;
    emu.sw_no_count(17usize, 2usize, 40u32, 2150484u32)?;
    emu.ani_no_count(10usize, 5usize, 1u32, 2150488u32);
    emu.apc_no_count(1usize, 2150488u32, 40960u32, 2150492u32);
    emu.add_memory_rw_events(42usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2150496u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1312u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0020d060(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 40u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 255u32, 2150500u32);
    emu.lw_no_count(11usize, 2usize, 76u32, 2150504u32)?;
    emu.lw_no_count(12usize, 2usize, 80u32, 2150508u32)?;
    emu.lw_no_count(13usize, 2usize, 84u32, 2150512u32)?;
    emu.lw_no_count(14usize, 2usize, 88u32, 2150516u32)?;
    emu.lw_no_count(15usize, 2usize, 92u32, 2150520u32)?;
    emu.lw_no_count(16usize, 2usize, 96u32, 2150524u32)?;
    emu.lw_no_count(17usize, 2usize, 100u32, 2150528u32)?;
    emu.lw_no_count(5usize, 2usize, 104u32, 2150532u32)?;
    emu.sri_no_count(11usize, 11usize, 1u32, 2150536u32);
    emu.sli_no_count(6usize, 12usize, 31u32, 2150540u32);
    emu.sri_no_count(12usize, 12usize, 1u32, 2150544u32);
    emu.orr_no_count(11usize, 6usize, 11usize, 2150548u32);
    emu.sli_no_count(6usize, 13usize, 31u32, 2150552u32);
    emu.sri_no_count(13usize, 13usize, 1u32, 2150556u32);
    emu.orr_no_count(12usize, 6usize, 12usize, 2150560u32);
    emu.sli_no_count(6usize, 14usize, 31u32, 2150564u32);
    emu.sri_no_count(14usize, 14usize, 1u32, 2150568u32);
    emu.orr_no_count(13usize, 6usize, 13usize, 2150572u32);
    emu.sli_no_count(6usize, 15usize, 31u32, 2150576u32);
    emu.sri_no_count(15usize, 15usize, 1u32, 2150580u32);
    emu.orr_no_count(14usize, 6usize, 14usize, 2150584u32);
    emu.sli_no_count(6usize, 16usize, 31u32, 2150588u32);
    emu.sri_no_count(16usize, 16usize, 1u32, 2150592u32);
    emu.orr_no_count(15usize, 6usize, 15usize, 2150596u32);
    emu.sli_no_count(6usize, 17usize, 31u32, 2150600u32);
    emu.sri_no_count(17usize, 17usize, 1u32, 2150604u32);
    emu.orr_no_count(16usize, 6usize, 16usize, 2150608u32);
    emu.sli_no_count(6usize, 5usize, 31u32, 2150612u32);
    emu.sri_no_count(5usize, 5usize, 1u32, 2150616u32);
    emu.orr_no_count(17usize, 6usize, 17usize, 2150620u32);
    emu.sw_no_count(11usize, 2usize, 76u32, 2150624u32)?;
    emu.sw_no_count(12usize, 2usize, 80u32, 2150628u32)?;
    emu.sw_no_count(13usize, 2usize, 84u32, 2150632u32)?;
    emu.sw_no_count(14usize, 2usize, 88u32, 2150636u32)?;
    emu.sw_no_count(15usize, 2usize, 92u32, 2150640u32)?;
    emu.sw_no_count(16usize, 2usize, 96u32, 2150644u32)?;
    emu.sw_no_count(17usize, 2usize, 100u32, 2150648u32)?;
    emu.sw_no_count(5usize, 2usize, 104u32, 2150652u32)?;
    emu.add_memory_rw_events(39usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2150288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020cf90));
    } else {
        emu.pc = 2150656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d100));
    }
}
#[inline(always)]
pub fn block_0x0020d100(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 140u32, 2150660u32);
    emu.adi_no_count(11usize, 2usize, 76u32, 2150664u32);
    emu.adi_no_count(12usize, 19usize, 0u32, 2150668u32);
    emu.adi_no_count(13usize, 18usize, 0u32, 2150672u32);
    emu.apc_no_count(1usize, 2150672u32, 20480u32, 2150676u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2150680u32;
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
pub fn block_0x0020d118(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 22u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 156u32, 2150684u32)?;
    emu.lw_no_count(11usize, 2usize, 160u32, 2150688u32)?;
    emu.lw_no_count(12usize, 2usize, 164u32, 2150692u32)?;
    emu.lw_no_count(13usize, 2usize, 168u32, 2150696u32)?;
    emu.sw_no_count(10usize, 2usize, 92u32, 2150700u32)?;
    emu.sw_no_count(11usize, 2usize, 96u32, 2150704u32)?;
    emu.sw_no_count(12usize, 2usize, 100u32, 2150708u32)?;
    emu.sw_no_count(13usize, 2usize, 104u32, 2150712u32)?;
    emu.lw_no_count(10usize, 2usize, 140u32, 2150716u32)?;
    emu.lw_no_count(11usize, 2usize, 144u32, 2150720u32)?;
    emu.lw_no_count(12usize, 2usize, 148u32, 2150724u32)?;
    emu.lw_no_count(13usize, 2usize, 152u32, 2150728u32)?;
    emu.sw_no_count(10usize, 2usize, 76u32, 2150732u32)?;
    emu.sw_no_count(11usize, 2usize, 80u32, 2150736u32)?;
    emu.sw_no_count(12usize, 2usize, 84u32, 2150740u32)?;
    emu.sw_no_count(13usize, 2usize, 88u32, 2150744u32)?;
    emu.adi_no_count(10usize, 2usize, 140u32, 2150748u32);
    emu.adi_no_count(11usize, 2usize, 76u32, 2150752u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2150756u32);
    emu.adi_no_count(13usize, 18usize, 0u32, 2150760u32);
    emu.apc_no_count(1usize, 2150760u32, 20480u32, 2150764u32);
    emu.add_memory_rw_events(22usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2150768u32;
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
pub fn block_0x0020d170(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 156u32, 2150772u32)?;
    emu.lw_no_count(11usize, 2usize, 160u32, 2150776u32)?;
    emu.lw_no_count(12usize, 2usize, 164u32, 2150780u32)?;
    emu.lw_no_count(13usize, 2usize, 168u32, 2150784u32)?;
    emu.sw_no_count(10usize, 2usize, 92u32, 2150788u32)?;
    emu.sw_no_count(11usize, 2usize, 96u32, 2150792u32)?;
    emu.sw_no_count(12usize, 2usize, 100u32, 2150796u32)?;
    emu.sw_no_count(13usize, 2usize, 104u32, 2150800u32)?;
    emu.lw_no_count(10usize, 2usize, 140u32, 2150804u32)?;
    emu.lw_no_count(11usize, 2usize, 144u32, 2150808u32)?;
    emu.lw_no_count(12usize, 2usize, 148u32, 2150812u32)?;
    emu.lw_no_count(13usize, 2usize, 152u32, 2150816u32)?;
    emu.sw_no_count(10usize, 2usize, 76u32, 2150820u32)?;
    emu.sw_no_count(11usize, 2usize, 80u32, 2150824u32)?;
    emu.sw_no_count(12usize, 2usize, 84u32, 2150828u32)?;
    emu.sw_no_count(13usize, 2usize, 88u32, 2150832u32)?;
    emu.add_memory_rw_events(17usize);
    let return_addr = 2150836u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2150288u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cf90));
}
#[inline(never)]
pub fn block_0x0020d1b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 42u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 44u32, 2150840u32)?;
    emu.lw_no_count(11usize, 2usize, 48u32, 2150844u32)?;
    emu.lw_no_count(12usize, 2usize, 52u32, 2150848u32)?;
    emu.lw_no_count(13usize, 2usize, 56u32, 2150852u32)?;
    emu.lw_no_count(14usize, 2usize, 60u32, 2150856u32)?;
    emu.lw_no_count(15usize, 2usize, 64u32, 2150860u32)?;
    emu.lw_no_count(16usize, 2usize, 68u32, 2150864u32)?;
    emu.lw_no_count(17usize, 2usize, 72u32, 2150868u32)?;
    emu.sri_no_count(10usize, 10usize, 1u32, 2150872u32);
    emu.sli_no_count(5usize, 11usize, 31u32, 2150876u32);
    emu.sri_no_count(11usize, 11usize, 1u32, 2150880u32);
    emu.orr_no_count(10usize, 5usize, 10usize, 2150884u32);
    emu.sli_no_count(5usize, 12usize, 31u32, 2150888u32);
    emu.sri_no_count(12usize, 12usize, 1u32, 2150892u32);
    emu.orr_no_count(11usize, 5usize, 11usize, 2150896u32);
    emu.sli_no_count(5usize, 13usize, 31u32, 2150900u32);
    emu.sri_no_count(13usize, 13usize, 1u32, 2150904u32);
    emu.orr_no_count(12usize, 5usize, 12usize, 2150908u32);
    emu.sli_no_count(5usize, 14usize, 31u32, 2150912u32);
    emu.sri_no_count(14usize, 14usize, 1u32, 2150916u32);
    emu.orr_no_count(13usize, 5usize, 13usize, 2150920u32);
    emu.sli_no_count(5usize, 15usize, 31u32, 2150924u32);
    emu.sri_no_count(15usize, 15usize, 1u32, 2150928u32);
    emu.orr_no_count(14usize, 5usize, 14usize, 2150932u32);
    emu.sli_no_count(5usize, 16usize, 31u32, 2150936u32);
    emu.sri_no_count(16usize, 16usize, 1u32, 2150940u32);
    emu.orr_no_count(15usize, 5usize, 15usize, 2150944u32);
    emu.sli_no_count(5usize, 17usize, 31u32, 2150948u32);
    emu.orr_no_count(16usize, 5usize, 16usize, 2150952u32);
    emu.lw_no_count(5usize, 2usize, 108u32, 2150956u32)?;
    emu.sri_no_count(17usize, 17usize, 1u32, 2150960u32);
    emu.sw_no_count(10usize, 2usize, 44u32, 2150964u32)?;
    emu.sw_no_count(11usize, 2usize, 48u32, 2150968u32)?;
    emu.sw_no_count(12usize, 2usize, 52u32, 2150972u32)?;
    emu.sw_no_count(13usize, 2usize, 56u32, 2150976u32)?;
    emu.sw_no_count(14usize, 2usize, 60u32, 2150980u32)?;
    emu.sw_no_count(15usize, 2usize, 64u32, 2150984u32)?;
    emu.sw_no_count(16usize, 2usize, 68u32, 2150988u32)?;
    emu.sw_no_count(17usize, 2usize, 72u32, 2150992u32)?;
    emu.ani_no_count(10usize, 5usize, 1u32, 2150996u32);
    emu.apc_no_count(1usize, 2150996u32, 40960u32, 2151000u32);
    emu.add_memory_rw_events(42usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2151004u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(804u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x0020d25c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 40u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 255u32, 2151008u32);
    emu.lw_no_count(11usize, 2usize, 108u32, 2151012u32)?;
    emu.lw_no_count(12usize, 2usize, 112u32, 2151016u32)?;
    emu.lw_no_count(13usize, 2usize, 116u32, 2151020u32)?;
    emu.lw_no_count(14usize, 2usize, 120u32, 2151024u32)?;
    emu.lw_no_count(15usize, 2usize, 124u32, 2151028u32)?;
    emu.lw_no_count(16usize, 2usize, 128u32, 2151032u32)?;
    emu.lw_no_count(17usize, 2usize, 132u32, 2151036u32)?;
    emu.lw_no_count(5usize, 2usize, 136u32, 2151040u32)?;
    emu.sri_no_count(11usize, 11usize, 1u32, 2151044u32);
    emu.sli_no_count(6usize, 12usize, 31u32, 2151048u32);
    emu.sri_no_count(12usize, 12usize, 1u32, 2151052u32);
    emu.orr_no_count(11usize, 6usize, 11usize, 2151056u32);
    emu.sli_no_count(6usize, 13usize, 31u32, 2151060u32);
    emu.sri_no_count(13usize, 13usize, 1u32, 2151064u32);
    emu.orr_no_count(12usize, 6usize, 12usize, 2151068u32);
    emu.sli_no_count(6usize, 14usize, 31u32, 2151072u32);
    emu.sri_no_count(14usize, 14usize, 1u32, 2151076u32);
    emu.orr_no_count(13usize, 6usize, 13usize, 2151080u32);
    emu.sli_no_count(6usize, 15usize, 31u32, 2151084u32);
    emu.sri_no_count(15usize, 15usize, 1u32, 2151088u32);
    emu.orr_no_count(14usize, 6usize, 14usize, 2151092u32);
    emu.sli_no_count(6usize, 16usize, 31u32, 2151096u32);
    emu.sri_no_count(16usize, 16usize, 1u32, 2151100u32);
    emu.orr_no_count(15usize, 6usize, 15usize, 2151104u32);
    emu.sli_no_count(6usize, 17usize, 31u32, 2151108u32);
    emu.sri_no_count(17usize, 17usize, 1u32, 2151112u32);
    emu.orr_no_count(16usize, 6usize, 16usize, 2151116u32);
    emu.sli_no_count(6usize, 5usize, 31u32, 2151120u32);
    emu.sri_no_count(5usize, 5usize, 1u32, 2151124u32);
    emu.orr_no_count(17usize, 6usize, 17usize, 2151128u32);
    emu.sw_no_count(11usize, 2usize, 108u32, 2151132u32)?;
    emu.sw_no_count(12usize, 2usize, 112u32, 2151136u32)?;
    emu.sw_no_count(13usize, 2usize, 116u32, 2151140u32)?;
    emu.sw_no_count(14usize, 2usize, 120u32, 2151144u32)?;
    emu.sw_no_count(15usize, 2usize, 124u32, 2151148u32)?;
    emu.sw_no_count(16usize, 2usize, 128u32, 2151152u32)?;
    emu.sw_no_count(17usize, 2usize, 132u32, 2151156u32)?;
    emu.sw_no_count(5usize, 2usize, 136u32, 2151160u32)?;
    emu.add_memory_rw_events(39usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2151340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d3ac));
    } else {
        emu.pc = 2151164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d2fc));
    }
}
#[inline(always)]
pub fn block_0x0020d2fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 140u32, 2151168u32);
    emu.adi_no_count(11usize, 2usize, 108u32, 2151172u32);
    emu.adi_no_count(12usize, 19usize, 0u32, 2151176u32);
    emu.adi_no_count(13usize, 18usize, 0u32, 2151180u32);
    emu.apc_no_count(1usize, 2151180u32, 20480u32, 2151184u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2151188u32;
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
pub fn block_0x0020d314(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 22u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 156u32, 2151192u32)?;
    emu.lw_no_count(11usize, 2usize, 160u32, 2151196u32)?;
    emu.lw_no_count(12usize, 2usize, 164u32, 2151200u32)?;
    emu.lw_no_count(13usize, 2usize, 168u32, 2151204u32)?;
    emu.sw_no_count(10usize, 2usize, 124u32, 2151208u32)?;
    emu.sw_no_count(11usize, 2usize, 128u32, 2151212u32)?;
    emu.sw_no_count(12usize, 2usize, 132u32, 2151216u32)?;
    emu.sw_no_count(13usize, 2usize, 136u32, 2151220u32)?;
    emu.lw_no_count(10usize, 2usize, 140u32, 2151224u32)?;
    emu.lw_no_count(11usize, 2usize, 144u32, 2151228u32)?;
    emu.lw_no_count(12usize, 2usize, 148u32, 2151232u32)?;
    emu.lw_no_count(13usize, 2usize, 152u32, 2151236u32)?;
    emu.sw_no_count(10usize, 2usize, 108u32, 2151240u32)?;
    emu.sw_no_count(11usize, 2usize, 112u32, 2151244u32)?;
    emu.sw_no_count(12usize, 2usize, 116u32, 2151248u32)?;
    emu.sw_no_count(13usize, 2usize, 120u32, 2151252u32)?;
    emu.adi_no_count(10usize, 2usize, 140u32, 2151256u32);
    emu.adi_no_count(11usize, 2usize, 108u32, 2151260u32);
    emu.adi_no_count(12usize, 20usize, 0u32, 2151264u32);
    emu.adi_no_count(13usize, 18usize, 0u32, 2151268u32);
    emu.apc_no_count(1usize, 2151268u32, 20480u32, 2151272u32);
    emu.add_memory_rw_events(22usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2151276u32;
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
pub fn block_0x0020d36c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 156u32, 2151280u32)?;
    emu.lw_no_count(11usize, 2usize, 160u32, 2151284u32)?;
    emu.lw_no_count(12usize, 2usize, 164u32, 2151288u32)?;
    emu.lw_no_count(13usize, 2usize, 168u32, 2151292u32)?;
    emu.sw_no_count(10usize, 2usize, 124u32, 2151296u32)?;
    emu.sw_no_count(11usize, 2usize, 128u32, 2151300u32)?;
    emu.sw_no_count(12usize, 2usize, 132u32, 2151304u32)?;
    emu.sw_no_count(13usize, 2usize, 136u32, 2151308u32)?;
    emu.lw_no_count(10usize, 2usize, 140u32, 2151312u32)?;
    emu.lw_no_count(11usize, 2usize, 144u32, 2151316u32)?;
    emu.lw_no_count(12usize, 2usize, 148u32, 2151320u32)?;
    emu.lw_no_count(13usize, 2usize, 152u32, 2151324u32)?;
    emu.sw_no_count(10usize, 2usize, 108u32, 2151328u32)?;
    emu.sw_no_count(11usize, 2usize, 112u32, 2151332u32)?;
    emu.sw_no_count(12usize, 2usize, 116u32, 2151336u32)?;
    emu.sw_no_count(13usize, 2usize, 120u32, 2151340u32)?;
    emu.add_memory_rw_events(16usize);
    emu.pc = 2151340u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020d3ac));
}
#[inline(always)]
pub fn block_0x0020d3ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 44u32, 2151344u32)?;
    emu.ani_no_count(10usize, 10usize, 1u32, 2151348u32);
    emu.apc_no_count(1usize, 2151348u32, 40960u32, 2151352u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2151356u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(452u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020d3bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(10usize, 10usize, 4294967295u32, 2151360u32);
    emu.ani_no_count(10usize, 10usize, 1u32, 2151364u32);
    emu.apc_no_count(1usize, 2151364u32, 40960u32, 2151368u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2151372u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(436u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020d3cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 10usize, 255u32, 2151376u32);
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2150836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d1b4));
    } else {
        emu.pc = 2151380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d3d4));
    }
}
#[inline(never)]
pub fn block_0x0020d3d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 67u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 44u32, 2151384u32)?;
    emu.lw_no_count(13usize, 2usize, 48u32, 2151388u32)?;
    emu.lw_no_count(14usize, 2usize, 52u32, 2151392u32)?;
    emu.lw_no_count(15usize, 2usize, 56u32, 2151396u32)?;
    emu.lw_no_count(11usize, 2usize, 12u32, 2151400u32)?;
    emu.lw_no_count(12usize, 2usize, 16u32, 2151404u32)?;
    emu.lw_no_count(16usize, 2usize, 20u32, 2151408u32)?;
    emu.lw_no_count(17usize, 2usize, 24u32, 2151412u32)?;
    emu.lw_no_count(5usize, 2usize, 60u32, 2151416u32)?;
    emu.lw_no_count(6usize, 2usize, 64u32, 2151420u32)?;
    emu.lw_no_count(7usize, 2usize, 68u32, 2151424u32)?;
    emu.lw_no_count(28usize, 2usize, 72u32, 2151428u32)?;
    emu.lw_no_count(29usize, 2usize, 28u32, 2151432u32)?;
    emu.lw_no_count(30usize, 2usize, 32u32, 2151436u32)?;
    emu.lw_no_count(31usize, 2usize, 36u32, 2151440u32)?;
    emu.lw_no_count(21usize, 2usize, 40u32, 2151444u32)?;
    emu.sltru_no_count(22usize, 10usize, 11usize, 2151448u32);
    emu.sbr_no_count(23usize, 14usize, 16usize, 2151452u32);
    emu.sltru_no_count(14usize, 14usize, 16usize, 2151456u32);
    emu.sbr_no_count(16usize, 15usize, 17usize, 2151460u32);
    emu.sltru_no_count(15usize, 15usize, 17usize, 2151464u32);
    emu.sbr_no_count(17usize, 5usize, 29usize, 2151468u32);
    emu.sltru_no_count(5usize, 5usize, 29usize, 2151472u32);
    emu.sbr_no_count(29usize, 6usize, 30usize, 2151476u32);
    emu.sltru_no_count(6usize, 6usize, 30usize, 2151480u32);
    emu.sbr_no_count(30usize, 7usize, 31usize, 2151484u32);
    emu.sltru_no_count(7usize, 7usize, 31usize, 2151488u32);
    emu.sbr_no_count(31usize, 28usize, 21usize, 2151492u32);
    emu.sltru_no_count(28usize, 28usize, 21usize, 2151496u32);
    emu.sbr_no_count(21usize, 0usize, 22usize, 2151500u32);
    emu.sbr_no_count(13usize, 13usize, 22usize, 2151504u32);
    emu.sltru_no_count(21usize, 13usize, 21usize, 2151508u32);
    emu.sbr_no_count(21usize, 21usize, 22usize, 2151512u32);
    emu.sltru_no_count(22usize, 13usize, 12usize, 2151516u32);
    emu.sbr_no_count(21usize, 21usize, 22usize, 2151520u32);
    emu.sai_no_count(21usize, 21usize, 1055u32, 2151524u32);
    emu.sbr_no_count(22usize, 21usize, 14usize, 2151528u32);
    emu.adr_no_count(14usize, 23usize, 21usize, 2151532u32);
    emu.sltru_no_count(21usize, 14usize, 23usize, 2151536u32);
    emu.adr_no_count(21usize, 22usize, 21usize, 2151540u32);
    emu.sai_no_count(21usize, 21usize, 1055u32, 2151544u32);
    emu.sbr_no_count(22usize, 21usize, 15usize, 2151548u32);
    emu.adr_no_count(15usize, 16usize, 21usize, 2151552u32);
    emu.sltru_no_count(16usize, 15usize, 16usize, 2151556u32);
    emu.adr_no_count(16usize, 22usize, 16usize, 2151560u32);
    emu.sai_no_count(16usize, 16usize, 1055u32, 2151564u32);
    emu.sbr_no_count(5usize, 16usize, 5usize, 2151568u32);
    emu.adr_no_count(16usize, 17usize, 16usize, 2151572u32);
    emu.sltru_no_count(17usize, 16usize, 17usize, 2151576u32);
    emu.adr_no_count(17usize, 5usize, 17usize, 2151580u32);
    emu.sai_no_count(17usize, 17usize, 1055u32, 2151584u32);
    emu.sbr_no_count(5usize, 17usize, 6usize, 2151588u32);
    emu.adr_no_count(17usize, 29usize, 17usize, 2151592u32);
    emu.sltru_no_count(6usize, 17usize, 29usize, 2151596u32);
    emu.adr_no_count(5usize, 5usize, 6usize, 2151600u32);
    emu.sai_no_count(5usize, 5usize, 1055u32, 2151604u32);
    emu.sbr_no_count(6usize, 5usize, 7usize, 2151608u32);
    emu.adr_no_count(5usize, 30usize, 5usize, 2151612u32);
    emu.sltru_no_count(7usize, 5usize, 30usize, 2151616u32);
    emu.adr_no_count(6usize, 6usize, 7usize, 2151620u32);
    emu.sai_no_count(6usize, 6usize, 1055u32, 2151624u32);
    emu.sbr_no_count(7usize, 6usize, 28usize, 2151628u32);
    emu.adr_no_count(6usize, 31usize, 6usize, 2151632u32);
    emu.sltru_no_count(28usize, 6usize, 31usize, 2151636u32);
    emu.adr_no_count(7usize, 7usize, 28usize, 2151640u32);
    emu.ani_no_count(7usize, 7usize, 2u32, 2151644u32);
    emu.add_memory_rw_events(66usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a != b {
        emu.pc = 2150028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ce8c));
    } else {
        emu.pc = 2151648u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d4e0));
    }
}
#[inline]
pub fn block_0x0020d4e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(10usize, 10usize, 11usize, 2151652u32);
    emu.sbr_no_count(13usize, 13usize, 12usize, 2151656u32);
    emu.orr_no_count(10usize, 13usize, 10usize, 2151660u32);
    emu.orr_no_count(10usize, 10usize, 14usize, 2151664u32);
    emu.orr_no_count(10usize, 10usize, 15usize, 2151668u32);
    emu.orr_no_count(10usize, 10usize, 16usize, 2151672u32);
    emu.orr_no_count(10usize, 10usize, 17usize, 2151676u32);
    emu.orr_no_count(10usize, 10usize, 5usize, 2151680u32);
    emu.orr_no_count(10usize, 10usize, 6usize, 2151684u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2150028u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ce8c));
    } else {
        emu.pc = 2151688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020d508));
    }
}
#[inline(always)]
pub fn block_0x0020d508(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 140u32, 2151692u32);
    emu.adi_no_count(11usize, 2usize, 44u32, 2151696u32);
    emu.adi_no_count(12usize, 2usize, 12u32, 2151700u32);
    emu.adi_no_count(13usize, 18usize, 0u32, 2151704u32);
    emu.apc_no_count(1usize, 2151704u32, 20480u32, 2151708u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2151712u32;
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
pub fn block_0x0020d520(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 22u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 156u32, 2151716u32)?;
    emu.lw_no_count(11usize, 2usize, 160u32, 2151720u32)?;
    emu.lw_no_count(12usize, 2usize, 164u32, 2151724u32)?;
    emu.lw_no_count(13usize, 2usize, 168u32, 2151728u32)?;
    emu.sw_no_count(10usize, 2usize, 60u32, 2151732u32)?;
    emu.sw_no_count(11usize, 2usize, 64u32, 2151736u32)?;
    emu.sw_no_count(12usize, 2usize, 68u32, 2151740u32)?;
    emu.sw_no_count(13usize, 2usize, 72u32, 2151744u32)?;
    emu.lw_no_count(10usize, 2usize, 140u32, 2151748u32)?;
    emu.lw_no_count(11usize, 2usize, 144u32, 2151752u32)?;
    emu.lw_no_count(12usize, 2usize, 148u32, 2151756u32)?;
    emu.lw_no_count(13usize, 2usize, 152u32, 2151760u32)?;
    emu.sw_no_count(10usize, 2usize, 44u32, 2151764u32)?;
    emu.sw_no_count(11usize, 2usize, 48u32, 2151768u32)?;
    emu.sw_no_count(12usize, 2usize, 52u32, 2151772u32)?;
    emu.sw_no_count(13usize, 2usize, 56u32, 2151776u32)?;
    emu.adi_no_count(10usize, 2usize, 140u32, 2151780u32);
    emu.adi_no_count(11usize, 2usize, 108u32, 2151784u32);
    emu.adi_no_count(12usize, 2usize, 76u32, 2151788u32);
    emu.adi_no_count(13usize, 18usize, 0u32, 2151792u32);
    emu.apc_no_count(1usize, 2151792u32, 20480u32, 2151796u32);
    emu.add_memory_rw_events(22usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2151800u32;
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
pub fn block_0x0020d578(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 156u32, 2151804u32)?;
    emu.lw_no_count(11usize, 2usize, 160u32, 2151808u32)?;
    emu.lw_no_count(12usize, 2usize, 164u32, 2151812u32)?;
    emu.lw_no_count(13usize, 2usize, 168u32, 2151816u32)?;
    emu.sw_no_count(10usize, 2usize, 124u32, 2151820u32)?;
    emu.sw_no_count(11usize, 2usize, 128u32, 2151824u32)?;
    emu.sw_no_count(12usize, 2usize, 132u32, 2151828u32)?;
    emu.sw_no_count(13usize, 2usize, 136u32, 2151832u32)?;
    emu.lw_no_count(10usize, 2usize, 140u32, 2151836u32)?;
    emu.lw_no_count(11usize, 2usize, 144u32, 2151840u32)?;
    emu.lw_no_count(12usize, 2usize, 148u32, 2151844u32)?;
    emu.lw_no_count(13usize, 2usize, 152u32, 2151848u32)?;
    emu.sw_no_count(10usize, 2usize, 108u32, 2151852u32)?;
    emu.sw_no_count(11usize, 2usize, 112u32, 2151856u32)?;
    emu.sw_no_count(12usize, 2usize, 116u32, 2151860u32)?;
    emu.sw_no_count(13usize, 2usize, 120u32, 2151864u32)?;
    emu.add_memory_rw_events(17usize);
    let return_addr = 2151868u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2150204u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020cf3c));
}
#[inline(never)]
pub fn block_0x0020d5bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 35u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 124u32, 2151872u32)?;
    emu.lw_no_count(11usize, 2usize, 128u32, 2151876u32)?;
    emu.lw_no_count(12usize, 2usize, 132u32, 2151880u32)?;
    emu.lw_no_count(13usize, 2usize, 136u32, 2151884u32)?;
    emu.lw_no_count(14usize, 2usize, 108u32, 2151888u32)?;
    emu.lw_no_count(15usize, 2usize, 112u32, 2151892u32)?;
    emu.lw_no_count(16usize, 2usize, 116u32, 2151896u32)?;
    emu.lw_no_count(17usize, 2usize, 120u32, 2151900u32)?;
    emu.sw_no_count(10usize, 8usize, 16u32, 2151904u32)?;
    emu.sw_no_count(11usize, 8usize, 20u32, 2151908u32)?;
    emu.sw_no_count(12usize, 8usize, 24u32, 2151912u32)?;
    emu.sw_no_count(13usize, 8usize, 28u32, 2151916u32)?;
    emu.lw_no_count(10usize, 9usize, 0u32, 2151920u32)?;
    emu.lw_no_count(11usize, 9usize, 4u32, 2151924u32)?;
    emu.lw_no_count(12usize, 9usize, 8u32, 2151928u32)?;
    emu.lw_no_count(13usize, 9usize, 12u32, 2151932u32)?;
    emu.sw_no_count(14usize, 8usize, 0u32, 2151936u32)?;
    emu.sw_no_count(15usize, 8usize, 4u32, 2151940u32)?;
    emu.sw_no_count(16usize, 8usize, 8u32, 2151944u32)?;
    emu.sw_no_count(17usize, 8usize, 12u32, 2151948u32)?;
    emu.lw_no_count(14usize, 9usize, 16u32, 2151952u32)?;
    emu.lw_no_count(15usize, 9usize, 20u32, 2151956u32)?;
    emu.lw_no_count(16usize, 9usize, 24u32, 2151960u32)?;
    emu.lw_no_count(17usize, 9usize, 28u32, 2151964u32)?;
    emu.orr_no_count(10usize, 11usize, 10usize, 2151968u32);
    emu.orr_no_count(12usize, 12usize, 13usize, 2151972u32);
    emu.orr_no_count(14usize, 14usize, 15usize, 2151976u32);
    emu.orr_no_count(10usize, 10usize, 12usize, 2151980u32);
    emu.orr_no_count(11usize, 14usize, 16usize, 2151984u32);
    emu.orr_no_count(10usize, 10usize, 11usize, 2151988u32);
    emu.orr_no_count(10usize, 10usize, 17usize, 2151992u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2151996u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2152000u32);
    emu.apc_no_count(1usize, 2152000u32, 40960u32, 2152004u32);
    emu.add_memory_rw_events(35usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2152008u32;
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
