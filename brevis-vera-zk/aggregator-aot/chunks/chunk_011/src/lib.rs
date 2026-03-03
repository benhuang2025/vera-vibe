pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2137140u32;
pub const PC_MAX: u32 = 2143584u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 176usize] = [
        block_0x00209c34,
        block_0x00209d44,
        block_0x00209d48,
        block_0x00209dc4,
        block_0x00209dc8,
        block_0x00209e24,
        block_0x00209e28,
        block_0x00209e98,
        block_0x00209e9c,
        block_0x00209f1c,
        block_0x00209f20,
        block_0x00209f4c,
        block_0x00209f50,
        block_0x00209fc8,
        block_0x00209fcc,
        block_0x00209ff0,
        block_0x00209ff4,
        block_0x0020a048,
        block_0x0020a04c,
        block_0x0020a070,
        block_0x0020a074,
        block_0x0020a0ec,
        block_0x0020a0f0,
        block_0x0020a170,
        block_0x0020a174,
        block_0x0020a1a0,
        block_0x0020a1a4,
        block_0x0020a21c,
        block_0x0020a220,
        block_0x0020a248,
        block_0x0020a24c,
        block_0x0020a2a4,
        block_0x0020a2a8,
        block_0x0020a2d0,
        block_0x0020a2d4,
        block_0x0020a344,
        block_0x0020a348,
        block_0x0020a3c4,
        block_0x0020a3c8,
        block_0x0020a3f4,
        block_0x0020a3f8,
        block_0x0020a470,
        block_0x0020a474,
        block_0x0020a4a0,
        block_0x0020a4a4,
        block_0x0020a4e4,
        block_0x0020a4e8,
        block_0x0020a500,
        block_0x0020a504,
        block_0x0020a530,
        block_0x0020a534,
        block_0x0020a598,
        block_0x0020a59c,
        block_0x0020a5bc,
        block_0x0020a5c0,
        block_0x0020a5e8,
        block_0x0020a5ec,
        block_0x0020a668,
        block_0x0020a66c,
        block_0x0020a6ac,
        block_0x0020a6b0,
        block_0x0020a6e4,
        block_0x0020a6e8,
        block_0x0020a738,
        block_0x0020a73c,
        block_0x0020a788,
        block_0x0020a78c,
        block_0x0020a7ac,
        block_0x0020a7b0,
        block_0x0020a7e0,
        block_0x0020a7e4,
        block_0x0020a828,
        block_0x0020a82c,
        block_0x0020a86c,
        block_0x0020a870,
        block_0x0020a884,
        block_0x0020a888,
        block_0x0020a8b4,
        block_0x0020a8b8,
        block_0x0020a8d8,
        block_0x0020a8e0,
        block_0x0020a8e4,
        block_0x0020a914,
        block_0x0020a918,
        block_0x0020a940,
        block_0x0020a944,
        block_0x0020a964,
        block_0x0020a96c,
        block_0x0020a970,
        block_0x0020a9a0,
        block_0x0020a9a4,
        block_0x0020a9c4,
        block_0x0020a9c8,
        block_0x0020aa10,
        block_0x0020aa14,
        block_0x0020aa94,
        block_0x0020aab0,
        block_0x0020aabc,
        block_0x0020abcc,
        block_0x0020abd0,
        block_0x0020ac4c,
        block_0x0020ac50,
        block_0x0020ac70,
        block_0x0020ac74,
        block_0x0020ac90,
        block_0x0020ac94,
        block_0x0020ad04,
        block_0x0020ad08,
        block_0x0020ad78,
        block_0x0020ad7c,
        block_0x0020ad9c,
        block_0x0020ada0,
        block_0x0020ae14,
        block_0x0020ae18,
        block_0x0020ae38,
        block_0x0020ae3c,
        block_0x0020ae5c,
        block_0x0020ae60,
        block_0x0020ae7c,
        block_0x0020ae80,
        block_0x0020aeac,
        block_0x0020aeb0,
        block_0x0020af20,
        block_0x0020af24,
        block_0x0020af94,
        block_0x0020af98,
        block_0x0020afc0,
        block_0x0020afc4,
        block_0x0020b040,
        block_0x0020b044,
        block_0x0020b068,
        block_0x0020b06c,
        block_0x0020b094,
        block_0x0020b098,
        block_0x0020b0b8,
        block_0x0020b0bc,
        block_0x0020b0dc,
        block_0x0020b0e0,
        block_0x0020b0f4,
        block_0x0020b0f8,
        block_0x0020b168,
        block_0x0020b16c,
        block_0x0020b1dc,
        block_0x0020b1e0,
        block_0x0020b200,
        block_0x0020b204,
        block_0x0020b27c,
        block_0x0020b280,
        block_0x0020b2a0,
        block_0x0020b2a4,
        block_0x0020b2c8,
        block_0x0020b2cc,
        block_0x0020b2ec,
        block_0x0020b2f0,
        block_0x0020b310,
        block_0x0020b314,
        block_0x0020b328,
        block_0x0020b32c,
        block_0x0020b39c,
        block_0x0020b3a0,
        block_0x0020b410,
        block_0x0020b414,
        block_0x0020b434,
        block_0x0020b438,
        block_0x0020b4b0,
        block_0x0020b4b4,
        block_0x0020b4d4,
        block_0x0020b4d8,
        block_0x0020b4fc,
        block_0x0020b500,
        block_0x0020b520,
        block_0x0020b524,
        block_0x0020b544,
        block_0x0020b548,
        block_0x0020b55c,
        block_0x0020b560,
    ];
    const IDX: [u16; 1612usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 2u16, 3u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 4u16, 5u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 6u16, 7u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 8u16, 9u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 10u16, 11u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 12u16, 13u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 14u16, 15u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 16u16, 17u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 18u16, 19u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 20u16, 21u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 22u16, 23u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 24u16, 25u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 26u16, 27u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 28u16, 29u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 30u16,
        31u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 32u16, 33u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 34u16, 35u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 36u16, 37u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 38u16, 39u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 40u16, 41u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 42u16, 43u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 44u16, 45u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 46u16, 47u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 48u16, 49u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 50u16, 51u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 52u16, 53u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 54u16,
        55u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 56u16, 57u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 58u16, 59u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 60u16, 61u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 62u16, 63u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 64u16, 65u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 66u16, 67u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 68u16, 69u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 70u16, 71u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 72u16, 73u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 74u16, 75u16, 0u16, 0u16, 0u16, 0u16, 76u16, 77u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 78u16, 79u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 80u16, 0u16, 81u16, 82u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 83u16, 84u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 85u16, 86u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 87u16,
        0u16, 88u16, 89u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 90u16, 91u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 92u16, 93u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 94u16, 95u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 96u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 97u16, 0u16, 0u16, 98u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 99u16, 100u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 101u16, 102u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        103u16, 104u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 105u16, 106u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 107u16,
        108u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 109u16, 110u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 111u16,
        112u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 113u16, 114u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        115u16, 116u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 117u16, 118u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 119u16, 120u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 121u16, 122u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 123u16, 124u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 125u16, 126u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 127u16, 128u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 129u16, 130u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 131u16,
        132u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 133u16, 134u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 135u16, 136u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 137u16, 138u16, 0u16, 0u16, 0u16, 0u16, 139u16, 140u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        141u16, 142u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 143u16, 144u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        145u16, 146u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 147u16, 148u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 149u16, 150u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 151u16,
        152u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 153u16, 154u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 155u16, 156u16, 0u16, 0u16, 0u16, 0u16, 157u16,
        158u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 159u16, 160u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 161u16, 162u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 163u16, 164u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 165u16, 166u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 167u16, 168u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        169u16, 170u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 171u16, 172u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 173u16, 174u16, 0u16, 0u16, 0u16, 0u16,
        175u16, 176u16,
    ];
    if pc < 2137140u32 || pc > 2143584u32 {
        return None;
    }
    let word_offset = ((pc - 2137140u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(never)]
pub fn block_0x00209c34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 68u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967200u32, 2137144u32);
    emu.sw_no_count(1usize, 2usize, 92u32, 2137148u32)?;
    emu.sw_no_count(8usize, 2usize, 88u32, 2137152u32)?;
    emu.sw_no_count(9usize, 2usize, 84u32, 2137156u32)?;
    emu.sw_no_count(18usize, 2usize, 80u32, 2137160u32)?;
    emu.sw_no_count(19usize, 2usize, 76u32, 2137164u32)?;
    emu.sw_no_count(20usize, 2usize, 72u32, 2137168u32)?;
    emu.sw_no_count(21usize, 2usize, 68u32, 2137172u32)?;
    emu.sw_no_count(22usize, 2usize, 64u32, 2137176u32)?;
    emu.sw_no_count(23usize, 2usize, 60u32, 2137180u32)?;
    emu.sw_no_count(24usize, 2usize, 56u32, 2137184u32)?;
    emu.sw_no_count(25usize, 2usize, 52u32, 2137188u32)?;
    emu.sw_no_count(26usize, 2usize, 48u32, 2137192u32)?;
    emu.sw_no_count(27usize, 2usize, 44u32, 2137196u32)?;
    emu.sw_no_count(10usize, 2usize, 28u32, 2137200u32)?;
    emu.lw_no_count(14usize, 11usize, 0u32, 2137204u32)?;
    emu.lw_no_count(9usize, 11usize, 4u32, 2137208u32)?;
    emu.lw_no_count(15usize, 12usize, 0u32, 2137212u32)?;
    emu.lw_no_count(29usize, 12usize, 4u32, 2137216u32)?;
    emu.lw_no_count(7usize, 12usize, 8u32, 2137220u32)?;
    emu.lw_no_count(28usize, 12usize, 12u32, 2137224u32)?;
    emu.lw_no_count(17usize, 12usize, 16u32, 2137228u32)?;
    emu.lw_no_count(5usize, 12usize, 20u32, 2137232u32)?;
    emu.mulhu_no_count(10usize, 15usize, 14usize, 2137236u32);
    emu.mul_no_count(13usize, 29usize, 14usize, 2137240u32);
    emu.mulhu_no_count(16usize, 29usize, 14usize, 2137244u32);
    emu.mul_no_count(6usize, 15usize, 9usize, 2137248u32);
    emu.mulhu_no_count(30usize, 15usize, 9usize, 2137252u32);
    emu.mul_no_count(31usize, 29usize, 9usize, 2137256u32);
    emu.mulhu_no_count(8usize, 7usize, 14usize, 2137260u32);
    emu.mul_no_count(18usize, 28usize, 14usize, 2137264u32);
    emu.adr_no_count(10usize, 13usize, 10usize, 2137268u32);
    emu.sltru_no_count(13usize, 10usize, 13usize, 2137272u32);
    emu.adr_no_count(13usize, 16usize, 13usize, 2137276u32);
    emu.mulhu_no_count(16usize, 28usize, 14usize, 2137280u32);
    emu.adr_no_count(8usize, 18usize, 8usize, 2137284u32);
    emu.sltru_no_count(18usize, 8usize, 18usize, 2137288u32);
    emu.adr_no_count(16usize, 16usize, 18usize, 2137292u32);
    emu.mul_no_count(18usize, 7usize, 9usize, 2137296u32);
    emu.adr_no_count(10usize, 6usize, 10usize, 2137300u32);
    emu.sw_no_count(10usize, 2usize, 24u32, 2137304u32)?;
    emu.sltru_no_count(10usize, 10usize, 6usize, 2137308u32);
    emu.adr_no_count(30usize, 30usize, 10usize, 2137312u32);
    emu.mulhu_no_count(6usize, 7usize, 9usize, 2137316u32);
    emu.adr_no_count(10usize, 18usize, 8usize, 2137320u32);
    emu.sltru_no_count(8usize, 10usize, 18usize, 2137324u32);
    emu.adr_no_count(6usize, 6usize, 8usize, 2137328u32);
    emu.mulhu_no_count(8usize, 29usize, 9usize, 2137332u32);
    emu.adr_no_count(30usize, 13usize, 30usize, 2137336u32);
    emu.sltru_no_count(13usize, 30usize, 13usize, 2137340u32);
    emu.adr_no_count(13usize, 8usize, 13usize, 2137344u32);
    emu.mulhu_no_count(8usize, 28usize, 9usize, 2137348u32);
    emu.adr_no_count(6usize, 16usize, 6usize, 2137352u32);
    emu.sltru_no_count(16usize, 6usize, 16usize, 2137356u32);
    emu.adr_no_count(8usize, 8usize, 16usize, 2137360u32);
    emu.mul_no_count(16usize, 28usize, 9usize, 2137364u32);
    emu.adr_no_count(30usize, 31usize, 30usize, 2137368u32);
    emu.sltru_no_count(31usize, 30usize, 31usize, 2137372u32);
    emu.adr_no_count(13usize, 13usize, 31usize, 2137376u32);
    emu.mul_no_count(31usize, 7usize, 14usize, 2137380u32);
    emu.adr_no_count(6usize, 16usize, 6usize, 2137384u32);
    emu.sltru_no_count(18usize, 6usize, 16usize, 2137388u32);
    emu.adr_no_count(30usize, 31usize, 30usize, 2137392u32);
    emu.adr_no_count(13usize, 10usize, 13usize, 2137396u32);
    emu.sltru_no_count(16usize, 30usize, 31usize, 2137400u32);
    emu.adr_no_count(21usize, 13usize, 16usize, 2137404u32);
    emu.adr_no_count(31usize, 8usize, 18usize, 2137408u32);
    emu.add_memory_rw_events(67usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a == b {
        emu.pc = 2137416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209d48));
    } else {
        emu.pc = 2137412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209d44));
    }
}
#[inline(always)]
pub fn block_0x00209d44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 21usize, 10usize, 2137416u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2137416u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209d48));
}
#[inline(never)]
pub fn block_0x00209d48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 31u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 12usize, 24u32, 2137420u32)?;
    emu.sw_no_count(10usize, 2usize, 36u32, 2137424u32)?;
    emu.lw_no_count(13usize, 12usize, 28u32, 2137428u32)?;
    emu.adr_no_count(16usize, 6usize, 16usize, 2137432u32);
    emu.mulhu_no_count(10usize, 17usize, 14usize, 2137436u32);
    emu.mul_no_count(8usize, 5usize, 14usize, 2137440u32);
    emu.mulhu_no_count(18usize, 5usize, 14usize, 2137444u32);
    emu.mul_no_count(19usize, 17usize, 9usize, 2137448u32);
    emu.sltru_no_count(6usize, 16usize, 6usize, 2137452u32);
    emu.adr_no_count(31usize, 31usize, 6usize, 2137456u32);
    emu.mulhu_no_count(20usize, 17usize, 9usize, 2137460u32);
    emu.adr_no_count(10usize, 8usize, 10usize, 2137464u32);
    emu.sltru_no_count(6usize, 10usize, 8usize, 2137468u32);
    emu.adr_no_count(18usize, 18usize, 6usize, 2137472u32);
    emu.mul_no_count(8usize, 5usize, 9usize, 2137476u32);
    emu.adr_no_count(6usize, 19usize, 10usize, 2137480u32);
    emu.sltru_no_count(10usize, 6usize, 19usize, 2137484u32);
    emu.adr_no_count(10usize, 20usize, 10usize, 2137488u32);
    emu.mulhu_no_count(19usize, 5usize, 9usize, 2137492u32);
    emu.adr_no_count(10usize, 18usize, 10usize, 2137496u32);
    emu.sltru_no_count(18usize, 10usize, 18usize, 2137500u32);
    emu.adr_no_count(18usize, 19usize, 18usize, 2137504u32);
    emu.mul_no_count(19usize, 17usize, 14usize, 2137508u32);
    emu.adr_no_count(22usize, 19usize, 16usize, 2137512u32);
    emu.sltru_no_count(16usize, 22usize, 19usize, 2137516u32);
    emu.adr_no_count(31usize, 6usize, 31usize, 2137520u32);
    emu.adr_no_count(10usize, 8usize, 10usize, 2137524u32);
    emu.sltru_no_count(8usize, 10usize, 8usize, 2137528u32);
    emu.adr_no_count(20usize, 31usize, 16usize, 2137532u32);
    emu.adr_no_count(18usize, 18usize, 8usize, 2137536u32);
    emu.add_memory_rw_events(30usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2137544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209dc8));
    } else {
        emu.pc = 2137540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209dc4));
    }
}
#[inline(always)]
pub fn block_0x00209dc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 20usize, 6usize, 2137544u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2137544u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209dc8));
}
#[inline]
pub fn block_0x00209dc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 23u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(8usize, 11usize, 8u32, 2137548u32)?;
    emu.lw_no_count(31usize, 11usize, 12u32, 2137552u32)?;
    emu.adr_no_count(16usize, 10usize, 16usize, 2137556u32);
    emu.lw_no_count(12usize, 2usize, 36u32, 2137560u32)?;
    emu.mulhu_no_count(6usize, 12usize, 14usize, 2137564u32);
    emu.mul_no_count(19usize, 13usize, 14usize, 2137568u32);
    emu.mulhu_no_count(25usize, 13usize, 14usize, 2137572u32);
    emu.mul_no_count(24usize, 12usize, 9usize, 2137576u32);
    emu.sltru_no_count(10usize, 16usize, 10usize, 2137580u32);
    emu.adr_no_count(18usize, 18usize, 10usize, 2137584u32);
    emu.mulhu_no_count(26usize, 12usize, 9usize, 2137588u32);
    emu.adr_no_count(10usize, 19usize, 6usize, 2137592u32);
    emu.sltru_no_count(6usize, 10usize, 19usize, 2137596u32);
    emu.adr_no_count(25usize, 25usize, 6usize, 2137600u32);
    emu.mul_no_count(6usize, 12usize, 14usize, 2137604u32);
    emu.adr_no_count(19usize, 6usize, 16usize, 2137608u32);
    emu.adr_no_count(10usize, 24usize, 10usize, 2137612u32);
    emu.sltru_no_count(23usize, 19usize, 6usize, 2137616u32);
    emu.sltru_no_count(16usize, 10usize, 24usize, 2137620u32);
    emu.adr_no_count(18usize, 10usize, 18usize, 2137624u32);
    emu.adr_no_count(18usize, 18usize, 23usize, 2137628u32);
    emu.adr_no_count(26usize, 26usize, 16usize, 2137632u32);
    emu.add_memory_rw_events(22usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2137640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209e28));
    } else {
        emu.pc = 2137636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209e24));
    }
}
#[inline(always)]
pub fn block_0x00209e24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(23usize, 18usize, 10usize, 2137640u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2137640u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209e28));
}
#[inline(never)]
pub fn block_0x00209e28(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(26usize, 25usize, 26usize, 2137644u32);
    emu.mul_no_count(24usize, 13usize, 9usize, 2137648u32);
    emu.mulhu_no_count(10usize, 15usize, 8usize, 2137652u32);
    emu.mul_no_count(16usize, 29usize, 8usize, 2137656u32);
    emu.mulhu_no_count(6usize, 29usize, 8usize, 2137660u32);
    emu.mul_no_count(27usize, 15usize, 31usize, 2137664u32);
    emu.mulhu_no_count(1usize, 15usize, 31usize, 2137668u32);
    emu.adr_no_count(10usize, 16usize, 10usize, 2137672u32);
    emu.sltru_no_count(16usize, 10usize, 16usize, 2137676u32);
    emu.adr_no_count(16usize, 6usize, 16usize, 2137680u32);
    emu.mul_no_count(6usize, 29usize, 31usize, 2137684u32);
    emu.adr_no_count(10usize, 27usize, 10usize, 2137688u32);
    emu.sltru_no_count(27usize, 10usize, 27usize, 2137692u32);
    emu.adr_no_count(27usize, 1usize, 27usize, 2137696u32);
    emu.mulhu_no_count(1usize, 29usize, 31usize, 2137700u32);
    emu.adr_no_count(27usize, 16usize, 27usize, 2137704u32);
    emu.sltru_no_count(16usize, 27usize, 16usize, 2137708u32);
    emu.adr_no_count(1usize, 1usize, 16usize, 2137712u32);
    emu.mul_no_count(16usize, 15usize, 8usize, 2137716u32);
    emu.adr_no_count(16usize, 30usize, 16usize, 2137720u32);
    emu.sw_no_count(16usize, 2usize, 32u32, 2137724u32)?;
    emu.sltru_no_count(16usize, 16usize, 30usize, 2137728u32);
    emu.adr_no_count(30usize, 10usize, 16usize, 2137732u32);
    emu.adr_no_count(10usize, 6usize, 27usize, 2137736u32);
    emu.sltru_no_count(6usize, 10usize, 6usize, 2137740u32);
    emu.adr_no_count(30usize, 21usize, 30usize, 2137744u32);
    emu.adr_no_count(6usize, 1usize, 6usize, 2137748u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(30usize);
    if a == b {
        emu.pc = 2137756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209e9c));
    } else {
        emu.pc = 2137752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209e98));
    }
}
#[inline(always)]
pub fn block_0x00209e98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 30usize, 21usize, 2137756u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2137756u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209e9c));
}
#[inline(never)]
pub fn block_0x00209e9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 32u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(30usize, 2usize, 20u32, 2137760u32)?;
    emu.adr_no_count(21usize, 24usize, 26usize, 2137764u32);
    emu.sltru_no_count(26usize, 26usize, 25usize, 2137768u32);
    emu.mulhu_no_count(9usize, 13usize, 9usize, 2137772u32);
    emu.adr_no_count(25usize, 10usize, 16usize, 2137776u32);
    emu.mulhu_no_count(30usize, 7usize, 8usize, 2137780u32);
    emu.mul_no_count(27usize, 28usize, 8usize, 2137784u32);
    emu.mulhu_no_count(1usize, 28usize, 8usize, 2137788u32);
    emu.mul_no_count(12usize, 7usize, 31usize, 2137792u32);
    emu.sltru_no_count(16usize, 25usize, 10usize, 2137796u32);
    emu.adr_no_count(16usize, 6usize, 16usize, 2137800u32);
    emu.mulhu_no_count(10usize, 7usize, 31usize, 2137804u32);
    emu.adr_no_count(30usize, 27usize, 30usize, 2137808u32);
    emu.sltru_no_count(6usize, 30usize, 27usize, 2137812u32);
    emu.adr_no_count(6usize, 1usize, 6usize, 2137816u32);
    emu.mul_no_count(27usize, 28usize, 31usize, 2137820u32);
    emu.adr_no_count(30usize, 12usize, 30usize, 2137824u32);
    emu.sltru_no_count(12usize, 30usize, 12usize, 2137828u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2137832u32);
    emu.mulhu_no_count(12usize, 28usize, 31usize, 2137836u32);
    emu.adr_no_count(10usize, 6usize, 10usize, 2137840u32);
    emu.sltru_no_count(6usize, 10usize, 6usize, 2137844u32);
    emu.adr_no_count(12usize, 12usize, 6usize, 2137848u32);
    emu.mul_no_count(1usize, 7usize, 8usize, 2137852u32);
    emu.adr_no_count(1usize, 22usize, 1usize, 2137856u32);
    emu.sltru_no_count(22usize, 1usize, 22usize, 2137860u32);
    emu.adr_no_count(6usize, 30usize, 22usize, 2137864u32);
    emu.adr_no_count(30usize, 27usize, 10usize, 2137868u32);
    emu.sltru_no_count(10usize, 30usize, 27usize, 2137872u32);
    emu.adr_no_count(27usize, 20usize, 6usize, 2137876u32);
    emu.adr_no_count(10usize, 12usize, 10usize, 2137880u32);
    emu.add_memory_rw_events(31usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(27usize);
    if a == b {
        emu.pc = 2137888u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209f20));
    } else {
        emu.pc = 2137884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209f1c));
    }
}
#[inline(always)]
pub fn block_0x00209f1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(22usize, 27usize, 20usize, 2137888u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2137888u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209f20));
}
#[inline]
pub fn block_0x00209f20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(6usize, 21usize, 24usize, 2137892u32);
    emu.adr_no_count(24usize, 9usize, 26usize, 2137896u32);
    emu.adr_no_count(9usize, 21usize, 23usize, 2137900u32);
    emu.adr_no_count(26usize, 30usize, 22usize, 2137904u32);
    emu.adr_no_count(20usize, 27usize, 16usize, 2137908u32);
    emu.adr_no_count(22usize, 1usize, 25usize, 2137912u32);
    emu.sltru_no_count(12usize, 26usize, 30usize, 2137916u32);
    emu.sltru_no_count(16usize, 22usize, 1usize, 2137920u32);
    emu.adr_no_count(20usize, 20usize, 16usize, 2137924u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2137928u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(27usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2137936u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209f50));
    } else {
        emu.pc = 2137932u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209f4c));
    }
}
#[inline(always)]
pub fn block_0x00209f4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 20usize, 27usize, 2137936u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2137936u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209f50));
}
#[inline(never)]
pub fn block_0x00209f50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 30u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(23usize, 24usize, 6usize, 2137940u32);
    emu.sltru_no_count(21usize, 9usize, 21usize, 2137944u32);
    emu.adr_no_count(24usize, 26usize, 16usize, 2137948u32);
    emu.mulhu_no_count(12usize, 17usize, 8usize, 2137952u32);
    emu.mul_no_count(16usize, 5usize, 8usize, 2137956u32);
    emu.mulhu_no_count(6usize, 5usize, 8usize, 2137960u32);
    emu.mul_no_count(30usize, 17usize, 31usize, 2137964u32);
    emu.sltru_no_count(25usize, 24usize, 26usize, 2137968u32);
    emu.adr_no_count(10usize, 10usize, 25usize, 2137972u32);
    emu.mulhu_no_count(25usize, 17usize, 31usize, 2137976u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2137980u32);
    emu.sltru_no_count(16usize, 12usize, 16usize, 2137984u32);
    emu.adr_no_count(16usize, 6usize, 16usize, 2137988u32);
    emu.mul_no_count(6usize, 5usize, 31usize, 2137992u32);
    emu.adr_no_count(12usize, 30usize, 12usize, 2137996u32);
    emu.sltru_no_count(30usize, 12usize, 30usize, 2138000u32);
    emu.adr_no_count(30usize, 25usize, 30usize, 2138004u32);
    emu.mulhu_no_count(25usize, 5usize, 31usize, 2138008u32);
    emu.adr_no_count(26usize, 16usize, 30usize, 2138012u32);
    emu.sltru_no_count(16usize, 26usize, 16usize, 2138016u32);
    emu.adr_no_count(27usize, 25usize, 16usize, 2138020u32);
    emu.mul_no_count(25usize, 17usize, 8usize, 2138024u32);
    emu.adr_no_count(25usize, 19usize, 25usize, 2138028u32);
    emu.sltru_no_count(30usize, 25usize, 19usize, 2138032u32);
    emu.adr_no_count(12usize, 12usize, 30usize, 2138036u32);
    emu.adr_no_count(16usize, 6usize, 26usize, 2138040u32);
    emu.sltru_no_count(19usize, 16usize, 6usize, 2138044u32);
    emu.adr_no_count(6usize, 18usize, 12usize, 2138048u32);
    emu.adr_no_count(19usize, 27usize, 19usize, 2138052u32);
    emu.add_memory_rw_events(29usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2138060u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209fcc));
    } else {
        emu.pc = 2138056u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209fc8));
    }
}
#[inline(always)]
pub fn block_0x00209fc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(30usize, 6usize, 18usize, 2138060u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2138060u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209fcc));
}
#[inline]
pub fn block_0x00209fcc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(21usize, 23usize, 21usize, 2138064u32);
    emu.adr_no_count(30usize, 16usize, 30usize, 2138068u32);
    emu.adr_no_count(23usize, 6usize, 10usize, 2138072u32);
    emu.adr_no_count(24usize, 25usize, 24usize, 2138076u32);
    emu.sltru_no_count(12usize, 30usize, 16usize, 2138080u32);
    emu.sltru_no_count(10usize, 24usize, 25usize, 2138084u32);
    emu.adr_no_count(23usize, 23usize, 10usize, 2138088u32);
    emu.adr_no_count(19usize, 19usize, 12usize, 2138092u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a == b {
        emu.pc = 2138100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209ff4));
    } else {
        emu.pc = 2138096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00209ff0));
    }
}
#[inline(always)]
pub fn block_0x00209ff0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 23usize, 6usize, 2138100u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2138100u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00209ff4));
}
#[inline]
pub fn block_0x00209ff4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 30usize, 10usize, 2138104u32);
    emu.lw_no_count(26usize, 2usize, 36u32, 2138108u32)?;
    emu.mulhu_no_count(12usize, 26usize, 8usize, 2138112u32);
    emu.mul_no_count(16usize, 13usize, 8usize, 2138116u32);
    emu.mulhu_no_count(6usize, 13usize, 8usize, 2138120u32);
    emu.sltru_no_count(30usize, 10usize, 30usize, 2138124u32);
    emu.adr_no_count(19usize, 19usize, 30usize, 2138128u32);
    emu.mul_no_count(18usize, 26usize, 31usize, 2138132u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2138136u32);
    emu.sltru_no_count(30usize, 12usize, 16usize, 2138140u32);
    emu.adr_no_count(30usize, 6usize, 30usize, 2138144u32);
    emu.mulhu_no_count(25usize, 26usize, 31usize, 2138148u32);
    emu.mul_no_count(16usize, 26usize, 8usize, 2138152u32);
    emu.adr_no_count(16usize, 9usize, 16usize, 2138156u32);
    emu.adr_no_count(12usize, 18usize, 12usize, 2138160u32);
    emu.sltru_no_count(27usize, 16usize, 9usize, 2138164u32);
    emu.sltru_no_count(8usize, 12usize, 18usize, 2138168u32);
    emu.adr_no_count(12usize, 21usize, 12usize, 2138172u32);
    emu.adr_no_count(6usize, 12usize, 27usize, 2138176u32);
    emu.adr_no_count(8usize, 25usize, 8usize, 2138180u32);
    emu.add_memory_rw_events(20usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2138188u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a04c));
    } else {
        emu.pc = 2138184u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a048));
    }
}
#[inline(always)]
pub fn block_0x0020a048(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(27usize, 6usize, 21usize, 2138188u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2138188u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a04c));
}
#[inline]
pub fn block_0x0020a04c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 30usize, 8usize, 2138192u32);
    emu.lw_no_count(9usize, 11usize, 16u32, 2138196u32)?;
    emu.lw_no_count(18usize, 11usize, 20u32, 2138200u32)?;
    emu.adr_no_count(21usize, 16usize, 10usize, 2138204u32);
    emu.sltru_no_count(25usize, 21usize, 16usize, 2138208u32);
    emu.adr_no_count(19usize, 6usize, 19usize, 2138212u32);
    emu.adr_no_count(19usize, 19usize, 25usize, 2138216u32);
    emu.mul_no_count(1usize, 13usize, 31usize, 2138220u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2138228u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a074));
    } else {
        emu.pc = 2138224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a070));
    }
}
#[inline(always)]
pub fn block_0x0020a070(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(25usize, 19usize, 6usize, 2138228u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2138228u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a074));
}
#[inline(never)]
pub fn block_0x0020a074(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 30u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(26usize, 1usize, 8usize, 2138232u32);
    emu.sltru_no_count(30usize, 8usize, 30usize, 2138236u32);
    emu.sw_no_count(13usize, 2usize, 40u32, 2138240u32)?;
    emu.mulhu_no_count(10usize, 13usize, 31usize, 2138244u32);
    emu.mulhu_no_count(12usize, 15usize, 9usize, 2138248u32);
    emu.mul_no_count(16usize, 29usize, 9usize, 2138252u32);
    emu.mulhu_no_count(6usize, 29usize, 9usize, 2138256u32);
    emu.mul_no_count(31usize, 15usize, 18usize, 2138260u32);
    emu.mulhu_no_count(8usize, 15usize, 18usize, 2138264u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2138268u32);
    emu.sltru_no_count(16usize, 12usize, 16usize, 2138272u32);
    emu.adr_no_count(16usize, 6usize, 16usize, 2138276u32);
    emu.mul_no_count(13usize, 29usize, 18usize, 2138280u32);
    emu.adr_no_count(12usize, 31usize, 12usize, 2138284u32);
    emu.sltru_no_count(6usize, 12usize, 31usize, 2138288u32);
    emu.adr_no_count(6usize, 8usize, 6usize, 2138292u32);
    emu.mulhu_no_count(31usize, 29usize, 18usize, 2138296u32);
    emu.adr_no_count(8usize, 16usize, 6usize, 2138300u32);
    emu.sltru_no_count(16usize, 8usize, 16usize, 2138304u32);
    emu.adr_no_count(16usize, 31usize, 16usize, 2138308u32);
    emu.mul_no_count(6usize, 15usize, 9usize, 2138312u32);
    emu.adr_no_count(6usize, 22usize, 6usize, 2138316u32);
    emu.sw_no_count(6usize, 2usize, 12u32, 2138320u32)?;
    emu.sltru_no_count(6usize, 6usize, 22usize, 2138324u32);
    emu.adr_no_count(12usize, 12usize, 6usize, 2138328u32);
    emu.adr_no_count(8usize, 13usize, 8usize, 2138332u32);
    emu.sltru_no_count(22usize, 8usize, 13usize, 2138336u32);
    emu.adr_no_count(12usize, 20usize, 12usize, 2138340u32);
    emu.adr_no_count(22usize, 16usize, 22usize, 2138344u32);
    emu.add_memory_rw_events(29usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2138352u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a0f0));
    } else {
        emu.pc = 2138348u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a0ec));
    }
}
#[inline(always)]
pub fn block_0x0020a0ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(6usize, 12usize, 20usize, 2138352u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2138352u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a0f0));
}
#[inline(never)]
pub fn block_0x0020a0f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 32u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(12usize, 2usize, 16u32, 2138356u32)?;
    emu.sltru_no_count(20usize, 26usize, 1usize, 2138360u32);
    emu.adr_no_count(1usize, 10usize, 30usize, 2138364u32);
    emu.adr_no_count(27usize, 26usize, 27usize, 2138368u32);
    emu.adr_no_count(6usize, 8usize, 6usize, 2138372u32);
    emu.mulhu_no_count(12usize, 7usize, 9usize, 2138376u32);
    emu.mul_no_count(13usize, 28usize, 9usize, 2138380u32);
    emu.mulhu_no_count(16usize, 28usize, 9usize, 2138384u32);
    emu.mul_no_count(30usize, 7usize, 18usize, 2138388u32);
    emu.sltru_no_count(10usize, 6usize, 8usize, 2138392u32);
    emu.adr_no_count(10usize, 22usize, 10usize, 2138396u32);
    emu.mulhu_no_count(8usize, 7usize, 18usize, 2138400u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2138404u32);
    emu.sltru_no_count(13usize, 12usize, 13usize, 2138408u32);
    emu.adr_no_count(13usize, 16usize, 13usize, 2138412u32);
    emu.mul_no_count(31usize, 28usize, 18usize, 2138416u32);
    emu.adr_no_count(12usize, 30usize, 12usize, 2138420u32);
    emu.sltru_no_count(16usize, 12usize, 30usize, 2138424u32);
    emu.adr_no_count(16usize, 8usize, 16usize, 2138428u32);
    emu.mulhu_no_count(30usize, 28usize, 18usize, 2138432u32);
    emu.adr_no_count(16usize, 13usize, 16usize, 2138436u32);
    emu.sltru_no_count(13usize, 16usize, 13usize, 2138440u32);
    emu.adr_no_count(13usize, 30usize, 13usize, 2138444u32);
    emu.mul_no_count(22usize, 7usize, 9usize, 2138448u32);
    emu.adr_no_count(22usize, 24usize, 22usize, 2138452u32);
    emu.sltru_no_count(30usize, 22usize, 24usize, 2138456u32);
    emu.adr_no_count(12usize, 12usize, 30usize, 2138460u32);
    emu.adr_no_count(16usize, 31usize, 16usize, 2138464u32);
    emu.sltru_no_count(24usize, 16usize, 31usize, 2138468u32);
    emu.adr_no_count(8usize, 23usize, 12usize, 2138472u32);
    emu.adr_no_count(24usize, 13usize, 24usize, 2138476u32);
    emu.add_memory_rw_events(31usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2138484u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a174));
    } else {
        emu.pc = 2138480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a170));
    }
}
#[inline(always)]
pub fn block_0x0020a170(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(30usize, 8usize, 23usize, 2138484u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2138484u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a174));
}
#[inline]
pub fn block_0x0020a174(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(23usize, 1usize, 20usize, 2138488u32);
    emu.sltru_no_count(26usize, 27usize, 26usize, 2138492u32);
    emu.adr_no_count(25usize, 27usize, 25usize, 2138496u32);
    emu.adr_no_count(30usize, 16usize, 30usize, 2138500u32);
    emu.adr_no_count(10usize, 8usize, 10usize, 2138504u32);
    emu.adr_no_count(6usize, 22usize, 6usize, 2138508u32);
    emu.sltru_no_count(12usize, 30usize, 16usize, 2138512u32);
    emu.sltru_no_count(16usize, 6usize, 22usize, 2138516u32);
    emu.adr_no_count(20usize, 10usize, 16usize, 2138520u32);
    emu.adr_no_count(10usize, 24usize, 12usize, 2138524u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2138532u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a1a4));
    } else {
        emu.pc = 2138528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a1a0));
    }
}
#[inline(always)]
pub fn block_0x0020a1a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 20usize, 8usize, 2138532u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2138532u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a1a4));
}
#[inline(never)]
pub fn block_0x0020a1a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 30u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(23usize, 23usize, 26usize, 2138536u32);
    emu.sltru_no_count(26usize, 25usize, 27usize, 2138540u32);
    emu.adr_no_count(24usize, 30usize, 16usize, 2138544u32);
    emu.mulhu_no_count(12usize, 17usize, 9usize, 2138548u32);
    emu.mul_no_count(13usize, 5usize, 9usize, 2138552u32);
    emu.mulhu_no_count(16usize, 5usize, 9usize, 2138556u32);
    emu.mul_no_count(31usize, 17usize, 18usize, 2138560u32);
    emu.sltru_no_count(30usize, 24usize, 30usize, 2138564u32);
    emu.adr_no_count(10usize, 10usize, 30usize, 2138568u32);
    emu.mulhu_no_count(30usize, 17usize, 18usize, 2138572u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2138576u32);
    emu.sltru_no_count(13usize, 12usize, 13usize, 2138580u32);
    emu.adr_no_count(13usize, 16usize, 13usize, 2138584u32);
    emu.mul_no_count(8usize, 5usize, 18usize, 2138588u32);
    emu.adr_no_count(12usize, 31usize, 12usize, 2138592u32);
    emu.sltru_no_count(16usize, 12usize, 31usize, 2138596u32);
    emu.adr_no_count(16usize, 30usize, 16usize, 2138600u32);
    emu.mulhu_no_count(30usize, 5usize, 18usize, 2138604u32);
    emu.adr_no_count(16usize, 13usize, 16usize, 2138608u32);
    emu.sltru_no_count(13usize, 16usize, 13usize, 2138612u32);
    emu.adr_no_count(13usize, 30usize, 13usize, 2138616u32);
    emu.mul_no_count(22usize, 17usize, 9usize, 2138620u32);
    emu.adr_no_count(22usize, 21usize, 22usize, 2138624u32);
    emu.sltru_no_count(30usize, 22usize, 21usize, 2138628u32);
    emu.adr_no_count(12usize, 12usize, 30usize, 2138632u32);
    emu.adr_no_count(16usize, 8usize, 16usize, 2138636u32);
    emu.sltru_no_count(27usize, 16usize, 8usize, 2138640u32);
    emu.adr_no_count(8usize, 19usize, 12usize, 2138644u32);
    emu.adr_no_count(27usize, 13usize, 27usize, 2138648u32);
    emu.add_memory_rw_events(29usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2138656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a220));
    } else {
        emu.pc = 2138652u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a21c));
    }
}
#[inline(always)]
pub fn block_0x0020a21c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(30usize, 8usize, 19usize, 2138656u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2138656u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a220));
}
#[inline]
pub fn block_0x0020a220(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(23usize, 23usize, 26usize, 2138660u32);
    emu.adr_no_count(30usize, 16usize, 30usize, 2138664u32);
    emu.adr_no_count(21usize, 8usize, 10usize, 2138668u32);
    emu.adr_no_count(24usize, 22usize, 24usize, 2138672u32);
    emu.sltru_no_count(19usize, 30usize, 16usize, 2138676u32);
    emu.sltru_no_count(10usize, 24usize, 22usize, 2138680u32);
    emu.adr_no_count(21usize, 21usize, 10usize, 2138684u32);
    emu.adr_no_count(19usize, 27usize, 19usize, 2138688u32);
    emu.lw_no_count(13usize, 2usize, 40u32, 2138692u32)?;
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a == b {
        emu.pc = 2138700u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a24c));
    } else {
        emu.pc = 2138696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a248));
    }
}
#[inline(always)]
pub fn block_0x0020a248(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 21usize, 8usize, 2138700u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2138700u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a24c));
}
#[inline]
pub fn block_0x0020a24c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 22u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 30usize, 10usize, 2138704u32);
    emu.lw_no_count(8usize, 2usize, 36u32, 2138708u32)?;
    emu.mulhu_no_count(12usize, 8usize, 9usize, 2138712u32);
    emu.adi_no_count(16usize, 13usize, 0u32, 2138716u32);
    emu.mul_no_count(13usize, 13usize, 9usize, 2138720u32);
    emu.mulhu_no_count(16usize, 16usize, 9usize, 2138724u32);
    emu.sltru_no_count(30usize, 10usize, 30usize, 2138728u32);
    emu.adr_no_count(19usize, 19usize, 30usize, 2138732u32);
    emu.mul_no_count(31usize, 8usize, 18usize, 2138736u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2138740u32);
    emu.sltru_no_count(30usize, 12usize, 13usize, 2138744u32);
    emu.adr_no_count(30usize, 16usize, 30usize, 2138748u32);
    emu.mulhu_no_count(22usize, 8usize, 18usize, 2138752u32);
    emu.mul_no_count(16usize, 8usize, 9usize, 2138756u32);
    emu.adr_no_count(16usize, 25usize, 16usize, 2138760u32);
    emu.adr_no_count(12usize, 31usize, 12usize, 2138764u32);
    emu.sltru_no_count(27usize, 16usize, 25usize, 2138768u32);
    emu.sltru_no_count(13usize, 12usize, 31usize, 2138772u32);
    emu.adr_no_count(12usize, 23usize, 12usize, 2138776u32);
    emu.adr_no_count(8usize, 12usize, 27usize, 2138780u32);
    emu.adr_no_count(22usize, 22usize, 13usize, 2138784u32);
    emu.add_memory_rw_events(21usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2138792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a2a8));
    } else {
        emu.pc = 2138788u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a2a4));
    }
}
#[inline(always)]
pub fn block_0x0020a2a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(27usize, 8usize, 23usize, 2138792u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2138792u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a2a8));
}
#[inline]
pub fn block_0x0020a2a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(22usize, 30usize, 22usize, 2138796u32);
    emu.lw_no_count(9usize, 11usize, 24u32, 2138800u32)?;
    emu.lw_no_count(11usize, 11usize, 28u32, 2138804u32)?;
    emu.adr_no_count(23usize, 16usize, 10usize, 2138808u32);
    emu.sltru_no_count(25usize, 23usize, 16usize, 2138812u32);
    emu.adr_no_count(19usize, 8usize, 19usize, 2138816u32);
    emu.adr_no_count(19usize, 19usize, 25usize, 2138820u32);
    emu.lw_no_count(10usize, 2usize, 40u32, 2138824u32)?;
    emu.mul_no_count(1usize, 10usize, 18usize, 2138828u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2138836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a2d4));
    } else {
        emu.pc = 2138832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a2d0));
    }
}
#[inline(always)]
pub fn block_0x0020a2d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(25usize, 19usize, 8usize, 2138836u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2138836u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a2d4));
}
#[inline(never)]
pub fn block_0x0020a2d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(26usize, 1usize, 22usize, 2138840u32);
    emu.sltru_no_count(30usize, 22usize, 30usize, 2138844u32);
    emu.mulhu_no_count(10usize, 10usize, 18usize, 2138848u32);
    emu.mulhu_no_count(12usize, 15usize, 9usize, 2138852u32);
    emu.mul_no_count(13usize, 29usize, 9usize, 2138856u32);
    emu.mulhu_no_count(16usize, 29usize, 9usize, 2138860u32);
    emu.mul_no_count(31usize, 15usize, 11usize, 2138864u32);
    emu.mulhu_no_count(8usize, 15usize, 11usize, 2138868u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2138872u32);
    emu.sltru_no_count(13usize, 12usize, 13usize, 2138876u32);
    emu.adr_no_count(13usize, 16usize, 13usize, 2138880u32);
    emu.mul_no_count(16usize, 29usize, 11usize, 2138884u32);
    emu.mulhu_no_count(29usize, 29usize, 11usize, 2138888u32);
    emu.adr_no_count(12usize, 31usize, 12usize, 2138892u32);
    emu.sltru_no_count(31usize, 12usize, 31usize, 2138896u32);
    emu.adr_no_count(31usize, 8usize, 31usize, 2138900u32);
    emu.mul_no_count(18usize, 15usize, 9usize, 2138904u32);
    emu.adr_no_count(18usize, 6usize, 18usize, 2138908u32);
    emu.sltru_no_count(8usize, 18usize, 6usize, 2138912u32);
    emu.adr_no_count(12usize, 12usize, 8usize, 2138916u32);
    emu.adr_no_count(31usize, 13usize, 31usize, 2138920u32);
    emu.sltru_no_count(13usize, 31usize, 13usize, 2138924u32);
    emu.adr_no_count(22usize, 16usize, 31usize, 2138928u32);
    emu.sltru_no_count(6usize, 22usize, 16usize, 2138932u32);
    emu.adr_no_count(13usize, 29usize, 13usize, 2138936u32);
    emu.adr_no_count(29usize, 20usize, 12usize, 2138940u32);
    emu.adr_no_count(6usize, 13usize, 6usize, 2138944u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(29usize);
    if a == b {
        emu.pc = 2138952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a348));
    } else {
        emu.pc = 2138948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a344));
    }
}
#[inline(always)]
pub fn block_0x0020a344(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(8usize, 29usize, 20usize, 2138952u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2138952u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a348));
}
#[inline(never)]
pub fn block_0x0020a348(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 31u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(20usize, 26usize, 1usize, 2138956u32);
    emu.adr_no_count(1usize, 10usize, 30usize, 2138960u32);
    emu.adr_no_count(27usize, 26usize, 27usize, 2138964u32);
    emu.adr_no_count(8usize, 22usize, 8usize, 2138968u32);
    emu.mulhu_no_count(12usize, 7usize, 9usize, 2138972u32);
    emu.mul_no_count(13usize, 28usize, 9usize, 2138976u32);
    emu.mulhu_no_count(16usize, 28usize, 9usize, 2138980u32);
    emu.mul_no_count(31usize, 7usize, 11usize, 2138984u32);
    emu.sltru_no_count(10usize, 8usize, 22usize, 2138988u32);
    emu.adr_no_count(10usize, 6usize, 10usize, 2138992u32);
    emu.mulhu_no_count(6usize, 7usize, 11usize, 2138996u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2139000u32);
    emu.sltru_no_count(13usize, 12usize, 13usize, 2139004u32);
    emu.adr_no_count(13usize, 16usize, 13usize, 2139008u32);
    emu.mul_no_count(16usize, 28usize, 11usize, 2139012u32);
    emu.mulhu_no_count(22usize, 28usize, 11usize, 2139016u32);
    emu.mul_no_count(28usize, 7usize, 9usize, 2139020u32);
    emu.adr_no_count(28usize, 24usize, 28usize, 2139024u32);
    emu.adr_no_count(12usize, 31usize, 12usize, 2139028u32);
    emu.sltru_no_count(30usize, 28usize, 24usize, 2139032u32);
    emu.sltru_no_count(7usize, 12usize, 31usize, 2139036u32);
    emu.adr_no_count(12usize, 12usize, 30usize, 2139040u32);
    emu.adr_no_count(6usize, 6usize, 7usize, 2139044u32);
    emu.adr_no_count(7usize, 13usize, 6usize, 2139048u32);
    emu.adr_no_count(6usize, 16usize, 7usize, 2139052u32);
    emu.sltru_no_count(13usize, 7usize, 13usize, 2139056u32);
    emu.sltru_no_count(16usize, 6usize, 16usize, 2139060u32);
    emu.adr_no_count(13usize, 22usize, 13usize, 2139064u32);
    emu.adr_no_count(24usize, 21usize, 12usize, 2139068u32);
    emu.adr_no_count(16usize, 13usize, 16usize, 2139072u32);
    emu.add_memory_rw_events(30usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(24usize);
    if a == b {
        emu.pc = 2139080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a3c8));
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
#[inline(always)]
pub fn block_0x0020a3c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(30usize, 24usize, 21usize, 2139080u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2139080u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a3c8));
}
#[inline]
pub fn block_0x0020a3c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(20usize, 1usize, 20usize, 2139084u32);
    emu.sltru_no_count(26usize, 27usize, 26usize, 2139088u32);
    emu.adr_no_count(7usize, 27usize, 25usize, 2139092u32);
    emu.adr_no_count(30usize, 6usize, 30usize, 2139096u32);
    emu.adr_no_count(10usize, 24usize, 10usize, 2139100u32);
    emu.adr_no_count(22usize, 28usize, 8usize, 2139104u32);
    emu.sltru_no_count(12usize, 30usize, 6usize, 2139108u32);
    emu.sltru_no_count(6usize, 22usize, 28usize, 2139112u32);
    emu.adr_no_count(21usize, 10usize, 6usize, 2139116u32);
    emu.adr_no_count(10usize, 16usize, 12usize, 2139120u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a == b {
        emu.pc = 2139128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a3f8));
    } else {
        emu.pc = 2139124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a3f4));
    }
}
#[inline(always)]
pub fn block_0x0020a3f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(6usize, 21usize, 24usize, 2139128u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2139128u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a3f8));
}
#[inline(never)]
pub fn block_0x0020a3f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 30u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(28usize, 20usize, 26usize, 2139132u32);
    emu.sltru_no_count(24usize, 7usize, 27usize, 2139136u32);
    emu.adr_no_count(20usize, 30usize, 6usize, 2139140u32);
    emu.mulhu_no_count(12usize, 17usize, 9usize, 2139144u32);
    emu.mul_no_count(13usize, 5usize, 9usize, 2139148u32);
    emu.mulhu_no_count(16usize, 5usize, 9usize, 2139152u32);
    emu.mul_no_count(6usize, 17usize, 11usize, 2139156u32);
    emu.sltru_no_count(30usize, 20usize, 30usize, 2139160u32);
    emu.adr_no_count(10usize, 10usize, 30usize, 2139164u32);
    emu.mulhu_no_count(31usize, 17usize, 11usize, 2139168u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2139172u32);
    emu.sltru_no_count(13usize, 12usize, 13usize, 2139176u32);
    emu.adr_no_count(13usize, 16usize, 13usize, 2139180u32);
    emu.mul_no_count(8usize, 5usize, 11usize, 2139184u32);
    emu.mulhu_no_count(25usize, 5usize, 11usize, 2139188u32);
    emu.mul_no_count(5usize, 17usize, 9usize, 2139192u32);
    emu.adr_no_count(5usize, 23usize, 5usize, 2139196u32);
    emu.adr_no_count(12usize, 6usize, 12usize, 2139200u32);
    emu.sltru_no_count(30usize, 5usize, 23usize, 2139204u32);
    emu.sltru_no_count(16usize, 12usize, 6usize, 2139208u32);
    emu.adr_no_count(12usize, 12usize, 30usize, 2139212u32);
    emu.adr_no_count(16usize, 31usize, 16usize, 2139216u32);
    emu.adr_no_count(17usize, 13usize, 16usize, 2139220u32);
    emu.adr_no_count(16usize, 8usize, 17usize, 2139224u32);
    emu.sltru_no_count(13usize, 17usize, 13usize, 2139228u32);
    emu.sltru_no_count(8usize, 16usize, 8usize, 2139232u32);
    emu.adr_no_count(13usize, 25usize, 13usize, 2139236u32);
    emu.adr_no_count(6usize, 19usize, 12usize, 2139240u32);
    emu.adr_no_count(8usize, 13usize, 8usize, 2139244u32);
    emu.add_memory_rw_events(29usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2139252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a474));
    } else {
        emu.pc = 2139248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a470));
    }
}
#[inline(always)]
pub fn block_0x0020a470(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(30usize, 6usize, 19usize, 2139252u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2139252u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a474));
}
#[inline]
pub fn block_0x0020a474(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(17usize, 28usize, 24usize, 2139256u32);
    emu.adr_no_count(28usize, 16usize, 30usize, 2139260u32);
    emu.adr_no_count(10usize, 6usize, 10usize, 2139264u32);
    emu.adr_no_count(20usize, 5usize, 20usize, 2139268u32);
    emu.sltru_no_count(12usize, 28usize, 16usize, 2139272u32);
    emu.sltru_no_count(5usize, 20usize, 5usize, 2139276u32);
    emu.adr_no_count(19usize, 10usize, 5usize, 2139280u32);
    emu.adr_no_count(10usize, 8usize, 12usize, 2139284u32);
    emu.lw_no_count(12usize, 2usize, 40u32, 2139288u32)?;
    emu.lw_no_count(31usize, 2usize, 24u32, 2139292u32)?;
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2139300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a4a4));
    } else {
        emu.pc = 2139296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a4a0));
    }
}
#[inline(always)]
pub fn block_0x0020a4a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 19usize, 6usize, 2139300u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2139300u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a4a4));
}
#[inline]
pub fn block_0x0020a4a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(5usize, 28usize, 5usize, 2139304u32);
    emu.lw_no_count(13usize, 2usize, 36u32, 2139308u32)?;
    emu.mulhu_no_count(25usize, 13usize, 9usize, 2139312u32);
    emu.mul_no_count(23usize, 12usize, 9usize, 2139316u32);
    emu.mul_no_count(24usize, 13usize, 11usize, 2139320u32);
    emu.mul_no_count(6usize, 13usize, 9usize, 2139324u32);
    emu.sltru_no_count(12usize, 5usize, 28usize, 2139328u32);
    emu.adr_no_count(25usize, 23usize, 25usize, 2139332u32);
    emu.adr_no_count(6usize, 7usize, 6usize, 2139336u32);
    emu.adr_no_count(26usize, 24usize, 25usize, 2139340u32);
    emu.sltru_no_count(28usize, 6usize, 7usize, 2139344u32);
    emu.adr_no_count(16usize, 17usize, 26usize, 2139348u32);
    emu.adr_no_count(16usize, 16usize, 28usize, 2139352u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2139356u32);
    emu.lw_no_count(12usize, 2usize, 20u32, 2139360u32)?;
    emu.add_memory_rw_events(15usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2139368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a4e8));
    } else {
        emu.pc = 2139364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a4e4));
    }
}
#[inline(always)]
pub fn block_0x0020a4e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 16usize, 17usize, 2139368u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2139368u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a4e8));
}
#[inline(always)]
pub fn block_0x0020a4e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(5usize, 6usize, 5usize, 2139372u32);
    emu.sltru_no_count(17usize, 5usize, 6usize, 2139376u32);
    emu.adr_no_count(10usize, 16usize, 10usize, 2139380u32);
    emu.adr_no_count(7usize, 10usize, 17usize, 2139384u32);
    emu.mul_no_count(10usize, 15usize, 14usize, 2139388u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a == b {
        emu.pc = 2139396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a504));
    } else {
        emu.pc = 2139392u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a500));
    }
}
#[inline(always)]
pub fn block_0x0020a500(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(17usize, 7usize, 16usize, 2139396u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2139396u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a504));
}
#[inline]
pub fn block_0x0020a504(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(27usize, 12usize, 10usize, 2139400u32);
    emu.sltru_no_count(12usize, 27usize, 12usize, 2139404u32);
    emu.adr_no_count(12usize, 31usize, 12usize, 2139408u32);
    emu.sltru_no_count(13usize, 12usize, 31usize, 2139412u32);
    emu.lw_no_count(14usize, 2usize, 12u32, 2139416u32)?;
    emu.adr_no_count(30usize, 14usize, 12usize, 2139420u32);
    emu.sltru_no_count(8usize, 30usize, 14usize, 2139424u32);
    emu.adr_no_count(13usize, 13usize, 8usize, 2139428u32);
    emu.lw_no_count(12usize, 2usize, 16u32, 2139432u32)?;
    emu.adr_no_count(15usize, 12usize, 13usize, 2139436u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2139444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a534));
    } else {
        emu.pc = 2139440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a530));
    }
}
#[inline(always)]
pub fn block_0x0020a530(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(8usize, 15usize, 12usize, 2139444u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2139444u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a534));
}
#[inline(never)]
pub fn block_0x0020a534(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 25u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(6usize, 31usize, 10usize, 2139448u32);
    emu.sbr_no_count(12usize, 0usize, 10usize, 2139452u32);
    emu.adi_no_count(14usize, 0usize, 4294967295u32, 2139456u32);
    emu.sltru_no_count(12usize, 6usize, 12usize, 2139460u32);
    emu.mulhu_no_count(13usize, 10usize, 14usize, 2139464u32);
    emu.sbr_no_count(13usize, 13usize, 31usize, 2139468u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2139472u32);
    emu.sbr_no_count(13usize, 0usize, 31usize, 2139476u32);
    emu.adr_no_count(16usize, 8usize, 10usize, 2139480u32);
    emu.mulhu_no_count(31usize, 31usize, 14usize, 2139484u32);
    emu.sltru_no_count(8usize, 16usize, 8usize, 2139488u32);
    emu.adr_no_count(10usize, 16usize, 18usize, 2139492u32);
    emu.adr_no_count(6usize, 6usize, 8usize, 2139496u32);
    emu.sltru_no_count(16usize, 10usize, 16usize, 2139500u32);
    emu.sltru_no_count(13usize, 12usize, 13usize, 2139504u32);
    emu.adr_no_count(13usize, 31usize, 13usize, 2139508u32);
    emu.sltru_no_count(31usize, 0usize, 6usize, 2139512u32);
    emu.adi_no_count(31usize, 31usize, 4294967295u32, 2139516u32);
    emu.anr_no_count(31usize, 31usize, 8usize, 2139520u32);
    emu.adr_no_count(1usize, 6usize, 16usize, 2139524u32);
    emu.adr_no_count(31usize, 12usize, 31usize, 2139528u32);
    emu.sltru_no_count(12usize, 31usize, 12usize, 2139532u32);
    emu.adr_no_count(1usize, 1usize, 29usize, 2139536u32);
    emu.adr_no_count(29usize, 13usize, 12usize, 2139540u32);
    emu.add_memory_rw_events(24usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(1usize);
    if a == b {
        emu.pc = 2139548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a59c));
    } else {
        emu.pc = 2139544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a598));
    }
}
#[inline(always)]
pub fn block_0x0020a598(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 1usize, 6usize, 2139548u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2139548u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a59c));
}
#[inline(always)]
pub fn block_0x0020a59c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(16usize, 31usize, 16usize, 2139552u32);
    emu.sltru_no_count(12usize, 16usize, 31usize, 2139556u32);
    emu.adr_no_count(18usize, 22usize, 16usize, 2139560u32);
    emu.adr_no_count(12usize, 29usize, 12usize, 2139564u32);
    emu.sltru_no_count(16usize, 18usize, 22usize, 2139568u32);
    emu.adr_no_count(12usize, 12usize, 16usize, 2139572u32);
    emu.adr_no_count(8usize, 21usize, 12usize, 2139576u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2139584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a5c0));
    } else {
        emu.pc = 2139580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a5bc));
    }
}
#[inline(always)]
pub fn block_0x0020a5bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 8usize, 21usize, 2139584u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2139584u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a5c0));
}
#[inline]
pub fn block_0x0020a5c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(29usize, 2usize, 32u32, 2139588u32)?;
    emu.adr_no_count(29usize, 15usize, 29usize, 2139592u32);
    emu.sltru_no_count(12usize, 29usize, 15usize, 2139596u32);
    emu.adr_no_count(12usize, 27usize, 12usize, 2139600u32);
    emu.sltru_no_count(13usize, 12usize, 27usize, 2139604u32);
    emu.adr_no_count(15usize, 10usize, 12usize, 2139608u32);
    emu.sltru_no_count(22usize, 15usize, 10usize, 2139612u32);
    emu.adr_no_count(13usize, 13usize, 22usize, 2139616u32);
    emu.adr_no_count(31usize, 1usize, 13usize, 2139620u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(1usize);
    let a = emu.read_reg_a_tracked(31usize);
    if a == b {
        emu.pc = 2139628u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a5ec));
    } else {
        emu.pc = 2139624u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a5e8));
    }
}
#[inline(always)]
pub fn block_0x0020a5e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(22usize, 31usize, 1usize, 2139628u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2139628u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a5ec));
}
#[inline(never)]
pub fn block_0x0020a5ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 31u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(21usize, 25usize, 23usize, 2139632u32);
    emu.lw_no_count(10usize, 2usize, 40u32, 2139636u32)?;
    emu.mulhu_no_count(10usize, 10usize, 9usize, 2139640u32);
    emu.sltru_no_count(9usize, 26usize, 24usize, 2139644u32);
    emu.lw_no_count(12usize, 2usize, 36u32, 2139648u32)?;
    emu.mulhu_no_count(13usize, 12usize, 11usize, 2139652u32);
    emu.lw_no_count(26usize, 2usize, 32u32, 2139656u32)?;
    emu.sbr_no_count(24usize, 27usize, 26usize, 2139660u32);
    emu.sbr_no_count(12usize, 0usize, 26usize, 2139664u32);
    emu.mulhu_no_count(6usize, 26usize, 14usize, 2139668u32);
    emu.sltru_no_count(12usize, 24usize, 12usize, 2139672u32);
    emu.sbr_no_count(6usize, 6usize, 27usize, 2139676u32);
    emu.adr_no_count(12usize, 6usize, 12usize, 2139680u32);
    emu.sbr_no_count(23usize, 0usize, 27usize, 2139684u32);
    emu.mulhu_no_count(25usize, 27usize, 14usize, 2139688u32);
    emu.adr_no_count(26usize, 22usize, 26usize, 2139692u32);
    emu.sltru_no_count(27usize, 26usize, 22usize, 2139696u32);
    emu.adr_no_count(6usize, 26usize, 18usize, 2139700u32);
    emu.adr_no_count(24usize, 24usize, 27usize, 2139704u32);
    emu.sltru_no_count(22usize, 6usize, 26usize, 2139708u32);
    emu.sltru_no_count(18usize, 12usize, 23usize, 2139712u32);
    emu.adr_no_count(25usize, 25usize, 18usize, 2139716u32);
    emu.sltru_no_count(18usize, 0usize, 24usize, 2139720u32);
    emu.adi_no_count(18usize, 18usize, 4294967295u32, 2139724u32);
    emu.anr_no_count(23usize, 18usize, 27usize, 2139728u32);
    emu.adr_no_count(18usize, 24usize, 22usize, 2139732u32);
    emu.adr_no_count(23usize, 12usize, 23usize, 2139736u32);
    emu.sltru_no_count(12usize, 23usize, 12usize, 2139740u32);
    emu.adr_no_count(18usize, 18usize, 8usize, 2139744u32);
    emu.adr_no_count(8usize, 25usize, 12usize, 2139748u32);
    emu.add_memory_rw_events(30usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2139756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a66c));
    } else {
        emu.pc = 2139752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a668));
    }
}
#[inline(always)]
pub fn block_0x0020a668(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(22usize, 18usize, 24usize, 2139756u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2139756u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a66c));
}
#[inline]
pub fn block_0x0020a66c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(21usize, 10usize, 21usize, 2139760u32);
    emu.adr_no_count(13usize, 13usize, 9usize, 2139764u32);
    emu.adr_no_count(22usize, 23usize, 22usize, 2139768u32);
    emu.adr_no_count(10usize, 16usize, 20usize, 2139772u32);
    emu.sltru_no_count(12usize, 22usize, 23usize, 2139776u32);
    emu.sltru_no_count(16usize, 10usize, 16usize, 2139780u32);
    emu.adr_no_count(20usize, 10usize, 22usize, 2139784u32);
    emu.adr_no_count(12usize, 8usize, 12usize, 2139788u32);
    emu.adr_no_count(19usize, 19usize, 16usize, 2139792u32);
    emu.sltru_no_count(8usize, 20usize, 10usize, 2139796u32);
    emu.sltru_no_count(10usize, 0usize, 19usize, 2139800u32);
    emu.adr_no_count(12usize, 19usize, 12usize, 2139804u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2139808u32);
    emu.adr_no_count(9usize, 12usize, 8usize, 2139812u32);
    emu.anr_no_count(22usize, 10usize, 16usize, 2139816u32);
    emu.add_memory_rw_events(15usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2139824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a6b0));
    } else {
        emu.pc = 2139820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a6ac));
    }
}
#[inline(always)]
pub fn block_0x0020a6ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(8usize, 9usize, 19usize, 2139824u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2139824u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a6b0));
}
#[inline]
pub fn block_0x0020a6b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(23usize, 21usize, 13usize, 2139828u32);
    emu.lw_no_count(10usize, 2usize, 40u32, 2139832u32)?;
    emu.mul_no_count(19usize, 10usize, 11usize, 2139836u32);
    emu.adr_no_count(16usize, 31usize, 30usize, 2139840u32);
    emu.sltru_no_count(10usize, 16usize, 31usize, 2139844u32);
    emu.adr_no_count(10usize, 29usize, 10usize, 2139848u32);
    emu.sltru_no_count(12usize, 10usize, 29usize, 2139852u32);
    emu.adr_no_count(13usize, 6usize, 10usize, 2139856u32);
    emu.adr_no_count(12usize, 18usize, 12usize, 2139860u32);
    emu.sltru_no_count(10usize, 13usize, 6usize, 2139864u32);
    emu.adr_no_count(6usize, 12usize, 10usize, 2139868u32);
    emu.adr_no_count(8usize, 22usize, 8usize, 2139872u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2139880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a6e8));
    } else {
        emu.pc = 2139876u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a6e4));
    }
}
#[inline(always)]
pub fn block_0x0020a6e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 6usize, 18usize, 2139880u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2139880u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a6e8));
}
#[inline]
pub fn block_0x0020a6e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(31usize, 19usize, 23usize, 2139884u32);
    emu.sltru_no_count(21usize, 23usize, 21usize, 2139888u32);
    emu.lw_no_count(12usize, 2usize, 40u32, 2139892u32)?;
    emu.mulhu_no_count(12usize, 12usize, 11usize, 2139896u32);
    emu.sltru_no_count(11usize, 8usize, 22usize, 2139900u32);
    emu.sbr_no_count(22usize, 29usize, 30usize, 2139904u32);
    emu.sbr_no_count(18usize, 0usize, 30usize, 2139908u32);
    emu.mulhu_no_count(23usize, 30usize, 14usize, 2139912u32);
    emu.sltru_no_count(18usize, 22usize, 18usize, 2139916u32);
    emu.sbr_no_count(23usize, 23usize, 29usize, 2139920u32);
    emu.adr_no_count(18usize, 23usize, 18usize, 2139924u32);
    emu.sbr_no_count(23usize, 0usize, 29usize, 2139928u32);
    emu.mulhu_no_count(24usize, 29usize, 14usize, 2139932u32);
    emu.adr_no_count(30usize, 20usize, 30usize, 2139936u32);
    emu.sltru_no_count(29usize, 30usize, 20usize, 2139940u32);
    emu.adr_no_count(20usize, 22usize, 29usize, 2139944u32);
    emu.sltru_no_count(22usize, 18usize, 23usize, 2139948u32);
    emu.adr_no_count(20usize, 9usize, 20usize, 2139952u32);
    emu.adr_no_count(22usize, 24usize, 22usize, 2139956u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2139964u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a73c));
    } else {
        emu.pc = 2139960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a738));
    }
}
#[inline(always)]
pub fn block_0x0020a738(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(29usize, 20usize, 9usize, 2139964u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2139964u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a73c));
}
#[inline]
pub fn block_0x0020a73c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(9usize, 31usize, 19usize, 2139968u32);
    emu.adr_no_count(19usize, 12usize, 21usize, 2139972u32);
    emu.adr_no_count(12usize, 31usize, 28usize, 2139976u32);
    emu.adr_no_count(21usize, 18usize, 29usize, 2139980u32);
    emu.adr_no_count(28usize, 30usize, 10usize, 2139984u32);
    emu.adr_no_count(5usize, 8usize, 5usize, 2139988u32);
    emu.adr_no_count(7usize, 11usize, 7usize, 2139992u32);
    emu.sltru_no_count(10usize, 21usize, 18usize, 2139996u32);
    emu.sltru_no_count(18usize, 28usize, 30usize, 2140000u32);
    emu.sltru_no_count(29usize, 5usize, 8usize, 2140004u32);
    emu.adr_no_count(22usize, 22usize, 10usize, 2140008u32);
    emu.adr_no_count(30usize, 20usize, 18usize, 2140012u32);
    emu.sltru_no_count(10usize, 30usize, 20usize, 2140016u32);
    emu.anr_no_count(10usize, 18usize, 10usize, 2140020u32);
    emu.adr_no_count(10usize, 21usize, 10usize, 2140024u32);
    emu.sltru_no_count(18usize, 10usize, 21usize, 2140028u32);
    emu.adr_no_count(8usize, 7usize, 29usize, 2140032u32);
    emu.adr_no_count(7usize, 22usize, 18usize, 2140036u32);
    emu.add_memory_rw_events(18usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2140044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a78c));
    } else {
        emu.pc = 2140040u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a788));
    }
}
#[inline(always)]
pub fn block_0x0020a788(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(29usize, 8usize, 11usize, 2140044u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2140044u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a78c));
}
#[inline(always)]
pub fn block_0x0020a78c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(19usize, 19usize, 9usize, 2140048u32);
    emu.sltru_no_count(11usize, 12usize, 31usize, 2140052u32);
    emu.adr_no_count(7usize, 8usize, 7usize, 2140056u32);
    emu.adr_no_count(10usize, 5usize, 10usize, 2140060u32);
    emu.sltru_no_count(9usize, 10usize, 5usize, 2140064u32);
    emu.adr_no_count(5usize, 7usize, 9usize, 2140068u32);
    emu.adr_no_count(7usize, 12usize, 17usize, 2140072u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2140080u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a7b0));
    } else {
        emu.pc = 2140076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a7ac));
    }
}
#[inline(always)]
pub fn block_0x0020a7ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(9usize, 5usize, 8usize, 2140080u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2140080u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a7b0));
}
#[inline]
pub fn block_0x0020a7b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 19usize, 11usize, 2140084u32);
    emu.sltru_no_count(18usize, 7usize, 12usize, 2140088u32);
    emu.adr_no_count(11usize, 6usize, 15usize, 2140092u32);
    emu.sltru_no_count(12usize, 11usize, 6usize, 2140096u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2140100u32);
    emu.sltru_no_count(17usize, 12usize, 16usize, 2140104u32);
    emu.adr_no_count(12usize, 28usize, 12usize, 2140108u32);
    emu.sltru_no_count(31usize, 12usize, 28usize, 2140112u32);
    emu.adr_no_count(17usize, 17usize, 31usize, 2140116u32);
    emu.adr_no_count(17usize, 30usize, 17usize, 2140120u32);
    emu.adr_no_count(6usize, 29usize, 9usize, 2140124u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(30usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2140132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a7e4));
    } else {
        emu.pc = 2140128u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a7e0));
    }
}
#[inline(always)]
pub fn block_0x0020a7e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(31usize, 17usize, 30usize, 2140132u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2140132u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a7e4));
}
#[inline]
pub fn block_0x0020a7e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(30usize, 8usize, 18usize, 2140136u32);
    emu.sltru_no_count(28usize, 6usize, 29usize, 2140140u32);
    emu.sbr_no_count(8usize, 16usize, 15usize, 2140144u32);
    emu.sbr_no_count(29usize, 0usize, 15usize, 2140148u32);
    emu.mulhu_no_count(9usize, 15usize, 14usize, 2140152u32);
    emu.mulhu_no_count(18usize, 16usize, 14usize, 2140156u32);
    emu.sbr_no_count(14usize, 9usize, 16usize, 2140160u32);
    emu.sbr_no_count(16usize, 0usize, 16usize, 2140164u32);
    emu.adr_no_count(15usize, 10usize, 15usize, 2140168u32);
    emu.sltru_no_count(9usize, 8usize, 29usize, 2140172u32);
    emu.sltru_no_count(29usize, 15usize, 10usize, 2140176u32);
    emu.adr_no_count(14usize, 14usize, 9usize, 2140180u32);
    emu.adr_no_count(10usize, 8usize, 29usize, 2140184u32);
    emu.sltru_no_count(16usize, 14usize, 16usize, 2140188u32);
    emu.adr_no_count(10usize, 5usize, 10usize, 2140192u32);
    emu.adr_no_count(16usize, 18usize, 16usize, 2140196u32);
    emu.add_memory_rw_events(16usize);
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2140204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a82c));
    } else {
        emu.pc = 2140200u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a828));
    }
}
#[inline(always)]
pub fn block_0x0020a828(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(29usize, 10usize, 5usize, 2140204u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2140204u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a82c));
}
#[inline]
pub fn block_0x0020a82c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(29usize, 14usize, 29usize, 2140208u32);
    emu.adr_no_count(5usize, 15usize, 31usize, 2140212u32);
    emu.adr_no_count(7usize, 6usize, 7usize, 2140216u32);
    emu.adr_no_count(30usize, 28usize, 30usize, 2140220u32);
    emu.sltru_no_count(14usize, 29usize, 14usize, 2140224u32);
    emu.sltru_no_count(31usize, 5usize, 15usize, 2140228u32);
    emu.sltru_no_count(15usize, 7usize, 6usize, 2140232u32);
    emu.adr_no_count(16usize, 16usize, 14usize, 2140236u32);
    emu.adr_no_count(6usize, 10usize, 31usize, 2140240u32);
    emu.sltru_no_count(10usize, 6usize, 10usize, 2140244u32);
    emu.anr_no_count(14usize, 31usize, 10usize, 2140248u32);
    emu.adr_no_count(14usize, 29usize, 14usize, 2140252u32);
    emu.sltru_no_count(29usize, 14usize, 29usize, 2140256u32);
    emu.adr_no_count(10usize, 30usize, 15usize, 2140260u32);
    emu.adr_no_count(16usize, 16usize, 29usize, 2140264u32);
    emu.add_memory_rw_events(15usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2140272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a870));
    } else {
        emu.pc = 2140268u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a86c));
    }
}
#[inline(always)]
pub fn block_0x0020a86c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(15usize, 10usize, 28usize, 2140272u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2140272u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a870));
}
#[inline(always)]
pub fn block_0x0020a870(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(16usize, 10usize, 16usize, 2140276u32);
    emu.adr_no_count(28usize, 7usize, 14usize, 2140280u32);
    emu.sltru_no_count(29usize, 28usize, 7usize, 2140284u32);
    emu.adr_no_count(7usize, 16usize, 29usize, 2140288u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a == b {
        emu.pc = 2140296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a888));
    } else {
        emu.pc = 2140292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a884));
    }
}
#[inline(always)]
pub fn block_0x0020a884(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(29usize, 7usize, 10usize, 2140296u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2140296u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a888));
}
#[inline]
pub fn block_0x0020a888(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 1u32, 2140300u32);
    emu.sltiu_no_count(10usize, 13usize, 1u32, 2140304u32);
    emu.adr_no_count(11usize, 11usize, 10usize, 2140308u32);
    emu.orr_no_count(10usize, 13usize, 11usize, 2140312u32);
    emu.sltiu_no_count(30usize, 10usize, 1u32, 2140316u32);
    emu.adi_no_count(30usize, 30usize, 4294967295u32, 2140320u32);
    emu.adr_no_count(17usize, 30usize, 17usize, 2140324u32);
    emu.adr_no_count(16usize, 30usize, 12usize, 2140328u32);
    emu.sltru_no_count(10usize, 16usize, 30usize, 2140332u32);
    emu.adr_no_count(17usize, 17usize, 10usize, 2140336u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(30usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2140344u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a8b8));
    } else {
        emu.pc = 2140340u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a8b4));
    }
}
#[inline(always)]
pub fn block_0x0020a8b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 17usize, 30usize, 2140344u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2140344u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a8b8));
}
#[inline(always)]
pub fn block_0x0020a8b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 30usize, 10usize, 2140348u32);
    emu.adi_no_count(14usize, 16usize, 1u32, 2140352u32);
    emu.sltru_no_count(31usize, 10usize, 30usize, 2140356u32);
    emu.sltiu_no_count(12usize, 14usize, 1u32, 2140360u32);
    emu.adr_no_count(12usize, 17usize, 12usize, 2140364u32);
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2140368u32);
    emu.adr_no_count(30usize, 30usize, 31usize, 2140372u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2140384u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a8e0));
    } else {
        emu.pc = 2140376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a8d8));
    }
}
#[inline(always)]
pub fn block_0x0020a8d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 12usize, 17usize, 2140380u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2140384u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2140388u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a8e4));
}
#[inline(always)]
pub fn block_0x0020a8e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 14usize, 16usize, 2140388u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2140388u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a8e4));
}
#[inline]
pub fn block_0x0020a8e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 10usize, 4294967295u32, 2140392u32);
    emu.sltiu_no_count(10usize, 10usize, 1u32, 2140396u32);
    emu.adr_no_count(16usize, 17usize, 16usize, 2140400u32);
    emu.sbr_no_count(10usize, 30usize, 10usize, 2140404u32);
    emu.sltru_no_count(16usize, 16usize, 17usize, 2140408u32);
    emu.adr_no_count(10usize, 10usize, 16usize, 2140412u32);
    emu.sai_no_count(10usize, 10usize, 1055u32, 2140416u32);
    emu.adr_no_count(17usize, 6usize, 10usize, 2140420u32);
    emu.adr_no_count(16usize, 5usize, 10usize, 2140424u32);
    emu.sltru_no_count(5usize, 16usize, 5usize, 2140428u32);
    emu.adr_no_count(17usize, 17usize, 5usize, 2140432u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2140440u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a918));
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
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 17usize, 6usize, 2140440u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2140440u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a918));
}
#[inline]
pub fn block_0x0020a918(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(5usize, 10usize, 5usize, 2140444u32);
    emu.sltru_no_count(5usize, 5usize, 10usize, 2140448u32);
    emu.adr_no_count(10usize, 10usize, 5usize, 2140452u32);
    emu.sai_no_count(30usize, 10usize, 1055u32, 2140456u32);
    emu.adr_no_count(10usize, 7usize, 30usize, 2140460u32);
    emu.adr_no_count(6usize, 28usize, 30usize, 2140464u32);
    emu.sltru_no_count(5usize, 6usize, 28usize, 2140468u32);
    emu.adr_no_count(28usize, 10usize, 5usize, 2140472u32);
    emu.adr_no_count(29usize, 15usize, 29usize, 2140476u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2140484u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a944));
    } else {
        emu.pc = 2140480u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a940));
    }
}
#[inline(always)]
pub fn block_0x0020a940(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 28usize, 7usize, 2140484u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2140484u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a944));
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
    emu.sltru_no_count(10usize, 29usize, 15usize, 2140488u32);
    emu.adr_no_count(7usize, 30usize, 5usize, 2140492u32);
    emu.sltru_no_count(5usize, 0usize, 6usize, 2140496u32);
    emu.sltru_no_count(15usize, 7usize, 30usize, 2140500u32);
    emu.adr_no_count(5usize, 28usize, 5usize, 2140504u32);
    emu.adr_no_count(30usize, 30usize, 15usize, 2140508u32);
    emu.adi_no_count(15usize, 6usize, 4294967295u32, 2140512u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2140524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a96c));
    } else {
        emu.pc = 2140516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a964));
    }
}
#[inline(always)]
pub fn block_0x0020a964(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(6usize, 5usize, 28usize, 2140520u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2140524u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2140528u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a970));
}
#[inline(always)]
pub fn block_0x0020a96c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(6usize, 15usize, 6usize, 2140528u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2140528u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a970));
}
#[inline]
pub fn block_0x0020a970(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(28usize, 7usize, 4294967295u32, 2140532u32);
    emu.sltiu_no_count(7usize, 7usize, 1u32, 2140536u32);
    emu.adr_no_count(6usize, 28usize, 6usize, 2140540u32);
    emu.sbr_no_count(7usize, 30usize, 7usize, 2140544u32);
    emu.sltru_no_count(6usize, 6usize, 28usize, 2140548u32);
    emu.adr_no_count(6usize, 7usize, 6usize, 2140552u32);
    emu.sai_no_count(7usize, 6usize, 1055u32, 2140556u32);
    emu.adr_no_count(29usize, 7usize, 29usize, 2140560u32);
    emu.sltru_no_count(6usize, 29usize, 7usize, 2140564u32);
    emu.adr_no_count(10usize, 7usize, 10usize, 2140568u32);
    emu.adr_no_count(10usize, 10usize, 6usize, 2140572u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2140580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a9a4));
    } else {
        emu.pc = 2140576u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a9a0));
    }
}
#[inline(always)]
pub fn block_0x0020a9a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(6usize, 10usize, 7usize, 2140580u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2140580u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a9a4));
}
#[inline(always)]
pub fn block_0x0020a9a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(6usize, 7usize, 6usize, 2140584u32);
    emu.sltru_no_count(10usize, 6usize, 7usize, 2140588u32);
    emu.adr_no_count(13usize, 6usize, 13usize, 2140592u32);
    emu.adr_no_count(7usize, 7usize, 10usize, 2140596u32);
    emu.sltru_no_count(28usize, 13usize, 6usize, 2140600u32);
    emu.adr_no_count(10usize, 7usize, 11usize, 2140604u32);
    emu.adr_no_count(10usize, 10usize, 28usize, 2140608u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2140616u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a9c8));
    } else {
        emu.pc = 2140612u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a9c4));
    }
}
#[inline(always)]
pub fn block_0x0020a9c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 10usize, 7usize, 2140616u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2140616u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a9c8));
}
#[inline]
pub fn block_0x0020a9c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(14usize, 6usize, 14usize, 2140620u32);
    emu.sltru_no_count(29usize, 14usize, 6usize, 2140624u32);
    emu.adr_no_count(11usize, 14usize, 28usize, 2140628u32);
    emu.adr_no_count(28usize, 12usize, 29usize, 2140632u32);
    emu.sltru_no_count(14usize, 11usize, 14usize, 2140636u32);
    emu.sltru_no_count(12usize, 0usize, 28usize, 2140640u32);
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2140644u32);
    emu.anr_no_count(29usize, 12usize, 29usize, 2140648u32);
    emu.adr_no_count(12usize, 28usize, 14usize, 2140652u32);
    emu.sltru_no_count(28usize, 12usize, 28usize, 2140656u32);
    emu.anr_no_count(14usize, 14usize, 28usize, 2140660u32);
    emu.adr_no_count(14usize, 29usize, 14usize, 2140664u32);
    emu.sltru_no_count(29usize, 14usize, 29usize, 2140668u32);
    emu.adr_no_count(16usize, 14usize, 16usize, 2140672u32);
    emu.sltru_no_count(28usize, 16usize, 14usize, 2140676u32);
    emu.adr_no_count(14usize, 29usize, 17usize, 2140680u32);
    emu.adr_no_count(14usize, 14usize, 28usize, 2140684u32);
    emu.add_memory_rw_events(17usize);
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2140692u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aa14));
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
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 14usize, 29usize, 2140692u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2140692u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020aa14));
}
#[inline(never)]
pub fn block_0x0020aa14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 32u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(17usize, 6usize, 1u32, 2140696u32);
    emu.adr_no_count(5usize, 7usize, 5usize, 2140700u32);
    emu.lw_no_count(6usize, 2usize, 28u32, 2140704u32)?;
    emu.sw_no_count(13usize, 6usize, 0u32, 2140708u32)?;
    emu.sw_no_count(10usize, 6usize, 4u32, 2140712u32)?;
    emu.sw_no_count(11usize, 6usize, 8u32, 2140716u32)?;
    emu.sw_no_count(12usize, 6usize, 12u32, 2140720u32)?;
    emu.adr_no_count(15usize, 17usize, 15usize, 2140724u32);
    emu.sltru_no_count(10usize, 15usize, 17usize, 2140728u32);
    emu.adr_no_count(28usize, 15usize, 28usize, 2140732u32);
    emu.adr_no_count(10usize, 5usize, 10usize, 2140736u32);
    emu.sltru_no_count(11usize, 28usize, 15usize, 2140740u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2140744u32);
    emu.sw_no_count(16usize, 6usize, 16u32, 2140748u32)?;
    emu.sw_no_count(14usize, 6usize, 20u32, 2140752u32)?;
    emu.sw_no_count(28usize, 6usize, 24u32, 2140756u32)?;
    emu.sw_no_count(10usize, 6usize, 28u32, 2140760u32)?;
    emu.lw_no_count(1usize, 2usize, 92u32, 2140764u32)?;
    emu.lw_no_count(8usize, 2usize, 88u32, 2140768u32)?;
    emu.lw_no_count(9usize, 2usize, 84u32, 2140772u32)?;
    emu.lw_no_count(18usize, 2usize, 80u32, 2140776u32)?;
    emu.lw_no_count(19usize, 2usize, 76u32, 2140780u32)?;
    emu.lw_no_count(20usize, 2usize, 72u32, 2140784u32)?;
    emu.lw_no_count(21usize, 2usize, 68u32, 2140788u32)?;
    emu.lw_no_count(22usize, 2usize, 64u32, 2140792u32)?;
    emu.lw_no_count(23usize, 2usize, 60u32, 2140796u32)?;
    emu.lw_no_count(24usize, 2usize, 56u32, 2140800u32)?;
    emu.lw_no_count(25usize, 2usize, 52u32, 2140804u32)?;
    emu.lw_no_count(26usize, 2usize, 48u32, 2140808u32)?;
    emu.lw_no_count(27usize, 2usize, 44u32, 2140812u32)?;
    emu.adi_no_count(2usize, 2usize, 96u32, 2140816u32);
    emu.add_memory_rw_events(32usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140820u32;
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
pub fn block_0x0020aa94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2140824u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2140828u32)?;
    emu.adi_no_count(12usize, 11usize, 0u32, 2140832u32);
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2140836u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294966760u32, 2140840u32);
    emu.apc_no_count(1usize, 2140840u32, 4294963200u32, 2140844u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140848u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967124u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020aab0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2140852u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2140856u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2140860u32;
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
pub fn block_0x0020aabc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 68u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2140864u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2140868u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2140872u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2140876u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2140880u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2140884u32)?;
    emu.sw_no_count(20usize, 2usize, 40u32, 2140888u32)?;
    emu.sw_no_count(21usize, 2usize, 36u32, 2140892u32)?;
    emu.sw_no_count(22usize, 2usize, 32u32, 2140896u32)?;
    emu.sw_no_count(23usize, 2usize, 28u32, 2140900u32)?;
    emu.sw_no_count(24usize, 2usize, 24u32, 2140904u32)?;
    emu.sw_no_count(25usize, 2usize, 20u32, 2140908u32)?;
    emu.sw_no_count(26usize, 2usize, 16u32, 2140912u32)?;
    emu.sw_no_count(27usize, 2usize, 12u32, 2140916u32)?;
    emu.lw_no_count(29usize, 11usize, 0u32, 2140920u32)?;
    emu.lw_no_count(6usize, 11usize, 4u32, 2140924u32)?;
    let a = 0u32.wrapping_add(4007632896u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2140928u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(19922944u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2140932u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(3743051776u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2140936u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1125711872u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2140940u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 12usize, 4294966270u32, 2140944u32);
    emu.adi_no_count(16usize, 13usize, 4294966661u32, 2140948u32);
    emu.adi_no_count(12usize, 14usize, 4294966305u32, 2140952u32);
    emu.adi_no_count(13usize, 15usize, 1362u32, 2140956u32);
    emu.mulhu_no_count(14usize, 29usize, 17usize, 2140960u32);
    emu.mul_no_count(15usize, 6usize, 17usize, 2140964u32);
    emu.mulhu_no_count(5usize, 6usize, 17usize, 2140968u32);
    emu.mul_no_count(7usize, 29usize, 16usize, 2140972u32);
    emu.mulhu_no_count(28usize, 29usize, 16usize, 2140976u32);
    emu.mul_no_count(30usize, 6usize, 16usize, 2140980u32);
    emu.mulhu_no_count(31usize, 29usize, 12usize, 2140984u32);
    emu.adr_no_count(14usize, 15usize, 14usize, 2140988u32);
    emu.sltru_no_count(15usize, 14usize, 15usize, 2140992u32);
    emu.adr_no_count(15usize, 5usize, 15usize, 2140996u32);
    emu.mul_no_count(5usize, 6usize, 12usize, 2141000u32);
    emu.adr_no_count(14usize, 7usize, 14usize, 2141004u32);
    emu.sltru_no_count(14usize, 14usize, 7usize, 2141008u32);
    emu.mulhu_no_count(7usize, 6usize, 12usize, 2141012u32);
    emu.adr_no_count(31usize, 5usize, 31usize, 2141016u32);
    emu.sltru_no_count(5usize, 31usize, 5usize, 2141020u32);
    emu.adr_no_count(5usize, 7usize, 5usize, 2141024u32);
    emu.mul_no_count(7usize, 29usize, 13usize, 2141028u32);
    emu.adr_no_count(14usize, 28usize, 14usize, 2141032u32);
    emu.mulhu_no_count(28usize, 29usize, 13usize, 2141036u32);
    emu.adr_no_count(31usize, 7usize, 31usize, 2141040u32);
    emu.sltru_no_count(7usize, 31usize, 7usize, 2141044u32);
    emu.adr_no_count(7usize, 28usize, 7usize, 2141048u32);
    emu.mulhu_no_count(28usize, 6usize, 16usize, 2141052u32);
    emu.adr_no_count(14usize, 15usize, 14usize, 2141056u32);
    emu.sltru_no_count(15usize, 14usize, 15usize, 2141060u32);
    emu.adr_no_count(15usize, 28usize, 15usize, 2141064u32);
    emu.mulhu_no_count(28usize, 6usize, 13usize, 2141068u32);
    emu.adr_no_count(7usize, 5usize, 7usize, 2141072u32);
    emu.sltru_no_count(5usize, 7usize, 5usize, 2141076u32);
    emu.adr_no_count(28usize, 28usize, 5usize, 2141080u32);
    emu.mul_no_count(8usize, 6usize, 13usize, 2141084u32);
    emu.adr_no_count(9usize, 30usize, 14usize, 2141088u32);
    emu.sltru_no_count(14usize, 9usize, 30usize, 2141092u32);
    emu.adr_no_count(14usize, 15usize, 14usize, 2141096u32);
    emu.mul_no_count(19usize, 29usize, 12usize, 2141100u32);
    emu.adr_no_count(5usize, 8usize, 7usize, 2141104u32);
    emu.sltru_no_count(7usize, 5usize, 8usize, 2141108u32);
    emu.adr_no_count(19usize, 9usize, 19usize, 2141112u32);
    emu.sltru_no_count(15usize, 19usize, 9usize, 2141116u32);
    emu.adr_no_count(31usize, 14usize, 31usize, 2141120u32);
    emu.adr_no_count(8usize, 31usize, 15usize, 2141124u32);
    emu.adr_no_count(7usize, 28usize, 7usize, 2141128u32);
    emu.add_memory_rw_events(67usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2141136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020abd0));
    } else {
        emu.pc = 2141132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020abcc));
    }
}
#[inline(always)]
pub fn block_0x0020abcc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(15usize, 8usize, 14usize, 2141136u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2141136u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020abd0));
}
#[inline(never)]
pub fn block_0x0020abd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 31u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(31usize, 5usize, 15usize, 2141140u32);
    emu.sbr_no_count(30usize, 0usize, 6usize, 2141144u32);
    emu.adi_no_count(15usize, 0usize, 4294967295u32, 2141148u32);
    emu.adr_no_count(9usize, 29usize, 29usize, 2141152u32);
    emu.adi_no_count(14usize, 0usize, 4294967294u32, 2141156u32);
    emu.adr_no_count(18usize, 6usize, 6usize, 2141160u32);
    emu.sltru_no_count(28usize, 31usize, 5usize, 2141164u32);
    emu.mulhu_no_count(5usize, 29usize, 15usize, 2141168u32);
    emu.mulhu_no_count(20usize, 6usize, 15usize, 2141172u32);
    emu.sbr_no_count(21usize, 0usize, 9usize, 2141176u32);
    emu.adr_no_count(28usize, 7usize, 28usize, 2141180u32);
    emu.mulhu_no_count(22usize, 29usize, 14usize, 2141184u32);
    emu.sbr_no_count(7usize, 5usize, 6usize, 2141188u32);
    emu.sltru_no_count(5usize, 7usize, 30usize, 2141192u32);
    emu.adr_no_count(5usize, 20usize, 5usize, 2141196u32);
    emu.sbr_no_count(20usize, 0usize, 18usize, 2141200u32);
    emu.sbr_no_count(9usize, 7usize, 9usize, 2141204u32);
    emu.sltru_no_count(30usize, 9usize, 21usize, 2141208u32);
    emu.adr_no_count(30usize, 22usize, 30usize, 2141212u32);
    emu.mulhu_no_count(21usize, 6usize, 14usize, 2141216u32);
    emu.adr_no_count(22usize, 5usize, 30usize, 2141220u32);
    emu.sbr_no_count(30usize, 22usize, 18usize, 2141224u32);
    emu.sltru_no_count(18usize, 22usize, 5usize, 2141228u32);
    emu.adr_no_count(21usize, 21usize, 18usize, 2141232u32);
    emu.sbr_no_count(18usize, 31usize, 29usize, 2141236u32);
    emu.sltru_no_count(31usize, 18usize, 31usize, 2141240u32);
    emu.adr_no_count(9usize, 28usize, 9usize, 2141244u32);
    emu.sltru_no_count(20usize, 30usize, 20usize, 2141248u32);
    emu.adr_no_count(9usize, 9usize, 31usize, 2141252u32);
    emu.adr_no_count(20usize, 21usize, 20usize, 2141256u32);
    emu.add_memory_rw_events(30usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2141264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ac50));
    } else {
        emu.pc = 2141260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ac4c));
    }
}
#[inline(always)]
pub fn block_0x0020ac4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(31usize, 9usize, 28usize, 2141264u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2141264u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ac50));
}
#[inline(always)]
pub fn block_0x0020ac50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(28usize, 30usize, 31usize, 2141268u32);
    emu.sltru_no_count(30usize, 28usize, 30usize, 2141272u32);
    emu.adr_no_count(20usize, 20usize, 30usize, 2141276u32);
    emu.sbr_no_count(31usize, 28usize, 29usize, 2141280u32);
    emu.sltru_no_count(28usize, 31usize, 28usize, 2141284u32);
    emu.adr_no_count(7usize, 20usize, 7usize, 2141288u32);
    emu.adr_no_count(30usize, 7usize, 28usize, 2141292u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(30usize);
    if a == b {
        emu.pc = 2141300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ac74));
    } else {
        emu.pc = 2141296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ac70));
    }
}
#[inline(always)]
pub fn block_0x0020ac70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 30usize, 20usize, 2141300u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2141300u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ac74));
}
#[inline(always)]
pub fn block_0x0020ac74(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(28usize, 5usize, 28usize, 2141304u32);
    emu.sltru_no_count(7usize, 28usize, 5usize, 2141308u32);
    emu.adr_no_count(29usize, 28usize, 29usize, 2141312u32);
    emu.sltru_no_count(5usize, 29usize, 28usize, 2141316u32);
    emu.adr_no_count(6usize, 7usize, 6usize, 2141320u32);
    emu.adr_no_count(28usize, 6usize, 5usize, 2141324u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2141332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ac94));
    } else {
        emu.pc = 2141328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ac90));
    }
}
#[inline(always)]
pub fn block_0x0020ac90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 28usize, 7usize, 2141332u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2141332u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ac94));
}
#[inline(never)]
pub fn block_0x0020ac94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(7usize, 11usize, 8u32, 2141336u32)?;
    emu.lw_no_count(6usize, 11usize, 12u32, 2141340u32)?;
    emu.ani_no_count(19usize, 19usize, 4294967294u32, 2141344u32);
    emu.mulhu_no_count(21usize, 7usize, 17usize, 2141348u32);
    emu.mul_no_count(22usize, 6usize, 17usize, 2141352u32);
    emu.mulhu_no_count(23usize, 6usize, 17usize, 2141356u32);
    emu.mul_no_count(24usize, 7usize, 16usize, 2141360u32);
    emu.mul_no_count(20usize, 7usize, 17usize, 2141364u32);
    emu.adr_no_count(20usize, 19usize, 20usize, 2141368u32);
    emu.sltru_no_count(20usize, 20usize, 19usize, 2141372u32);
    emu.mulhu_no_count(19usize, 7usize, 16usize, 2141376u32);
    emu.adr_no_count(21usize, 22usize, 21usize, 2141380u32);
    emu.sltru_no_count(22usize, 21usize, 22usize, 2141384u32);
    emu.adr_no_count(22usize, 23usize, 22usize, 2141388u32);
    emu.mul_no_count(23usize, 6usize, 16usize, 2141392u32);
    emu.adr_no_count(21usize, 24usize, 21usize, 2141396u32);
    emu.sltru_no_count(24usize, 21usize, 24usize, 2141400u32);
    emu.adr_no_count(19usize, 19usize, 24usize, 2141404u32);
    emu.mulhu_no_count(24usize, 6usize, 16usize, 2141408u32);
    emu.adr_no_count(21usize, 8usize, 21usize, 2141412u32);
    emu.adr_no_count(19usize, 22usize, 19usize, 2141416u32);
    emu.sltru_no_count(22usize, 19usize, 22usize, 2141420u32);
    emu.adr_no_count(19usize, 23usize, 19usize, 2141424u32);
    emu.sltru_no_count(23usize, 19usize, 23usize, 2141428u32);
    emu.adr_no_count(24usize, 24usize, 22usize, 2141432u32);
    emu.adr_no_count(22usize, 21usize, 20usize, 2141436u32);
    emu.adr_no_count(21usize, 24usize, 23usize, 2141440u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a == b {
        emu.pc = 2141448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ad08));
    } else {
        emu.pc = 2141444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ad04));
    }
}
#[inline(always)]
pub fn block_0x0020ad04(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(20usize, 22usize, 8usize, 2141448u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2141448u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ad08));
}
#[inline(never)]
pub fn block_0x0020ad08(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(20usize, 19usize, 20usize, 2141452u32);
    emu.mulhu_no_count(22usize, 7usize, 12usize, 2141456u32);
    emu.mul_no_count(23usize, 6usize, 12usize, 2141460u32);
    emu.mulhu_no_count(24usize, 6usize, 12usize, 2141464u32);
    emu.mul_no_count(25usize, 7usize, 13usize, 2141468u32);
    emu.sltru_no_count(8usize, 20usize, 19usize, 2141472u32);
    emu.adr_no_count(8usize, 21usize, 8usize, 2141476u32);
    emu.mulhu_no_count(19usize, 7usize, 13usize, 2141480u32);
    emu.adr_no_count(22usize, 23usize, 22usize, 2141484u32);
    emu.sltru_no_count(21usize, 22usize, 23usize, 2141488u32);
    emu.adr_no_count(21usize, 24usize, 21usize, 2141492u32);
    emu.mul_no_count(23usize, 6usize, 13usize, 2141496u32);
    emu.adr_no_count(22usize, 25usize, 22usize, 2141500u32);
    emu.sltru_no_count(24usize, 22usize, 25usize, 2141504u32);
    emu.adr_no_count(19usize, 19usize, 24usize, 2141508u32);
    emu.mulhu_no_count(24usize, 6usize, 13usize, 2141512u32);
    emu.adr_no_count(19usize, 21usize, 19usize, 2141516u32);
    emu.sltru_no_count(21usize, 19usize, 21usize, 2141520u32);
    emu.adr_no_count(24usize, 24usize, 21usize, 2141524u32);
    emu.mul_no_count(21usize, 7usize, 12usize, 2141528u32);
    emu.adr_no_count(21usize, 18usize, 21usize, 2141532u32);
    emu.sltru_no_count(18usize, 21usize, 18usize, 2141536u32);
    emu.adr_no_count(25usize, 22usize, 18usize, 2141540u32);
    emu.adr_no_count(22usize, 23usize, 19usize, 2141544u32);
    emu.sltru_no_count(23usize, 22usize, 23usize, 2141548u32);
    emu.adr_no_count(19usize, 9usize, 25usize, 2141552u32);
    emu.adr_no_count(23usize, 24usize, 23usize, 2141556u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2141564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ad7c));
    } else {
        emu.pc = 2141560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ad78));
    }
}
#[inline(always)]
pub fn block_0x0020ad78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(18usize, 19usize, 9usize, 2141564u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2141564u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ad7c));
}
#[inline(always)]
pub fn block_0x0020ad7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(18usize, 22usize, 18usize, 2141568u32);
    emu.adr_no_count(8usize, 19usize, 8usize, 2141572u32);
    emu.adr_no_count(20usize, 21usize, 20usize, 2141576u32);
    emu.sltru_no_count(22usize, 18usize, 22usize, 2141580u32);
    emu.sltru_no_count(21usize, 20usize, 21usize, 2141584u32);
    emu.adr_no_count(9usize, 8usize, 21usize, 2141588u32);
    emu.adr_no_count(8usize, 23usize, 22usize, 2141592u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2141600u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ada0));
    } else {
        emu.pc = 2141596u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ad9c));
    }
}
#[inline(always)]
pub fn block_0x0020ad9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(21usize, 9usize, 19usize, 2141600u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2141600u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ada0));
}
#[inline(never)]
pub fn block_0x0020ada0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 29u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(19usize, 18usize, 21usize, 2141604u32);
    emu.sbr_no_count(22usize, 0usize, 6usize, 2141608u32);
    emu.mulhu_no_count(21usize, 7usize, 15usize, 2141612u32);
    emu.mulhu_no_count(23usize, 6usize, 15usize, 2141616u32);
    emu.adr_no_count(24usize, 7usize, 7usize, 2141620u32);
    emu.mulhu_no_count(25usize, 7usize, 14usize, 2141624u32);
    emu.adr_no_count(26usize, 6usize, 6usize, 2141628u32);
    emu.sltru_no_count(18usize, 19usize, 18usize, 2141632u32);
    emu.adr_no_count(18usize, 8usize, 18usize, 2141636u32);
    emu.mulhu_no_count(27usize, 6usize, 14usize, 2141640u32);
    emu.sbr_no_count(21usize, 21usize, 6usize, 2141644u32);
    emu.sltru_no_count(8usize, 21usize, 22usize, 2141648u32);
    emu.adr_no_count(8usize, 23usize, 8usize, 2141652u32);
    emu.sbr_no_count(22usize, 0usize, 24usize, 2141656u32);
    emu.sbr_no_count(24usize, 21usize, 24usize, 2141660u32);
    emu.sltru_no_count(22usize, 24usize, 22usize, 2141664u32);
    emu.adr_no_count(22usize, 25usize, 22usize, 2141668u32);
    emu.adr_no_count(23usize, 8usize, 22usize, 2141672u32);
    emu.sltru_no_count(22usize, 23usize, 8usize, 2141676u32);
    emu.adr_no_count(27usize, 27usize, 22usize, 2141680u32);
    emu.sbr_no_count(22usize, 31usize, 7usize, 2141684u32);
    emu.sbr_no_count(23usize, 23usize, 26usize, 2141688u32);
    emu.sbr_no_count(26usize, 0usize, 26usize, 2141692u32);
    emu.sltru_no_count(25usize, 22usize, 31usize, 2141696u32);
    emu.adr_no_count(31usize, 24usize, 25usize, 2141700u32);
    emu.sltru_no_count(24usize, 23usize, 26usize, 2141704u32);
    emu.adr_no_count(31usize, 30usize, 31usize, 2141708u32);
    emu.adr_no_count(24usize, 27usize, 24usize, 2141712u32);
    emu.add_memory_rw_events(28usize);
    let b = emu.read_reg_b_tracked(30usize);
    let a = emu.read_reg_a_tracked(31usize);
    if a == b {
        emu.pc = 2141720u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ae18));
    } else {
        emu.pc = 2141716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ae14));
    }
}
#[inline(always)]
pub fn block_0x0020ae14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(25usize, 31usize, 30usize, 2141720u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2141720u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ae18));
}
#[inline(always)]
pub fn block_0x0020ae18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(30usize, 23usize, 25usize, 2141724u32);
    emu.adr_no_count(18usize, 31usize, 18usize, 2141728u32);
    emu.adr_no_count(19usize, 22usize, 19usize, 2141732u32);
    emu.sltru_no_count(23usize, 30usize, 23usize, 2141736u32);
    emu.sltru_no_count(22usize, 19usize, 22usize, 2141740u32);
    emu.adr_no_count(18usize, 18usize, 22usize, 2141744u32);
    emu.adr_no_count(24usize, 24usize, 23usize, 2141748u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2141756u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ae3c));
    } else {
        emu.pc = 2141752u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ae38));
    }
}
#[inline(always)]
pub fn block_0x0020ae38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(22usize, 18usize, 31usize, 2141756u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2141756u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ae3c));
}
#[inline(always)]
pub fn block_0x0020ae3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(31usize, 30usize, 22usize, 2141760u32);
    emu.sbr_no_count(22usize, 29usize, 7usize, 2141764u32);
    emu.sltru_no_count(23usize, 31usize, 30usize, 2141768u32);
    emu.sltru_no_count(30usize, 22usize, 29usize, 2141772u32);
    emu.adr_no_count(29usize, 28usize, 21usize, 2141776u32);
    emu.adr_no_count(29usize, 29usize, 30usize, 2141780u32);
    emu.adr_no_count(24usize, 24usize, 23usize, 2141784u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(29usize);
    if a == b {
        emu.pc = 2141792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ae60));
    } else {
        emu.pc = 2141788u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ae5c));
    }
}
#[inline(always)]
pub fn block_0x0020ae5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(30usize, 29usize, 28usize, 2141792u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2141792u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ae60));
}
#[inline(always)]
pub fn block_0x0020ae60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(28usize, 8usize, 30usize, 2141796u32);
    emu.adr_no_count(30usize, 29usize, 24usize, 2141800u32);
    emu.adr_no_count(31usize, 22usize, 31usize, 2141804u32);
    emu.sltru_no_count(22usize, 31usize, 22usize, 2141808u32);
    emu.adr_no_count(30usize, 30usize, 22usize, 2141812u32);
    emu.sltru_no_count(21usize, 28usize, 8usize, 2141816u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(30usize);
    if a == b {
        emu.pc = 2141824u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ae80));
    } else {
        emu.pc = 2141820u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ae7c));
    }
}
#[inline(always)]
pub fn block_0x0020ae7c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(22usize, 30usize, 29usize, 2141824u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2141824u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ae80));
}
#[inline]
pub fn block_0x0020ae80(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(22usize, 28usize, 22usize, 2141828u32);
    emu.adr_no_count(7usize, 5usize, 7usize, 2141832u32);
    emu.sltru_no_count(29usize, 22usize, 28usize, 2141836u32);
    emu.sltru_no_count(8usize, 7usize, 5usize, 2141840u32);
    emu.adr_no_count(28usize, 7usize, 22usize, 2141844u32);
    emu.adr_no_count(29usize, 21usize, 29usize, 2141848u32);
    emu.adr_no_count(21usize, 6usize, 8usize, 2141852u32);
    emu.adr_no_count(5usize, 21usize, 29usize, 2141856u32);
    emu.sltru_no_count(29usize, 28usize, 7usize, 2141860u32);
    emu.adr_no_count(7usize, 5usize, 29usize, 2141864u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a == b {
        emu.pc = 2141872u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aeb0));
    } else {
        emu.pc = 2141868u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aeac));
    }
}
#[inline(always)]
pub fn block_0x0020aeac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(29usize, 7usize, 21usize, 2141872u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2141872u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020aeb0));
}
#[inline(never)]
pub fn block_0x0020aeb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(6usize, 11usize, 16u32, 2141876u32)?;
    emu.lw_no_count(5usize, 11usize, 20u32, 2141880u32)?;
    emu.ani_no_count(20usize, 20usize, 4294967294u32, 2141884u32);
    emu.mulhu_no_count(22usize, 6usize, 17usize, 2141888u32);
    emu.mul_no_count(23usize, 5usize, 17usize, 2141892u32);
    emu.mulhu_no_count(24usize, 5usize, 17usize, 2141896u32);
    emu.mul_no_count(25usize, 6usize, 16usize, 2141900u32);
    emu.mul_no_count(26usize, 6usize, 17usize, 2141904u32);
    emu.adr_no_count(26usize, 20usize, 26usize, 2141908u32);
    emu.sltru_no_count(20usize, 26usize, 20usize, 2141912u32);
    emu.mulhu_no_count(26usize, 6usize, 16usize, 2141916u32);
    emu.adr_no_count(22usize, 23usize, 22usize, 2141920u32);
    emu.sltru_no_count(23usize, 22usize, 23usize, 2141924u32);
    emu.adr_no_count(23usize, 24usize, 23usize, 2141928u32);
    emu.mul_no_count(24usize, 5usize, 16usize, 2141932u32);
    emu.adr_no_count(22usize, 25usize, 22usize, 2141936u32);
    emu.sltru_no_count(25usize, 22usize, 25usize, 2141940u32);
    emu.adr_no_count(25usize, 26usize, 25usize, 2141944u32);
    emu.mulhu_no_count(26usize, 5usize, 16usize, 2141948u32);
    emu.adr_no_count(27usize, 22usize, 20usize, 2141952u32);
    emu.adr_no_count(22usize, 23usize, 25usize, 2141956u32);
    emu.sltru_no_count(23usize, 22usize, 23usize, 2141960u32);
    emu.adr_no_count(22usize, 24usize, 22usize, 2141964u32);
    emu.sltru_no_count(25usize, 22usize, 24usize, 2141968u32);
    emu.adr_no_count(23usize, 26usize, 23usize, 2141972u32);
    emu.adr_no_count(24usize, 9usize, 27usize, 2141976u32);
    emu.adr_no_count(23usize, 23usize, 25usize, 2141980u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(24usize);
    if a == b {
        emu.pc = 2141988u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020af24));
    } else {
        emu.pc = 2141984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020af20));
    }
}
#[inline(always)]
pub fn block_0x0020af20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(20usize, 24usize, 9usize, 2141988u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2141988u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020af24));
}
#[inline(never)]
pub fn block_0x0020af24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(20usize, 22usize, 20usize, 2141992u32);
    emu.mulhu_no_count(24usize, 6usize, 12usize, 2141996u32);
    emu.mul_no_count(25usize, 5usize, 12usize, 2142000u32);
    emu.mulhu_no_count(26usize, 5usize, 12usize, 2142004u32);
    emu.mul_no_count(27usize, 6usize, 13usize, 2142008u32);
    emu.sltru_no_count(9usize, 20usize, 22usize, 2142012u32);
    emu.adr_no_count(9usize, 23usize, 9usize, 2142016u32);
    emu.mulhu_no_count(22usize, 6usize, 13usize, 2142020u32);
    emu.adr_no_count(24usize, 25usize, 24usize, 2142024u32);
    emu.sltru_no_count(23usize, 24usize, 25usize, 2142028u32);
    emu.adr_no_count(23usize, 26usize, 23usize, 2142032u32);
    emu.mul_no_count(26usize, 5usize, 13usize, 2142036u32);
    emu.adr_no_count(24usize, 27usize, 24usize, 2142040u32);
    emu.sltru_no_count(25usize, 24usize, 27usize, 2142044u32);
    emu.adr_no_count(22usize, 22usize, 25usize, 2142048u32);
    emu.mulhu_no_count(25usize, 5usize, 13usize, 2142052u32);
    emu.adr_no_count(27usize, 23usize, 22usize, 2142056u32);
    emu.sltru_no_count(22usize, 27usize, 23usize, 2142060u32);
    emu.adr_no_count(1usize, 25usize, 22usize, 2142064u32);
    emu.mul_no_count(22usize, 6usize, 12usize, 2142068u32);
    emu.adr_no_count(22usize, 19usize, 22usize, 2142072u32);
    emu.sltru_no_count(25usize, 22usize, 19usize, 2142076u32);
    emu.adr_no_count(19usize, 24usize, 25usize, 2142080u32);
    emu.adr_no_count(23usize, 26usize, 27usize, 2142084u32);
    emu.sltru_no_count(24usize, 23usize, 26usize, 2142088u32);
    emu.adr_no_count(19usize, 18usize, 19usize, 2142092u32);
    emu.adr_no_count(24usize, 1usize, 24usize, 2142096u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2142104u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020af98));
    } else {
        emu.pc = 2142100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020af94));
    }
}
#[inline(always)]
pub fn block_0x0020af94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(25usize, 19usize, 18usize, 2142104u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2142104u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020af98));
}
#[inline]
pub fn block_0x0020af98(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(18usize, 0usize, 21usize, 2142108u32);
    emu.adr_no_count(21usize, 23usize, 25usize, 2142112u32);
    emu.adr_no_count(9usize, 19usize, 9usize, 2142116u32);
    emu.adr_no_count(20usize, 22usize, 20usize, 2142120u32);
    emu.sltru_no_count(23usize, 21usize, 23usize, 2142124u32);
    emu.sltru_no_count(22usize, 20usize, 22usize, 2142128u32);
    emu.adr_no_count(9usize, 9usize, 22usize, 2142132u32);
    emu.adr_no_count(23usize, 24usize, 23usize, 2142136u32);
    emu.adi_no_count(15usize, 0usize, 4294967295u32, 2142140u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2142148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020afc4));
    } else {
        emu.pc = 2142144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020afc0));
    }
}
#[inline(always)]
pub fn block_0x0020afc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(22usize, 9usize, 19usize, 2142148u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2142148u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020afc4));
}
#[inline(never)]
pub fn block_0x0020afc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 31u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 18usize, 4294967295u32, 2142152u32);
    emu.adr_no_count(19usize, 21usize, 22usize, 2142156u32);
    emu.sbr_no_count(24usize, 0usize, 5usize, 2142160u32);
    emu.mulhu_no_count(22usize, 6usize, 15usize, 2142164u32);
    emu.mulhu_no_count(25usize, 5usize, 15usize, 2142168u32);
    emu.adr_no_count(26usize, 6usize, 6usize, 2142172u32);
    emu.mulhu_no_count(27usize, 6usize, 14usize, 2142176u32);
    emu.adr_no_count(1usize, 5usize, 5usize, 2142180u32);
    emu.sltru_no_count(21usize, 19usize, 21usize, 2142184u32);
    emu.adr_no_count(23usize, 23usize, 21usize, 2142188u32);
    emu.adi_no_count(14usize, 0usize, 4294967294u32, 2142192u32);
    emu.mulhu_no_count(14usize, 5usize, 14usize, 2142196u32);
    emu.sbr_no_count(22usize, 22usize, 5usize, 2142200u32);
    emu.sltru_no_count(21usize, 22usize, 24usize, 2142204u32);
    emu.adr_no_count(21usize, 25usize, 21usize, 2142208u32);
    emu.sbr_no_count(24usize, 0usize, 26usize, 2142212u32);
    emu.sbr_no_count(15usize, 22usize, 26usize, 2142216u32);
    emu.sltru_no_count(24usize, 15usize, 24usize, 2142220u32);
    emu.adr_no_count(24usize, 27usize, 24usize, 2142224u32);
    emu.adr_no_count(25usize, 21usize, 24usize, 2142228u32);
    emu.sltru_no_count(24usize, 25usize, 21usize, 2142232u32);
    emu.adr_no_count(14usize, 14usize, 24usize, 2142236u32);
    emu.sbr_no_count(24usize, 31usize, 6usize, 2142240u32);
    emu.sbr_no_count(25usize, 25usize, 1usize, 2142244u32);
    emu.sbr_no_count(27usize, 0usize, 1usize, 2142248u32);
    emu.sltru_no_count(26usize, 24usize, 31usize, 2142252u32);
    emu.adr_no_count(15usize, 15usize, 26usize, 2142256u32);
    emu.sltru_no_count(27usize, 25usize, 27usize, 2142260u32);
    emu.adr_no_count(31usize, 30usize, 15usize, 2142264u32);
    emu.adr_no_count(27usize, 14usize, 27usize, 2142268u32);
    emu.add_memory_rw_events(30usize);
    let b = emu.read_reg_b_tracked(30usize);
    let a = emu.read_reg_a_tracked(31usize);
    if a == b {
        emu.pc = 2142276u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b044));
    } else {
        emu.pc = 2142272u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b040));
    }
}
#[inline(always)]
pub fn block_0x0020b040(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(26usize, 31usize, 30usize, 2142276u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2142276u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b044));
}
#[inline]
pub fn block_0x0020b044(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(30usize, 18usize, 8usize, 2142280u32);
    emu.adr_no_count(26usize, 25usize, 26usize, 2142284u32);
    emu.adr_no_count(18usize, 31usize, 23usize, 2142288u32);
    emu.adr_no_count(19usize, 24usize, 19usize, 2142292u32);
    emu.sltru_no_count(23usize, 26usize, 25usize, 2142296u32);
    emu.sltru_no_count(8usize, 19usize, 24usize, 2142300u32);
    emu.adr_no_count(18usize, 18usize, 8usize, 2142304u32);
    emu.adr_no_count(23usize, 27usize, 23usize, 2142308u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2142316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b06c));
    } else {
        emu.pc = 2142312u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b068));
    }
}
#[inline(always)]
pub fn block_0x0020b068(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(8usize, 18usize, 31usize, 2142316u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2142316u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b06c));
}
#[inline]
pub fn block_0x0020b06c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 26usize, 8usize, 2142320u32);
    emu.sltru_no_count(14usize, 8usize, 26usize, 2142324u32);
    emu.adr_no_count(23usize, 23usize, 14usize, 2142328u32);
    emu.sbr_no_count(31usize, 28usize, 6usize, 2142332u32);
    emu.adr_no_count(14usize, 7usize, 22usize, 2142336u32);
    emu.sltru_no_count(22usize, 31usize, 28usize, 2142340u32);
    emu.adr_no_count(28usize, 14usize, 22usize, 2142344u32);
    emu.adr_no_count(29usize, 30usize, 29usize, 2142348u32);
    emu.adi_no_count(1usize, 0usize, 4294967295u32, 2142352u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2142360u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b098));
    } else {
        emu.pc = 2142356u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b094));
    }
}
#[inline(always)]
pub fn block_0x0020b094(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(22usize, 28usize, 7usize, 2142360u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2142360u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b098));
}
#[inline(always)]
pub fn block_0x0020b098(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(7usize, 29usize, 30usize, 2142364u32);
    emu.adr_no_count(30usize, 21usize, 22usize, 2142368u32);
    emu.adr_no_count(23usize, 28usize, 23usize, 2142372u32);
    emu.adr_no_count(8usize, 31usize, 8usize, 2142376u32);
    emu.sltru_no_count(22usize, 8usize, 31usize, 2142380u32);
    emu.adr_no_count(31usize, 23usize, 22usize, 2142384u32);
    emu.sltru_no_count(21usize, 30usize, 21usize, 2142388u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(31usize);
    if a == b {
        emu.pc = 2142396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b0bc));
    } else {
        emu.pc = 2142392u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b0b8));
    }
}
#[inline(always)]
pub fn block_0x0020b0b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(22usize, 31usize, 28usize, 2142396u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2142396u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b0bc));
}
#[inline(always)]
pub fn block_0x0020b0bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(28usize, 30usize, 22usize, 2142400u32);
    emu.adr_no_count(22usize, 29usize, 6usize, 2142404u32);
    emu.sltru_no_count(14usize, 28usize, 30usize, 2142408u32);
    emu.sltru_no_count(6usize, 22usize, 29usize, 2142412u32);
    emu.adr_no_count(5usize, 7usize, 5usize, 2142416u32);
    emu.adr_no_count(5usize, 5usize, 6usize, 2142420u32);
    emu.adr_no_count(21usize, 21usize, 14usize, 2142424u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2142432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b0e0));
    } else {
        emu.pc = 2142428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b0dc));
    }
}
#[inline(always)]
pub fn block_0x0020b0dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(6usize, 5usize, 7usize, 2142432u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2142432u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b0e0));
}
#[inline(always)]
pub fn block_0x0020b0e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(29usize, 22usize, 28usize, 2142436u32);
    emu.sltru_no_count(30usize, 29usize, 22usize, 2142440u32);
    emu.adr_no_count(28usize, 5usize, 21usize, 2142444u32);
    emu.adr_no_count(28usize, 28usize, 30usize, 2142448u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2142456u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b0f8));
    } else {
        emu.pc = 2142452u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b0f4));
    }
}
#[inline(always)]
pub fn block_0x0020b0f4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(30usize, 28usize, 5usize, 2142456u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2142456u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b0f8));
}
#[inline(never)]
pub fn block_0x0020b0f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(5usize, 11usize, 24u32, 2142460u32)?;
    emu.lw_no_count(7usize, 11usize, 28u32, 2142464u32)?;
    emu.ani_no_count(14usize, 20usize, 4294967294u32, 2142468u32);
    emu.mulhu_no_count(15usize, 5usize, 17usize, 2142472u32);
    emu.mul_no_count(21usize, 7usize, 17usize, 2142476u32);
    emu.mulhu_no_count(22usize, 7usize, 17usize, 2142480u32);
    emu.mul_no_count(23usize, 5usize, 16usize, 2142484u32);
    emu.mul_no_count(20usize, 5usize, 17usize, 2142488u32);
    emu.adr_no_count(20usize, 14usize, 20usize, 2142492u32);
    emu.sltru_no_count(20usize, 20usize, 14usize, 2142496u32);
    emu.mulhu_no_count(14usize, 5usize, 16usize, 2142500u32);
    emu.adr_no_count(15usize, 21usize, 15usize, 2142504u32);
    emu.sltru_no_count(21usize, 15usize, 21usize, 2142508u32);
    emu.adr_no_count(21usize, 22usize, 21usize, 2142512u32);
    emu.mul_no_count(22usize, 7usize, 16usize, 2142516u32);
    emu.adr_no_count(15usize, 23usize, 15usize, 2142520u32);
    emu.sltru_no_count(23usize, 15usize, 23usize, 2142524u32);
    emu.adr_no_count(14usize, 14usize, 23usize, 2142528u32);
    emu.mulhu_no_count(23usize, 7usize, 16usize, 2142532u32);
    emu.adr_no_count(15usize, 15usize, 20usize, 2142536u32);
    emu.adr_no_count(14usize, 21usize, 14usize, 2142540u32);
    emu.sltru_no_count(24usize, 14usize, 21usize, 2142544u32);
    emu.adr_no_count(21usize, 22usize, 14usize, 2142548u32);
    emu.sltru_no_count(22usize, 21usize, 22usize, 2142552u32);
    emu.adr_no_count(24usize, 23usize, 24usize, 2142556u32);
    emu.adr_no_count(23usize, 9usize, 15usize, 2142560u32);
    emu.adr_no_count(22usize, 24usize, 22usize, 2142564u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a == b {
        emu.pc = 2142572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b16c));
    } else {
        emu.pc = 2142568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b168));
    }
}
#[inline(always)]
pub fn block_0x0020b168(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(20usize, 23usize, 9usize, 2142572u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2142572u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b16c));
}
#[inline(never)]
pub fn block_0x0020b16c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(9usize, 21usize, 20usize, 2142576u32);
    emu.mulhu_no_count(14usize, 5usize, 12usize, 2142580u32);
    emu.mul_no_count(15usize, 7usize, 12usize, 2142584u32);
    emu.mulhu_no_count(23usize, 7usize, 12usize, 2142588u32);
    emu.mul_no_count(24usize, 5usize, 13usize, 2142592u32);
    emu.sltru_no_count(20usize, 9usize, 21usize, 2142596u32);
    emu.adr_no_count(20usize, 22usize, 20usize, 2142600u32);
    emu.mulhu_no_count(21usize, 5usize, 13usize, 2142604u32);
    emu.adr_no_count(14usize, 15usize, 14usize, 2142608u32);
    emu.sltru_no_count(15usize, 14usize, 15usize, 2142612u32);
    emu.adr_no_count(15usize, 23usize, 15usize, 2142616u32);
    emu.mul_no_count(23usize, 7usize, 13usize, 2142620u32);
    emu.adr_no_count(14usize, 24usize, 14usize, 2142624u32);
    emu.sltru_no_count(22usize, 14usize, 24usize, 2142628u32);
    emu.adr_no_count(21usize, 21usize, 22usize, 2142632u32);
    emu.mulhu_no_count(22usize, 7usize, 13usize, 2142636u32);
    emu.adr_no_count(24usize, 15usize, 21usize, 2142640u32);
    emu.sltru_no_count(15usize, 24usize, 15usize, 2142644u32);
    emu.adr_no_count(15usize, 22usize, 15usize, 2142648u32);
    emu.mul_no_count(22usize, 5usize, 12usize, 2142652u32);
    emu.adr_no_count(22usize, 19usize, 22usize, 2142656u32);
    emu.sltru_no_count(21usize, 22usize, 19usize, 2142660u32);
    emu.adr_no_count(14usize, 14usize, 21usize, 2142664u32);
    emu.adr_no_count(24usize, 23usize, 24usize, 2142668u32);
    emu.sltru_no_count(25usize, 24usize, 23usize, 2142672u32);
    emu.adr_no_count(23usize, 18usize, 14usize, 2142676u32);
    emu.adr_no_count(25usize, 15usize, 25usize, 2142680u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a == b {
        emu.pc = 2142688u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b1e0));
    } else {
        emu.pc = 2142684u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b1dc));
    }
}
#[inline(always)]
pub fn block_0x0020b1dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(21usize, 23usize, 18usize, 2142688u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2142688u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b1e0));
}
#[inline(always)]
pub fn block_0x0020b1e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(21usize, 24usize, 21usize, 2142692u32);
    emu.adr_no_count(20usize, 23usize, 20usize, 2142696u32);
    emu.adr_no_count(19usize, 22usize, 9usize, 2142700u32);
    emu.sltru_no_count(14usize, 21usize, 24usize, 2142704u32);
    emu.sltru_no_count(18usize, 19usize, 22usize, 2142708u32);
    emu.adr_no_count(9usize, 20usize, 18usize, 2142712u32);
    emu.adr_no_count(22usize, 25usize, 14usize, 2142716u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2142724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b204));
    } else {
        emu.pc = 2142720u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b200));
    }
}
#[inline(always)]
pub fn block_0x0020b200(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(18usize, 9usize, 23usize, 2142724u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2142724u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b204));
}
#[inline(never)]
pub fn block_0x0020b204(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 30u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(18usize, 21usize, 18usize, 2142728u32);
    emu.sbr_no_count(14usize, 0usize, 7usize, 2142732u32);
    emu.mulhu_no_count(15usize, 5usize, 1usize, 2142736u32);
    emu.mulhu_no_count(20usize, 7usize, 1usize, 2142740u32);
    emu.adr_no_count(23usize, 5usize, 5usize, 2142744u32);
    emu.adi_no_count(25usize, 0usize, 4294967294u32, 2142748u32);
    emu.mulhu_no_count(24usize, 5usize, 25usize, 2142752u32);
    emu.adr_no_count(26usize, 7usize, 7usize, 2142756u32);
    emu.sltru_no_count(21usize, 18usize, 21usize, 2142760u32);
    emu.adr_no_count(22usize, 22usize, 21usize, 2142764u32);
    emu.mulhu_no_count(25usize, 7usize, 25usize, 2142768u32);
    emu.sbr_no_count(21usize, 15usize, 7usize, 2142772u32);
    emu.sltru_no_count(14usize, 21usize, 14usize, 2142776u32);
    emu.adr_no_count(20usize, 20usize, 14usize, 2142780u32);
    emu.sbr_no_count(14usize, 0usize, 23usize, 2142784u32);
    emu.sbr_no_count(15usize, 21usize, 23usize, 2142788u32);
    emu.sltru_no_count(14usize, 15usize, 14usize, 2142792u32);
    emu.adr_no_count(14usize, 24usize, 14usize, 2142796u32);
    emu.adr_no_count(14usize, 20usize, 14usize, 2142800u32);
    emu.sltru_no_count(23usize, 14usize, 20usize, 2142804u32);
    emu.adr_no_count(27usize, 25usize, 23usize, 2142808u32);
    emu.sbr_no_count(23usize, 8usize, 5usize, 2142812u32);
    emu.sbr_no_count(25usize, 14usize, 26usize, 2142816u32);
    emu.sbr_no_count(14usize, 0usize, 26usize, 2142820u32);
    emu.sltru_no_count(26usize, 23usize, 8usize, 2142824u32);
    emu.adr_no_count(15usize, 15usize, 26usize, 2142828u32);
    emu.sltru_no_count(14usize, 25usize, 14usize, 2142832u32);
    emu.adr_no_count(24usize, 31usize, 15usize, 2142836u32);
    emu.adr_no_count(27usize, 27usize, 14usize, 2142840u32);
    emu.add_memory_rw_events(29usize);
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(24usize);
    if a == b {
        emu.pc = 2142848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b280));
    } else {
        emu.pc = 2142844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b27c));
    }
}
#[inline(always)]
pub fn block_0x0020b27c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(26usize, 24usize, 31usize, 2142848u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2142848u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b280));
}
#[inline(always)]
pub fn block_0x0020b280(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(26usize, 25usize, 26usize, 2142852u32);
    emu.adr_no_count(8usize, 24usize, 22usize, 2142856u32);
    emu.adr_no_count(18usize, 23usize, 18usize, 2142860u32);
    emu.sltru_no_count(14usize, 26usize, 25usize, 2142864u32);
    emu.sltru_no_count(31usize, 18usize, 23usize, 2142868u32);
    emu.adr_no_count(8usize, 8usize, 31usize, 2142872u32);
    emu.adr_no_count(23usize, 27usize, 14usize, 2142876u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2142884u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b2a4));
    } else {
        emu.pc = 2142880u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b2a0));
    }
}
#[inline(always)]
pub fn block_0x0020b2a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(31usize, 8usize, 24usize, 2142884u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2142884u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b2a4));
}
#[inline]
pub fn block_0x0020b2a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(31usize, 26usize, 31usize, 2142888u32);
    emu.sltru_no_count(14usize, 31usize, 26usize, 2142892u32);
    emu.adr_no_count(23usize, 23usize, 14usize, 2142896u32);
    emu.sbr_no_count(22usize, 29usize, 5usize, 2142900u32);
    emu.adr_no_count(21usize, 28usize, 21usize, 2142904u32);
    emu.sltru_no_count(24usize, 22usize, 29usize, 2142908u32);
    emu.adr_no_count(21usize, 21usize, 24usize, 2142912u32);
    emu.adr_no_count(29usize, 6usize, 30usize, 2142916u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a == b {
        emu.pc = 2142924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b2cc));
    } else {
        emu.pc = 2142920u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b2c8));
    }
}
#[inline(always)]
pub fn block_0x0020b2c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(24usize, 21usize, 28usize, 2142924u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2142924u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b2cc));
}
#[inline(always)]
pub fn block_0x0020b2cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(6usize, 29usize, 6usize, 2142928u32);
    emu.adr_no_count(28usize, 20usize, 24usize, 2142932u32);
    emu.adr_no_count(30usize, 21usize, 23usize, 2142936u32);
    emu.adr_no_count(31usize, 22usize, 31usize, 2142940u32);
    emu.sltru_no_count(22usize, 31usize, 22usize, 2142944u32);
    emu.adr_no_count(30usize, 30usize, 22usize, 2142948u32);
    emu.sltru_no_count(20usize, 28usize, 20usize, 2142952u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(30usize);
    if a == b {
        emu.pc = 2142960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b2f0));
    } else {
        emu.pc = 2142956u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b2ec));
    }
}
#[inline(always)]
pub fn block_0x0020b2ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(22usize, 30usize, 21usize, 2142960u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2142960u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b2f0));
}
#[inline(always)]
pub fn block_0x0020b2f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(21usize, 28usize, 22usize, 2142964u32);
    emu.adr_no_count(23usize, 29usize, 5usize, 2142968u32);
    emu.sltru_no_count(14usize, 21usize, 28usize, 2142972u32);
    emu.sltru_no_count(5usize, 23usize, 29usize, 2142976u32);
    emu.adr_no_count(7usize, 6usize, 7usize, 2142980u32);
    emu.adr_no_count(22usize, 7usize, 5usize, 2142984u32);
    emu.adr_no_count(20usize, 20usize, 14usize, 2142988u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a == b {
        emu.pc = 2142996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b314));
    } else {
        emu.pc = 2142992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b310));
    }
}
#[inline(always)]
pub fn block_0x0020b310(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 22usize, 6usize, 2142996u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2142996u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b314));
}
#[inline(always)]
pub fn block_0x0020b314(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(28usize, 23usize, 21usize, 2143000u32);
    emu.sltru_no_count(29usize, 28usize, 23usize, 2143004u32);
    emu.adr_no_count(7usize, 22usize, 20usize, 2143008u32);
    emu.adr_no_count(7usize, 7usize, 29usize, 2143012u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a == b {
        emu.pc = 2143020u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b32c));
    } else {
        emu.pc = 2143016u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b328));
    }
}
#[inline(always)]
pub fn block_0x0020b328(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(29usize, 7usize, 22usize, 2143020u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2143020u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b32c));
}
#[inline(never)]
pub fn block_0x0020b32c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(6usize, 11usize, 32u32, 2143024u32)?;
    emu.lw_no_count(11usize, 11usize, 36u32, 2143028u32)?;
    emu.ani_no_count(14usize, 19usize, 4294967294u32, 2143032u32);
    emu.mulhu_no_count(15usize, 6usize, 17usize, 2143036u32);
    emu.mul_no_count(20usize, 11usize, 17usize, 2143040u32);
    emu.mulhu_no_count(21usize, 11usize, 17usize, 2143044u32);
    emu.mul_no_count(22usize, 6usize, 16usize, 2143048u32);
    emu.mulhu_no_count(23usize, 6usize, 16usize, 2143052u32);
    emu.mul_no_count(17usize, 6usize, 17usize, 2143056u32);
    emu.adr_no_count(17usize, 14usize, 17usize, 2143060u32);
    emu.sltru_no_count(19usize, 17usize, 14usize, 2143064u32);
    emu.mul_no_count(14usize, 11usize, 16usize, 2143068u32);
    emu.mulhu_no_count(16usize, 11usize, 16usize, 2143072u32);
    emu.adr_no_count(15usize, 20usize, 15usize, 2143076u32);
    emu.sltru_no_count(17usize, 15usize, 20usize, 2143080u32);
    emu.adr_no_count(15usize, 22usize, 15usize, 2143084u32);
    emu.adr_no_count(21usize, 21usize, 17usize, 2143088u32);
    emu.sltru_no_count(17usize, 15usize, 22usize, 2143092u32);
    emu.adr_no_count(15usize, 15usize, 19usize, 2143096u32);
    emu.adr_no_count(17usize, 23usize, 17usize, 2143100u32);
    emu.adr_no_count(20usize, 21usize, 17usize, 2143104u32);
    emu.adr_no_count(17usize, 14usize, 20usize, 2143108u32);
    emu.sltru_no_count(20usize, 20usize, 21usize, 2143112u32);
    emu.sltru_no_count(14usize, 17usize, 14usize, 2143116u32);
    emu.adr_no_count(20usize, 16usize, 20usize, 2143120u32);
    emu.adr_no_count(16usize, 9usize, 15usize, 2143124u32);
    emu.adr_no_count(20usize, 20usize, 14usize, 2143128u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2143136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b3a0));
    } else {
        emu.pc = 2143132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b39c));
    }
}
#[inline(always)]
pub fn block_0x0020b39c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(19usize, 16usize, 9usize, 2143136u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2143136u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b3a0));
}
#[inline(never)]
pub fn block_0x0020b3a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(16usize, 17usize, 19usize, 2143140u32);
    emu.mulhu_no_count(14usize, 6usize, 12usize, 2143144u32);
    emu.mul_no_count(15usize, 11usize, 12usize, 2143148u32);
    emu.mulhu_no_count(9usize, 11usize, 12usize, 2143152u32);
    emu.mul_no_count(19usize, 6usize, 13usize, 2143156u32);
    emu.sltru_no_count(17usize, 16usize, 17usize, 2143160u32);
    emu.adr_no_count(17usize, 20usize, 17usize, 2143164u32);
    emu.mulhu_no_count(20usize, 6usize, 13usize, 2143168u32);
    emu.adr_no_count(14usize, 15usize, 14usize, 2143172u32);
    emu.sltru_no_count(15usize, 14usize, 15usize, 2143176u32);
    emu.adr_no_count(15usize, 9usize, 15usize, 2143180u32);
    emu.mul_no_count(9usize, 11usize, 13usize, 2143184u32);
    emu.mulhu_no_count(21usize, 11usize, 13usize, 2143188u32);
    emu.mul_no_count(13usize, 6usize, 12usize, 2143192u32);
    emu.adr_no_count(13usize, 18usize, 13usize, 2143196u32);
    emu.adr_no_count(14usize, 19usize, 14usize, 2143200u32);
    emu.sltru_no_count(12usize, 13usize, 18usize, 2143204u32);
    emu.sltru_no_count(18usize, 14usize, 19usize, 2143208u32);
    emu.adr_no_count(14usize, 14usize, 12usize, 2143212u32);
    emu.adr_no_count(18usize, 20usize, 18usize, 2143216u32);
    emu.adr_no_count(19usize, 15usize, 18usize, 2143220u32);
    emu.adr_no_count(18usize, 9usize, 19usize, 2143224u32);
    emu.sltru_no_count(15usize, 19usize, 15usize, 2143228u32);
    emu.sltru_no_count(19usize, 18usize, 9usize, 2143232u32);
    emu.adr_no_count(15usize, 21usize, 15usize, 2143236u32);
    emu.adr_no_count(9usize, 8usize, 14usize, 2143240u32);
    emu.adr_no_count(19usize, 15usize, 19usize, 2143244u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2143252u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b414));
    } else {
        emu.pc = 2143248u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b410));
    }
}
#[inline(always)]
pub fn block_0x0020b410(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(12usize, 9usize, 8usize, 2143252u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2143252u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b414));
}
#[inline(always)]
pub fn block_0x0020b414(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 18usize, 12usize, 2143256u32);
    emu.adr_no_count(14usize, 9usize, 17usize, 2143260u32);
    emu.adr_no_count(12usize, 13usize, 16usize, 2143264u32);
    emu.sltru_no_count(16usize, 8usize, 18usize, 2143268u32);
    emu.sltru_no_count(17usize, 12usize, 13usize, 2143272u32);
    emu.adr_no_count(13usize, 14usize, 17usize, 2143276u32);
    emu.adr_no_count(16usize, 19usize, 16usize, 2143280u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2143288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b438));
    } else {
        emu.pc = 2143284u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b434));
    }
}
#[inline(always)]
pub fn block_0x0020b434(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(17usize, 13usize, 9usize, 2143288u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2143288u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b438));
}
#[inline(never)]
pub fn block_0x0020b438(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 30u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(17usize, 8usize, 17usize, 2143292u32);
    emu.sbr_no_count(14usize, 0usize, 11usize, 2143296u32);
    emu.mulhu_no_count(9usize, 6usize, 1usize, 2143300u32);
    emu.mulhu_no_count(18usize, 11usize, 1usize, 2143304u32);
    emu.adr_no_count(19usize, 6usize, 6usize, 2143308u32);
    emu.adi_no_count(22usize, 0usize, 4294967294u32, 2143312u32);
    emu.mulhu_no_count(20usize, 6usize, 22usize, 2143316u32);
    emu.sltru_no_count(15usize, 17usize, 8usize, 2143320u32);
    emu.adr_no_count(15usize, 16usize, 15usize, 2143324u32);
    emu.adr_no_count(21usize, 11usize, 11usize, 2143328u32);
    emu.mulhu_no_count(22usize, 11usize, 22usize, 2143332u32);
    emu.sbr_no_count(8usize, 9usize, 11usize, 2143336u32);
    emu.sltru_no_count(16usize, 8usize, 14usize, 2143340u32);
    emu.adr_no_count(16usize, 18usize, 16usize, 2143344u32);
    emu.sbr_no_count(14usize, 0usize, 19usize, 2143348u32);
    emu.sbr_no_count(19usize, 8usize, 19usize, 2143352u32);
    emu.sltru_no_count(14usize, 19usize, 14usize, 2143356u32);
    emu.adr_no_count(14usize, 20usize, 14usize, 2143360u32);
    emu.adr_no_count(14usize, 16usize, 14usize, 2143364u32);
    emu.sltru_no_count(9usize, 14usize, 16usize, 2143368u32);
    emu.adr_no_count(22usize, 22usize, 9usize, 2143372u32);
    emu.sbr_no_count(9usize, 31usize, 6usize, 2143376u32);
    emu.sbr_no_count(18usize, 14usize, 21usize, 2143380u32);
    emu.sbr_no_count(20usize, 0usize, 21usize, 2143384u32);
    emu.sltru_no_count(14usize, 9usize, 31usize, 2143388u32);
    emu.adr_no_count(31usize, 19usize, 14usize, 2143392u32);
    emu.sltru_no_count(19usize, 18usize, 20usize, 2143396u32);
    emu.adr_no_count(31usize, 30usize, 31usize, 2143400u32);
    emu.adr_no_count(19usize, 22usize, 19usize, 2143404u32);
    emu.add_memory_rw_events(29usize);
    let b = emu.read_reg_b_tracked(30usize);
    let a = emu.read_reg_a_tracked(31usize);
    if a == b {
        emu.pc = 2143412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b4b4));
    } else {
        emu.pc = 2143408u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b4b0));
    }
}
#[inline(always)]
pub fn block_0x0020b4b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(14usize, 31usize, 30usize, 2143412u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2143412u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b4b4));
}
#[inline(always)]
pub fn block_0x0020b4b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(30usize, 18usize, 14usize, 2143416u32);
    emu.adr_no_count(15usize, 31usize, 15usize, 2143420u32);
    emu.adr_no_count(14usize, 9usize, 17usize, 2143424u32);
    emu.sltru_no_count(18usize, 30usize, 18usize, 2143428u32);
    emu.sltru_no_count(17usize, 14usize, 9usize, 2143432u32);
    emu.adr_no_count(15usize, 15usize, 17usize, 2143436u32);
    emu.adr_no_count(9usize, 19usize, 18usize, 2143440u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2143448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b4d8));
    } else {
        emu.pc = 2143444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b4d4));
    }
}
#[inline(always)]
pub fn block_0x0020b4d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(17usize, 15usize, 31usize, 2143448u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2143448u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b4d8));
}
#[inline]
pub fn block_0x0020b4d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(17usize, 30usize, 17usize, 2143452u32);
    emu.sltru_no_count(30usize, 17usize, 30usize, 2143456u32);
    emu.adr_no_count(9usize, 9usize, 30usize, 2143460u32);
    emu.sbr_no_count(30usize, 28usize, 6usize, 2143464u32);
    emu.adr_no_count(8usize, 7usize, 8usize, 2143468u32);
    emu.sltru_no_count(31usize, 30usize, 28usize, 2143472u32);
    emu.adr_no_count(28usize, 8usize, 31usize, 2143476u32);
    emu.adr_no_count(29usize, 5usize, 29usize, 2143480u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2143488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b500));
    } else {
        emu.pc = 2143484u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b4fc));
    }
}
#[inline(always)]
pub fn block_0x0020b4fc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(31usize, 28usize, 7usize, 2143488u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2143488u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b500));
}
#[inline(always)]
pub fn block_0x0020b500(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(7usize, 29usize, 5usize, 2143492u32);
    emu.adr_no_count(31usize, 16usize, 31usize, 2143496u32);
    emu.adr_no_count(5usize, 28usize, 9usize, 2143500u32);
    emu.adr_no_count(17usize, 30usize, 17usize, 2143504u32);
    emu.sltru_no_count(8usize, 17usize, 30usize, 2143508u32);
    emu.adr_no_count(5usize, 5usize, 8usize, 2143512u32);
    emu.sltru_no_count(30usize, 31usize, 16usize, 2143516u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2143524u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b524));
    } else {
        emu.pc = 2143520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b520));
    }
}
#[inline(always)]
pub fn block_0x0020b520(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(8usize, 5usize, 28usize, 2143524u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2143524u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b524));
}
#[inline(always)]
pub fn block_0x0020b524(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(28usize, 31usize, 8usize, 2143528u32);
    emu.adr_no_count(6usize, 29usize, 6usize, 2143532u32);
    emu.sltru_no_count(31usize, 28usize, 31usize, 2143536u32);
    emu.sltru_no_count(16usize, 6usize, 29usize, 2143540u32);
    emu.adr_no_count(11usize, 7usize, 11usize, 2143544u32);
    emu.adr_no_count(29usize, 11usize, 16usize, 2143548u32);
    emu.adr_no_count(30usize, 30usize, 31usize, 2143552u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(29usize);
    if a == b {
        emu.pc = 2143560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b548));
    } else {
        emu.pc = 2143556u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b544));
    }
}
#[inline(always)]
pub fn block_0x0020b544(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 29usize, 7usize, 2143560u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2143560u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b548));
}
#[inline(always)]
pub fn block_0x0020b548(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(11usize, 6usize, 28usize, 2143564u32);
    emu.sltru_no_count(7usize, 11usize, 6usize, 2143568u32);
    emu.adr_no_count(6usize, 29usize, 30usize, 2143572u32);
    emu.adr_no_count(6usize, 6usize, 7usize, 2143576u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2143584u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b560));
    } else {
        emu.pc = 2143580u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b55c));
    }
}
#[inline(always)]
pub fn block_0x0020b55c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(7usize, 6usize, 29usize, 2143584u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2143584u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b560));
}
#[inline(never)]
pub fn block_0x0020b560(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(7usize, 16usize, 7usize, 2143588u32);
    emu.sw_no_count(12usize, 10usize, 0u32, 2143592u32)?;
    emu.sw_no_count(13usize, 10usize, 4u32, 2143596u32)?;
    emu.sw_no_count(14usize, 10usize, 8u32, 2143600u32)?;
    emu.sw_no_count(15usize, 10usize, 12u32, 2143604u32)?;
    emu.sw_no_count(17usize, 10usize, 16u32, 2143608u32)?;
    emu.sw_no_count(5usize, 10usize, 20u32, 2143612u32)?;
    emu.sw_no_count(11usize, 10usize, 24u32, 2143616u32)?;
    emu.sw_no_count(6usize, 10usize, 28u32, 2143620u32)?;
    emu.sltru_no_count(11usize, 7usize, 16usize, 2143624u32);
    emu.sw_no_count(7usize, 10usize, 32u32, 2143628u32)?;
    emu.sw_no_count(11usize, 10usize, 36u32, 2143632u32)?;
    emu.lw_no_count(1usize, 2usize, 60u32, 2143636u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2143640u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2143644u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2143648u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2143652u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2143656u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2143660u32)?;
    emu.lw_no_count(22usize, 2usize, 32u32, 2143664u32)?;
    emu.lw_no_count(23usize, 2usize, 28u32, 2143668u32)?;
    emu.lw_no_count(24usize, 2usize, 24u32, 2143672u32)?;
    emu.lw_no_count(25usize, 2usize, 20u32, 2143676u32)?;
    emu.lw_no_count(26usize, 2usize, 16u32, 2143680u32)?;
    emu.lw_no_count(27usize, 2usize, 12u32, 2143684u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2143688u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2143692u32;
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
