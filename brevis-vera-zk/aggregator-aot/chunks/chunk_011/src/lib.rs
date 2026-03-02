pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2139988u32;
pub const PC_MAX: u32 = 2146432u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 176usize] = [
        block_0x0020a754,
        block_0x0020a864,
        block_0x0020a868,
        block_0x0020a8e4,
        block_0x0020a8e8,
        block_0x0020a944,
        block_0x0020a948,
        block_0x0020a9b8,
        block_0x0020a9bc,
        block_0x0020aa3c,
        block_0x0020aa40,
        block_0x0020aa6c,
        block_0x0020aa70,
        block_0x0020aae8,
        block_0x0020aaec,
        block_0x0020ab10,
        block_0x0020ab14,
        block_0x0020ab68,
        block_0x0020ab6c,
        block_0x0020ab90,
        block_0x0020ab94,
        block_0x0020ac0c,
        block_0x0020ac10,
        block_0x0020ac90,
        block_0x0020ac94,
        block_0x0020acc0,
        block_0x0020acc4,
        block_0x0020ad3c,
        block_0x0020ad40,
        block_0x0020ad68,
        block_0x0020ad6c,
        block_0x0020adc4,
        block_0x0020adc8,
        block_0x0020adf0,
        block_0x0020adf4,
        block_0x0020ae64,
        block_0x0020ae68,
        block_0x0020aee4,
        block_0x0020aee8,
        block_0x0020af14,
        block_0x0020af18,
        block_0x0020af90,
        block_0x0020af94,
        block_0x0020afc0,
        block_0x0020afc4,
        block_0x0020b004,
        block_0x0020b008,
        block_0x0020b020,
        block_0x0020b024,
        block_0x0020b050,
        block_0x0020b054,
        block_0x0020b0b8,
        block_0x0020b0bc,
        block_0x0020b0dc,
        block_0x0020b0e0,
        block_0x0020b108,
        block_0x0020b10c,
        block_0x0020b188,
        block_0x0020b18c,
        block_0x0020b1cc,
        block_0x0020b1d0,
        block_0x0020b204,
        block_0x0020b208,
        block_0x0020b258,
        block_0x0020b25c,
        block_0x0020b2a8,
        block_0x0020b2ac,
        block_0x0020b2cc,
        block_0x0020b2d0,
        block_0x0020b300,
        block_0x0020b304,
        block_0x0020b348,
        block_0x0020b34c,
        block_0x0020b38c,
        block_0x0020b390,
        block_0x0020b3a4,
        block_0x0020b3a8,
        block_0x0020b3d4,
        block_0x0020b3d8,
        block_0x0020b3f8,
        block_0x0020b400,
        block_0x0020b404,
        block_0x0020b434,
        block_0x0020b438,
        block_0x0020b460,
        block_0x0020b464,
        block_0x0020b484,
        block_0x0020b48c,
        block_0x0020b490,
        block_0x0020b4c0,
        block_0x0020b4c4,
        block_0x0020b4e4,
        block_0x0020b4e8,
        block_0x0020b530,
        block_0x0020b534,
        block_0x0020b5b4,
        block_0x0020b5d0,
        block_0x0020b5dc,
        block_0x0020b6ec,
        block_0x0020b6f0,
        block_0x0020b76c,
        block_0x0020b770,
        block_0x0020b790,
        block_0x0020b794,
        block_0x0020b7b0,
        block_0x0020b7b4,
        block_0x0020b824,
        block_0x0020b828,
        block_0x0020b898,
        block_0x0020b89c,
        block_0x0020b8bc,
        block_0x0020b8c0,
        block_0x0020b934,
        block_0x0020b938,
        block_0x0020b958,
        block_0x0020b95c,
        block_0x0020b97c,
        block_0x0020b980,
        block_0x0020b99c,
        block_0x0020b9a0,
        block_0x0020b9cc,
        block_0x0020b9d0,
        block_0x0020ba40,
        block_0x0020ba44,
        block_0x0020bab4,
        block_0x0020bab8,
        block_0x0020bae0,
        block_0x0020bae4,
        block_0x0020bb60,
        block_0x0020bb64,
        block_0x0020bb88,
        block_0x0020bb8c,
        block_0x0020bbb4,
        block_0x0020bbb8,
        block_0x0020bbd8,
        block_0x0020bbdc,
        block_0x0020bbfc,
        block_0x0020bc00,
        block_0x0020bc14,
        block_0x0020bc18,
        block_0x0020bc88,
        block_0x0020bc8c,
        block_0x0020bcfc,
        block_0x0020bd00,
        block_0x0020bd20,
        block_0x0020bd24,
        block_0x0020bd9c,
        block_0x0020bda0,
        block_0x0020bdc0,
        block_0x0020bdc4,
        block_0x0020bde8,
        block_0x0020bdec,
        block_0x0020be0c,
        block_0x0020be10,
        block_0x0020be30,
        block_0x0020be34,
        block_0x0020be48,
        block_0x0020be4c,
        block_0x0020bebc,
        block_0x0020bec0,
        block_0x0020bf30,
        block_0x0020bf34,
        block_0x0020bf54,
        block_0x0020bf58,
        block_0x0020bfd0,
        block_0x0020bfd4,
        block_0x0020bff4,
        block_0x0020bff8,
        block_0x0020c01c,
        block_0x0020c020,
        block_0x0020c040,
        block_0x0020c044,
        block_0x0020c064,
        block_0x0020c068,
        block_0x0020c07c,
        block_0x0020c080,
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
    if pc < 2139988u32 || pc > 2146432u32 {
        return None;
    }
    let word_offset = ((pc - 2139988u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(never)]
pub fn block_0x0020a754(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 68u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967200u32, 2139992u32);
    emu.sw_no_count(1usize, 2usize, 92u32, 2139996u32)?;
    emu.sw_no_count(8usize, 2usize, 88u32, 2140000u32)?;
    emu.sw_no_count(9usize, 2usize, 84u32, 2140004u32)?;
    emu.sw_no_count(18usize, 2usize, 80u32, 2140008u32)?;
    emu.sw_no_count(19usize, 2usize, 76u32, 2140012u32)?;
    emu.sw_no_count(20usize, 2usize, 72u32, 2140016u32)?;
    emu.sw_no_count(21usize, 2usize, 68u32, 2140020u32)?;
    emu.sw_no_count(22usize, 2usize, 64u32, 2140024u32)?;
    emu.sw_no_count(23usize, 2usize, 60u32, 2140028u32)?;
    emu.sw_no_count(24usize, 2usize, 56u32, 2140032u32)?;
    emu.sw_no_count(25usize, 2usize, 52u32, 2140036u32)?;
    emu.sw_no_count(26usize, 2usize, 48u32, 2140040u32)?;
    emu.sw_no_count(27usize, 2usize, 44u32, 2140044u32)?;
    emu.sw_no_count(10usize, 2usize, 28u32, 2140048u32)?;
    emu.lw_no_count(14usize, 11usize, 0u32, 2140052u32)?;
    emu.lw_no_count(9usize, 11usize, 4u32, 2140056u32)?;
    emu.lw_no_count(15usize, 12usize, 0u32, 2140060u32)?;
    emu.lw_no_count(29usize, 12usize, 4u32, 2140064u32)?;
    emu.lw_no_count(7usize, 12usize, 8u32, 2140068u32)?;
    emu.lw_no_count(28usize, 12usize, 12u32, 2140072u32)?;
    emu.lw_no_count(17usize, 12usize, 16u32, 2140076u32)?;
    emu.lw_no_count(5usize, 12usize, 20u32, 2140080u32)?;
    emu.mulhu_no_count(10usize, 15usize, 14usize, 2140084u32);
    emu.mul_no_count(13usize, 29usize, 14usize, 2140088u32);
    emu.mulhu_no_count(16usize, 29usize, 14usize, 2140092u32);
    emu.mul_no_count(6usize, 15usize, 9usize, 2140096u32);
    emu.mulhu_no_count(30usize, 15usize, 9usize, 2140100u32);
    emu.mul_no_count(31usize, 29usize, 9usize, 2140104u32);
    emu.mulhu_no_count(8usize, 7usize, 14usize, 2140108u32);
    emu.mul_no_count(18usize, 28usize, 14usize, 2140112u32);
    emu.adr_no_count(10usize, 13usize, 10usize, 2140116u32);
    emu.sltru_no_count(13usize, 10usize, 13usize, 2140120u32);
    emu.adr_no_count(13usize, 16usize, 13usize, 2140124u32);
    emu.mulhu_no_count(16usize, 28usize, 14usize, 2140128u32);
    emu.adr_no_count(8usize, 18usize, 8usize, 2140132u32);
    emu.sltru_no_count(18usize, 8usize, 18usize, 2140136u32);
    emu.adr_no_count(16usize, 16usize, 18usize, 2140140u32);
    emu.mul_no_count(18usize, 7usize, 9usize, 2140144u32);
    emu.adr_no_count(10usize, 6usize, 10usize, 2140148u32);
    emu.sw_no_count(10usize, 2usize, 24u32, 2140152u32)?;
    emu.sltru_no_count(10usize, 10usize, 6usize, 2140156u32);
    emu.adr_no_count(30usize, 30usize, 10usize, 2140160u32);
    emu.mulhu_no_count(6usize, 7usize, 9usize, 2140164u32);
    emu.adr_no_count(10usize, 18usize, 8usize, 2140168u32);
    emu.sltru_no_count(8usize, 10usize, 18usize, 2140172u32);
    emu.adr_no_count(6usize, 6usize, 8usize, 2140176u32);
    emu.mulhu_no_count(8usize, 29usize, 9usize, 2140180u32);
    emu.adr_no_count(30usize, 13usize, 30usize, 2140184u32);
    emu.sltru_no_count(13usize, 30usize, 13usize, 2140188u32);
    emu.adr_no_count(13usize, 8usize, 13usize, 2140192u32);
    emu.mulhu_no_count(8usize, 28usize, 9usize, 2140196u32);
    emu.adr_no_count(6usize, 16usize, 6usize, 2140200u32);
    emu.sltru_no_count(16usize, 6usize, 16usize, 2140204u32);
    emu.adr_no_count(8usize, 8usize, 16usize, 2140208u32);
    emu.mul_no_count(16usize, 28usize, 9usize, 2140212u32);
    emu.adr_no_count(30usize, 31usize, 30usize, 2140216u32);
    emu.sltru_no_count(31usize, 30usize, 31usize, 2140220u32);
    emu.adr_no_count(13usize, 13usize, 31usize, 2140224u32);
    emu.mul_no_count(31usize, 7usize, 14usize, 2140228u32);
    emu.adr_no_count(6usize, 16usize, 6usize, 2140232u32);
    emu.sltru_no_count(18usize, 6usize, 16usize, 2140236u32);
    emu.adr_no_count(30usize, 31usize, 30usize, 2140240u32);
    emu.adr_no_count(13usize, 10usize, 13usize, 2140244u32);
    emu.sltru_no_count(16usize, 30usize, 31usize, 2140248u32);
    emu.adr_no_count(21usize, 13usize, 16usize, 2140252u32);
    emu.adr_no_count(31usize, 8usize, 18usize, 2140256u32);
    emu.add_memory_rw_events(67usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a == b {
        emu.pc = 2140264u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a868));
    } else {
        emu.pc = 2140260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a864));
    }
}
#[inline(always)]
pub fn block_0x0020a864(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 21usize, 10usize, 2140264u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2140264u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a868));
}
#[inline(never)]
pub fn block_0x0020a868(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 31u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 12usize, 24u32, 2140268u32)?;
    emu.sw_no_count(10usize, 2usize, 36u32, 2140272u32)?;
    emu.lw_no_count(13usize, 12usize, 28u32, 2140276u32)?;
    emu.adr_no_count(16usize, 6usize, 16usize, 2140280u32);
    emu.mulhu_no_count(10usize, 17usize, 14usize, 2140284u32);
    emu.mul_no_count(8usize, 5usize, 14usize, 2140288u32);
    emu.mulhu_no_count(18usize, 5usize, 14usize, 2140292u32);
    emu.mul_no_count(19usize, 17usize, 9usize, 2140296u32);
    emu.sltru_no_count(6usize, 16usize, 6usize, 2140300u32);
    emu.adr_no_count(31usize, 31usize, 6usize, 2140304u32);
    emu.mulhu_no_count(20usize, 17usize, 9usize, 2140308u32);
    emu.adr_no_count(10usize, 8usize, 10usize, 2140312u32);
    emu.sltru_no_count(6usize, 10usize, 8usize, 2140316u32);
    emu.adr_no_count(18usize, 18usize, 6usize, 2140320u32);
    emu.mul_no_count(8usize, 5usize, 9usize, 2140324u32);
    emu.adr_no_count(6usize, 19usize, 10usize, 2140328u32);
    emu.sltru_no_count(10usize, 6usize, 19usize, 2140332u32);
    emu.adr_no_count(10usize, 20usize, 10usize, 2140336u32);
    emu.mulhu_no_count(19usize, 5usize, 9usize, 2140340u32);
    emu.adr_no_count(10usize, 18usize, 10usize, 2140344u32);
    emu.sltru_no_count(18usize, 10usize, 18usize, 2140348u32);
    emu.adr_no_count(18usize, 19usize, 18usize, 2140352u32);
    emu.mul_no_count(19usize, 17usize, 14usize, 2140356u32);
    emu.adr_no_count(22usize, 19usize, 16usize, 2140360u32);
    emu.sltru_no_count(16usize, 22usize, 19usize, 2140364u32);
    emu.adr_no_count(31usize, 6usize, 31usize, 2140368u32);
    emu.adr_no_count(10usize, 8usize, 10usize, 2140372u32);
    emu.sltru_no_count(8usize, 10usize, 8usize, 2140376u32);
    emu.adr_no_count(20usize, 31usize, 16usize, 2140380u32);
    emu.adr_no_count(18usize, 18usize, 8usize, 2140384u32);
    emu.add_memory_rw_events(30usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2140392u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a8e8));
    } else {
        emu.pc = 2140388u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a8e4));
    }
}
#[inline(always)]
pub fn block_0x0020a8e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 20usize, 6usize, 2140392u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2140392u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a8e8));
}
#[inline]
pub fn block_0x0020a8e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 23u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(8usize, 11usize, 8u32, 2140396u32)?;
    emu.lw_no_count(31usize, 11usize, 12u32, 2140400u32)?;
    emu.adr_no_count(16usize, 10usize, 16usize, 2140404u32);
    emu.lw_no_count(12usize, 2usize, 36u32, 2140408u32)?;
    emu.mulhu_no_count(6usize, 12usize, 14usize, 2140412u32);
    emu.mul_no_count(19usize, 13usize, 14usize, 2140416u32);
    emu.mulhu_no_count(25usize, 13usize, 14usize, 2140420u32);
    emu.mul_no_count(24usize, 12usize, 9usize, 2140424u32);
    emu.sltru_no_count(10usize, 16usize, 10usize, 2140428u32);
    emu.adr_no_count(18usize, 18usize, 10usize, 2140432u32);
    emu.mulhu_no_count(26usize, 12usize, 9usize, 2140436u32);
    emu.adr_no_count(10usize, 19usize, 6usize, 2140440u32);
    emu.sltru_no_count(6usize, 10usize, 19usize, 2140444u32);
    emu.adr_no_count(25usize, 25usize, 6usize, 2140448u32);
    emu.mul_no_count(6usize, 12usize, 14usize, 2140452u32);
    emu.adr_no_count(19usize, 6usize, 16usize, 2140456u32);
    emu.adr_no_count(10usize, 24usize, 10usize, 2140460u32);
    emu.sltru_no_count(23usize, 19usize, 6usize, 2140464u32);
    emu.sltru_no_count(16usize, 10usize, 24usize, 2140468u32);
    emu.adr_no_count(18usize, 10usize, 18usize, 2140472u32);
    emu.adr_no_count(18usize, 18usize, 23usize, 2140476u32);
    emu.adr_no_count(26usize, 26usize, 16usize, 2140480u32);
    emu.add_memory_rw_events(22usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2140488u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a948));
    } else {
        emu.pc = 2140484u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a944));
    }
}
#[inline(always)]
pub fn block_0x0020a944(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(23usize, 18usize, 10usize, 2140488u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2140488u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a948));
}
#[inline(never)]
pub fn block_0x0020a948(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(26usize, 25usize, 26usize, 2140492u32);
    emu.mul_no_count(24usize, 13usize, 9usize, 2140496u32);
    emu.mulhu_no_count(10usize, 15usize, 8usize, 2140500u32);
    emu.mul_no_count(16usize, 29usize, 8usize, 2140504u32);
    emu.mulhu_no_count(6usize, 29usize, 8usize, 2140508u32);
    emu.mul_no_count(27usize, 15usize, 31usize, 2140512u32);
    emu.mulhu_no_count(1usize, 15usize, 31usize, 2140516u32);
    emu.adr_no_count(10usize, 16usize, 10usize, 2140520u32);
    emu.sltru_no_count(16usize, 10usize, 16usize, 2140524u32);
    emu.adr_no_count(16usize, 6usize, 16usize, 2140528u32);
    emu.mul_no_count(6usize, 29usize, 31usize, 2140532u32);
    emu.adr_no_count(10usize, 27usize, 10usize, 2140536u32);
    emu.sltru_no_count(27usize, 10usize, 27usize, 2140540u32);
    emu.adr_no_count(27usize, 1usize, 27usize, 2140544u32);
    emu.mulhu_no_count(1usize, 29usize, 31usize, 2140548u32);
    emu.adr_no_count(27usize, 16usize, 27usize, 2140552u32);
    emu.sltru_no_count(16usize, 27usize, 16usize, 2140556u32);
    emu.adr_no_count(1usize, 1usize, 16usize, 2140560u32);
    emu.mul_no_count(16usize, 15usize, 8usize, 2140564u32);
    emu.adr_no_count(16usize, 30usize, 16usize, 2140568u32);
    emu.sw_no_count(16usize, 2usize, 32u32, 2140572u32)?;
    emu.sltru_no_count(16usize, 16usize, 30usize, 2140576u32);
    emu.adr_no_count(30usize, 10usize, 16usize, 2140580u32);
    emu.adr_no_count(10usize, 6usize, 27usize, 2140584u32);
    emu.sltru_no_count(6usize, 10usize, 6usize, 2140588u32);
    emu.adr_no_count(30usize, 21usize, 30usize, 2140592u32);
    emu.adr_no_count(6usize, 1usize, 6usize, 2140596u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(30usize);
    if a == b {
        emu.pc = 2140604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a9bc));
    } else {
        emu.pc = 2140600u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020a9b8));
    }
}
#[inline(always)]
pub fn block_0x0020a9b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 30usize, 21usize, 2140604u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2140604u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020a9bc));
}
#[inline(never)]
pub fn block_0x0020a9bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 32u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(30usize, 2usize, 20u32, 2140608u32)?;
    emu.adr_no_count(21usize, 24usize, 26usize, 2140612u32);
    emu.sltru_no_count(26usize, 26usize, 25usize, 2140616u32);
    emu.mulhu_no_count(9usize, 13usize, 9usize, 2140620u32);
    emu.adr_no_count(25usize, 10usize, 16usize, 2140624u32);
    emu.mulhu_no_count(30usize, 7usize, 8usize, 2140628u32);
    emu.mul_no_count(27usize, 28usize, 8usize, 2140632u32);
    emu.mulhu_no_count(1usize, 28usize, 8usize, 2140636u32);
    emu.mul_no_count(12usize, 7usize, 31usize, 2140640u32);
    emu.sltru_no_count(16usize, 25usize, 10usize, 2140644u32);
    emu.adr_no_count(16usize, 6usize, 16usize, 2140648u32);
    emu.mulhu_no_count(10usize, 7usize, 31usize, 2140652u32);
    emu.adr_no_count(30usize, 27usize, 30usize, 2140656u32);
    emu.sltru_no_count(6usize, 30usize, 27usize, 2140660u32);
    emu.adr_no_count(6usize, 1usize, 6usize, 2140664u32);
    emu.mul_no_count(27usize, 28usize, 31usize, 2140668u32);
    emu.adr_no_count(30usize, 12usize, 30usize, 2140672u32);
    emu.sltru_no_count(12usize, 30usize, 12usize, 2140676u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2140680u32);
    emu.mulhu_no_count(12usize, 28usize, 31usize, 2140684u32);
    emu.adr_no_count(10usize, 6usize, 10usize, 2140688u32);
    emu.sltru_no_count(6usize, 10usize, 6usize, 2140692u32);
    emu.adr_no_count(12usize, 12usize, 6usize, 2140696u32);
    emu.mul_no_count(1usize, 7usize, 8usize, 2140700u32);
    emu.adr_no_count(1usize, 22usize, 1usize, 2140704u32);
    emu.sltru_no_count(22usize, 1usize, 22usize, 2140708u32);
    emu.adr_no_count(6usize, 30usize, 22usize, 2140712u32);
    emu.adr_no_count(30usize, 27usize, 10usize, 2140716u32);
    emu.sltru_no_count(10usize, 30usize, 27usize, 2140720u32);
    emu.adr_no_count(27usize, 20usize, 6usize, 2140724u32);
    emu.adr_no_count(10usize, 12usize, 10usize, 2140728u32);
    emu.add_memory_rw_events(31usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(27usize);
    if a == b {
        emu.pc = 2140736u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aa40));
    } else {
        emu.pc = 2140732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aa3c));
    }
}
#[inline(always)]
pub fn block_0x0020aa3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(22usize, 27usize, 20usize, 2140736u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2140736u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020aa40));
}
#[inline]
pub fn block_0x0020aa40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(6usize, 21usize, 24usize, 2140740u32);
    emu.adr_no_count(24usize, 9usize, 26usize, 2140744u32);
    emu.adr_no_count(9usize, 21usize, 23usize, 2140748u32);
    emu.adr_no_count(26usize, 30usize, 22usize, 2140752u32);
    emu.adr_no_count(20usize, 27usize, 16usize, 2140756u32);
    emu.adr_no_count(22usize, 1usize, 25usize, 2140760u32);
    emu.sltru_no_count(12usize, 26usize, 30usize, 2140764u32);
    emu.sltru_no_count(16usize, 22usize, 1usize, 2140768u32);
    emu.adr_no_count(20usize, 20usize, 16usize, 2140772u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2140776u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(27usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2140784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aa70));
    } else {
        emu.pc = 2140780u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aa6c));
    }
}
#[inline(always)]
pub fn block_0x0020aa6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 20usize, 27usize, 2140784u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2140784u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020aa70));
}
#[inline(never)]
pub fn block_0x0020aa70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 30u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(23usize, 24usize, 6usize, 2140788u32);
    emu.sltru_no_count(21usize, 9usize, 21usize, 2140792u32);
    emu.adr_no_count(24usize, 26usize, 16usize, 2140796u32);
    emu.mulhu_no_count(12usize, 17usize, 8usize, 2140800u32);
    emu.mul_no_count(16usize, 5usize, 8usize, 2140804u32);
    emu.mulhu_no_count(6usize, 5usize, 8usize, 2140808u32);
    emu.mul_no_count(30usize, 17usize, 31usize, 2140812u32);
    emu.sltru_no_count(25usize, 24usize, 26usize, 2140816u32);
    emu.adr_no_count(10usize, 10usize, 25usize, 2140820u32);
    emu.mulhu_no_count(25usize, 17usize, 31usize, 2140824u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2140828u32);
    emu.sltru_no_count(16usize, 12usize, 16usize, 2140832u32);
    emu.adr_no_count(16usize, 6usize, 16usize, 2140836u32);
    emu.mul_no_count(6usize, 5usize, 31usize, 2140840u32);
    emu.adr_no_count(12usize, 30usize, 12usize, 2140844u32);
    emu.sltru_no_count(30usize, 12usize, 30usize, 2140848u32);
    emu.adr_no_count(30usize, 25usize, 30usize, 2140852u32);
    emu.mulhu_no_count(25usize, 5usize, 31usize, 2140856u32);
    emu.adr_no_count(26usize, 16usize, 30usize, 2140860u32);
    emu.sltru_no_count(16usize, 26usize, 16usize, 2140864u32);
    emu.adr_no_count(27usize, 25usize, 16usize, 2140868u32);
    emu.mul_no_count(25usize, 17usize, 8usize, 2140872u32);
    emu.adr_no_count(25usize, 19usize, 25usize, 2140876u32);
    emu.sltru_no_count(30usize, 25usize, 19usize, 2140880u32);
    emu.adr_no_count(12usize, 12usize, 30usize, 2140884u32);
    emu.adr_no_count(16usize, 6usize, 26usize, 2140888u32);
    emu.sltru_no_count(19usize, 16usize, 6usize, 2140892u32);
    emu.adr_no_count(6usize, 18usize, 12usize, 2140896u32);
    emu.adr_no_count(19usize, 27usize, 19usize, 2140900u32);
    emu.add_memory_rw_events(29usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2140908u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aaec));
    } else {
        emu.pc = 2140904u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aae8));
    }
}
#[inline(always)]
pub fn block_0x0020aae8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(30usize, 6usize, 18usize, 2140908u32);
    emu.add_memory_rw_events(1usize);
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
    emu.adr_no_count(21usize, 23usize, 21usize, 2140912u32);
    emu.adr_no_count(30usize, 16usize, 30usize, 2140916u32);
    emu.adr_no_count(23usize, 6usize, 10usize, 2140920u32);
    emu.adr_no_count(24usize, 25usize, 24usize, 2140924u32);
    emu.sltru_no_count(12usize, 30usize, 16usize, 2140928u32);
    emu.sltru_no_count(10usize, 24usize, 25usize, 2140932u32);
    emu.adr_no_count(23usize, 23usize, 10usize, 2140936u32);
    emu.adr_no_count(19usize, 19usize, 12usize, 2140940u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a == b {
        emu.pc = 2140948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ab14));
    } else {
        emu.pc = 2140944u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ab10));
    }
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
    emu.sltru_no_count(10usize, 23usize, 6usize, 2140948u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2140948u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ab14));
}
#[inline]
pub fn block_0x0020ab14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 21u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 30usize, 10usize, 2140952u32);
    emu.lw_no_count(26usize, 2usize, 36u32, 2140956u32)?;
    emu.mulhu_no_count(12usize, 26usize, 8usize, 2140960u32);
    emu.mul_no_count(16usize, 13usize, 8usize, 2140964u32);
    emu.mulhu_no_count(6usize, 13usize, 8usize, 2140968u32);
    emu.sltru_no_count(30usize, 10usize, 30usize, 2140972u32);
    emu.adr_no_count(19usize, 19usize, 30usize, 2140976u32);
    emu.mul_no_count(18usize, 26usize, 31usize, 2140980u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2140984u32);
    emu.sltru_no_count(30usize, 12usize, 16usize, 2140988u32);
    emu.adr_no_count(30usize, 6usize, 30usize, 2140992u32);
    emu.mulhu_no_count(25usize, 26usize, 31usize, 2140996u32);
    emu.mul_no_count(16usize, 26usize, 8usize, 2141000u32);
    emu.adr_no_count(16usize, 9usize, 16usize, 2141004u32);
    emu.adr_no_count(12usize, 18usize, 12usize, 2141008u32);
    emu.sltru_no_count(27usize, 16usize, 9usize, 2141012u32);
    emu.sltru_no_count(8usize, 12usize, 18usize, 2141016u32);
    emu.adr_no_count(12usize, 21usize, 12usize, 2141020u32);
    emu.adr_no_count(6usize, 12usize, 27usize, 2141024u32);
    emu.adr_no_count(8usize, 25usize, 8usize, 2141028u32);
    emu.add_memory_rw_events(20usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2141036u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ab6c));
    } else {
        emu.pc = 2141032u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ab68));
    }
}
#[inline(always)]
pub fn block_0x0020ab68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(27usize, 6usize, 21usize, 2141036u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2141036u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ab6c));
}
#[inline]
pub fn block_0x0020ab6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 30usize, 8usize, 2141040u32);
    emu.lw_no_count(9usize, 11usize, 16u32, 2141044u32)?;
    emu.lw_no_count(18usize, 11usize, 20u32, 2141048u32)?;
    emu.adr_no_count(21usize, 16usize, 10usize, 2141052u32);
    emu.sltru_no_count(25usize, 21usize, 16usize, 2141056u32);
    emu.adr_no_count(19usize, 6usize, 19usize, 2141060u32);
    emu.adr_no_count(19usize, 19usize, 25usize, 2141064u32);
    emu.mul_no_count(1usize, 13usize, 31usize, 2141068u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2141076u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ab94));
    } else {
        emu.pc = 2141072u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ab90));
    }
}
#[inline(always)]
pub fn block_0x0020ab90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(25usize, 19usize, 6usize, 2141076u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2141076u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ab94));
}
#[inline(never)]
pub fn block_0x0020ab94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 30u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(26usize, 1usize, 8usize, 2141080u32);
    emu.sltru_no_count(30usize, 8usize, 30usize, 2141084u32);
    emu.sw_no_count(13usize, 2usize, 40u32, 2141088u32)?;
    emu.mulhu_no_count(10usize, 13usize, 31usize, 2141092u32);
    emu.mulhu_no_count(12usize, 15usize, 9usize, 2141096u32);
    emu.mul_no_count(16usize, 29usize, 9usize, 2141100u32);
    emu.mulhu_no_count(6usize, 29usize, 9usize, 2141104u32);
    emu.mul_no_count(31usize, 15usize, 18usize, 2141108u32);
    emu.mulhu_no_count(8usize, 15usize, 18usize, 2141112u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2141116u32);
    emu.sltru_no_count(16usize, 12usize, 16usize, 2141120u32);
    emu.adr_no_count(16usize, 6usize, 16usize, 2141124u32);
    emu.mul_no_count(13usize, 29usize, 18usize, 2141128u32);
    emu.adr_no_count(12usize, 31usize, 12usize, 2141132u32);
    emu.sltru_no_count(6usize, 12usize, 31usize, 2141136u32);
    emu.adr_no_count(6usize, 8usize, 6usize, 2141140u32);
    emu.mulhu_no_count(31usize, 29usize, 18usize, 2141144u32);
    emu.adr_no_count(8usize, 16usize, 6usize, 2141148u32);
    emu.sltru_no_count(16usize, 8usize, 16usize, 2141152u32);
    emu.adr_no_count(16usize, 31usize, 16usize, 2141156u32);
    emu.mul_no_count(6usize, 15usize, 9usize, 2141160u32);
    emu.adr_no_count(6usize, 22usize, 6usize, 2141164u32);
    emu.sw_no_count(6usize, 2usize, 12u32, 2141168u32)?;
    emu.sltru_no_count(6usize, 6usize, 22usize, 2141172u32);
    emu.adr_no_count(12usize, 12usize, 6usize, 2141176u32);
    emu.adr_no_count(8usize, 13usize, 8usize, 2141180u32);
    emu.sltru_no_count(22usize, 8usize, 13usize, 2141184u32);
    emu.adr_no_count(12usize, 20usize, 12usize, 2141188u32);
    emu.adr_no_count(22usize, 16usize, 22usize, 2141192u32);
    emu.add_memory_rw_events(29usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2141200u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ac10));
    } else {
        emu.pc = 2141196u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ac0c));
    }
}
#[inline(always)]
pub fn block_0x0020ac0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(6usize, 12usize, 20usize, 2141200u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2141200u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ac10));
}
#[inline(never)]
pub fn block_0x0020ac10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 32u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(12usize, 2usize, 16u32, 2141204u32)?;
    emu.sltru_no_count(20usize, 26usize, 1usize, 2141208u32);
    emu.adr_no_count(1usize, 10usize, 30usize, 2141212u32);
    emu.adr_no_count(27usize, 26usize, 27usize, 2141216u32);
    emu.adr_no_count(6usize, 8usize, 6usize, 2141220u32);
    emu.mulhu_no_count(12usize, 7usize, 9usize, 2141224u32);
    emu.mul_no_count(13usize, 28usize, 9usize, 2141228u32);
    emu.mulhu_no_count(16usize, 28usize, 9usize, 2141232u32);
    emu.mul_no_count(30usize, 7usize, 18usize, 2141236u32);
    emu.sltru_no_count(10usize, 6usize, 8usize, 2141240u32);
    emu.adr_no_count(10usize, 22usize, 10usize, 2141244u32);
    emu.mulhu_no_count(8usize, 7usize, 18usize, 2141248u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2141252u32);
    emu.sltru_no_count(13usize, 12usize, 13usize, 2141256u32);
    emu.adr_no_count(13usize, 16usize, 13usize, 2141260u32);
    emu.mul_no_count(31usize, 28usize, 18usize, 2141264u32);
    emu.adr_no_count(12usize, 30usize, 12usize, 2141268u32);
    emu.sltru_no_count(16usize, 12usize, 30usize, 2141272u32);
    emu.adr_no_count(16usize, 8usize, 16usize, 2141276u32);
    emu.mulhu_no_count(30usize, 28usize, 18usize, 2141280u32);
    emu.adr_no_count(16usize, 13usize, 16usize, 2141284u32);
    emu.sltru_no_count(13usize, 16usize, 13usize, 2141288u32);
    emu.adr_no_count(13usize, 30usize, 13usize, 2141292u32);
    emu.mul_no_count(22usize, 7usize, 9usize, 2141296u32);
    emu.adr_no_count(22usize, 24usize, 22usize, 2141300u32);
    emu.sltru_no_count(30usize, 22usize, 24usize, 2141304u32);
    emu.adr_no_count(12usize, 12usize, 30usize, 2141308u32);
    emu.adr_no_count(16usize, 31usize, 16usize, 2141312u32);
    emu.sltru_no_count(24usize, 16usize, 31usize, 2141316u32);
    emu.adr_no_count(8usize, 23usize, 12usize, 2141320u32);
    emu.adr_no_count(24usize, 13usize, 24usize, 2141324u32);
    emu.add_memory_rw_events(31usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(8usize);
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
    emu.sltru_no_count(30usize, 8usize, 23usize, 2141332u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2141332u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ac94));
}
#[inline]
pub fn block_0x0020ac94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(23usize, 1usize, 20usize, 2141336u32);
    emu.sltru_no_count(26usize, 27usize, 26usize, 2141340u32);
    emu.adr_no_count(25usize, 27usize, 25usize, 2141344u32);
    emu.adr_no_count(30usize, 16usize, 30usize, 2141348u32);
    emu.adr_no_count(10usize, 8usize, 10usize, 2141352u32);
    emu.adr_no_count(6usize, 22usize, 6usize, 2141356u32);
    emu.sltru_no_count(12usize, 30usize, 16usize, 2141360u32);
    emu.sltru_no_count(16usize, 6usize, 22usize, 2141364u32);
    emu.adr_no_count(20usize, 10usize, 16usize, 2141368u32);
    emu.adr_no_count(10usize, 24usize, 12usize, 2141372u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2141380u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020acc4));
    } else {
        emu.pc = 2141376u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020acc0));
    }
}
#[inline(always)]
pub fn block_0x0020acc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 20usize, 8usize, 2141380u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2141380u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020acc4));
}
#[inline(never)]
pub fn block_0x0020acc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 30u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(23usize, 23usize, 26usize, 2141384u32);
    emu.sltru_no_count(26usize, 25usize, 27usize, 2141388u32);
    emu.adr_no_count(24usize, 30usize, 16usize, 2141392u32);
    emu.mulhu_no_count(12usize, 17usize, 9usize, 2141396u32);
    emu.mul_no_count(13usize, 5usize, 9usize, 2141400u32);
    emu.mulhu_no_count(16usize, 5usize, 9usize, 2141404u32);
    emu.mul_no_count(31usize, 17usize, 18usize, 2141408u32);
    emu.sltru_no_count(30usize, 24usize, 30usize, 2141412u32);
    emu.adr_no_count(10usize, 10usize, 30usize, 2141416u32);
    emu.mulhu_no_count(30usize, 17usize, 18usize, 2141420u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2141424u32);
    emu.sltru_no_count(13usize, 12usize, 13usize, 2141428u32);
    emu.adr_no_count(13usize, 16usize, 13usize, 2141432u32);
    emu.mul_no_count(8usize, 5usize, 18usize, 2141436u32);
    emu.adr_no_count(12usize, 31usize, 12usize, 2141440u32);
    emu.sltru_no_count(16usize, 12usize, 31usize, 2141444u32);
    emu.adr_no_count(16usize, 30usize, 16usize, 2141448u32);
    emu.mulhu_no_count(30usize, 5usize, 18usize, 2141452u32);
    emu.adr_no_count(16usize, 13usize, 16usize, 2141456u32);
    emu.sltru_no_count(13usize, 16usize, 13usize, 2141460u32);
    emu.adr_no_count(13usize, 30usize, 13usize, 2141464u32);
    emu.mul_no_count(22usize, 17usize, 9usize, 2141468u32);
    emu.adr_no_count(22usize, 21usize, 22usize, 2141472u32);
    emu.sltru_no_count(30usize, 22usize, 21usize, 2141476u32);
    emu.adr_no_count(12usize, 12usize, 30usize, 2141480u32);
    emu.adr_no_count(16usize, 8usize, 16usize, 2141484u32);
    emu.sltru_no_count(27usize, 16usize, 8usize, 2141488u32);
    emu.adr_no_count(8usize, 19usize, 12usize, 2141492u32);
    emu.adr_no_count(27usize, 13usize, 27usize, 2141496u32);
    emu.add_memory_rw_events(29usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2141504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ad40));
    } else {
        emu.pc = 2141500u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ad3c));
    }
}
#[inline(always)]
pub fn block_0x0020ad3c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(30usize, 8usize, 19usize, 2141504u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2141504u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ad40));
}
#[inline]
pub fn block_0x0020ad40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(23usize, 23usize, 26usize, 2141508u32);
    emu.adr_no_count(30usize, 16usize, 30usize, 2141512u32);
    emu.adr_no_count(21usize, 8usize, 10usize, 2141516u32);
    emu.adr_no_count(24usize, 22usize, 24usize, 2141520u32);
    emu.sltru_no_count(19usize, 30usize, 16usize, 2141524u32);
    emu.sltru_no_count(10usize, 24usize, 22usize, 2141528u32);
    emu.adr_no_count(21usize, 21usize, 10usize, 2141532u32);
    emu.adr_no_count(19usize, 27usize, 19usize, 2141536u32);
    emu.lw_no_count(13usize, 2usize, 40u32, 2141540u32)?;
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a == b {
        emu.pc = 2141548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ad6c));
    } else {
        emu.pc = 2141544u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ad68));
    }
}
#[inline(always)]
pub fn block_0x0020ad68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 21usize, 8usize, 2141548u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2141548u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ad6c));
}
#[inline]
pub fn block_0x0020ad6c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 22u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 30usize, 10usize, 2141552u32);
    emu.lw_no_count(8usize, 2usize, 36u32, 2141556u32)?;
    emu.mulhu_no_count(12usize, 8usize, 9usize, 2141560u32);
    emu.adi_no_count(16usize, 13usize, 0u32, 2141564u32);
    emu.mul_no_count(13usize, 13usize, 9usize, 2141568u32);
    emu.mulhu_no_count(16usize, 16usize, 9usize, 2141572u32);
    emu.sltru_no_count(30usize, 10usize, 30usize, 2141576u32);
    emu.adr_no_count(19usize, 19usize, 30usize, 2141580u32);
    emu.mul_no_count(31usize, 8usize, 18usize, 2141584u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2141588u32);
    emu.sltru_no_count(30usize, 12usize, 13usize, 2141592u32);
    emu.adr_no_count(30usize, 16usize, 30usize, 2141596u32);
    emu.mulhu_no_count(22usize, 8usize, 18usize, 2141600u32);
    emu.mul_no_count(16usize, 8usize, 9usize, 2141604u32);
    emu.adr_no_count(16usize, 25usize, 16usize, 2141608u32);
    emu.adr_no_count(12usize, 31usize, 12usize, 2141612u32);
    emu.sltru_no_count(27usize, 16usize, 25usize, 2141616u32);
    emu.sltru_no_count(13usize, 12usize, 31usize, 2141620u32);
    emu.adr_no_count(12usize, 23usize, 12usize, 2141624u32);
    emu.adr_no_count(8usize, 12usize, 27usize, 2141628u32);
    emu.adr_no_count(22usize, 22usize, 13usize, 2141632u32);
    emu.add_memory_rw_events(21usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2141640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020adc8));
    } else {
        emu.pc = 2141636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020adc4));
    }
}
#[inline(always)]
pub fn block_0x0020adc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(27usize, 8usize, 23usize, 2141640u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2141640u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020adc8));
}
#[inline]
pub fn block_0x0020adc8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(22usize, 30usize, 22usize, 2141644u32);
    emu.lw_no_count(9usize, 11usize, 24u32, 2141648u32)?;
    emu.lw_no_count(11usize, 11usize, 28u32, 2141652u32)?;
    emu.adr_no_count(23usize, 16usize, 10usize, 2141656u32);
    emu.sltru_no_count(25usize, 23usize, 16usize, 2141660u32);
    emu.adr_no_count(19usize, 8usize, 19usize, 2141664u32);
    emu.adr_no_count(19usize, 19usize, 25usize, 2141668u32);
    emu.lw_no_count(10usize, 2usize, 40u32, 2141672u32)?;
    emu.mul_no_count(1usize, 10usize, 18usize, 2141676u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2141684u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020adf4));
    } else {
        emu.pc = 2141680u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020adf0));
    }
}
#[inline(always)]
pub fn block_0x0020adf0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(25usize, 19usize, 8usize, 2141684u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2141684u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020adf4));
}
#[inline(never)]
pub fn block_0x0020adf4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(26usize, 1usize, 22usize, 2141688u32);
    emu.sltru_no_count(30usize, 22usize, 30usize, 2141692u32);
    emu.mulhu_no_count(10usize, 10usize, 18usize, 2141696u32);
    emu.mulhu_no_count(12usize, 15usize, 9usize, 2141700u32);
    emu.mul_no_count(13usize, 29usize, 9usize, 2141704u32);
    emu.mulhu_no_count(16usize, 29usize, 9usize, 2141708u32);
    emu.mul_no_count(31usize, 15usize, 11usize, 2141712u32);
    emu.mulhu_no_count(8usize, 15usize, 11usize, 2141716u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2141720u32);
    emu.sltru_no_count(13usize, 12usize, 13usize, 2141724u32);
    emu.adr_no_count(13usize, 16usize, 13usize, 2141728u32);
    emu.mul_no_count(16usize, 29usize, 11usize, 2141732u32);
    emu.mulhu_no_count(29usize, 29usize, 11usize, 2141736u32);
    emu.adr_no_count(12usize, 31usize, 12usize, 2141740u32);
    emu.sltru_no_count(31usize, 12usize, 31usize, 2141744u32);
    emu.adr_no_count(31usize, 8usize, 31usize, 2141748u32);
    emu.mul_no_count(18usize, 15usize, 9usize, 2141752u32);
    emu.adr_no_count(18usize, 6usize, 18usize, 2141756u32);
    emu.sltru_no_count(8usize, 18usize, 6usize, 2141760u32);
    emu.adr_no_count(12usize, 12usize, 8usize, 2141764u32);
    emu.adr_no_count(31usize, 13usize, 31usize, 2141768u32);
    emu.sltru_no_count(13usize, 31usize, 13usize, 2141772u32);
    emu.adr_no_count(22usize, 16usize, 31usize, 2141776u32);
    emu.sltru_no_count(6usize, 22usize, 16usize, 2141780u32);
    emu.adr_no_count(13usize, 29usize, 13usize, 2141784u32);
    emu.adr_no_count(29usize, 20usize, 12usize, 2141788u32);
    emu.adr_no_count(6usize, 13usize, 6usize, 2141792u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(29usize);
    if a == b {
        emu.pc = 2141800u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ae68));
    } else {
        emu.pc = 2141796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ae64));
    }
}
#[inline(always)]
pub fn block_0x0020ae64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(8usize, 29usize, 20usize, 2141800u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2141800u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ae68));
}
#[inline(never)]
pub fn block_0x0020ae68(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 31u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(20usize, 26usize, 1usize, 2141804u32);
    emu.adr_no_count(1usize, 10usize, 30usize, 2141808u32);
    emu.adr_no_count(27usize, 26usize, 27usize, 2141812u32);
    emu.adr_no_count(8usize, 22usize, 8usize, 2141816u32);
    emu.mulhu_no_count(12usize, 7usize, 9usize, 2141820u32);
    emu.mul_no_count(13usize, 28usize, 9usize, 2141824u32);
    emu.mulhu_no_count(16usize, 28usize, 9usize, 2141828u32);
    emu.mul_no_count(31usize, 7usize, 11usize, 2141832u32);
    emu.sltru_no_count(10usize, 8usize, 22usize, 2141836u32);
    emu.adr_no_count(10usize, 6usize, 10usize, 2141840u32);
    emu.mulhu_no_count(6usize, 7usize, 11usize, 2141844u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2141848u32);
    emu.sltru_no_count(13usize, 12usize, 13usize, 2141852u32);
    emu.adr_no_count(13usize, 16usize, 13usize, 2141856u32);
    emu.mul_no_count(16usize, 28usize, 11usize, 2141860u32);
    emu.mulhu_no_count(22usize, 28usize, 11usize, 2141864u32);
    emu.mul_no_count(28usize, 7usize, 9usize, 2141868u32);
    emu.adr_no_count(28usize, 24usize, 28usize, 2141872u32);
    emu.adr_no_count(12usize, 31usize, 12usize, 2141876u32);
    emu.sltru_no_count(30usize, 28usize, 24usize, 2141880u32);
    emu.sltru_no_count(7usize, 12usize, 31usize, 2141884u32);
    emu.adr_no_count(12usize, 12usize, 30usize, 2141888u32);
    emu.adr_no_count(6usize, 6usize, 7usize, 2141892u32);
    emu.adr_no_count(7usize, 13usize, 6usize, 2141896u32);
    emu.adr_no_count(6usize, 16usize, 7usize, 2141900u32);
    emu.sltru_no_count(13usize, 7usize, 13usize, 2141904u32);
    emu.sltru_no_count(16usize, 6usize, 16usize, 2141908u32);
    emu.adr_no_count(13usize, 22usize, 13usize, 2141912u32);
    emu.adr_no_count(24usize, 21usize, 12usize, 2141916u32);
    emu.adr_no_count(16usize, 13usize, 16usize, 2141920u32);
    emu.add_memory_rw_events(30usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(24usize);
    if a == b {
        emu.pc = 2141928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aee8));
    } else {
        emu.pc = 2141924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020aee4));
    }
}
#[inline(always)]
pub fn block_0x0020aee4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(30usize, 24usize, 21usize, 2141928u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2141928u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020aee8));
}
#[inline]
pub fn block_0x0020aee8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(20usize, 1usize, 20usize, 2141932u32);
    emu.sltru_no_count(26usize, 27usize, 26usize, 2141936u32);
    emu.adr_no_count(7usize, 27usize, 25usize, 2141940u32);
    emu.adr_no_count(30usize, 6usize, 30usize, 2141944u32);
    emu.adr_no_count(10usize, 24usize, 10usize, 2141948u32);
    emu.adr_no_count(22usize, 28usize, 8usize, 2141952u32);
    emu.sltru_no_count(12usize, 30usize, 6usize, 2141956u32);
    emu.sltru_no_count(6usize, 22usize, 28usize, 2141960u32);
    emu.adr_no_count(21usize, 10usize, 6usize, 2141964u32);
    emu.adr_no_count(10usize, 16usize, 12usize, 2141968u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a == b {
        emu.pc = 2141976u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020af18));
    } else {
        emu.pc = 2141972u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020af14));
    }
}
#[inline(always)]
pub fn block_0x0020af14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(6usize, 21usize, 24usize, 2141976u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2141976u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020af18));
}
#[inline(never)]
pub fn block_0x0020af18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 30u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(28usize, 20usize, 26usize, 2141980u32);
    emu.sltru_no_count(24usize, 7usize, 27usize, 2141984u32);
    emu.adr_no_count(20usize, 30usize, 6usize, 2141988u32);
    emu.mulhu_no_count(12usize, 17usize, 9usize, 2141992u32);
    emu.mul_no_count(13usize, 5usize, 9usize, 2141996u32);
    emu.mulhu_no_count(16usize, 5usize, 9usize, 2142000u32);
    emu.mul_no_count(6usize, 17usize, 11usize, 2142004u32);
    emu.sltru_no_count(30usize, 20usize, 30usize, 2142008u32);
    emu.adr_no_count(10usize, 10usize, 30usize, 2142012u32);
    emu.mulhu_no_count(31usize, 17usize, 11usize, 2142016u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2142020u32);
    emu.sltru_no_count(13usize, 12usize, 13usize, 2142024u32);
    emu.adr_no_count(13usize, 16usize, 13usize, 2142028u32);
    emu.mul_no_count(8usize, 5usize, 11usize, 2142032u32);
    emu.mulhu_no_count(25usize, 5usize, 11usize, 2142036u32);
    emu.mul_no_count(5usize, 17usize, 9usize, 2142040u32);
    emu.adr_no_count(5usize, 23usize, 5usize, 2142044u32);
    emu.adr_no_count(12usize, 6usize, 12usize, 2142048u32);
    emu.sltru_no_count(30usize, 5usize, 23usize, 2142052u32);
    emu.sltru_no_count(16usize, 12usize, 6usize, 2142056u32);
    emu.adr_no_count(12usize, 12usize, 30usize, 2142060u32);
    emu.adr_no_count(16usize, 31usize, 16usize, 2142064u32);
    emu.adr_no_count(17usize, 13usize, 16usize, 2142068u32);
    emu.adr_no_count(16usize, 8usize, 17usize, 2142072u32);
    emu.sltru_no_count(13usize, 17usize, 13usize, 2142076u32);
    emu.sltru_no_count(8usize, 16usize, 8usize, 2142080u32);
    emu.adr_no_count(13usize, 25usize, 13usize, 2142084u32);
    emu.adr_no_count(6usize, 19usize, 12usize, 2142088u32);
    emu.adr_no_count(8usize, 13usize, 8usize, 2142092u32);
    emu.add_memory_rw_events(29usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2142100u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020af94));
    } else {
        emu.pc = 2142096u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020af90));
    }
}
#[inline(always)]
pub fn block_0x0020af90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(30usize, 6usize, 19usize, 2142100u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2142100u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020af94));
}
#[inline]
pub fn block_0x0020af94(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(17usize, 28usize, 24usize, 2142104u32);
    emu.adr_no_count(28usize, 16usize, 30usize, 2142108u32);
    emu.adr_no_count(10usize, 6usize, 10usize, 2142112u32);
    emu.adr_no_count(20usize, 5usize, 20usize, 2142116u32);
    emu.sltru_no_count(12usize, 28usize, 16usize, 2142120u32);
    emu.sltru_no_count(5usize, 20usize, 5usize, 2142124u32);
    emu.adr_no_count(19usize, 10usize, 5usize, 2142128u32);
    emu.adr_no_count(10usize, 8usize, 12usize, 2142132u32);
    emu.lw_no_count(12usize, 2usize, 40u32, 2142136u32)?;
    emu.lw_no_count(31usize, 2usize, 24u32, 2142140u32)?;
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(19usize);
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
    emu.sltru_no_count(5usize, 19usize, 6usize, 2142148u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2142148u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020afc4));
}
#[inline]
pub fn block_0x0020afc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(5usize, 28usize, 5usize, 2142152u32);
    emu.lw_no_count(13usize, 2usize, 36u32, 2142156u32)?;
    emu.mulhu_no_count(25usize, 13usize, 9usize, 2142160u32);
    emu.mul_no_count(23usize, 12usize, 9usize, 2142164u32);
    emu.mul_no_count(24usize, 13usize, 11usize, 2142168u32);
    emu.mul_no_count(6usize, 13usize, 9usize, 2142172u32);
    emu.sltru_no_count(12usize, 5usize, 28usize, 2142176u32);
    emu.adr_no_count(25usize, 23usize, 25usize, 2142180u32);
    emu.adr_no_count(6usize, 7usize, 6usize, 2142184u32);
    emu.adr_no_count(26usize, 24usize, 25usize, 2142188u32);
    emu.sltru_no_count(28usize, 6usize, 7usize, 2142192u32);
    emu.adr_no_count(16usize, 17usize, 26usize, 2142196u32);
    emu.adr_no_count(16usize, 16usize, 28usize, 2142200u32);
    emu.adr_no_count(10usize, 10usize, 12usize, 2142204u32);
    emu.lw_no_count(12usize, 2usize, 20u32, 2142208u32)?;
    emu.add_memory_rw_events(15usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2142216u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b008));
    } else {
        emu.pc = 2142212u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b004));
    }
}
#[inline(always)]
pub fn block_0x0020b004(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 16usize, 17usize, 2142216u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2142216u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b008));
}
#[inline(always)]
pub fn block_0x0020b008(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(5usize, 6usize, 5usize, 2142220u32);
    emu.sltru_no_count(17usize, 5usize, 6usize, 2142224u32);
    emu.adr_no_count(10usize, 16usize, 10usize, 2142228u32);
    emu.adr_no_count(7usize, 10usize, 17usize, 2142232u32);
    emu.mul_no_count(10usize, 15usize, 14usize, 2142236u32);
    emu.add_memory_rw_events(5usize);
    let b = emu.read_reg_b_tracked(16usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a == b {
        emu.pc = 2142244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b024));
    } else {
        emu.pc = 2142240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b020));
    }
}
#[inline(always)]
pub fn block_0x0020b020(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(17usize, 7usize, 16usize, 2142244u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2142244u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b024));
}
#[inline]
pub fn block_0x0020b024(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(27usize, 12usize, 10usize, 2142248u32);
    emu.sltru_no_count(12usize, 27usize, 12usize, 2142252u32);
    emu.adr_no_count(12usize, 31usize, 12usize, 2142256u32);
    emu.sltru_no_count(13usize, 12usize, 31usize, 2142260u32);
    emu.lw_no_count(14usize, 2usize, 12u32, 2142264u32)?;
    emu.adr_no_count(30usize, 14usize, 12usize, 2142268u32);
    emu.sltru_no_count(8usize, 30usize, 14usize, 2142272u32);
    emu.adr_no_count(13usize, 13usize, 8usize, 2142276u32);
    emu.lw_no_count(12usize, 2usize, 16u32, 2142280u32)?;
    emu.adr_no_count(15usize, 12usize, 13usize, 2142284u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2142292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b054));
    } else {
        emu.pc = 2142288u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b050));
    }
}
#[inline(always)]
pub fn block_0x0020b050(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(8usize, 15usize, 12usize, 2142292u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2142292u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b054));
}
#[inline(never)]
pub fn block_0x0020b054(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 25u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(6usize, 31usize, 10usize, 2142296u32);
    emu.sbr_no_count(12usize, 0usize, 10usize, 2142300u32);
    emu.adi_no_count(14usize, 0usize, 4294967295u32, 2142304u32);
    emu.sltru_no_count(12usize, 6usize, 12usize, 2142308u32);
    emu.mulhu_no_count(13usize, 10usize, 14usize, 2142312u32);
    emu.sbr_no_count(13usize, 13usize, 31usize, 2142316u32);
    emu.adr_no_count(12usize, 13usize, 12usize, 2142320u32);
    emu.sbr_no_count(13usize, 0usize, 31usize, 2142324u32);
    emu.adr_no_count(16usize, 8usize, 10usize, 2142328u32);
    emu.mulhu_no_count(31usize, 31usize, 14usize, 2142332u32);
    emu.sltru_no_count(8usize, 16usize, 8usize, 2142336u32);
    emu.adr_no_count(10usize, 16usize, 18usize, 2142340u32);
    emu.adr_no_count(6usize, 6usize, 8usize, 2142344u32);
    emu.sltru_no_count(16usize, 10usize, 16usize, 2142348u32);
    emu.sltru_no_count(13usize, 12usize, 13usize, 2142352u32);
    emu.adr_no_count(13usize, 31usize, 13usize, 2142356u32);
    emu.sltru_no_count(31usize, 0usize, 6usize, 2142360u32);
    emu.adi_no_count(31usize, 31usize, 4294967295u32, 2142364u32);
    emu.anr_no_count(31usize, 31usize, 8usize, 2142368u32);
    emu.adr_no_count(1usize, 6usize, 16usize, 2142372u32);
    emu.adr_no_count(31usize, 12usize, 31usize, 2142376u32);
    emu.sltru_no_count(12usize, 31usize, 12usize, 2142380u32);
    emu.adr_no_count(1usize, 1usize, 29usize, 2142384u32);
    emu.adr_no_count(29usize, 13usize, 12usize, 2142388u32);
    emu.add_memory_rw_events(24usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(1usize);
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
    emu.sltru_no_count(16usize, 1usize, 6usize, 2142396u32);
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
    emu.adr_no_count(16usize, 31usize, 16usize, 2142400u32);
    emu.sltru_no_count(12usize, 16usize, 31usize, 2142404u32);
    emu.adr_no_count(18usize, 22usize, 16usize, 2142408u32);
    emu.adr_no_count(12usize, 29usize, 12usize, 2142412u32);
    emu.sltru_no_count(16usize, 18usize, 22usize, 2142416u32);
    emu.adr_no_count(12usize, 12usize, 16usize, 2142420u32);
    emu.adr_no_count(8usize, 21usize, 12usize, 2142424u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(8usize);
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
    emu.sltru_no_count(16usize, 8usize, 21usize, 2142432u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2142432u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b0e0));
}
#[inline]
pub fn block_0x0020b0e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(29usize, 2usize, 32u32, 2142436u32)?;
    emu.adr_no_count(29usize, 15usize, 29usize, 2142440u32);
    emu.sltru_no_count(12usize, 29usize, 15usize, 2142444u32);
    emu.adr_no_count(12usize, 27usize, 12usize, 2142448u32);
    emu.sltru_no_count(13usize, 12usize, 27usize, 2142452u32);
    emu.adr_no_count(15usize, 10usize, 12usize, 2142456u32);
    emu.sltru_no_count(22usize, 15usize, 10usize, 2142460u32);
    emu.adr_no_count(13usize, 13usize, 22usize, 2142464u32);
    emu.adr_no_count(31usize, 1usize, 13usize, 2142468u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(1usize);
    let a = emu.read_reg_a_tracked(31usize);
    if a == b {
        emu.pc = 2142476u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b10c));
    } else {
        emu.pc = 2142472u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b108));
    }
}
#[inline(always)]
pub fn block_0x0020b108(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(22usize, 31usize, 1usize, 2142476u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2142476u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b10c));
}
#[inline(never)]
pub fn block_0x0020b10c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 31u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(21usize, 25usize, 23usize, 2142480u32);
    emu.lw_no_count(10usize, 2usize, 40u32, 2142484u32)?;
    emu.mulhu_no_count(10usize, 10usize, 9usize, 2142488u32);
    emu.sltru_no_count(9usize, 26usize, 24usize, 2142492u32);
    emu.lw_no_count(12usize, 2usize, 36u32, 2142496u32)?;
    emu.mulhu_no_count(13usize, 12usize, 11usize, 2142500u32);
    emu.lw_no_count(26usize, 2usize, 32u32, 2142504u32)?;
    emu.sbr_no_count(24usize, 27usize, 26usize, 2142508u32);
    emu.sbr_no_count(12usize, 0usize, 26usize, 2142512u32);
    emu.mulhu_no_count(6usize, 26usize, 14usize, 2142516u32);
    emu.sltru_no_count(12usize, 24usize, 12usize, 2142520u32);
    emu.sbr_no_count(6usize, 6usize, 27usize, 2142524u32);
    emu.adr_no_count(12usize, 6usize, 12usize, 2142528u32);
    emu.sbr_no_count(23usize, 0usize, 27usize, 2142532u32);
    emu.mulhu_no_count(25usize, 27usize, 14usize, 2142536u32);
    emu.adr_no_count(26usize, 22usize, 26usize, 2142540u32);
    emu.sltru_no_count(27usize, 26usize, 22usize, 2142544u32);
    emu.adr_no_count(6usize, 26usize, 18usize, 2142548u32);
    emu.adr_no_count(24usize, 24usize, 27usize, 2142552u32);
    emu.sltru_no_count(22usize, 6usize, 26usize, 2142556u32);
    emu.sltru_no_count(18usize, 12usize, 23usize, 2142560u32);
    emu.adr_no_count(25usize, 25usize, 18usize, 2142564u32);
    emu.sltru_no_count(18usize, 0usize, 24usize, 2142568u32);
    emu.adi_no_count(18usize, 18usize, 4294967295u32, 2142572u32);
    emu.anr_no_count(23usize, 18usize, 27usize, 2142576u32);
    emu.adr_no_count(18usize, 24usize, 22usize, 2142580u32);
    emu.adr_no_count(23usize, 12usize, 23usize, 2142584u32);
    emu.sltru_no_count(12usize, 23usize, 12usize, 2142588u32);
    emu.adr_no_count(18usize, 18usize, 8usize, 2142592u32);
    emu.adr_no_count(8usize, 25usize, 12usize, 2142596u32);
    emu.add_memory_rw_events(30usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2142604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b18c));
    } else {
        emu.pc = 2142600u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b188));
    }
}
#[inline(always)]
pub fn block_0x0020b188(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(22usize, 18usize, 24usize, 2142604u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2142604u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b18c));
}
#[inline]
pub fn block_0x0020b18c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(21usize, 10usize, 21usize, 2142608u32);
    emu.adr_no_count(13usize, 13usize, 9usize, 2142612u32);
    emu.adr_no_count(22usize, 23usize, 22usize, 2142616u32);
    emu.adr_no_count(10usize, 16usize, 20usize, 2142620u32);
    emu.sltru_no_count(12usize, 22usize, 23usize, 2142624u32);
    emu.sltru_no_count(16usize, 10usize, 16usize, 2142628u32);
    emu.adr_no_count(20usize, 10usize, 22usize, 2142632u32);
    emu.adr_no_count(12usize, 8usize, 12usize, 2142636u32);
    emu.adr_no_count(19usize, 19usize, 16usize, 2142640u32);
    emu.sltru_no_count(8usize, 20usize, 10usize, 2142644u32);
    emu.sltru_no_count(10usize, 0usize, 19usize, 2142648u32);
    emu.adr_no_count(12usize, 19usize, 12usize, 2142652u32);
    emu.adi_no_count(10usize, 10usize, 4294967295u32, 2142656u32);
    emu.adr_no_count(9usize, 12usize, 8usize, 2142660u32);
    emu.anr_no_count(22usize, 10usize, 16usize, 2142664u32);
    emu.add_memory_rw_events(15usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2142672u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b1d0));
    } else {
        emu.pc = 2142668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b1cc));
    }
}
#[inline(always)]
pub fn block_0x0020b1cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(8usize, 9usize, 19usize, 2142672u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2142672u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b1d0));
}
#[inline]
pub fn block_0x0020b1d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(23usize, 21usize, 13usize, 2142676u32);
    emu.lw_no_count(10usize, 2usize, 40u32, 2142680u32)?;
    emu.mul_no_count(19usize, 10usize, 11usize, 2142684u32);
    emu.adr_no_count(16usize, 31usize, 30usize, 2142688u32);
    emu.sltru_no_count(10usize, 16usize, 31usize, 2142692u32);
    emu.adr_no_count(10usize, 29usize, 10usize, 2142696u32);
    emu.sltru_no_count(12usize, 10usize, 29usize, 2142700u32);
    emu.adr_no_count(13usize, 6usize, 10usize, 2142704u32);
    emu.adr_no_count(12usize, 18usize, 12usize, 2142708u32);
    emu.sltru_no_count(10usize, 13usize, 6usize, 2142712u32);
    emu.adr_no_count(6usize, 12usize, 10usize, 2142716u32);
    emu.adr_no_count(8usize, 22usize, 8usize, 2142720u32);
    emu.add_memory_rw_events(12usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2142728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b208));
    } else {
        emu.pc = 2142724u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b204));
    }
}
#[inline(always)]
pub fn block_0x0020b204(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 6usize, 18usize, 2142728u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2142728u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b208));
}
#[inline]
pub fn block_0x0020b208(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 20u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(31usize, 19usize, 23usize, 2142732u32);
    emu.sltru_no_count(21usize, 23usize, 21usize, 2142736u32);
    emu.lw_no_count(12usize, 2usize, 40u32, 2142740u32)?;
    emu.mulhu_no_count(12usize, 12usize, 11usize, 2142744u32);
    emu.sltru_no_count(11usize, 8usize, 22usize, 2142748u32);
    emu.sbr_no_count(22usize, 29usize, 30usize, 2142752u32);
    emu.sbr_no_count(18usize, 0usize, 30usize, 2142756u32);
    emu.mulhu_no_count(23usize, 30usize, 14usize, 2142760u32);
    emu.sltru_no_count(18usize, 22usize, 18usize, 2142764u32);
    emu.sbr_no_count(23usize, 23usize, 29usize, 2142768u32);
    emu.adr_no_count(18usize, 23usize, 18usize, 2142772u32);
    emu.sbr_no_count(23usize, 0usize, 29usize, 2142776u32);
    emu.mulhu_no_count(24usize, 29usize, 14usize, 2142780u32);
    emu.adr_no_count(30usize, 20usize, 30usize, 2142784u32);
    emu.sltru_no_count(29usize, 30usize, 20usize, 2142788u32);
    emu.adr_no_count(20usize, 22usize, 29usize, 2142792u32);
    emu.sltru_no_count(22usize, 18usize, 23usize, 2142796u32);
    emu.adr_no_count(20usize, 9usize, 20usize, 2142800u32);
    emu.adr_no_count(22usize, 24usize, 22usize, 2142804u32);
    emu.add_memory_rw_events(19usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(20usize);
    if a == b {
        emu.pc = 2142812u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b25c));
    } else {
        emu.pc = 2142808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b258));
    }
}
#[inline(always)]
pub fn block_0x0020b258(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(29usize, 20usize, 9usize, 2142812u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2142812u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b25c));
}
#[inline]
pub fn block_0x0020b25c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(9usize, 31usize, 19usize, 2142816u32);
    emu.adr_no_count(19usize, 12usize, 21usize, 2142820u32);
    emu.adr_no_count(12usize, 31usize, 28usize, 2142824u32);
    emu.adr_no_count(21usize, 18usize, 29usize, 2142828u32);
    emu.adr_no_count(28usize, 30usize, 10usize, 2142832u32);
    emu.adr_no_count(5usize, 8usize, 5usize, 2142836u32);
    emu.adr_no_count(7usize, 11usize, 7usize, 2142840u32);
    emu.sltru_no_count(10usize, 21usize, 18usize, 2142844u32);
    emu.sltru_no_count(18usize, 28usize, 30usize, 2142848u32);
    emu.sltru_no_count(29usize, 5usize, 8usize, 2142852u32);
    emu.adr_no_count(22usize, 22usize, 10usize, 2142856u32);
    emu.adr_no_count(30usize, 20usize, 18usize, 2142860u32);
    emu.sltru_no_count(10usize, 30usize, 20usize, 2142864u32);
    emu.anr_no_count(10usize, 18usize, 10usize, 2142868u32);
    emu.adr_no_count(10usize, 21usize, 10usize, 2142872u32);
    emu.sltru_no_count(18usize, 10usize, 21usize, 2142876u32);
    emu.adr_no_count(8usize, 7usize, 29usize, 2142880u32);
    emu.adr_no_count(7usize, 22usize, 18usize, 2142884u32);
    emu.add_memory_rw_events(18usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2142892u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b2ac));
    } else {
        emu.pc = 2142888u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b2a8));
    }
}
#[inline(always)]
pub fn block_0x0020b2a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(29usize, 8usize, 11usize, 2142892u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2142892u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b2ac));
}
#[inline(always)]
pub fn block_0x0020b2ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(19usize, 19usize, 9usize, 2142896u32);
    emu.sltru_no_count(11usize, 12usize, 31usize, 2142900u32);
    emu.adr_no_count(7usize, 8usize, 7usize, 2142904u32);
    emu.adr_no_count(10usize, 5usize, 10usize, 2142908u32);
    emu.sltru_no_count(9usize, 10usize, 5usize, 2142912u32);
    emu.adr_no_count(5usize, 7usize, 9usize, 2142916u32);
    emu.adr_no_count(7usize, 12usize, 17usize, 2142920u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2142928u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b2d0));
    } else {
        emu.pc = 2142924u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b2cc));
    }
}
#[inline(always)]
pub fn block_0x0020b2cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(9usize, 5usize, 8usize, 2142928u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2142928u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b2d0));
}
#[inline]
pub fn block_0x0020b2d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 19usize, 11usize, 2142932u32);
    emu.sltru_no_count(18usize, 7usize, 12usize, 2142936u32);
    emu.adr_no_count(11usize, 6usize, 15usize, 2142940u32);
    emu.sltru_no_count(12usize, 11usize, 6usize, 2142944u32);
    emu.adr_no_count(12usize, 16usize, 12usize, 2142948u32);
    emu.sltru_no_count(17usize, 12usize, 16usize, 2142952u32);
    emu.adr_no_count(12usize, 28usize, 12usize, 2142956u32);
    emu.sltru_no_count(31usize, 12usize, 28usize, 2142960u32);
    emu.adr_no_count(17usize, 17usize, 31usize, 2142964u32);
    emu.adr_no_count(17usize, 30usize, 17usize, 2142968u32);
    emu.adr_no_count(6usize, 29usize, 9usize, 2142972u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(30usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2142980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b304));
    } else {
        emu.pc = 2142976u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b300));
    }
}
#[inline(always)]
pub fn block_0x0020b300(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(31usize, 17usize, 30usize, 2142980u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2142980u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b304));
}
#[inline]
pub fn block_0x0020b304(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(30usize, 8usize, 18usize, 2142984u32);
    emu.sltru_no_count(28usize, 6usize, 29usize, 2142988u32);
    emu.sbr_no_count(8usize, 16usize, 15usize, 2142992u32);
    emu.sbr_no_count(29usize, 0usize, 15usize, 2142996u32);
    emu.mulhu_no_count(9usize, 15usize, 14usize, 2143000u32);
    emu.mulhu_no_count(18usize, 16usize, 14usize, 2143004u32);
    emu.sbr_no_count(14usize, 9usize, 16usize, 2143008u32);
    emu.sbr_no_count(16usize, 0usize, 16usize, 2143012u32);
    emu.adr_no_count(15usize, 10usize, 15usize, 2143016u32);
    emu.sltru_no_count(9usize, 8usize, 29usize, 2143020u32);
    emu.sltru_no_count(29usize, 15usize, 10usize, 2143024u32);
    emu.adr_no_count(14usize, 14usize, 9usize, 2143028u32);
    emu.adr_no_count(10usize, 8usize, 29usize, 2143032u32);
    emu.sltru_no_count(16usize, 14usize, 16usize, 2143036u32);
    emu.adr_no_count(10usize, 5usize, 10usize, 2143040u32);
    emu.adr_no_count(16usize, 18usize, 16usize, 2143044u32);
    emu.add_memory_rw_events(16usize);
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2143052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b34c));
    } else {
        emu.pc = 2143048u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b348));
    }
}
#[inline(always)]
pub fn block_0x0020b348(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(29usize, 10usize, 5usize, 2143052u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2143052u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b34c));
}
#[inline]
pub fn block_0x0020b34c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 16u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(29usize, 14usize, 29usize, 2143056u32);
    emu.adr_no_count(5usize, 15usize, 31usize, 2143060u32);
    emu.adr_no_count(7usize, 6usize, 7usize, 2143064u32);
    emu.adr_no_count(30usize, 28usize, 30usize, 2143068u32);
    emu.sltru_no_count(14usize, 29usize, 14usize, 2143072u32);
    emu.sltru_no_count(31usize, 5usize, 15usize, 2143076u32);
    emu.sltru_no_count(15usize, 7usize, 6usize, 2143080u32);
    emu.adr_no_count(16usize, 16usize, 14usize, 2143084u32);
    emu.adr_no_count(6usize, 10usize, 31usize, 2143088u32);
    emu.sltru_no_count(10usize, 6usize, 10usize, 2143092u32);
    emu.anr_no_count(14usize, 31usize, 10usize, 2143096u32);
    emu.adr_no_count(14usize, 29usize, 14usize, 2143100u32);
    emu.sltru_no_count(29usize, 14usize, 29usize, 2143104u32);
    emu.adr_no_count(10usize, 30usize, 15usize, 2143108u32);
    emu.adr_no_count(16usize, 16usize, 29usize, 2143112u32);
    emu.add_memory_rw_events(15usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2143120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b390));
    } else {
        emu.pc = 2143116u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b38c));
    }
}
#[inline(always)]
pub fn block_0x0020b38c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(15usize, 10usize, 28usize, 2143120u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2143120u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b390));
}
#[inline(always)]
pub fn block_0x0020b390(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(16usize, 10usize, 16usize, 2143124u32);
    emu.adr_no_count(28usize, 7usize, 14usize, 2143128u32);
    emu.sltru_no_count(29usize, 28usize, 7usize, 2143132u32);
    emu.adr_no_count(7usize, 16usize, 29usize, 2143136u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a == b {
        emu.pc = 2143144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b3a8));
    } else {
        emu.pc = 2143140u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b3a4));
    }
}
#[inline(always)]
pub fn block_0x0020b3a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(29usize, 7usize, 10usize, 2143144u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2143144u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b3a8));
}
#[inline]
pub fn block_0x0020b3a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(13usize, 13usize, 1u32, 2143148u32);
    emu.sltiu_no_count(10usize, 13usize, 1u32, 2143152u32);
    emu.adr_no_count(11usize, 11usize, 10usize, 2143156u32);
    emu.orr_no_count(10usize, 13usize, 11usize, 2143160u32);
    emu.sltiu_no_count(30usize, 10usize, 1u32, 2143164u32);
    emu.adi_no_count(30usize, 30usize, 4294967295u32, 2143168u32);
    emu.adr_no_count(17usize, 30usize, 17usize, 2143172u32);
    emu.adr_no_count(16usize, 30usize, 12usize, 2143176u32);
    emu.sltru_no_count(10usize, 16usize, 30usize, 2143180u32);
    emu.adr_no_count(17usize, 17usize, 10usize, 2143184u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(30usize);
    let a = emu.read_reg_a_tracked(17usize);
    if a == b {
        emu.pc = 2143192u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b3d8));
    } else {
        emu.pc = 2143188u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b3d4));
    }
}
#[inline(always)]
pub fn block_0x0020b3d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 17usize, 30usize, 2143192u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2143192u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b3d8));
}
#[inline(always)]
pub fn block_0x0020b3d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 30usize, 10usize, 2143196u32);
    emu.adi_no_count(14usize, 16usize, 1u32, 2143200u32);
    emu.sltru_no_count(31usize, 10usize, 30usize, 2143204u32);
    emu.sltiu_no_count(12usize, 14usize, 1u32, 2143208u32);
    emu.adr_no_count(12usize, 17usize, 12usize, 2143212u32);
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2143216u32);
    emu.adr_no_count(30usize, 30usize, 31usize, 2143220u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2143232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b400));
    } else {
        emu.pc = 2143224u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b3f8));
    }
}
#[inline(always)]
pub fn block_0x0020b3f8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 12usize, 17usize, 2143228u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2143232u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2143236u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b404));
}
#[inline(always)]
pub fn block_0x0020b400(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 14usize, 16usize, 2143236u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2143236u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b404));
}
#[inline]
pub fn block_0x0020b404(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(17usize, 10usize, 4294967295u32, 2143240u32);
    emu.sltiu_no_count(10usize, 10usize, 1u32, 2143244u32);
    emu.adr_no_count(16usize, 17usize, 16usize, 2143248u32);
    emu.sbr_no_count(10usize, 30usize, 10usize, 2143252u32);
    emu.sltru_no_count(16usize, 16usize, 17usize, 2143256u32);
    emu.adr_no_count(10usize, 10usize, 16usize, 2143260u32);
    emu.sai_no_count(10usize, 10usize, 1055u32, 2143264u32);
    emu.adr_no_count(17usize, 6usize, 10usize, 2143268u32);
    emu.adr_no_count(16usize, 5usize, 10usize, 2143272u32);
    emu.sltru_no_count(5usize, 16usize, 5usize, 2143276u32);
    emu.adr_no_count(17usize, 17usize, 5usize, 2143280u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(17usize);
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
    emu.sltru_no_count(5usize, 17usize, 6usize, 2143288u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2143288u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b438));
}
#[inline]
pub fn block_0x0020b438(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(5usize, 10usize, 5usize, 2143292u32);
    emu.sltru_no_count(5usize, 5usize, 10usize, 2143296u32);
    emu.adr_no_count(10usize, 10usize, 5usize, 2143300u32);
    emu.sai_no_count(30usize, 10usize, 1055u32, 2143304u32);
    emu.adr_no_count(10usize, 7usize, 30usize, 2143308u32);
    emu.adr_no_count(6usize, 28usize, 30usize, 2143312u32);
    emu.sltru_no_count(5usize, 6usize, 28usize, 2143316u32);
    emu.adr_no_count(28usize, 10usize, 5usize, 2143320u32);
    emu.adr_no_count(29usize, 15usize, 29usize, 2143324u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2143332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b464));
    } else {
        emu.pc = 2143328u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b460));
    }
}
#[inline(always)]
pub fn block_0x0020b460(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 28usize, 7usize, 2143332u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2143332u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b464));
}
#[inline(always)]
pub fn block_0x0020b464(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(10usize, 29usize, 15usize, 2143336u32);
    emu.adr_no_count(7usize, 30usize, 5usize, 2143340u32);
    emu.sltru_no_count(5usize, 0usize, 6usize, 2143344u32);
    emu.sltru_no_count(15usize, 7usize, 30usize, 2143348u32);
    emu.adr_no_count(5usize, 28usize, 5usize, 2143352u32);
    emu.adr_no_count(30usize, 30usize, 15usize, 2143356u32);
    emu.adi_no_count(15usize, 6usize, 4294967295u32, 2143360u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2143372u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b48c));
    } else {
        emu.pc = 2143364u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b484));
    }
}
#[inline(always)]
pub fn block_0x0020b484(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(6usize, 5usize, 28usize, 2143368u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2143372u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2143376u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b490));
}
#[inline(always)]
pub fn block_0x0020b48c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(6usize, 15usize, 6usize, 2143376u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2143376u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b490));
}
#[inline]
pub fn block_0x0020b490(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(28usize, 7usize, 4294967295u32, 2143380u32);
    emu.sltiu_no_count(7usize, 7usize, 1u32, 2143384u32);
    emu.adr_no_count(6usize, 28usize, 6usize, 2143388u32);
    emu.sbr_no_count(7usize, 30usize, 7usize, 2143392u32);
    emu.sltru_no_count(6usize, 6usize, 28usize, 2143396u32);
    emu.adr_no_count(6usize, 7usize, 6usize, 2143400u32);
    emu.sai_no_count(7usize, 6usize, 1055u32, 2143404u32);
    emu.adr_no_count(29usize, 7usize, 29usize, 2143408u32);
    emu.sltru_no_count(6usize, 29usize, 7usize, 2143412u32);
    emu.adr_no_count(10usize, 7usize, 10usize, 2143416u32);
    emu.adr_no_count(10usize, 10usize, 6usize, 2143420u32);
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2143428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b4c4));
    } else {
        emu.pc = 2143424u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b4c0));
    }
}
#[inline(always)]
pub fn block_0x0020b4c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(6usize, 10usize, 7usize, 2143428u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2143428u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b4c4));
}
#[inline(always)]
pub fn block_0x0020b4c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(6usize, 7usize, 6usize, 2143432u32);
    emu.sltru_no_count(10usize, 6usize, 7usize, 2143436u32);
    emu.adr_no_count(13usize, 6usize, 13usize, 2143440u32);
    emu.adr_no_count(7usize, 7usize, 10usize, 2143444u32);
    emu.sltru_no_count(28usize, 13usize, 6usize, 2143448u32);
    emu.adr_no_count(10usize, 7usize, 11usize, 2143452u32);
    emu.adr_no_count(10usize, 10usize, 28usize, 2143456u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2143464u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b4e8));
    } else {
        emu.pc = 2143460u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b4e4));
    }
}
#[inline(always)]
pub fn block_0x0020b4e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 10usize, 7usize, 2143464u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2143464u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b4e8));
}
#[inline]
pub fn block_0x0020b4e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(14usize, 6usize, 14usize, 2143468u32);
    emu.sltru_no_count(29usize, 14usize, 6usize, 2143472u32);
    emu.adr_no_count(11usize, 14usize, 28usize, 2143476u32);
    emu.adr_no_count(28usize, 12usize, 29usize, 2143480u32);
    emu.sltru_no_count(14usize, 11usize, 14usize, 2143484u32);
    emu.sltru_no_count(12usize, 0usize, 28usize, 2143488u32);
    emu.adi_no_count(12usize, 12usize, 4294967295u32, 2143492u32);
    emu.anr_no_count(29usize, 12usize, 29usize, 2143496u32);
    emu.adr_no_count(12usize, 28usize, 14usize, 2143500u32);
    emu.sltru_no_count(28usize, 12usize, 28usize, 2143504u32);
    emu.anr_no_count(14usize, 14usize, 28usize, 2143508u32);
    emu.adr_no_count(14usize, 29usize, 14usize, 2143512u32);
    emu.sltru_no_count(29usize, 14usize, 29usize, 2143516u32);
    emu.adr_no_count(16usize, 14usize, 16usize, 2143520u32);
    emu.sltru_no_count(28usize, 16usize, 14usize, 2143524u32);
    emu.adr_no_count(14usize, 29usize, 17usize, 2143528u32);
    emu.adr_no_count(14usize, 14usize, 28usize, 2143532u32);
    emu.add_memory_rw_events(17usize);
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(14usize);
    if a == b {
        emu.pc = 2143540u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b534));
    } else {
        emu.pc = 2143536u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b530));
    }
}
#[inline(always)]
pub fn block_0x0020b530(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 14usize, 29usize, 2143540u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2143540u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b534));
}
#[inline(never)]
pub fn block_0x0020b534(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 32u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(17usize, 6usize, 1u32, 2143544u32);
    emu.adr_no_count(5usize, 7usize, 5usize, 2143548u32);
    emu.lw_no_count(6usize, 2usize, 28u32, 2143552u32)?;
    emu.sw_no_count(13usize, 6usize, 0u32, 2143556u32)?;
    emu.sw_no_count(10usize, 6usize, 4u32, 2143560u32)?;
    emu.sw_no_count(11usize, 6usize, 8u32, 2143564u32)?;
    emu.sw_no_count(12usize, 6usize, 12u32, 2143568u32)?;
    emu.adr_no_count(15usize, 17usize, 15usize, 2143572u32);
    emu.sltru_no_count(10usize, 15usize, 17usize, 2143576u32);
    emu.adr_no_count(28usize, 15usize, 28usize, 2143580u32);
    emu.adr_no_count(10usize, 5usize, 10usize, 2143584u32);
    emu.sltru_no_count(11usize, 28usize, 15usize, 2143588u32);
    emu.adr_no_count(10usize, 10usize, 11usize, 2143592u32);
    emu.sw_no_count(16usize, 6usize, 16u32, 2143596u32)?;
    emu.sw_no_count(14usize, 6usize, 20u32, 2143600u32)?;
    emu.sw_no_count(28usize, 6usize, 24u32, 2143604u32)?;
    emu.sw_no_count(10usize, 6usize, 28u32, 2143608u32)?;
    emu.lw_no_count(1usize, 2usize, 92u32, 2143612u32)?;
    emu.lw_no_count(8usize, 2usize, 88u32, 2143616u32)?;
    emu.lw_no_count(9usize, 2usize, 84u32, 2143620u32)?;
    emu.lw_no_count(18usize, 2usize, 80u32, 2143624u32)?;
    emu.lw_no_count(19usize, 2usize, 76u32, 2143628u32)?;
    emu.lw_no_count(20usize, 2usize, 72u32, 2143632u32)?;
    emu.lw_no_count(21usize, 2usize, 68u32, 2143636u32)?;
    emu.lw_no_count(22usize, 2usize, 64u32, 2143640u32)?;
    emu.lw_no_count(23usize, 2usize, 60u32, 2143644u32)?;
    emu.lw_no_count(24usize, 2usize, 56u32, 2143648u32)?;
    emu.lw_no_count(25usize, 2usize, 52u32, 2143652u32)?;
    emu.lw_no_count(26usize, 2usize, 48u32, 2143656u32)?;
    emu.lw_no_count(27usize, 2usize, 44u32, 2143660u32)?;
    emu.adi_no_count(2usize, 2usize, 96u32, 2143664u32);
    emu.add_memory_rw_events(32usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2143668u32;
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
pub fn block_0x0020b5b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2143672u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2143676u32)?;
    emu.adi_no_count(12usize, 11usize, 0u32, 2143680u32);
    let a = 0u32.wrapping_add(2228224u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2143684u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 856u32, 2143688u32);
    emu.apc_no_count(1usize, 2143688u32, 4294963200u32, 2143692u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2143696u32;
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
pub fn block_0x0020b5d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2143700u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2143704u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2143708u32;
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
pub fn block_0x0020b5dc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 68u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967232u32, 2143712u32);
    emu.sw_no_count(1usize, 2usize, 60u32, 2143716u32)?;
    emu.sw_no_count(8usize, 2usize, 56u32, 2143720u32)?;
    emu.sw_no_count(9usize, 2usize, 52u32, 2143724u32)?;
    emu.sw_no_count(18usize, 2usize, 48u32, 2143728u32)?;
    emu.sw_no_count(19usize, 2usize, 44u32, 2143732u32)?;
    emu.sw_no_count(20usize, 2usize, 40u32, 2143736u32)?;
    emu.sw_no_count(21usize, 2usize, 36u32, 2143740u32)?;
    emu.sw_no_count(22usize, 2usize, 32u32, 2143744u32)?;
    emu.sw_no_count(23usize, 2usize, 28u32, 2143748u32)?;
    emu.sw_no_count(24usize, 2usize, 24u32, 2143752u32)?;
    emu.sw_no_count(25usize, 2usize, 20u32, 2143756u32)?;
    emu.sw_no_count(26usize, 2usize, 16u32, 2143760u32)?;
    emu.sw_no_count(27usize, 2usize, 12u32, 2143764u32)?;
    emu.lw_no_count(29usize, 11usize, 0u32, 2143768u32)?;
    emu.lw_no_count(6usize, 11usize, 4u32, 2143772u32)?;
    let a = 0u32.wrapping_add(4007632896u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2143776u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(19922944u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2143780u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(3743051776u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2143784u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1125711872u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2143788u32;
    emu.update_insn_clock();
    emu.adi_no_count(17usize, 12usize, 4294966270u32, 2143792u32);
    emu.adi_no_count(16usize, 13usize, 4294966661u32, 2143796u32);
    emu.adi_no_count(12usize, 14usize, 4294966305u32, 2143800u32);
    emu.adi_no_count(13usize, 15usize, 1362u32, 2143804u32);
    emu.mulhu_no_count(14usize, 29usize, 17usize, 2143808u32);
    emu.mul_no_count(15usize, 6usize, 17usize, 2143812u32);
    emu.mulhu_no_count(5usize, 6usize, 17usize, 2143816u32);
    emu.mul_no_count(7usize, 29usize, 16usize, 2143820u32);
    emu.mulhu_no_count(28usize, 29usize, 16usize, 2143824u32);
    emu.mul_no_count(30usize, 6usize, 16usize, 2143828u32);
    emu.mulhu_no_count(31usize, 29usize, 12usize, 2143832u32);
    emu.adr_no_count(14usize, 15usize, 14usize, 2143836u32);
    emu.sltru_no_count(15usize, 14usize, 15usize, 2143840u32);
    emu.adr_no_count(15usize, 5usize, 15usize, 2143844u32);
    emu.mul_no_count(5usize, 6usize, 12usize, 2143848u32);
    emu.adr_no_count(14usize, 7usize, 14usize, 2143852u32);
    emu.sltru_no_count(14usize, 14usize, 7usize, 2143856u32);
    emu.mulhu_no_count(7usize, 6usize, 12usize, 2143860u32);
    emu.adr_no_count(31usize, 5usize, 31usize, 2143864u32);
    emu.sltru_no_count(5usize, 31usize, 5usize, 2143868u32);
    emu.adr_no_count(5usize, 7usize, 5usize, 2143872u32);
    emu.mul_no_count(7usize, 29usize, 13usize, 2143876u32);
    emu.adr_no_count(14usize, 28usize, 14usize, 2143880u32);
    emu.mulhu_no_count(28usize, 29usize, 13usize, 2143884u32);
    emu.adr_no_count(31usize, 7usize, 31usize, 2143888u32);
    emu.sltru_no_count(7usize, 31usize, 7usize, 2143892u32);
    emu.adr_no_count(7usize, 28usize, 7usize, 2143896u32);
    emu.mulhu_no_count(28usize, 6usize, 16usize, 2143900u32);
    emu.adr_no_count(14usize, 15usize, 14usize, 2143904u32);
    emu.sltru_no_count(15usize, 14usize, 15usize, 2143908u32);
    emu.adr_no_count(15usize, 28usize, 15usize, 2143912u32);
    emu.mulhu_no_count(28usize, 6usize, 13usize, 2143916u32);
    emu.adr_no_count(7usize, 5usize, 7usize, 2143920u32);
    emu.sltru_no_count(5usize, 7usize, 5usize, 2143924u32);
    emu.adr_no_count(28usize, 28usize, 5usize, 2143928u32);
    emu.mul_no_count(8usize, 6usize, 13usize, 2143932u32);
    emu.adr_no_count(9usize, 30usize, 14usize, 2143936u32);
    emu.sltru_no_count(14usize, 9usize, 30usize, 2143940u32);
    emu.adr_no_count(14usize, 15usize, 14usize, 2143944u32);
    emu.mul_no_count(19usize, 29usize, 12usize, 2143948u32);
    emu.adr_no_count(5usize, 8usize, 7usize, 2143952u32);
    emu.sltru_no_count(7usize, 5usize, 8usize, 2143956u32);
    emu.adr_no_count(19usize, 9usize, 19usize, 2143960u32);
    emu.sltru_no_count(15usize, 19usize, 9usize, 2143964u32);
    emu.adr_no_count(31usize, 14usize, 31usize, 2143968u32);
    emu.adr_no_count(8usize, 31usize, 15usize, 2143972u32);
    emu.adr_no_count(7usize, 28usize, 7usize, 2143976u32);
    emu.add_memory_rw_events(67usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2143984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b6f0));
    } else {
        emu.pc = 2143980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b6ec));
    }
}
#[inline(always)]
pub fn block_0x0020b6ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(15usize, 8usize, 14usize, 2143984u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2143984u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b6f0));
}
#[inline(never)]
pub fn block_0x0020b6f0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 31u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(31usize, 5usize, 15usize, 2143988u32);
    emu.sbr_no_count(30usize, 0usize, 6usize, 2143992u32);
    emu.adi_no_count(15usize, 0usize, 4294967295u32, 2143996u32);
    emu.adr_no_count(9usize, 29usize, 29usize, 2144000u32);
    emu.adi_no_count(14usize, 0usize, 4294967294u32, 2144004u32);
    emu.adr_no_count(18usize, 6usize, 6usize, 2144008u32);
    emu.sltru_no_count(28usize, 31usize, 5usize, 2144012u32);
    emu.mulhu_no_count(5usize, 29usize, 15usize, 2144016u32);
    emu.mulhu_no_count(20usize, 6usize, 15usize, 2144020u32);
    emu.sbr_no_count(21usize, 0usize, 9usize, 2144024u32);
    emu.adr_no_count(28usize, 7usize, 28usize, 2144028u32);
    emu.mulhu_no_count(22usize, 29usize, 14usize, 2144032u32);
    emu.sbr_no_count(7usize, 5usize, 6usize, 2144036u32);
    emu.sltru_no_count(5usize, 7usize, 30usize, 2144040u32);
    emu.adr_no_count(5usize, 20usize, 5usize, 2144044u32);
    emu.sbr_no_count(20usize, 0usize, 18usize, 2144048u32);
    emu.sbr_no_count(9usize, 7usize, 9usize, 2144052u32);
    emu.sltru_no_count(30usize, 9usize, 21usize, 2144056u32);
    emu.adr_no_count(30usize, 22usize, 30usize, 2144060u32);
    emu.mulhu_no_count(21usize, 6usize, 14usize, 2144064u32);
    emu.adr_no_count(22usize, 5usize, 30usize, 2144068u32);
    emu.sbr_no_count(30usize, 22usize, 18usize, 2144072u32);
    emu.sltru_no_count(18usize, 22usize, 5usize, 2144076u32);
    emu.adr_no_count(21usize, 21usize, 18usize, 2144080u32);
    emu.sbr_no_count(18usize, 31usize, 29usize, 2144084u32);
    emu.sltru_no_count(31usize, 18usize, 31usize, 2144088u32);
    emu.adr_no_count(9usize, 28usize, 9usize, 2144092u32);
    emu.sltru_no_count(20usize, 30usize, 20usize, 2144096u32);
    emu.adr_no_count(9usize, 9usize, 31usize, 2144100u32);
    emu.adr_no_count(20usize, 21usize, 20usize, 2144104u32);
    emu.add_memory_rw_events(30usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2144112u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b770));
    } else {
        emu.pc = 2144108u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b76c));
    }
}
#[inline(always)]
pub fn block_0x0020b76c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(31usize, 9usize, 28usize, 2144112u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2144112u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b770));
}
#[inline(always)]
pub fn block_0x0020b770(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(28usize, 30usize, 31usize, 2144116u32);
    emu.sltru_no_count(30usize, 28usize, 30usize, 2144120u32);
    emu.adr_no_count(20usize, 20usize, 30usize, 2144124u32);
    emu.sbr_no_count(31usize, 28usize, 29usize, 2144128u32);
    emu.sltru_no_count(28usize, 31usize, 28usize, 2144132u32);
    emu.adr_no_count(7usize, 20usize, 7usize, 2144136u32);
    emu.adr_no_count(30usize, 7usize, 28usize, 2144140u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(20usize);
    let a = emu.read_reg_a_tracked(30usize);
    if a == b {
        emu.pc = 2144148u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b794));
    } else {
        emu.pc = 2144144u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b790));
    }
}
#[inline(always)]
pub fn block_0x0020b790(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(28usize, 30usize, 20usize, 2144148u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2144148u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b794));
}
#[inline(always)]
pub fn block_0x0020b794(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(28usize, 5usize, 28usize, 2144152u32);
    emu.sltru_no_count(7usize, 28usize, 5usize, 2144156u32);
    emu.adr_no_count(29usize, 28usize, 29usize, 2144160u32);
    emu.sltru_no_count(5usize, 29usize, 28usize, 2144164u32);
    emu.adr_no_count(6usize, 7usize, 6usize, 2144168u32);
    emu.adr_no_count(28usize, 6usize, 5usize, 2144172u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2144180u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b7b4));
    } else {
        emu.pc = 2144176u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b7b0));
    }
}
#[inline(always)]
pub fn block_0x0020b7b0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 28usize, 7usize, 2144180u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2144180u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b7b4));
}
#[inline(never)]
pub fn block_0x0020b7b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(7usize, 11usize, 8u32, 2144184u32)?;
    emu.lw_no_count(6usize, 11usize, 12u32, 2144188u32)?;
    emu.ani_no_count(19usize, 19usize, 4294967294u32, 2144192u32);
    emu.mulhu_no_count(21usize, 7usize, 17usize, 2144196u32);
    emu.mul_no_count(22usize, 6usize, 17usize, 2144200u32);
    emu.mulhu_no_count(23usize, 6usize, 17usize, 2144204u32);
    emu.mul_no_count(24usize, 7usize, 16usize, 2144208u32);
    emu.mul_no_count(20usize, 7usize, 17usize, 2144212u32);
    emu.adr_no_count(20usize, 19usize, 20usize, 2144216u32);
    emu.sltru_no_count(20usize, 20usize, 19usize, 2144220u32);
    emu.mulhu_no_count(19usize, 7usize, 16usize, 2144224u32);
    emu.adr_no_count(21usize, 22usize, 21usize, 2144228u32);
    emu.sltru_no_count(22usize, 21usize, 22usize, 2144232u32);
    emu.adr_no_count(22usize, 23usize, 22usize, 2144236u32);
    emu.mul_no_count(23usize, 6usize, 16usize, 2144240u32);
    emu.adr_no_count(21usize, 24usize, 21usize, 2144244u32);
    emu.sltru_no_count(24usize, 21usize, 24usize, 2144248u32);
    emu.adr_no_count(19usize, 19usize, 24usize, 2144252u32);
    emu.mulhu_no_count(24usize, 6usize, 16usize, 2144256u32);
    emu.adr_no_count(21usize, 8usize, 21usize, 2144260u32);
    emu.adr_no_count(19usize, 22usize, 19usize, 2144264u32);
    emu.sltru_no_count(22usize, 19usize, 22usize, 2144268u32);
    emu.adr_no_count(19usize, 23usize, 19usize, 2144272u32);
    emu.sltru_no_count(23usize, 19usize, 23usize, 2144276u32);
    emu.adr_no_count(24usize, 24usize, 22usize, 2144280u32);
    emu.adr_no_count(22usize, 21usize, 20usize, 2144284u32);
    emu.adr_no_count(21usize, 24usize, 23usize, 2144288u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a == b {
        emu.pc = 2144296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b828));
    } else {
        emu.pc = 2144292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b824));
    }
}
#[inline(always)]
pub fn block_0x0020b824(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(20usize, 22usize, 8usize, 2144296u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2144296u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b828));
}
#[inline(never)]
pub fn block_0x0020b828(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(20usize, 19usize, 20usize, 2144300u32);
    emu.mulhu_no_count(22usize, 7usize, 12usize, 2144304u32);
    emu.mul_no_count(23usize, 6usize, 12usize, 2144308u32);
    emu.mulhu_no_count(24usize, 6usize, 12usize, 2144312u32);
    emu.mul_no_count(25usize, 7usize, 13usize, 2144316u32);
    emu.sltru_no_count(8usize, 20usize, 19usize, 2144320u32);
    emu.adr_no_count(8usize, 21usize, 8usize, 2144324u32);
    emu.mulhu_no_count(19usize, 7usize, 13usize, 2144328u32);
    emu.adr_no_count(22usize, 23usize, 22usize, 2144332u32);
    emu.sltru_no_count(21usize, 22usize, 23usize, 2144336u32);
    emu.adr_no_count(21usize, 24usize, 21usize, 2144340u32);
    emu.mul_no_count(23usize, 6usize, 13usize, 2144344u32);
    emu.adr_no_count(22usize, 25usize, 22usize, 2144348u32);
    emu.sltru_no_count(24usize, 22usize, 25usize, 2144352u32);
    emu.adr_no_count(19usize, 19usize, 24usize, 2144356u32);
    emu.mulhu_no_count(24usize, 6usize, 13usize, 2144360u32);
    emu.adr_no_count(19usize, 21usize, 19usize, 2144364u32);
    emu.sltru_no_count(21usize, 19usize, 21usize, 2144368u32);
    emu.adr_no_count(24usize, 24usize, 21usize, 2144372u32);
    emu.mul_no_count(21usize, 7usize, 12usize, 2144376u32);
    emu.adr_no_count(21usize, 18usize, 21usize, 2144380u32);
    emu.sltru_no_count(18usize, 21usize, 18usize, 2144384u32);
    emu.adr_no_count(25usize, 22usize, 18usize, 2144388u32);
    emu.adr_no_count(22usize, 23usize, 19usize, 2144392u32);
    emu.sltru_no_count(23usize, 22usize, 23usize, 2144396u32);
    emu.adr_no_count(19usize, 9usize, 25usize, 2144400u32);
    emu.adr_no_count(23usize, 24usize, 23usize, 2144404u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2144412u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b89c));
    } else {
        emu.pc = 2144408u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b898));
    }
}
#[inline(always)]
pub fn block_0x0020b898(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(18usize, 19usize, 9usize, 2144412u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2144412u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b89c));
}
#[inline(always)]
pub fn block_0x0020b89c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(18usize, 22usize, 18usize, 2144416u32);
    emu.adr_no_count(8usize, 19usize, 8usize, 2144420u32);
    emu.adr_no_count(20usize, 21usize, 20usize, 2144424u32);
    emu.sltru_no_count(22usize, 18usize, 22usize, 2144428u32);
    emu.sltru_no_count(21usize, 20usize, 21usize, 2144432u32);
    emu.adr_no_count(9usize, 8usize, 21usize, 2144436u32);
    emu.adr_no_count(8usize, 23usize, 22usize, 2144440u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2144448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b8c0));
    } else {
        emu.pc = 2144444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b8bc));
    }
}
#[inline(always)]
pub fn block_0x0020b8bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(21usize, 9usize, 19usize, 2144448u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2144448u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b8c0));
}
#[inline(never)]
pub fn block_0x0020b8c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 29u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(19usize, 18usize, 21usize, 2144452u32);
    emu.sbr_no_count(22usize, 0usize, 6usize, 2144456u32);
    emu.mulhu_no_count(21usize, 7usize, 15usize, 2144460u32);
    emu.mulhu_no_count(23usize, 6usize, 15usize, 2144464u32);
    emu.adr_no_count(24usize, 7usize, 7usize, 2144468u32);
    emu.mulhu_no_count(25usize, 7usize, 14usize, 2144472u32);
    emu.adr_no_count(26usize, 6usize, 6usize, 2144476u32);
    emu.sltru_no_count(18usize, 19usize, 18usize, 2144480u32);
    emu.adr_no_count(18usize, 8usize, 18usize, 2144484u32);
    emu.mulhu_no_count(27usize, 6usize, 14usize, 2144488u32);
    emu.sbr_no_count(21usize, 21usize, 6usize, 2144492u32);
    emu.sltru_no_count(8usize, 21usize, 22usize, 2144496u32);
    emu.adr_no_count(8usize, 23usize, 8usize, 2144500u32);
    emu.sbr_no_count(22usize, 0usize, 24usize, 2144504u32);
    emu.sbr_no_count(24usize, 21usize, 24usize, 2144508u32);
    emu.sltru_no_count(22usize, 24usize, 22usize, 2144512u32);
    emu.adr_no_count(22usize, 25usize, 22usize, 2144516u32);
    emu.adr_no_count(23usize, 8usize, 22usize, 2144520u32);
    emu.sltru_no_count(22usize, 23usize, 8usize, 2144524u32);
    emu.adr_no_count(27usize, 27usize, 22usize, 2144528u32);
    emu.sbr_no_count(22usize, 31usize, 7usize, 2144532u32);
    emu.sbr_no_count(23usize, 23usize, 26usize, 2144536u32);
    emu.sbr_no_count(26usize, 0usize, 26usize, 2144540u32);
    emu.sltru_no_count(25usize, 22usize, 31usize, 2144544u32);
    emu.adr_no_count(31usize, 24usize, 25usize, 2144548u32);
    emu.sltru_no_count(24usize, 23usize, 26usize, 2144552u32);
    emu.adr_no_count(31usize, 30usize, 31usize, 2144556u32);
    emu.adr_no_count(24usize, 27usize, 24usize, 2144560u32);
    emu.add_memory_rw_events(28usize);
    let b = emu.read_reg_b_tracked(30usize);
    let a = emu.read_reg_a_tracked(31usize);
    if a == b {
        emu.pc = 2144568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b938));
    } else {
        emu.pc = 2144564u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b934));
    }
}
#[inline(always)]
pub fn block_0x0020b934(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(25usize, 31usize, 30usize, 2144568u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2144568u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b938));
}
#[inline(always)]
pub fn block_0x0020b938(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(30usize, 23usize, 25usize, 2144572u32);
    emu.adr_no_count(18usize, 31usize, 18usize, 2144576u32);
    emu.adr_no_count(19usize, 22usize, 19usize, 2144580u32);
    emu.sltru_no_count(23usize, 30usize, 23usize, 2144584u32);
    emu.sltru_no_count(22usize, 19usize, 22usize, 2144588u32);
    emu.adr_no_count(18usize, 18usize, 22usize, 2144592u32);
    emu.adr_no_count(24usize, 24usize, 23usize, 2144596u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2144604u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b95c));
    } else {
        emu.pc = 2144600u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b958));
    }
}
#[inline(always)]
pub fn block_0x0020b958(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(22usize, 18usize, 31usize, 2144604u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2144604u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b95c));
}
#[inline(always)]
pub fn block_0x0020b95c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(31usize, 30usize, 22usize, 2144608u32);
    emu.sbr_no_count(22usize, 29usize, 7usize, 2144612u32);
    emu.sltru_no_count(23usize, 31usize, 30usize, 2144616u32);
    emu.sltru_no_count(30usize, 22usize, 29usize, 2144620u32);
    emu.adr_no_count(29usize, 28usize, 21usize, 2144624u32);
    emu.adr_no_count(29usize, 29usize, 30usize, 2144628u32);
    emu.adr_no_count(24usize, 24usize, 23usize, 2144632u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(29usize);
    if a == b {
        emu.pc = 2144640u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b980));
    } else {
        emu.pc = 2144636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b97c));
    }
}
#[inline(always)]
pub fn block_0x0020b97c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(30usize, 29usize, 28usize, 2144640u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2144640u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b980));
}
#[inline(always)]
pub fn block_0x0020b980(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(28usize, 8usize, 30usize, 2144644u32);
    emu.adr_no_count(30usize, 29usize, 24usize, 2144648u32);
    emu.adr_no_count(31usize, 22usize, 31usize, 2144652u32);
    emu.sltru_no_count(22usize, 31usize, 22usize, 2144656u32);
    emu.adr_no_count(30usize, 30usize, 22usize, 2144660u32);
    emu.sltru_no_count(21usize, 28usize, 8usize, 2144664u32);
    emu.add_memory_rw_events(6usize);
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(30usize);
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
    emu.sltru_no_count(22usize, 30usize, 29usize, 2144672u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2144672u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b9a0));
}
#[inline]
pub fn block_0x0020b9a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(22usize, 28usize, 22usize, 2144676u32);
    emu.adr_no_count(7usize, 5usize, 7usize, 2144680u32);
    emu.sltru_no_count(29usize, 22usize, 28usize, 2144684u32);
    emu.sltru_no_count(8usize, 7usize, 5usize, 2144688u32);
    emu.adr_no_count(28usize, 7usize, 22usize, 2144692u32);
    emu.adr_no_count(29usize, 21usize, 29usize, 2144696u32);
    emu.adr_no_count(21usize, 6usize, 8usize, 2144700u32);
    emu.adr_no_count(5usize, 21usize, 29usize, 2144704u32);
    emu.sltru_no_count(29usize, 28usize, 7usize, 2144708u32);
    emu.adr_no_count(7usize, 5usize, 29usize, 2144712u32);
    emu.add_memory_rw_events(10usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a == b {
        emu.pc = 2144720u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b9d0));
    } else {
        emu.pc = 2144716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020b9cc));
    }
}
#[inline(always)]
pub fn block_0x0020b9cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(29usize, 7usize, 21usize, 2144720u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2144720u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020b9d0));
}
#[inline(never)]
pub fn block_0x0020b9d0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(6usize, 11usize, 16u32, 2144724u32)?;
    emu.lw_no_count(5usize, 11usize, 20u32, 2144728u32)?;
    emu.ani_no_count(20usize, 20usize, 4294967294u32, 2144732u32);
    emu.mulhu_no_count(22usize, 6usize, 17usize, 2144736u32);
    emu.mul_no_count(23usize, 5usize, 17usize, 2144740u32);
    emu.mulhu_no_count(24usize, 5usize, 17usize, 2144744u32);
    emu.mul_no_count(25usize, 6usize, 16usize, 2144748u32);
    emu.mul_no_count(26usize, 6usize, 17usize, 2144752u32);
    emu.adr_no_count(26usize, 20usize, 26usize, 2144756u32);
    emu.sltru_no_count(20usize, 26usize, 20usize, 2144760u32);
    emu.mulhu_no_count(26usize, 6usize, 16usize, 2144764u32);
    emu.adr_no_count(22usize, 23usize, 22usize, 2144768u32);
    emu.sltru_no_count(23usize, 22usize, 23usize, 2144772u32);
    emu.adr_no_count(23usize, 24usize, 23usize, 2144776u32);
    emu.mul_no_count(24usize, 5usize, 16usize, 2144780u32);
    emu.adr_no_count(22usize, 25usize, 22usize, 2144784u32);
    emu.sltru_no_count(25usize, 22usize, 25usize, 2144788u32);
    emu.adr_no_count(25usize, 26usize, 25usize, 2144792u32);
    emu.mulhu_no_count(26usize, 5usize, 16usize, 2144796u32);
    emu.adr_no_count(27usize, 22usize, 20usize, 2144800u32);
    emu.adr_no_count(22usize, 23usize, 25usize, 2144804u32);
    emu.sltru_no_count(23usize, 22usize, 23usize, 2144808u32);
    emu.adr_no_count(22usize, 24usize, 22usize, 2144812u32);
    emu.sltru_no_count(25usize, 22usize, 24usize, 2144816u32);
    emu.adr_no_count(23usize, 26usize, 23usize, 2144820u32);
    emu.adr_no_count(24usize, 9usize, 27usize, 2144824u32);
    emu.adr_no_count(23usize, 23usize, 25usize, 2144828u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(24usize);
    if a == b {
        emu.pc = 2144836u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ba44));
    } else {
        emu.pc = 2144832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020ba40));
    }
}
#[inline(always)]
pub fn block_0x0020ba40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(20usize, 24usize, 9usize, 2144836u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2144836u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020ba44));
}
#[inline(never)]
pub fn block_0x0020ba44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(20usize, 22usize, 20usize, 2144840u32);
    emu.mulhu_no_count(24usize, 6usize, 12usize, 2144844u32);
    emu.mul_no_count(25usize, 5usize, 12usize, 2144848u32);
    emu.mulhu_no_count(26usize, 5usize, 12usize, 2144852u32);
    emu.mul_no_count(27usize, 6usize, 13usize, 2144856u32);
    emu.sltru_no_count(9usize, 20usize, 22usize, 2144860u32);
    emu.adr_no_count(9usize, 23usize, 9usize, 2144864u32);
    emu.mulhu_no_count(22usize, 6usize, 13usize, 2144868u32);
    emu.adr_no_count(24usize, 25usize, 24usize, 2144872u32);
    emu.sltru_no_count(23usize, 24usize, 25usize, 2144876u32);
    emu.adr_no_count(23usize, 26usize, 23usize, 2144880u32);
    emu.mul_no_count(26usize, 5usize, 13usize, 2144884u32);
    emu.adr_no_count(24usize, 27usize, 24usize, 2144888u32);
    emu.sltru_no_count(25usize, 24usize, 27usize, 2144892u32);
    emu.adr_no_count(22usize, 22usize, 25usize, 2144896u32);
    emu.mulhu_no_count(25usize, 5usize, 13usize, 2144900u32);
    emu.adr_no_count(27usize, 23usize, 22usize, 2144904u32);
    emu.sltru_no_count(22usize, 27usize, 23usize, 2144908u32);
    emu.adr_no_count(1usize, 25usize, 22usize, 2144912u32);
    emu.mul_no_count(22usize, 6usize, 12usize, 2144916u32);
    emu.adr_no_count(22usize, 19usize, 22usize, 2144920u32);
    emu.sltru_no_count(25usize, 22usize, 19usize, 2144924u32);
    emu.adr_no_count(19usize, 24usize, 25usize, 2144928u32);
    emu.adr_no_count(23usize, 26usize, 27usize, 2144932u32);
    emu.sltru_no_count(24usize, 23usize, 26usize, 2144936u32);
    emu.adr_no_count(19usize, 18usize, 19usize, 2144940u32);
    emu.adr_no_count(24usize, 1usize, 24usize, 2144944u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(19usize);
    if a == b {
        emu.pc = 2144952u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bab8));
    } else {
        emu.pc = 2144948u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bab4));
    }
}
#[inline(always)]
pub fn block_0x0020bab4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(25usize, 19usize, 18usize, 2144952u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2144952u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bab8));
}
#[inline]
pub fn block_0x0020bab8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(18usize, 0usize, 21usize, 2144956u32);
    emu.adr_no_count(21usize, 23usize, 25usize, 2144960u32);
    emu.adr_no_count(9usize, 19usize, 9usize, 2144964u32);
    emu.adr_no_count(20usize, 22usize, 20usize, 2144968u32);
    emu.sltru_no_count(23usize, 21usize, 23usize, 2144972u32);
    emu.sltru_no_count(22usize, 20usize, 22usize, 2144976u32);
    emu.adr_no_count(9usize, 9usize, 22usize, 2144980u32);
    emu.adr_no_count(23usize, 24usize, 23usize, 2144984u32);
    emu.adi_no_count(15usize, 0usize, 4294967295u32, 2144988u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(19usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2144996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bae4));
    } else {
        emu.pc = 2144992u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bae0));
    }
}
#[inline(always)]
pub fn block_0x0020bae0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(22usize, 9usize, 19usize, 2144996u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2144996u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bae4));
}
#[inline(never)]
pub fn block_0x0020bae4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 31u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 18usize, 4294967295u32, 2145000u32);
    emu.adr_no_count(19usize, 21usize, 22usize, 2145004u32);
    emu.sbr_no_count(24usize, 0usize, 5usize, 2145008u32);
    emu.mulhu_no_count(22usize, 6usize, 15usize, 2145012u32);
    emu.mulhu_no_count(25usize, 5usize, 15usize, 2145016u32);
    emu.adr_no_count(26usize, 6usize, 6usize, 2145020u32);
    emu.mulhu_no_count(27usize, 6usize, 14usize, 2145024u32);
    emu.adr_no_count(1usize, 5usize, 5usize, 2145028u32);
    emu.sltru_no_count(21usize, 19usize, 21usize, 2145032u32);
    emu.adr_no_count(23usize, 23usize, 21usize, 2145036u32);
    emu.adi_no_count(14usize, 0usize, 4294967294u32, 2145040u32);
    emu.mulhu_no_count(14usize, 5usize, 14usize, 2145044u32);
    emu.sbr_no_count(22usize, 22usize, 5usize, 2145048u32);
    emu.sltru_no_count(21usize, 22usize, 24usize, 2145052u32);
    emu.adr_no_count(21usize, 25usize, 21usize, 2145056u32);
    emu.sbr_no_count(24usize, 0usize, 26usize, 2145060u32);
    emu.sbr_no_count(15usize, 22usize, 26usize, 2145064u32);
    emu.sltru_no_count(24usize, 15usize, 24usize, 2145068u32);
    emu.adr_no_count(24usize, 27usize, 24usize, 2145072u32);
    emu.adr_no_count(25usize, 21usize, 24usize, 2145076u32);
    emu.sltru_no_count(24usize, 25usize, 21usize, 2145080u32);
    emu.adr_no_count(14usize, 14usize, 24usize, 2145084u32);
    emu.sbr_no_count(24usize, 31usize, 6usize, 2145088u32);
    emu.sbr_no_count(25usize, 25usize, 1usize, 2145092u32);
    emu.sbr_no_count(27usize, 0usize, 1usize, 2145096u32);
    emu.sltru_no_count(26usize, 24usize, 31usize, 2145100u32);
    emu.adr_no_count(15usize, 15usize, 26usize, 2145104u32);
    emu.sltru_no_count(27usize, 25usize, 27usize, 2145108u32);
    emu.adr_no_count(31usize, 30usize, 15usize, 2145112u32);
    emu.adr_no_count(27usize, 14usize, 27usize, 2145116u32);
    emu.add_memory_rw_events(30usize);
    let b = emu.read_reg_b_tracked(30usize);
    let a = emu.read_reg_a_tracked(31usize);
    if a == b {
        emu.pc = 2145124u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bb64));
    } else {
        emu.pc = 2145120u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bb60));
    }
}
#[inline(always)]
pub fn block_0x0020bb60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(26usize, 31usize, 30usize, 2145124u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2145124u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bb64));
}
#[inline]
pub fn block_0x0020bb64(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.anr_no_count(30usize, 18usize, 8usize, 2145128u32);
    emu.adr_no_count(26usize, 25usize, 26usize, 2145132u32);
    emu.adr_no_count(18usize, 31usize, 23usize, 2145136u32);
    emu.adr_no_count(19usize, 24usize, 19usize, 2145140u32);
    emu.sltru_no_count(23usize, 26usize, 25usize, 2145144u32);
    emu.sltru_no_count(8usize, 19usize, 24usize, 2145148u32);
    emu.adr_no_count(18usize, 18usize, 8usize, 2145152u32);
    emu.adr_no_count(23usize, 27usize, 23usize, 2145156u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2145164u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bb8c));
    } else {
        emu.pc = 2145160u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bb88));
    }
}
#[inline(always)]
pub fn block_0x0020bb88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(8usize, 18usize, 31usize, 2145164u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2145164u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bb8c));
}
#[inline]
pub fn block_0x0020bb8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 26usize, 8usize, 2145168u32);
    emu.sltru_no_count(14usize, 8usize, 26usize, 2145172u32);
    emu.adr_no_count(23usize, 23usize, 14usize, 2145176u32);
    emu.sbr_no_count(31usize, 28usize, 6usize, 2145180u32);
    emu.adr_no_count(14usize, 7usize, 22usize, 2145184u32);
    emu.sltru_no_count(22usize, 31usize, 28usize, 2145188u32);
    emu.adr_no_count(28usize, 14usize, 22usize, 2145192u32);
    emu.adr_no_count(29usize, 30usize, 29usize, 2145196u32);
    emu.adi_no_count(1usize, 0usize, 4294967295u32, 2145200u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2145208u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bbb8));
    } else {
        emu.pc = 2145204u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bbb4));
    }
}
#[inline(always)]
pub fn block_0x0020bbb4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(22usize, 28usize, 7usize, 2145208u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2145208u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bbb8));
}
#[inline(always)]
pub fn block_0x0020bbb8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(7usize, 29usize, 30usize, 2145212u32);
    emu.adr_no_count(30usize, 21usize, 22usize, 2145216u32);
    emu.adr_no_count(23usize, 28usize, 23usize, 2145220u32);
    emu.adr_no_count(8usize, 31usize, 8usize, 2145224u32);
    emu.sltru_no_count(22usize, 8usize, 31usize, 2145228u32);
    emu.adr_no_count(31usize, 23usize, 22usize, 2145232u32);
    emu.sltru_no_count(21usize, 30usize, 21usize, 2145236u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(31usize);
    if a == b {
        emu.pc = 2145244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bbdc));
    } else {
        emu.pc = 2145240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bbd8));
    }
}
#[inline(always)]
pub fn block_0x0020bbd8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(22usize, 31usize, 28usize, 2145244u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2145244u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bbdc));
}
#[inline(always)]
pub fn block_0x0020bbdc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(28usize, 30usize, 22usize, 2145248u32);
    emu.adr_no_count(22usize, 29usize, 6usize, 2145252u32);
    emu.sltru_no_count(14usize, 28usize, 30usize, 2145256u32);
    emu.sltru_no_count(6usize, 22usize, 29usize, 2145260u32);
    emu.adr_no_count(5usize, 7usize, 5usize, 2145264u32);
    emu.adr_no_count(5usize, 5usize, 6usize, 2145268u32);
    emu.adr_no_count(21usize, 21usize, 14usize, 2145272u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2145280u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bc00));
    } else {
        emu.pc = 2145276u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bbfc));
    }
}
#[inline(always)]
pub fn block_0x0020bbfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(6usize, 5usize, 7usize, 2145280u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2145280u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bc00));
}
#[inline(always)]
pub fn block_0x0020bc00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(29usize, 22usize, 28usize, 2145284u32);
    emu.sltru_no_count(30usize, 29usize, 22usize, 2145288u32);
    emu.adr_no_count(28usize, 5usize, 21usize, 2145292u32);
    emu.adr_no_count(28usize, 28usize, 30usize, 2145296u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(5usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2145304u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bc18));
    } else {
        emu.pc = 2145300u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bc14));
    }
}
#[inline(always)]
pub fn block_0x0020bc14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(30usize, 28usize, 5usize, 2145304u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2145304u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bc18));
}
#[inline(never)]
pub fn block_0x0020bc18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(5usize, 11usize, 24u32, 2145308u32)?;
    emu.lw_no_count(7usize, 11usize, 28u32, 2145312u32)?;
    emu.ani_no_count(14usize, 20usize, 4294967294u32, 2145316u32);
    emu.mulhu_no_count(15usize, 5usize, 17usize, 2145320u32);
    emu.mul_no_count(21usize, 7usize, 17usize, 2145324u32);
    emu.mulhu_no_count(22usize, 7usize, 17usize, 2145328u32);
    emu.mul_no_count(23usize, 5usize, 16usize, 2145332u32);
    emu.mul_no_count(20usize, 5usize, 17usize, 2145336u32);
    emu.adr_no_count(20usize, 14usize, 20usize, 2145340u32);
    emu.sltru_no_count(20usize, 20usize, 14usize, 2145344u32);
    emu.mulhu_no_count(14usize, 5usize, 16usize, 2145348u32);
    emu.adr_no_count(15usize, 21usize, 15usize, 2145352u32);
    emu.sltru_no_count(21usize, 15usize, 21usize, 2145356u32);
    emu.adr_no_count(21usize, 22usize, 21usize, 2145360u32);
    emu.mul_no_count(22usize, 7usize, 16usize, 2145364u32);
    emu.adr_no_count(15usize, 23usize, 15usize, 2145368u32);
    emu.sltru_no_count(23usize, 15usize, 23usize, 2145372u32);
    emu.adr_no_count(14usize, 14usize, 23usize, 2145376u32);
    emu.mulhu_no_count(23usize, 7usize, 16usize, 2145380u32);
    emu.adr_no_count(15usize, 15usize, 20usize, 2145384u32);
    emu.adr_no_count(14usize, 21usize, 14usize, 2145388u32);
    emu.sltru_no_count(24usize, 14usize, 21usize, 2145392u32);
    emu.adr_no_count(21usize, 22usize, 14usize, 2145396u32);
    emu.sltru_no_count(22usize, 21usize, 22usize, 2145400u32);
    emu.adr_no_count(24usize, 23usize, 24usize, 2145404u32);
    emu.adr_no_count(23usize, 9usize, 15usize, 2145408u32);
    emu.adr_no_count(22usize, 24usize, 22usize, 2145412u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a == b {
        emu.pc = 2145420u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bc8c));
    } else {
        emu.pc = 2145416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bc88));
    }
}
#[inline(always)]
pub fn block_0x0020bc88(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(20usize, 23usize, 9usize, 2145420u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2145420u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bc8c));
}
#[inline(never)]
pub fn block_0x0020bc8c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(9usize, 21usize, 20usize, 2145424u32);
    emu.mulhu_no_count(14usize, 5usize, 12usize, 2145428u32);
    emu.mul_no_count(15usize, 7usize, 12usize, 2145432u32);
    emu.mulhu_no_count(23usize, 7usize, 12usize, 2145436u32);
    emu.mul_no_count(24usize, 5usize, 13usize, 2145440u32);
    emu.sltru_no_count(20usize, 9usize, 21usize, 2145444u32);
    emu.adr_no_count(20usize, 22usize, 20usize, 2145448u32);
    emu.mulhu_no_count(21usize, 5usize, 13usize, 2145452u32);
    emu.adr_no_count(14usize, 15usize, 14usize, 2145456u32);
    emu.sltru_no_count(15usize, 14usize, 15usize, 2145460u32);
    emu.adr_no_count(15usize, 23usize, 15usize, 2145464u32);
    emu.mul_no_count(23usize, 7usize, 13usize, 2145468u32);
    emu.adr_no_count(14usize, 24usize, 14usize, 2145472u32);
    emu.sltru_no_count(22usize, 14usize, 24usize, 2145476u32);
    emu.adr_no_count(21usize, 21usize, 22usize, 2145480u32);
    emu.mulhu_no_count(22usize, 7usize, 13usize, 2145484u32);
    emu.adr_no_count(24usize, 15usize, 21usize, 2145488u32);
    emu.sltru_no_count(15usize, 24usize, 15usize, 2145492u32);
    emu.adr_no_count(15usize, 22usize, 15usize, 2145496u32);
    emu.mul_no_count(22usize, 5usize, 12usize, 2145500u32);
    emu.adr_no_count(22usize, 19usize, 22usize, 2145504u32);
    emu.sltru_no_count(21usize, 22usize, 19usize, 2145508u32);
    emu.adr_no_count(14usize, 14usize, 21usize, 2145512u32);
    emu.adr_no_count(24usize, 23usize, 24usize, 2145516u32);
    emu.sltru_no_count(25usize, 24usize, 23usize, 2145520u32);
    emu.adr_no_count(23usize, 18usize, 14usize, 2145524u32);
    emu.adr_no_count(25usize, 15usize, 25usize, 2145528u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(18usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a == b {
        emu.pc = 2145536u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bd00));
    } else {
        emu.pc = 2145532u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bcfc));
    }
}
#[inline(always)]
pub fn block_0x0020bcfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(21usize, 23usize, 18usize, 2145536u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2145536u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bd00));
}
#[inline(always)]
pub fn block_0x0020bd00(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(21usize, 24usize, 21usize, 2145540u32);
    emu.adr_no_count(20usize, 23usize, 20usize, 2145544u32);
    emu.adr_no_count(19usize, 22usize, 9usize, 2145548u32);
    emu.sltru_no_count(14usize, 21usize, 24usize, 2145552u32);
    emu.sltru_no_count(18usize, 19usize, 22usize, 2145556u32);
    emu.adr_no_count(9usize, 20usize, 18usize, 2145560u32);
    emu.adr_no_count(22usize, 25usize, 14usize, 2145564u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(23usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2145572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bd24));
    } else {
        emu.pc = 2145568u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bd20));
    }
}
#[inline(always)]
pub fn block_0x0020bd20(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(18usize, 9usize, 23usize, 2145572u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2145572u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bd24));
}
#[inline(never)]
pub fn block_0x0020bd24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 30u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(18usize, 21usize, 18usize, 2145576u32);
    emu.sbr_no_count(14usize, 0usize, 7usize, 2145580u32);
    emu.mulhu_no_count(15usize, 5usize, 1usize, 2145584u32);
    emu.mulhu_no_count(20usize, 7usize, 1usize, 2145588u32);
    emu.adr_no_count(23usize, 5usize, 5usize, 2145592u32);
    emu.adi_no_count(25usize, 0usize, 4294967294u32, 2145596u32);
    emu.mulhu_no_count(24usize, 5usize, 25usize, 2145600u32);
    emu.adr_no_count(26usize, 7usize, 7usize, 2145604u32);
    emu.sltru_no_count(21usize, 18usize, 21usize, 2145608u32);
    emu.adr_no_count(22usize, 22usize, 21usize, 2145612u32);
    emu.mulhu_no_count(25usize, 7usize, 25usize, 2145616u32);
    emu.sbr_no_count(21usize, 15usize, 7usize, 2145620u32);
    emu.sltru_no_count(14usize, 21usize, 14usize, 2145624u32);
    emu.adr_no_count(20usize, 20usize, 14usize, 2145628u32);
    emu.sbr_no_count(14usize, 0usize, 23usize, 2145632u32);
    emu.sbr_no_count(15usize, 21usize, 23usize, 2145636u32);
    emu.sltru_no_count(14usize, 15usize, 14usize, 2145640u32);
    emu.adr_no_count(14usize, 24usize, 14usize, 2145644u32);
    emu.adr_no_count(14usize, 20usize, 14usize, 2145648u32);
    emu.sltru_no_count(23usize, 14usize, 20usize, 2145652u32);
    emu.adr_no_count(27usize, 25usize, 23usize, 2145656u32);
    emu.sbr_no_count(23usize, 8usize, 5usize, 2145660u32);
    emu.sbr_no_count(25usize, 14usize, 26usize, 2145664u32);
    emu.sbr_no_count(14usize, 0usize, 26usize, 2145668u32);
    emu.sltru_no_count(26usize, 23usize, 8usize, 2145672u32);
    emu.adr_no_count(15usize, 15usize, 26usize, 2145676u32);
    emu.sltru_no_count(14usize, 25usize, 14usize, 2145680u32);
    emu.adr_no_count(24usize, 31usize, 15usize, 2145684u32);
    emu.adr_no_count(27usize, 27usize, 14usize, 2145688u32);
    emu.add_memory_rw_events(29usize);
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(24usize);
    if a == b {
        emu.pc = 2145696u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bda0));
    } else {
        emu.pc = 2145692u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bd9c));
    }
}
#[inline(always)]
pub fn block_0x0020bd9c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(26usize, 24usize, 31usize, 2145696u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2145696u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bda0));
}
#[inline(always)]
pub fn block_0x0020bda0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(26usize, 25usize, 26usize, 2145700u32);
    emu.adr_no_count(8usize, 24usize, 22usize, 2145704u32);
    emu.adr_no_count(18usize, 23usize, 18usize, 2145708u32);
    emu.sltru_no_count(14usize, 26usize, 25usize, 2145712u32);
    emu.sltru_no_count(31usize, 18usize, 23usize, 2145716u32);
    emu.adr_no_count(8usize, 8usize, 31usize, 2145720u32);
    emu.adr_no_count(23usize, 27usize, 14usize, 2145724u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(24usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2145732u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bdc4));
    } else {
        emu.pc = 2145728u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bdc0));
    }
}
#[inline(always)]
pub fn block_0x0020bdc0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(31usize, 8usize, 24usize, 2145732u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2145732u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bdc4));
}
#[inline]
pub fn block_0x0020bdc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(31usize, 26usize, 31usize, 2145736u32);
    emu.sltru_no_count(14usize, 31usize, 26usize, 2145740u32);
    emu.adr_no_count(23usize, 23usize, 14usize, 2145744u32);
    emu.sbr_no_count(22usize, 29usize, 5usize, 2145748u32);
    emu.adr_no_count(21usize, 28usize, 21usize, 2145752u32);
    emu.sltru_no_count(24usize, 22usize, 29usize, 2145756u32);
    emu.adr_no_count(21usize, 21usize, 24usize, 2145760u32);
    emu.adr_no_count(29usize, 6usize, 30usize, 2145764u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a == b {
        emu.pc = 2145772u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bdec));
    } else {
        emu.pc = 2145768u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bde8));
    }
}
#[inline(always)]
pub fn block_0x0020bde8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(24usize, 21usize, 28usize, 2145772u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2145772u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bdec));
}
#[inline(always)]
pub fn block_0x0020bdec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(6usize, 29usize, 6usize, 2145776u32);
    emu.adr_no_count(28usize, 20usize, 24usize, 2145780u32);
    emu.adr_no_count(30usize, 21usize, 23usize, 2145784u32);
    emu.adr_no_count(31usize, 22usize, 31usize, 2145788u32);
    emu.sltru_no_count(22usize, 31usize, 22usize, 2145792u32);
    emu.adr_no_count(30usize, 30usize, 22usize, 2145796u32);
    emu.sltru_no_count(20usize, 28usize, 20usize, 2145800u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(30usize);
    if a == b {
        emu.pc = 2145808u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020be10));
    } else {
        emu.pc = 2145804u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020be0c));
    }
}
#[inline(always)]
pub fn block_0x0020be0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(22usize, 30usize, 21usize, 2145808u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2145808u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020be10));
}
#[inline(always)]
pub fn block_0x0020be10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(21usize, 28usize, 22usize, 2145812u32);
    emu.adr_no_count(23usize, 29usize, 5usize, 2145816u32);
    emu.sltru_no_count(14usize, 21usize, 28usize, 2145820u32);
    emu.sltru_no_count(5usize, 23usize, 29usize, 2145824u32);
    emu.adr_no_count(7usize, 6usize, 7usize, 2145828u32);
    emu.adr_no_count(22usize, 7usize, 5usize, 2145832u32);
    emu.adr_no_count(20usize, 20usize, 14usize, 2145836u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(6usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a == b {
        emu.pc = 2145844u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020be34));
    } else {
        emu.pc = 2145840u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020be30));
    }
}
#[inline(always)]
pub fn block_0x0020be30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(5usize, 22usize, 6usize, 2145844u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2145844u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020be34));
}
#[inline(always)]
pub fn block_0x0020be34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(28usize, 23usize, 21usize, 2145848u32);
    emu.sltru_no_count(29usize, 28usize, 23usize, 2145852u32);
    emu.adr_no_count(7usize, 22usize, 20usize, 2145856u32);
    emu.adr_no_count(7usize, 7usize, 29usize, 2145860u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(22usize);
    let a = emu.read_reg_a_tracked(7usize);
    if a == b {
        emu.pc = 2145868u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020be4c));
    } else {
        emu.pc = 2145864u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020be48));
    }
}
#[inline(always)]
pub fn block_0x0020be48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(29usize, 7usize, 22usize, 2145868u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2145868u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020be4c));
}
#[inline(never)]
pub fn block_0x0020be4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(6usize, 11usize, 32u32, 2145872u32)?;
    emu.lw_no_count(11usize, 11usize, 36u32, 2145876u32)?;
    emu.ani_no_count(14usize, 19usize, 4294967294u32, 2145880u32);
    emu.mulhu_no_count(15usize, 6usize, 17usize, 2145884u32);
    emu.mul_no_count(20usize, 11usize, 17usize, 2145888u32);
    emu.mulhu_no_count(21usize, 11usize, 17usize, 2145892u32);
    emu.mul_no_count(22usize, 6usize, 16usize, 2145896u32);
    emu.mulhu_no_count(23usize, 6usize, 16usize, 2145900u32);
    emu.mul_no_count(17usize, 6usize, 17usize, 2145904u32);
    emu.adr_no_count(17usize, 14usize, 17usize, 2145908u32);
    emu.sltru_no_count(19usize, 17usize, 14usize, 2145912u32);
    emu.mul_no_count(14usize, 11usize, 16usize, 2145916u32);
    emu.mulhu_no_count(16usize, 11usize, 16usize, 2145920u32);
    emu.adr_no_count(15usize, 20usize, 15usize, 2145924u32);
    emu.sltru_no_count(17usize, 15usize, 20usize, 2145928u32);
    emu.adr_no_count(15usize, 22usize, 15usize, 2145932u32);
    emu.adr_no_count(21usize, 21usize, 17usize, 2145936u32);
    emu.sltru_no_count(17usize, 15usize, 22usize, 2145940u32);
    emu.adr_no_count(15usize, 15usize, 19usize, 2145944u32);
    emu.adr_no_count(17usize, 23usize, 17usize, 2145948u32);
    emu.adr_no_count(20usize, 21usize, 17usize, 2145952u32);
    emu.adr_no_count(17usize, 14usize, 20usize, 2145956u32);
    emu.sltru_no_count(20usize, 20usize, 21usize, 2145960u32);
    emu.sltru_no_count(14usize, 17usize, 14usize, 2145964u32);
    emu.adr_no_count(20usize, 16usize, 20usize, 2145968u32);
    emu.adr_no_count(16usize, 9usize, 15usize, 2145972u32);
    emu.adr_no_count(20usize, 20usize, 14usize, 2145976u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(16usize);
    if a == b {
        emu.pc = 2145984u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bec0));
    } else {
        emu.pc = 2145980u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bebc));
    }
}
#[inline(always)]
pub fn block_0x0020bebc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(19usize, 16usize, 9usize, 2145984u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2145984u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bec0));
}
#[inline(never)]
pub fn block_0x0020bec0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(16usize, 17usize, 19usize, 2145988u32);
    emu.mulhu_no_count(14usize, 6usize, 12usize, 2145992u32);
    emu.mul_no_count(15usize, 11usize, 12usize, 2145996u32);
    emu.mulhu_no_count(9usize, 11usize, 12usize, 2146000u32);
    emu.mul_no_count(19usize, 6usize, 13usize, 2146004u32);
    emu.sltru_no_count(17usize, 16usize, 17usize, 2146008u32);
    emu.adr_no_count(17usize, 20usize, 17usize, 2146012u32);
    emu.mulhu_no_count(20usize, 6usize, 13usize, 2146016u32);
    emu.adr_no_count(14usize, 15usize, 14usize, 2146020u32);
    emu.sltru_no_count(15usize, 14usize, 15usize, 2146024u32);
    emu.adr_no_count(15usize, 9usize, 15usize, 2146028u32);
    emu.mul_no_count(9usize, 11usize, 13usize, 2146032u32);
    emu.mulhu_no_count(21usize, 11usize, 13usize, 2146036u32);
    emu.mul_no_count(13usize, 6usize, 12usize, 2146040u32);
    emu.adr_no_count(13usize, 18usize, 13usize, 2146044u32);
    emu.adr_no_count(14usize, 19usize, 14usize, 2146048u32);
    emu.sltru_no_count(12usize, 13usize, 18usize, 2146052u32);
    emu.sltru_no_count(18usize, 14usize, 19usize, 2146056u32);
    emu.adr_no_count(14usize, 14usize, 12usize, 2146060u32);
    emu.adr_no_count(18usize, 20usize, 18usize, 2146064u32);
    emu.adr_no_count(19usize, 15usize, 18usize, 2146068u32);
    emu.adr_no_count(18usize, 9usize, 19usize, 2146072u32);
    emu.sltru_no_count(15usize, 19usize, 15usize, 2146076u32);
    emu.sltru_no_count(19usize, 18usize, 9usize, 2146080u32);
    emu.adr_no_count(15usize, 21usize, 15usize, 2146084u32);
    emu.adr_no_count(9usize, 8usize, 14usize, 2146088u32);
    emu.adr_no_count(19usize, 15usize, 19usize, 2146092u32);
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(9usize);
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
    emu.sltru_no_count(12usize, 9usize, 8usize, 2146100u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2146100u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bf34));
}
#[inline(always)]
pub fn block_0x0020bf34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(8usize, 18usize, 12usize, 2146104u32);
    emu.adr_no_count(14usize, 9usize, 17usize, 2146108u32);
    emu.adr_no_count(12usize, 13usize, 16usize, 2146112u32);
    emu.sltru_no_count(16usize, 8usize, 18usize, 2146116u32);
    emu.sltru_no_count(17usize, 12usize, 13usize, 2146120u32);
    emu.adr_no_count(13usize, 14usize, 17usize, 2146124u32);
    emu.adr_no_count(16usize, 19usize, 16usize, 2146128u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a == b {
        emu.pc = 2146136u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bf58));
    } else {
        emu.pc = 2146132u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bf54));
    }
}
#[inline(always)]
pub fn block_0x0020bf54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(17usize, 13usize, 9usize, 2146136u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2146136u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bf58));
}
#[inline(never)]
pub fn block_0x0020bf58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 30u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(17usize, 8usize, 17usize, 2146140u32);
    emu.sbr_no_count(14usize, 0usize, 11usize, 2146144u32);
    emu.mulhu_no_count(9usize, 6usize, 1usize, 2146148u32);
    emu.mulhu_no_count(18usize, 11usize, 1usize, 2146152u32);
    emu.adr_no_count(19usize, 6usize, 6usize, 2146156u32);
    emu.adi_no_count(22usize, 0usize, 4294967294u32, 2146160u32);
    emu.mulhu_no_count(20usize, 6usize, 22usize, 2146164u32);
    emu.sltru_no_count(15usize, 17usize, 8usize, 2146168u32);
    emu.adr_no_count(15usize, 16usize, 15usize, 2146172u32);
    emu.adr_no_count(21usize, 11usize, 11usize, 2146176u32);
    emu.mulhu_no_count(22usize, 11usize, 22usize, 2146180u32);
    emu.sbr_no_count(8usize, 9usize, 11usize, 2146184u32);
    emu.sltru_no_count(16usize, 8usize, 14usize, 2146188u32);
    emu.adr_no_count(16usize, 18usize, 16usize, 2146192u32);
    emu.sbr_no_count(14usize, 0usize, 19usize, 2146196u32);
    emu.sbr_no_count(19usize, 8usize, 19usize, 2146200u32);
    emu.sltru_no_count(14usize, 19usize, 14usize, 2146204u32);
    emu.adr_no_count(14usize, 20usize, 14usize, 2146208u32);
    emu.adr_no_count(14usize, 16usize, 14usize, 2146212u32);
    emu.sltru_no_count(9usize, 14usize, 16usize, 2146216u32);
    emu.adr_no_count(22usize, 22usize, 9usize, 2146220u32);
    emu.sbr_no_count(9usize, 31usize, 6usize, 2146224u32);
    emu.sbr_no_count(18usize, 14usize, 21usize, 2146228u32);
    emu.sbr_no_count(20usize, 0usize, 21usize, 2146232u32);
    emu.sltru_no_count(14usize, 9usize, 31usize, 2146236u32);
    emu.adr_no_count(31usize, 19usize, 14usize, 2146240u32);
    emu.sltru_no_count(19usize, 18usize, 20usize, 2146244u32);
    emu.adr_no_count(31usize, 30usize, 31usize, 2146248u32);
    emu.adr_no_count(19usize, 22usize, 19usize, 2146252u32);
    emu.add_memory_rw_events(29usize);
    let b = emu.read_reg_b_tracked(30usize);
    let a = emu.read_reg_a_tracked(31usize);
    if a == b {
        emu.pc = 2146260u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bfd4));
    } else {
        emu.pc = 2146256u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bfd0));
    }
}
#[inline(always)]
pub fn block_0x0020bfd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(14usize, 31usize, 30usize, 2146260u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2146260u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bfd4));
}
#[inline(always)]
pub fn block_0x0020bfd4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(30usize, 18usize, 14usize, 2146264u32);
    emu.adr_no_count(15usize, 31usize, 15usize, 2146268u32);
    emu.adr_no_count(14usize, 9usize, 17usize, 2146272u32);
    emu.sltru_no_count(18usize, 30usize, 18usize, 2146276u32);
    emu.sltru_no_count(17usize, 14usize, 9usize, 2146280u32);
    emu.adr_no_count(15usize, 15usize, 17usize, 2146284u32);
    emu.adr_no_count(9usize, 19usize, 18usize, 2146288u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(31usize);
    let a = emu.read_reg_a_tracked(15usize);
    if a == b {
        emu.pc = 2146296u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bff8));
    } else {
        emu.pc = 2146292u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020bff4));
    }
}
#[inline(always)]
pub fn block_0x0020bff4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(17usize, 15usize, 31usize, 2146296u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2146296u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020bff8));
}
#[inline]
pub fn block_0x0020bff8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(17usize, 30usize, 17usize, 2146300u32);
    emu.sltru_no_count(30usize, 17usize, 30usize, 2146304u32);
    emu.adr_no_count(9usize, 9usize, 30usize, 2146308u32);
    emu.sbr_no_count(30usize, 28usize, 6usize, 2146312u32);
    emu.adr_no_count(8usize, 7usize, 8usize, 2146316u32);
    emu.sltru_no_count(31usize, 30usize, 28usize, 2146320u32);
    emu.adr_no_count(28usize, 8usize, 31usize, 2146324u32);
    emu.adr_no_count(29usize, 5usize, 29usize, 2146328u32);
    emu.add_memory_rw_events(8usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(28usize);
    if a == b {
        emu.pc = 2146336u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c020));
    } else {
        emu.pc = 2146332u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c01c));
    }
}
#[inline(always)]
pub fn block_0x0020c01c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(31usize, 28usize, 7usize, 2146336u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2146336u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c020));
}
#[inline(always)]
pub fn block_0x0020c020(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(7usize, 29usize, 5usize, 2146340u32);
    emu.adr_no_count(31usize, 16usize, 31usize, 2146344u32);
    emu.adr_no_count(5usize, 28usize, 9usize, 2146348u32);
    emu.adr_no_count(17usize, 30usize, 17usize, 2146352u32);
    emu.sltru_no_count(8usize, 17usize, 30usize, 2146356u32);
    emu.adr_no_count(5usize, 5usize, 8usize, 2146360u32);
    emu.sltru_no_count(30usize, 31usize, 16usize, 2146364u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(28usize);
    let a = emu.read_reg_a_tracked(5usize);
    if a == b {
        emu.pc = 2146372u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c044));
    } else {
        emu.pc = 2146368u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c040));
    }
}
#[inline(always)]
pub fn block_0x0020c040(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(8usize, 5usize, 28usize, 2146372u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2146372u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c044));
}
#[inline(always)]
pub fn block_0x0020c044(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(28usize, 31usize, 8usize, 2146376u32);
    emu.adr_no_count(6usize, 29usize, 6usize, 2146380u32);
    emu.sltru_no_count(31usize, 28usize, 31usize, 2146384u32);
    emu.sltru_no_count(16usize, 6usize, 29usize, 2146388u32);
    emu.adr_no_count(11usize, 7usize, 11usize, 2146392u32);
    emu.adr_no_count(29usize, 11usize, 16usize, 2146396u32);
    emu.adr_no_count(30usize, 30usize, 31usize, 2146400u32);
    emu.add_memory_rw_events(7usize);
    let b = emu.read_reg_b_tracked(7usize);
    let a = emu.read_reg_a_tracked(29usize);
    if a == b {
        emu.pc = 2146408u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c068));
    } else {
        emu.pc = 2146404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c064));
    }
}
#[inline(always)]
pub fn block_0x0020c064(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(16usize, 29usize, 7usize, 2146408u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2146408u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c068));
}
#[inline(always)]
pub fn block_0x0020c068(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(11usize, 6usize, 28usize, 2146412u32);
    emu.sltru_no_count(7usize, 11usize, 6usize, 2146416u32);
    emu.adr_no_count(6usize, 29usize, 30usize, 2146420u32);
    emu.adr_no_count(6usize, 6usize, 7usize, 2146424u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(29usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2146432u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c080));
    } else {
        emu.pc = 2146428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020c07c));
    }
}
#[inline(always)]
pub fn block_0x0020c07c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sltru_no_count(7usize, 6usize, 29usize, 2146432u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2146432u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020c080));
}
#[inline(never)]
pub fn block_0x0020c080(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(7usize, 16usize, 7usize, 2146436u32);
    emu.sw_no_count(12usize, 10usize, 0u32, 2146440u32)?;
    emu.sw_no_count(13usize, 10usize, 4u32, 2146444u32)?;
    emu.sw_no_count(14usize, 10usize, 8u32, 2146448u32)?;
    emu.sw_no_count(15usize, 10usize, 12u32, 2146452u32)?;
    emu.sw_no_count(17usize, 10usize, 16u32, 2146456u32)?;
    emu.sw_no_count(5usize, 10usize, 20u32, 2146460u32)?;
    emu.sw_no_count(11usize, 10usize, 24u32, 2146464u32)?;
    emu.sw_no_count(6usize, 10usize, 28u32, 2146468u32)?;
    emu.sltru_no_count(11usize, 7usize, 16usize, 2146472u32);
    emu.sw_no_count(7usize, 10usize, 32u32, 2146476u32)?;
    emu.sw_no_count(11usize, 10usize, 36u32, 2146480u32)?;
    emu.lw_no_count(1usize, 2usize, 60u32, 2146484u32)?;
    emu.lw_no_count(8usize, 2usize, 56u32, 2146488u32)?;
    emu.lw_no_count(9usize, 2usize, 52u32, 2146492u32)?;
    emu.lw_no_count(18usize, 2usize, 48u32, 2146496u32)?;
    emu.lw_no_count(19usize, 2usize, 44u32, 2146500u32)?;
    emu.lw_no_count(20usize, 2usize, 40u32, 2146504u32)?;
    emu.lw_no_count(21usize, 2usize, 36u32, 2146508u32)?;
    emu.lw_no_count(22usize, 2usize, 32u32, 2146512u32)?;
    emu.lw_no_count(23usize, 2usize, 28u32, 2146516u32)?;
    emu.lw_no_count(24usize, 2usize, 24u32, 2146520u32)?;
    emu.lw_no_count(25usize, 2usize, 20u32, 2146524u32)?;
    emu.lw_no_count(26usize, 2usize, 16u32, 2146528u32)?;
    emu.lw_no_count(27usize, 2usize, 12u32, 2146532u32)?;
    emu.adi_no_count(2usize, 2usize, 64u32, 2146536u32);
    emu.add_memory_rw_events(27usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2146540u32;
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
