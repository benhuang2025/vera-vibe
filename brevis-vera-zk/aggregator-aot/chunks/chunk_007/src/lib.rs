pub use pico_aot_runtime::{AotEmulatorCore, BlockClock, BlockFn, NextStep};
pub const PC_MIN: u32 = 2126388u32;
pub const PC_MAX: u32 = 2131636u32;
pub fn lookup(pc: u32) -> Option<BlockFn> {
    const FN: [BlockFn; 120usize] = [
        block_0x00207234,
        block_0x00207248,
        block_0x0020725c,
        block_0x00207288,
        block_0x00207294,
        block_0x0020729c,
        block_0x002072a8,
        block_0x002072b4,
        block_0x002072b8,
        block_0x002072bc,
        block_0x00207300,
        block_0x00207338,
        block_0x00207348,
        block_0x0020734c,
        block_0x00207360,
        block_0x00207384,
        block_0x002073bc,
        block_0x002073c8,
        block_0x002073cc,
        block_0x00207464,
        block_0x00207484,
        block_0x00207494,
        block_0x00207498,
        block_0x002074ac,
        block_0x002074c0,
        block_0x002074c8,
        block_0x002074cc,
        block_0x002074e8,
        block_0x00207584,
        block_0x0020758c,
        block_0x002075a4,
        block_0x002075c4,
        block_0x00207638,
        block_0x0020764c,
        block_0x00207654,
        block_0x00207658,
        block_0x00207670,
        block_0x00207690,
        block_0x00207694,
        block_0x002076a8,
        block_0x002076d4,
        block_0x002076e8,
        block_0x00207700,
        block_0x00207718,
        block_0x00207720,
        block_0x00207738,
        block_0x002077a8,
        block_0x002077cc,
        block_0x002077d8,
        block_0x002077e0,
        block_0x002077ec,
        block_0x00207804,
        block_0x0020788c,
        block_0x002078a0,
        block_0x002078ac,
        block_0x002078c0,
        block_0x002078d4,
        block_0x0020791c,
        block_0x00207930,
        block_0x00207938,
        block_0x00207968,
        block_0x002079a0,
        block_0x00207a0c,
        block_0x00207a40,
        block_0x00207a4c,
        block_0x00207a50,
        block_0x00207a78,
        block_0x00207a84,
        block_0x00207aa0,
        block_0x00207aa4,
        block_0x00207ab0,
        block_0x00207ac4,
        block_0x00207af4,
        block_0x00207b0c,
        block_0x00207b10,
        block_0x00207b24,
        block_0x00207bb0,
        block_0x00207bc4,
        block_0x00207bd0,
        block_0x00207be4,
        block_0x00207bf8,
        block_0x00207c40,
        block_0x00207c54,
        block_0x00207c5c,
        block_0x00207ca0,
        block_0x00207d14,
        block_0x00207d38,
        block_0x00207d44,
        block_0x00207d4c,
        block_0x00207d58,
        block_0x00207d70,
        block_0x00207dfc,
        block_0x00207e10,
        block_0x00207e1c,
        block_0x00207e34,
        block_0x00207e48,
        block_0x00207e90,
        block_0x00207ea8,
        block_0x00207eb0,
        block_0x00207f18,
        block_0x00207f30,
        block_0x00207f34,
        block_0x00207f40,
        block_0x00207f58,
        block_0x00207f60,
        block_0x00208410,
        block_0x00208438,
        block_0x002084ec,
        block_0x00208528,
        block_0x00208574,
        block_0x002085ac,
        block_0x002085e4,
        block_0x00208628,
        block_0x00208638,
        block_0x0020864c,
        block_0x00208654,
        block_0x0020865c,
        block_0x00208660,
        block_0x00208688,
        block_0x002086b4,
    ];
    const IDX: [u16; 1313usize] = [
        1u16, 0u16, 0u16, 0u16, 0u16, 2u16, 0u16, 0u16, 0u16, 0u16, 3u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 4u16, 0u16, 0u16, 5u16, 0u16,
        6u16, 0u16, 0u16, 7u16, 0u16, 0u16, 8u16, 9u16, 10u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 11u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        12u16, 0u16, 0u16, 0u16, 13u16, 14u16, 0u16, 0u16, 0u16, 0u16, 15u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 16u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 17u16, 0u16, 0u16, 18u16, 19u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 20u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 21u16, 0u16, 0u16, 0u16, 22u16, 23u16, 0u16, 0u16,
        0u16, 0u16, 24u16, 0u16, 0u16, 0u16, 0u16, 25u16, 0u16, 26u16, 27u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 28u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 29u16, 0u16, 30u16, 0u16, 0u16, 0u16, 0u16, 0u16, 31u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 32u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 33u16, 0u16, 0u16,
        0u16, 0u16, 34u16, 0u16, 35u16, 36u16, 0u16, 0u16, 0u16, 0u16, 0u16, 37u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 38u16, 39u16, 0u16, 0u16, 0u16, 0u16, 40u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 41u16, 0u16, 0u16,
        0u16, 0u16, 42u16, 0u16, 0u16, 0u16, 0u16, 0u16, 43u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 44u16, 0u16, 45u16, 0u16, 0u16, 0u16, 0u16, 0u16, 46u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 47u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 48u16, 0u16, 0u16, 49u16, 0u16, 50u16,
        0u16, 0u16, 51u16, 0u16, 0u16, 0u16, 0u16, 0u16, 52u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 53u16, 0u16, 0u16, 0u16, 0u16, 54u16, 0u16, 0u16, 55u16, 0u16,
        0u16, 0u16, 0u16, 56u16, 0u16, 0u16, 0u16, 0u16, 57u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        58u16, 0u16, 0u16, 0u16, 0u16, 59u16, 0u16, 60u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 61u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 62u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 63u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 64u16, 0u16, 0u16, 65u16, 66u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 67u16, 0u16, 0u16, 68u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 69u16, 70u16, 0u16, 0u16, 71u16, 0u16, 0u16,
        0u16, 0u16, 72u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 73u16, 0u16, 0u16, 0u16, 0u16, 0u16, 74u16, 75u16, 0u16, 0u16, 0u16, 0u16,
        76u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 77u16, 0u16, 0u16, 0u16,
        0u16, 78u16, 0u16, 0u16, 79u16, 0u16, 0u16, 0u16, 0u16, 80u16, 0u16, 0u16, 0u16,
        0u16, 81u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 82u16, 0u16, 0u16, 0u16, 0u16, 83u16, 0u16,
        84u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 85u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 86u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 87u16, 0u16, 0u16, 88u16, 0u16, 89u16, 0u16, 0u16, 90u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 91u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        92u16, 0u16, 0u16, 0u16, 0u16, 93u16, 0u16, 0u16, 94u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 95u16, 0u16, 0u16, 0u16, 0u16, 96u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 97u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 98u16, 0u16, 99u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 100u16, 0u16, 0u16, 0u16, 0u16, 0u16, 101u16,
        102u16, 0u16, 0u16, 103u16, 0u16, 0u16, 0u16, 0u16, 0u16, 104u16, 0u16, 105u16,
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
        106u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 107u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 108u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 109u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 110u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 111u16,
        0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        112u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 0u16, 113u16, 0u16, 0u16, 0u16, 114u16, 0u16, 0u16, 0u16, 0u16,
        115u16, 0u16, 116u16, 0u16, 117u16, 118u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 0u16, 0u16, 119u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16, 0u16,
        0u16, 120u16,
    ];
    if pc < 2126388u32 || pc > 2131636u32 {
        return None;
    }
    let word_offset = ((pc - 2126388u32) >> 2) as usize;
    if word_offset >= IDX.len() {
        return None;
    }
    let idx_val = IDX[word_offset];
    if idx_val == 0 { None } else { Some(FN[(idx_val - 1) as usize]) }
}
#[inline(always)]
pub fn block_0x00207234(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 10usize, 0u32, 2126392u32);
    emu.lbu_no_count(13usize, 10usize, 0u32, 2126396u32);
    emu.adi_no_count(14usize, 0usize, 23u32, 2126400u32);
    emu.adi_no_count(10usize, 11usize, 0u32, 2126404u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(14usize);
    let a = emu.read_reg_a_tracked(13usize);
    if a != b {
        emu.pc = 2126428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020725c));
    } else {
        emu.pc = 2126408u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207248));
    }
}
#[inline(always)]
pub fn block_0x00207248(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2126412u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967080u32, 2126416u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2126420u32);
    emu.apc_no_count(6usize, 2126420u32, 94208u32, 2126424u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2126428u32;
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
pub fn block_0x0020725c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294967280u32, 2126432u32);
    emu.sw_no_count(1usize, 2usize, 12u32, 2126436u32)?;
    emu.sw_no_count(12usize, 2usize, 8u32, 2126440u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2126444u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294967064u32, 2126448u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2126452u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 768u32, 2126456u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2126460u32);
    emu.adi_no_count(13usize, 2usize, 8u32, 2126464u32);
    emu.apc_no_count(1usize, 2126464u32, 94208u32, 2126468u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126472u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965968u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207288(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 12u32, 2126476u32)?;
    emu.adi_no_count(2usize, 2usize, 16u32, 2126480u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126484u32;
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
pub fn block_0x00207294(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(6usize, 2126484u32, 0u32, 2126488u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2126492u32;
    emu.write_reg_no_count(0usize, return_addr);
    let target = base.wrapping_add(40u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020729c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 10usize, 0u32, 2126496u32);
    emu.lw_no_count(10usize, 10usize, 0u32, 2126500u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2126520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002072b8));
    } else {
        emu.pc = 2126504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002072a8));
    }
}
#[inline(always)]
pub fn block_0x002072a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 11usize, 4u32, 2126508u32)?;
    emu.lw_no_count(6usize, 11usize, 0u32, 2126512u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(6usize);
    if a == b {
        emu.pc = 2126520u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002072b8));
    } else {
        emu.pc = 2126516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002072b4));
    }
}
#[inline(always)]
pub fn block_0x002072b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(6usize);
    let return_addr = 2126520u32;
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
pub fn block_0x002072b8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126524u32;
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
pub fn block_0x002072bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(2usize, 2usize, 4294966176u32, 2126528u32);
    emu.sw_no_count(1usize, 2usize, 1116u32, 2126532u32)?;
    emu.sw_no_count(8usize, 2usize, 1112u32, 2126536u32)?;
    emu.sw_no_count(9usize, 2usize, 1108u32, 2126540u32)?;
    emu.sw_no_count(18usize, 2usize, 1104u32, 2126544u32)?;
    emu.sw_no_count(19usize, 2usize, 1100u32, 2126548u32)?;
    emu.sw_no_count(20usize, 2usize, 1096u32, 2126552u32)?;
    emu.sw_no_count(21usize, 2usize, 1092u32, 2126556u32)?;
    emu.sw_no_count(22usize, 2usize, 1088u32, 2126560u32)?;
    emu.sw_no_count(23usize, 2usize, 1084u32, 2126564u32)?;
    emu.sw_no_count(24usize, 2usize, 1080u32, 2126568u32)?;
    emu.sw_no_count(25usize, 2usize, 1076u32, 2126572u32)?;
    emu.sw_no_count(26usize, 2usize, 1072u32, 2126576u32)?;
    emu.sw_no_count(27usize, 2usize, 1068u32, 2126580u32)?;
    emu.adi_no_count(10usize, 2usize, 736u32, 2126584u32);
    emu.apc_no_count(1usize, 2126584u32, 8192u32, 2126588u32);
    emu.add_memory_rw_events(17usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126592u32;
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
pub fn block_0x00207300(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 740u32, 2126596u32)?;
    emu.lw_no_count(11usize, 2usize, 744u32, 2126600u32)?;
    emu.sw_no_count(10usize, 2usize, 624u32, 2126604u32)?;
    emu.sw_no_count(11usize, 2usize, 628u32, 2126608u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2126612u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966888u32, 2126616u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2126620u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966948u32, 2126624u32);
    emu.adi_no_count(10usize, 2usize, 848u32, 2126628u32);
    emu.adi_no_count(11usize, 2usize, 624u32, 2126632u32);
    emu.adi_no_count(13usize, 0usize, 13u32, 2126636u32);
    emu.adi_no_count(15usize, 0usize, 6u32, 2126640u32);
    emu.apc_no_count(1usize, 2126640u32, 4294946816u32, 2126644u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126648u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965588u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207338(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(9usize, 2usize, 908u32, 2126652u32)?;
    emu.lw_no_count(18usize, 2usize, 848u32, 2126656u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(8usize, a);
    emu.pc = 2126660u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a != b {
        emu.pc = 2126668u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020734c));
    } else {
        emu.pc = 2126664u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207348));
    }
}
#[inline(always)]
pub fn block_0x00207348(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2126668u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2131532u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020864c));
}
#[inline(always)]
pub fn block_0x0020734c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 2usize, 852u32, 2126672u32);
    emu.adi_no_count(10usize, 2usize, 164u32, 2126676u32);
    emu.adi_no_count(12usize, 0usize, 56u32, 2126680u32);
    emu.apc_no_count(1usize, 2126680u32, 8192u32, 2126684u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126688u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1292u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00207360(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 912u32, 2126692u32)?;
    emu.lw_no_count(11usize, 2usize, 916u32, 2126696u32)?;
    emu.sw_no_count(18usize, 2usize, 160u32, 2126700u32)?;
    emu.sw_no_count(9usize, 2usize, 220u32, 2126704u32)?;
    emu.sw_no_count(10usize, 2usize, 224u32, 2126708u32)?;
    emu.sw_no_count(11usize, 2usize, 228u32, 2126712u32)?;
    emu.adi_no_count(10usize, 2usize, 736u32, 2126716u32);
    emu.apc_no_count(1usize, 2126716u32, 8192u32, 2126720u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126724u32;
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
pub fn block_0x00207384(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 740u32, 2126728u32)?;
    emu.lw_no_count(11usize, 2usize, 744u32, 2126732u32)?;
    emu.sw_no_count(10usize, 2usize, 624u32, 2126736u32)?;
    emu.sw_no_count(11usize, 2usize, 628u32, 2126740u32)?;
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2126744u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294966776u32, 2126748u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2126752u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294966840u32, 2126756u32);
    emu.adi_no_count(10usize, 2usize, 848u32, 2126760u32);
    emu.adi_no_count(11usize, 2usize, 624u32, 2126764u32);
    emu.adi_no_count(13usize, 0usize, 9u32, 2126768u32);
    emu.adi_no_count(15usize, 0usize, 6u32, 2126772u32);
    emu.apc_no_count(1usize, 2126772u32, 4294942720u32, 2126776u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126780u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(428u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002073bc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 848u32, 2126784u32)?;
    emu.lw_no_count(27usize, 2usize, 852u32, 2126788u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(8usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2126796u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002073cc));
    } else {
        emu.pc = 2126792u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002073c8));
    }
}
#[inline(always)]
pub fn block_0x002073c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2126796u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2131540u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208654));
}
#[inline(never)]
pub fn block_0x002073cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 38u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(8usize, 2usize, 856u32, 2126800u32)?;
    emu.lw_no_count(21usize, 2usize, 864u32, 2126804u32)?;
    emu.lw_no_count(20usize, 2usize, 868u32, 2126808u32)?;
    emu.lw_no_count(10usize, 2usize, 872u32, 2126812u32)?;
    emu.lw_no_count(11usize, 2usize, 876u32, 2126816u32)?;
    emu.lw_no_count(12usize, 2usize, 880u32, 2126820u32)?;
    emu.lw_no_count(13usize, 2usize, 884u32, 2126824u32)?;
    emu.lw_no_count(14usize, 2usize, 888u32, 2126828u32)?;
    emu.lw_no_count(15usize, 2usize, 908u32, 2126832u32)?;
    emu.lw_no_count(16usize, 2usize, 912u32, 2126836u32)?;
    emu.lw_no_count(17usize, 2usize, 916u32, 2126840u32)?;
    emu.lw_no_count(5usize, 2usize, 920u32, 2126844u32)?;
    emu.sw_no_count(10usize, 2usize, 264u32, 2126848u32)?;
    emu.sw_no_count(11usize, 2usize, 268u32, 2126852u32)?;
    emu.sw_no_count(12usize, 2usize, 272u32, 2126856u32)?;
    emu.sw_no_count(13usize, 2usize, 276u32, 2126860u32)?;
    emu.lw_no_count(10usize, 2usize, 892u32, 2126864u32)?;
    emu.lw_no_count(11usize, 2usize, 896u32, 2126868u32)?;
    emu.lw_no_count(12usize, 2usize, 900u32, 2126872u32)?;
    emu.lw_no_count(13usize, 2usize, 904u32, 2126876u32)?;
    emu.sw_no_count(14usize, 2usize, 280u32, 2126880u32)?;
    emu.sw_no_count(10usize, 2usize, 284u32, 2126884u32)?;
    emu.sw_no_count(11usize, 2usize, 288u32, 2126888u32)?;
    emu.sw_no_count(12usize, 2usize, 292u32, 2126892u32)?;
    emu.lw_no_count(10usize, 2usize, 924u32, 2126896u32)?;
    emu.lw_no_count(11usize, 2usize, 928u32, 2126900u32)?;
    emu.lw_no_count(12usize, 2usize, 932u32, 2126904u32)?;
    emu.sw_no_count(5usize, 2usize, 248u32, 2126908u32)?;
    emu.sw_no_count(10usize, 2usize, 252u32, 2126912u32)?;
    emu.sw_no_count(11usize, 2usize, 256u32, 2126916u32)?;
    emu.sw_no_count(12usize, 2usize, 260u32, 2126920u32)?;
    emu.sw_no_count(13usize, 2usize, 232u32, 2126924u32)?;
    emu.sw_no_count(15usize, 2usize, 236u32, 2126928u32)?;
    emu.sw_no_count(16usize, 2usize, 240u32, 2126932u32)?;
    emu.sw_no_count(17usize, 2usize, 244u32, 2126936u32)?;
    emu.adi_no_count(10usize, 2usize, 736u32, 2126940u32);
    emu.apc_no_count(1usize, 2126940u32, 8192u32, 2126944u32);
    emu.add_memory_rw_events(38usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126948u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966248u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207464(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 740u32, 2126952u32)?;
    emu.lw_no_count(11usize, 2usize, 744u32, 2126956u32)?;
    emu.sw_no_count(10usize, 2usize, 624u32, 2126960u32)?;
    emu.sw_no_count(11usize, 2usize, 628u32, 2126964u32)?;
    emu.adi_no_count(10usize, 2usize, 848u32, 2126968u32);
    emu.adi_no_count(11usize, 2usize, 624u32, 2126972u32);
    emu.apc_no_count(1usize, 2126972u32, 4294938624u32, 2126976u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2126980u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1100u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207484(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 848u32, 2126984u32)?;
    emu.lw_no_count(19usize, 2usize, 852u32, 2126988u32)?;
    let a = 0u32.wrapping_add(2147483648u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2126992u32;
    emu.update_insn_clock();
    emu.add_memory_rw_events(3usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2127000u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207498));
    } else {
        emu.pc = 2126996u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207494));
    }
}
#[inline(always)]
pub fn block_0x00207494(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2127000u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2131548u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x0020865c));
}
#[inline(always)]
pub fn block_0x00207498(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(9usize, 2usize, 856u32, 2127004u32)?;
    emu.adi_no_count(10usize, 2usize, 296u32, 2127008u32);
    emu.adi_no_count(11usize, 2usize, 160u32, 2127012u32);
    emu.apc_no_count(1usize, 2127012u32, 65536u32, 2127016u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127020u32;
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
pub fn block_0x002074ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 848u32, 2127024u32);
    emu.adi_no_count(11usize, 27usize, 0u32, 2127028u32);
    emu.adi_no_count(12usize, 8usize, 0u32, 2127032u32);
    emu.apc_no_count(1usize, 2127032u32, 4294959104u32, 2127036u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127040u32;
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
pub fn block_0x002074c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 848u32, 2127044u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2127052u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002074cc));
    } else {
        emu.pc = 2127048u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002074c8));
    }
}
#[inline(always)]
pub fn block_0x002074c8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2127052u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2131592u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208688));
}
#[inline(always)]
pub fn block_0x002074cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(18usize, 2usize, 852u32, 2127056u32)?;
    emu.lw_no_count(22usize, 2usize, 856u32, 2127060u32)?;
    emu.adi_no_count(11usize, 2usize, 860u32, 2127064u32);
    emu.adi_no_count(10usize, 2usize, 316u32, 2127068u32);
    emu.adi_no_count(12usize, 0usize, 60u32, 2127072u32);
    emu.apc_no_count(1usize, 2127072u32, 8192u32, 2127076u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127080u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(900u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x002074e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 39u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 2usize, 308u32, 2127084u32)?;
    emu.sw_no_count(22usize, 2usize, 312u32, 2127088u32)?;
    emu.lw_no_count(10usize, 2usize, 264u32, 2127092u32)?;
    emu.lw_no_count(11usize, 2usize, 268u32, 2127096u32)?;
    emu.lw_no_count(12usize, 2usize, 272u32, 2127100u32)?;
    emu.lw_no_count(13usize, 2usize, 276u32, 2127104u32)?;
    emu.sw_no_count(10usize, 2usize, 376u32, 2127108u32)?;
    emu.sw_no_count(11usize, 2usize, 380u32, 2127112u32)?;
    emu.sw_no_count(12usize, 2usize, 384u32, 2127116u32)?;
    emu.sw_no_count(13usize, 2usize, 388u32, 2127120u32)?;
    emu.lw_no_count(10usize, 2usize, 280u32, 2127124u32)?;
    emu.lw_no_count(11usize, 2usize, 284u32, 2127128u32)?;
    emu.lw_no_count(12usize, 2usize, 288u32, 2127132u32)?;
    emu.lw_no_count(13usize, 2usize, 292u32, 2127136u32)?;
    emu.sw_no_count(10usize, 2usize, 392u32, 2127140u32)?;
    emu.sw_no_count(11usize, 2usize, 396u32, 2127144u32)?;
    emu.sw_no_count(12usize, 2usize, 400u32, 2127148u32)?;
    emu.sw_no_count(13usize, 2usize, 404u32, 2127152u32)?;
    emu.lw_no_count(10usize, 2usize, 248u32, 2127156u32)?;
    emu.lw_no_count(11usize, 2usize, 252u32, 2127160u32)?;
    emu.lw_no_count(12usize, 2usize, 256u32, 2127164u32)?;
    emu.lw_no_count(13usize, 2usize, 260u32, 2127168u32)?;
    emu.sw_no_count(10usize, 2usize, 424u32, 2127172u32)?;
    emu.sw_no_count(11usize, 2usize, 428u32, 2127176u32)?;
    emu.sw_no_count(12usize, 2usize, 432u32, 2127180u32)?;
    emu.sw_no_count(13usize, 2usize, 436u32, 2127184u32)?;
    emu.lw_no_count(10usize, 2usize, 232u32, 2127188u32)?;
    emu.lw_no_count(11usize, 2usize, 236u32, 2127192u32)?;
    emu.lw_no_count(12usize, 2usize, 240u32, 2127196u32)?;
    emu.lw_no_count(13usize, 2usize, 244u32, 2127200u32)?;
    emu.sw_no_count(10usize, 2usize, 408u32, 2127204u32)?;
    emu.sw_no_count(11usize, 2usize, 412u32, 2127208u32)?;
    emu.sw_no_count(12usize, 2usize, 416u32, 2127212u32)?;
    emu.sw_no_count(13usize, 2usize, 420u32, 2127216u32)?;
    emu.adi_no_count(10usize, 2usize, 848u32, 2127220u32);
    emu.adi_no_count(11usize, 2usize, 376u32, 2127224u32);
    emu.adi_no_count(12usize, 2usize, 408u32, 2127228u32);
    emu.apc_no_count(1usize, 2127228u32, 4294963200u32, 2127232u32);
    emu.add_memory_rw_events(39usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127236u32;
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
pub fn block_0x00207584(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 848u32, 2127240u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2131316u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208574));
    } else {
        emu.pc = 2127244u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020758c));
    }
}
#[inline(always)]
pub fn block_0x0020758c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 2usize, 852u32, 2127248u32);
    emu.adi_no_count(10usize, 2usize, 440u32, 2127252u32);
    emu.adi_no_count(12usize, 0usize, 64u32, 2127256u32);
    emu.adi_no_count(18usize, 0usize, 64u32, 2127260u32);
    emu.apc_no_count(1usize, 2127260u32, 8192u32, 2127264u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127268u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(712u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002075a4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(23usize, 2usize, 300u32, 2127272u32)?;
    emu.lw_no_count(22usize, 2usize, 304u32, 2127276u32)?;
    emu.adi_no_count(25usize, 2usize, 888u32, 2127280u32);
    emu.adi_no_count(12usize, 0usize, 65u32, 2127284u32);
    emu.adi_no_count(10usize, 25usize, 0u32, 2127288u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2127292u32);
    emu.apc_no_count(1usize, 2127292u32, 8192u32, 2127296u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127300u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(432u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x002075c4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 29u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(1779032064u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2127304u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(3144134656u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2127308u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1013903360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2127312u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2773479424u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2127316u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1359892480u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2127320u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2600824832u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2127324u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(528736256u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2127328u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1541459968u32);
    emu.write_reg_no_count(17usize, a);
    emu.pc = 2127332u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1639u32, 2127336u32);
    emu.adi_no_count(11usize, 11usize, 4294966917u32, 2127340u32);
    emu.adi_no_count(12usize, 12usize, 882u32, 2127344u32);
    emu.adi_no_count(13usize, 13usize, 1338u32, 2127348u32);
    emu.sw_no_count(10usize, 2usize, 848u32, 2127352u32)?;
    emu.sw_no_count(11usize, 2usize, 852u32, 2127356u32)?;
    emu.sw_no_count(12usize, 2usize, 856u32, 2127360u32)?;
    emu.sw_no_count(13usize, 2usize, 860u32, 2127364u32)?;
    emu.lbu_no_count(24usize, 2usize, 952u32, 2127368u32);
    emu.adi_no_count(10usize, 14usize, 639u32, 2127372u32);
    emu.adi_no_count(11usize, 15usize, 4294965388u32, 2127376u32);
    emu.adi_no_count(12usize, 16usize, 4294965675u32, 2127380u32);
    emu.adi_no_count(13usize, 17usize, 4294966553u32, 2127384u32);
    emu.sw_no_count(10usize, 2usize, 864u32, 2127388u32)?;
    emu.sw_no_count(11usize, 2usize, 868u32, 2127392u32)?;
    emu.sw_no_count(12usize, 2usize, 872u32, 2127396u32)?;
    emu.sw_no_count(13usize, 2usize, 876u32, 2127400u32)?;
    emu.sbr_no_count(12usize, 18usize, 24usize, 2127404u32);
    emu.sw_no_count(0usize, 2usize, 880u32, 2127408u32)?;
    emu.sw_no_count(0usize, 2usize, 884u32, 2127412u32)?;
    emu.add_memory_rw_events(28usize);
    let b = emu.read_reg_b_tracked(12usize);
    let a = emu.read_reg_a_tracked(22usize);
    if a >= b {
        emu.pc = 2127444u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207654));
    } else {
        emu.pc = 2127416u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207638));
    }
}
#[inline(always)]
pub fn block_0x00207638(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(10usize, 25usize, 24usize, 2127420u32);
    emu.adi_no_count(11usize, 23usize, 0u32, 2127424u32);
    emu.adi_no_count(12usize, 22usize, 0u32, 2127428u32);
    emu.apc_no_count(1usize, 2127428u32, 8192u32, 2127432u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127436u32;
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
pub fn block_0x0020764c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adr_no_count(24usize, 22usize, 24usize, 2127440u32);
    emu.add_memory_rw_events(2usize);
    let return_addr = 2127444u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2127592u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002076e8));
}
#[inline(always)]
pub fn block_0x00207654(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(24usize);
    if a == b {
        emu.pc = 2127508u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207694));
    } else {
        emu.pc = 2127448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207658));
    }
}
#[inline(always)]
pub fn block_0x00207658(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(22usize, 22usize, 12usize, 2127452u32);
    emu.adr_no_count(18usize, 23usize, 12usize, 2127456u32);
    emu.adr_no_count(10usize, 25usize, 24usize, 2127460u32);
    emu.adi_no_count(11usize, 23usize, 0u32, 2127464u32);
    emu.apc_no_count(1usize, 2127464u32, 8192u32, 2127468u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127472u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(508u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207670(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 8u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2127476u32);
    emu.sw_no_count(10usize, 2usize, 880u32, 2127480u32)?;
    emu.sw_no_count(0usize, 2usize, 884u32, 2127484u32)?;
    emu.adi_no_count(10usize, 2usize, 848u32, 2127488u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2127492u32);
    emu.adi_no_count(11usize, 25usize, 0u32, 2127496u32);
    emu.apc_no_count(1usize, 2127496u32, 45056u32, 2127500u32);
    emu.add_memory_rw_events(8usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127504u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967148u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207690(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(23usize, 18usize, 0u32, 2127508u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2127508u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207694));
}
#[inline(always)]
pub fn block_0x00207694(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(10usize, 22usize, 4294967232u32, 2127512u32);
    emu.ani_no_count(24usize, 22usize, 63u32, 2127516u32);
    emu.sri_no_count(12usize, 22usize, 6u32, 2127520u32);
    emu.adr_no_count(22usize, 23usize, 10usize, 2127524u32);
    emu.add_memory_rw_events(4usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2127572u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002076d4));
    } else {
        emu.pc = 2127528u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002076a8));
    }
}
#[inline]
pub fn block_0x002076a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 880u32, 2127532u32)?;
    emu.lw_no_count(11usize, 2usize, 884u32, 2127536u32)?;
    emu.adr_no_count(13usize, 10usize, 12usize, 2127540u32);
    emu.sltru_no_count(10usize, 13usize, 10usize, 2127544u32);
    emu.adr_no_count(10usize, 11usize, 10usize, 2127548u32);
    emu.sw_no_count(13usize, 2usize, 880u32, 2127552u32)?;
    emu.sw_no_count(10usize, 2usize, 884u32, 2127556u32)?;
    emu.adi_no_count(10usize, 2usize, 848u32, 2127560u32);
    emu.adi_no_count(11usize, 23usize, 0u32, 2127564u32);
    emu.apc_no_count(1usize, 2127564u32, 45056u32, 2127568u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127572u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967080u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002076d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 25usize, 0u32, 2127576u32);
    emu.adi_no_count(11usize, 22usize, 0u32, 2127580u32);
    emu.adi_no_count(12usize, 24usize, 0u32, 2127584u32);
    emu.apc_no_count(1usize, 2127584u32, 8192u32, 2127588u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127592u32;
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
pub fn block_0x002076e8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(24usize, 2usize, 952u32, 2127596u32);
    emu.adi_no_count(10usize, 2usize, 736u32, 2127600u32);
    emu.adi_no_count(11usize, 2usize, 848u32, 2127604u32);
    emu.adi_no_count(12usize, 0usize, 112u32, 2127608u32);
    emu.apc_no_count(1usize, 2127608u32, 8192u32, 2127612u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127616u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(364u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207700(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 848u32, 2127620u32);
    emu.adi_no_count(11usize, 2usize, 308u32, 2127624u32);
    emu.adi_no_count(12usize, 2usize, 736u32, 2127628u32);
    emu.adi_no_count(13usize, 2usize, 440u32, 2127632u32);
    emu.apc_no_count(1usize, 2127632u32, 4294946816u32, 2127636u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127640u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1796u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207718(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 848u32, 2127644u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a != b {
        emu.pc = 2131372u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002085ac));
    } else {
        emu.pc = 2127648u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207720));
    }
}
#[inline(always)]
pub fn block_0x00207720(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(22usize, 2usize, 544u32, 2127652u32);
    emu.adi_no_count(12usize, 0usize, 65u32, 2127656u32);
    emu.adi_no_count(10usize, 22usize, 0u32, 2127660u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2127664u32);
    emu.apc_no_count(1usize, 2127664u32, 8192u32, 2127668u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127672u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(60u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00207738(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 28u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(1779032064u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2127676u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(3144134656u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2127680u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1013903360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2127684u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2773479424u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2127688u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1359892480u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2127692u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2600824832u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2127696u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(528736256u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2127700u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1639u32, 2127704u32);
    emu.adi_no_count(11usize, 11usize, 4294966917u32, 2127708u32);
    emu.adi_no_count(12usize, 12usize, 882u32, 2127712u32);
    emu.adi_no_count(13usize, 13usize, 1338u32, 2127716u32);
    emu.sw_no_count(10usize, 2usize, 504u32, 2127720u32)?;
    emu.sw_no_count(11usize, 2usize, 508u32, 2127724u32)?;
    emu.sw_no_count(12usize, 2usize, 512u32, 2127728u32)?;
    emu.sw_no_count(13usize, 2usize, 516u32, 2127732u32)?;
    let a = 0u32.wrapping_add(1541459968u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2127736u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 14usize, 639u32, 2127740u32);
    emu.adi_no_count(12usize, 15usize, 4294965388u32, 2127744u32);
    emu.adi_no_count(13usize, 16usize, 4294965675u32, 2127748u32);
    emu.adi_no_count(10usize, 10usize, 4294966553u32, 2127752u32);
    emu.sw_no_count(11usize, 2usize, 520u32, 2127756u32)?;
    emu.sw_no_count(12usize, 2usize, 524u32, 2127760u32)?;
    emu.sw_no_count(13usize, 2usize, 528u32, 2127764u32)?;
    emu.sw_no_count(10usize, 2usize, 532u32, 2127768u32)?;
    emu.sri_no_count(12usize, 20usize, 6u32, 2127772u32);
    emu.sw_no_count(0usize, 2usize, 536u32, 2127776u32)?;
    emu.sw_no_count(0usize, 2usize, 540u32, 2127780u32)?;
    emu.add_memory_rw_events(27usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2127832u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002077d8));
    } else {
        emu.pc = 2127784u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002077a8));
    }
}
#[inline]
pub fn block_0x002077a8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(23usize, 20usize, 4294967232u32, 2127788u32);
    emu.ani_no_count(20usize, 20usize, 63u32, 2127792u32);
    emu.adr_no_count(23usize, 21usize, 23usize, 2127796u32);
    emu.sw_no_count(12usize, 2usize, 536u32, 2127800u32)?;
    emu.sw_no_count(0usize, 2usize, 540u32, 2127804u32)?;
    emu.adi_no_count(10usize, 2usize, 504u32, 2127808u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2127812u32);
    emu.apc_no_count(1usize, 2127812u32, 45056u32, 2127816u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127820u32;
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
#[inline(always)]
pub fn block_0x002077cc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 22usize, 0u32, 2127824u32);
    emu.adi_no_count(11usize, 23usize, 0u32, 2127828u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2127832u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2127840u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002077e0));
}
#[inline(always)]
pub fn block_0x002077d8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 22usize, 0u32, 2127836u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2127840u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2127840u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x002077e0));
}
#[inline(always)]
pub fn block_0x002077e0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 20usize, 0u32, 2127844u32);
    emu.apc_no_count(1usize, 2127844u32, 8192u32, 2127848u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127852u32;
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
pub fn block_0x002077ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(20usize, 2usize, 608u32, 2127856u32);
    emu.adi_no_count(10usize, 2usize, 848u32, 2127860u32);
    emu.adi_no_count(11usize, 2usize, 504u32, 2127864u32);
    emu.adi_no_count(12usize, 0usize, 112u32, 2127868u32);
    emu.apc_no_count(1usize, 2127868u32, 8192u32, 2127872u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2127876u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(104u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00207804(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 34u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(18usize, 2usize, 952u32, 2127880u32);
    emu.lw_no_count(10usize, 2usize, 880u32, 2127884u32)?;
    emu.lw_no_count(11usize, 2usize, 884u32, 2127888u32)?;
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2127892u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 0usize, 128u32, 2127896u32);
    emu.adi_no_count(12usize, 12usize, 4294967040u32, 2127900u32);
    emu.sli_no_count(14usize, 11usize, 9u32, 2127904u32);
    emu.sli_no_count(15usize, 10usize, 9u32, 2127908u32);
    emu.sli_no_count(16usize, 18usize, 3u32, 2127912u32);
    emu.sli_no_count(17usize, 10usize, 1u32, 2127916u32);
    emu.sli_no_count(11usize, 11usize, 1u32, 2127920u32);
    emu.orr_no_count(16usize, 15usize, 16usize, 2127924u32);
    emu.anr_no_count(17usize, 17usize, 12usize, 2127928u32);
    emu.sri_no_count(15usize, 15usize, 24u32, 2127932u32);
    emu.anr_no_count(11usize, 11usize, 12usize, 2127936u32);
    emu.orr_no_count(15usize, 17usize, 15usize, 2127940u32);
    emu.sri_no_count(17usize, 14usize, 24u32, 2127944u32);
    emu.orr_no_count(11usize, 11usize, 17usize, 2127948u32);
    emu.adi_no_count(17usize, 0usize, 63u32, 2127952u32);
    emu.sri_no_count(5usize, 10usize, 23u32, 2127956u32);
    emu.orr_no_count(10usize, 14usize, 5usize, 2127960u32);
    emu.anr_no_count(14usize, 16usize, 12usize, 2127964u32);
    emu.anr_no_count(12usize, 10usize, 12usize, 2127968u32);
    emu.sli_no_count(10usize, 18usize, 27u32, 2127972u32);
    emu.orr_no_count(15usize, 10usize, 15usize, 2127976u32);
    emu.adr_no_count(10usize, 25usize, 18usize, 2127980u32);
    emu.sli_no_count(5usize, 5usize, 24u32, 2127984u32);
    emu.sli_no_count(14usize, 14usize, 8u32, 2127988u32);
    emu.sli_no_count(12usize, 12usize, 8u32, 2127992u32);
    emu.orr_no_count(11usize, 5usize, 11usize, 2127996u32);
    emu.orr_no_count(21usize, 15usize, 14usize, 2128000u32);
    emu.orr_no_count(20usize, 11usize, 12usize, 2128004u32);
    emu.sb_no_count(13usize, 10usize, 0u32, 2128008u32);
    emu.add_memory_rw_events(33usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(18usize);
    if a == b {
        emu.pc = 2128044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002078ac));
    } else {
        emu.pc = 2128012u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020788c));
    }
}
#[inline(always)]
pub fn block_0x0020788c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 10usize, 1u32, 2128016u32);
    emu.xri_no_count(12usize, 18usize, 63u32, 2128020u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2128024u32);
    emu.apc_no_count(1usize, 2128024u32, 8192u32, 2128028u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128032u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966996u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002078a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(10usize, 18usize, 56u32, 2128036u32);
    emu.adi_no_count(11usize, 0usize, 7u32, 2128040u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2128156u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x0020791c));
    } else {
        emu.pc = 2128044u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002078ac));
    }
}
#[inline(always)]
pub fn block_0x002078ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 848u32, 2128048u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2128052u32);
    emu.adi_no_count(11usize, 25usize, 0u32, 2128056u32);
    emu.apc_no_count(1usize, 2128056u32, 45056u32, 2128060u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128064u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966588u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002078c0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 736u32, 2128068u32);
    emu.adi_no_count(12usize, 0usize, 56u32, 2128072u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2128076u32);
    emu.apc_no_count(1usize, 2128076u32, 8192u32, 2128080u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128084u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966944u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002078d4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 21usize, 24u32, 2128088u32);
    emu.sri_no_count(11usize, 21usize, 16u32, 2128092u32);
    emu.sri_no_count(12usize, 21usize, 8u32, 2128096u32);
    emu.sri_no_count(13usize, 20usize, 24u32, 2128100u32);
    emu.sri_no_count(14usize, 20usize, 16u32, 2128104u32);
    emu.sb_no_count(21usize, 2usize, 796u32, 2128108u32);
    emu.sb_no_count(12usize, 2usize, 797u32, 2128112u32);
    emu.sb_no_count(11usize, 2usize, 798u32, 2128116u32);
    emu.sb_no_count(10usize, 2usize, 799u32, 2128120u32);
    emu.sri_no_count(10usize, 20usize, 8u32, 2128124u32);
    emu.sb_no_count(20usize, 2usize, 792u32, 2128128u32);
    emu.sb_no_count(10usize, 2usize, 793u32, 2128132u32);
    emu.sb_no_count(14usize, 2usize, 794u32, 2128136u32);
    emu.sb_no_count(13usize, 2usize, 795u32, 2128140u32);
    emu.adi_no_count(10usize, 2usize, 848u32, 2128144u32);
    emu.adi_no_count(11usize, 2usize, 736u32, 2128148u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2128152u32);
    emu.add_memory_rw_events(18usize);
    let return_addr = 2128156u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2128176u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207930));
}
#[inline(always)]
pub fn block_0x0020791c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(20usize, 2usize, 944u32, 2128160u32)?;
    emu.sw_no_count(21usize, 2usize, 948u32, 2128164u32)?;
    emu.adi_no_count(10usize, 2usize, 848u32, 2128168u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2128172u32);
    emu.adi_no_count(11usize, 25usize, 0u32, 2128176u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2128176u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207930));
}
#[inline(always)]
pub fn block_0x00207930(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2128176u32, 45056u32, 2128180u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128184u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966468u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00207938(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 848u32, 2128188u32)?;
    emu.lw_no_count(12usize, 2usize, 852u32, 2128192u32)?;
    emu.lw_no_count(13usize, 2usize, 856u32, 2128196u32)?;
    emu.lw_no_count(14usize, 2usize, 860u32, 2128200u32)?;
    emu.lw_no_count(10usize, 2usize, 228u32, 2128204u32)?;
    emu.lw_no_count(15usize, 2usize, 864u32, 2128208u32)?;
    emu.lw_no_count(16usize, 2usize, 868u32, 2128212u32)?;
    emu.lw_no_count(17usize, 2usize, 872u32, 2128216u32)?;
    emu.lw_no_count(5usize, 2usize, 876u32, 2128220u32)?;
    emu.sw_no_count(9usize, 2usize, 616u32, 2128224u32)?;
    emu.sw_no_count(10usize, 2usize, 620u32, 2128228u32)?;
    emu.add_memory_rw_events(11usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a != b {
        emu.pc = 2131428u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002085e4));
    } else {
        emu.pc = 2128232u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207968));
    }
}
#[inline]
pub fn block_0x00207968(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(5usize, 2usize, 128u32, 2128236u32)?;
    emu.sw_no_count(17usize, 2usize, 132u32, 2128240u32)?;
    emu.sw_no_count(16usize, 2usize, 136u32, 2128244u32)?;
    emu.sw_no_count(15usize, 2usize, 140u32, 2128248u32)?;
    emu.sw_no_count(14usize, 2usize, 144u32, 2128252u32)?;
    emu.sw_no_count(13usize, 2usize, 148u32, 2128256u32)?;
    emu.sw_no_count(12usize, 2usize, 152u32, 2128260u32)?;
    emu.sw_no_count(11usize, 2usize, 156u32, 2128264u32)?;
    emu.adi_no_count(20usize, 2usize, 664u32, 2128268u32);
    emu.adi_no_count(12usize, 0usize, 65u32, 2128272u32);
    emu.adi_no_count(10usize, 20usize, 0u32, 2128276u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2128280u32);
    emu.apc_no_count(1usize, 2128280u32, 8192u32, 2128284u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128288u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966740u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x002079a0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 27u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(1779032064u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2128292u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(3144134656u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2128296u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1013903360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2128300u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2773479424u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2128304u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1359892480u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2128308u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2600824832u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2128312u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(528736256u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2128316u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1639u32, 2128320u32);
    emu.adi_no_count(11usize, 11usize, 4294966917u32, 2128324u32);
    emu.adi_no_count(12usize, 12usize, 882u32, 2128328u32);
    emu.adi_no_count(13usize, 13usize, 1338u32, 2128332u32);
    emu.sw_no_count(10usize, 2usize, 624u32, 2128336u32)?;
    emu.sw_no_count(11usize, 2usize, 628u32, 2128340u32)?;
    emu.sw_no_count(12usize, 2usize, 632u32, 2128344u32)?;
    emu.sw_no_count(13usize, 2usize, 636u32, 2128348u32)?;
    let a = 0u32.wrapping_add(1541459968u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2128352u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 14usize, 639u32, 2128356u32);
    emu.adi_no_count(12usize, 15usize, 4294965388u32, 2128360u32);
    emu.adi_no_count(13usize, 16usize, 4294965675u32, 2128364u32);
    emu.adi_no_count(10usize, 10usize, 4294966553u32, 2128368u32);
    emu.sw_no_count(11usize, 2usize, 640u32, 2128372u32)?;
    emu.sw_no_count(12usize, 2usize, 644u32, 2128376u32)?;
    emu.sw_no_count(13usize, 2usize, 648u32, 2128380u32)?;
    emu.sw_no_count(10usize, 2usize, 652u32, 2128384u32)?;
    emu.sw_no_count(0usize, 2usize, 656u32, 2128388u32)?;
    emu.sw_no_count(0usize, 2usize, 660u32, 2128392u32)?;
    emu.add_memory_rw_events(26usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2128656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207b10));
    } else {
        emu.pc = 2128396u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207a0c));
    }
}
#[inline]
pub fn block_0x00207a0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 13u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(18usize, 0usize, 0u32, 2128400u32);
    emu.adi_no_count(21usize, 0usize, 0u32, 2128404u32);
    emu.adi_no_count(26usize, 0usize, 0u32, 2128408u32);
    emu.sli_no_count(22usize, 9usize, 6u32, 2128412u32);
    emu.adi_no_count(10usize, 19usize, 64u32, 2128416u32);
    emu.adi_no_count(11usize, 9usize, 4294967295u32, 2128420u32);
    emu.adi_no_count(9usize, 0usize, 32u32, 2128424u32);
    emu.adr_no_count(22usize, 19usize, 22usize, 2128428u32);
    emu.sli_no_count(11usize, 11usize, 6u32, 2128432u32);
    emu.sri_no_count(11usize, 11usize, 6u32, 2128436u32);
    emu.adi_no_count(23usize, 11usize, 1u32, 2128440u32);
    emu.adi_no_count(24usize, 0usize, 64u32, 2128444u32);
    emu.add_memory_rw_events(13usize);
    let return_addr = 2128448u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2128504u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207a78));
}
#[inline(always)]
pub fn block_0x00207a40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 0usize, 32u32, 2128452u32);
    emu.apc_no_count(1usize, 2128452u32, 8192u32, 2128456u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128460u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966816u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207a4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(19usize, 26usize, 32u32, 2128464u32);
    emu.add_memory_rw_events(1usize);
    emu.pc = 2128464u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207a50));
}
#[inline]
pub fn block_0x00207a50(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(26usize, 19usize, 255u32, 2128468u32);
    emu.sb_no_count(19usize, 2usize, 728u32, 2128472u32);
    emu.xrr_no_count(10usize, 25usize, 22usize, 2128476u32);
    emu.adi_no_count(21usize, 21usize, 1u32, 2128480u32);
    emu.sltru_no_count(10usize, 0usize, 10usize, 2128484u32);
    emu.sli_no_count(10usize, 10usize, 6u32, 2128488u32);
    emu.adr_no_count(10usize, 25usize, 10usize, 2128492u32);
    emu.adi_no_count(18usize, 18usize, 32u32, 2128496u32);
    emu.adi_no_count(19usize, 25usize, 0u32, 2128500u32);
    emu.add_memory_rw_events(9usize);
    let b = emu.read_reg_b_tracked(21usize);
    let a = emu.read_reg_a_tracked(23usize);
    if a == b {
        emu.pc = 2128656u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207b10));
    } else {
        emu.pc = 2128504u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207a78));
    }
}
#[inline(always)]
pub fn block_0x00207a78(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(11usize, 2usize, 228u32, 2128508u32)?;
    emu.sw_no_count(21usize, 2usize, 1004u32, 2128512u32)?;
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(11usize);
    let a = emu.read_reg_a_tracked(21usize);
    if a >= b {
        emu.pc = 2131512u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208638));
    } else {
        emu.pc = 2128516u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207a84));
    }
}
#[inline(always)]
pub fn block_0x00207a84(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 7u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(25usize, 10usize, 0u32, 2128520u32);
    emu.lw_no_count(11usize, 2usize, 224u32, 2128524u32)?;
    emu.adr_no_count(11usize, 11usize, 18usize, 2128528u32);
    emu.adi_no_count(12usize, 0usize, 32u32, 2128532u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2128536u32);
    emu.apc_no_count(1usize, 2128536u32, 94208u32, 2128540u32);
    emu.add_memory_rw_events(7usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128544u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(2016u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207aa0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2131240u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208528));
    } else {
        emu.pc = 2128548u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207aa4));
    }
}
#[inline(always)]
pub fn block_0x00207aa4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(11usize, 19usize, 32u32, 2128552u32);
    emu.adr_no_count(10usize, 20usize, 26usize, 2128556u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(9usize);
    let a = emu.read_reg_a_tracked(26usize);
    if a < b {
        emu.pc = 2128448u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207a40));
    } else {
        emu.pc = 2128560u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207ab0));
    }
}
#[inline(always)]
pub fn block_0x00207ab0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sbr_no_count(12usize, 24usize, 26usize, 2128564u32);
    emu.adi_no_count(19usize, 26usize, 4294967264u32, 2128568u32);
    emu.adr_no_count(26usize, 11usize, 12usize, 2128572u32);
    emu.apc_no_count(1usize, 2128572u32, 8192u32, 2128576u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128580u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966696u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00207ac4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 12u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 656u32, 2128584u32)?;
    emu.lw_no_count(11usize, 2usize, 660u32, 2128588u32)?;
    emu.adi_no_count(10usize, 10usize, 1u32, 2128592u32);
    emu.sltiu_no_count(12usize, 10usize, 1u32, 2128596u32);
    emu.adr_no_count(11usize, 11usize, 12usize, 2128600u32);
    emu.sw_no_count(10usize, 2usize, 656u32, 2128604u32)?;
    emu.sw_no_count(11usize, 2usize, 660u32, 2128608u32)?;
    emu.adi_no_count(10usize, 2usize, 624u32, 2128612u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2128616u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2128620u32);
    emu.apc_no_count(1usize, 2128620u32, 45056u32, 2128624u32);
    emu.add_memory_rw_events(12usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128628u32;
    emu.write_reg_no_count(1usize, return_addr);
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
pub fn block_0x00207af4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(11usize, 19usize, 4294967232u32, 2128632u32);
    emu.adr_no_count(11usize, 26usize, 11usize, 2128636u32);
    emu.adi_no_count(10usize, 20usize, 0u32, 2128640u32);
    emu.adi_no_count(12usize, 19usize, 0u32, 2128644u32);
    emu.apc_no_count(1usize, 2128644u32, 8192u32, 2128648u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128652u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966624u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207b0c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.add_memory_rw_events(1usize);
    let return_addr = 2128656u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2128464u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207a50));
}
#[inline(always)]
pub fn block_0x00207b10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 848u32, 2128660u32);
    emu.adi_no_count(11usize, 2usize, 624u32, 2128664u32);
    emu.adi_no_count(12usize, 0usize, 112u32, 2128668u32);
    emu.apc_no_count(1usize, 2128668u32, 8192u32, 2128672u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128676u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966600u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00207b24(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 35u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(9usize, 2usize, 952u32, 2128680u32);
    emu.lw_no_count(10usize, 2usize, 880u32, 2128684u32)?;
    emu.lw_no_count(11usize, 2usize, 884u32, 2128688u32)?;
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2128692u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 0usize, 128u32, 2128696u32);
    emu.adi_no_count(12usize, 12usize, 4294967040u32, 2128700u32);
    emu.sli_no_count(14usize, 11usize, 9u32, 2128704u32);
    emu.sli_no_count(15usize, 10usize, 9u32, 2128708u32);
    emu.sli_no_count(16usize, 9usize, 3u32, 2128712u32);
    emu.sli_no_count(17usize, 10usize, 1u32, 2128716u32);
    emu.sli_no_count(11usize, 11usize, 1u32, 2128720u32);
    emu.orr_no_count(16usize, 15usize, 16usize, 2128724u32);
    emu.anr_no_count(17usize, 17usize, 12usize, 2128728u32);
    emu.sri_no_count(15usize, 15usize, 24u32, 2128732u32);
    emu.anr_no_count(11usize, 11usize, 12usize, 2128736u32);
    emu.orr_no_count(15usize, 17usize, 15usize, 2128740u32);
    emu.sri_no_count(17usize, 14usize, 24u32, 2128744u32);
    emu.orr_no_count(11usize, 11usize, 17usize, 2128748u32);
    emu.adi_no_count(17usize, 0usize, 63u32, 2128752u32);
    emu.sri_no_count(5usize, 10usize, 23u32, 2128756u32);
    emu.orr_no_count(10usize, 14usize, 5usize, 2128760u32);
    emu.anr_no_count(14usize, 16usize, 12usize, 2128764u32);
    emu.anr_no_count(12usize, 10usize, 12usize, 2128768u32);
    emu.sli_no_count(10usize, 9usize, 27u32, 2128772u32);
    emu.orr_no_count(15usize, 10usize, 15usize, 2128776u32);
    emu.adi_no_count(21usize, 2usize, 888u32, 2128780u32);
    emu.adr_no_count(10usize, 21usize, 9usize, 2128784u32);
    emu.sli_no_count(5usize, 5usize, 24u32, 2128788u32);
    emu.sli_no_count(14usize, 14usize, 8u32, 2128792u32);
    emu.sli_no_count(12usize, 12usize, 8u32, 2128796u32);
    emu.orr_no_count(11usize, 5usize, 11usize, 2128800u32);
    emu.orr_no_count(19usize, 15usize, 14usize, 2128804u32);
    emu.orr_no_count(18usize, 11usize, 12usize, 2128808u32);
    emu.sb_no_count(13usize, 10usize, 0u32, 2128812u32);
    emu.add_memory_rw_events(34usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(9usize);
    if a == b {
        emu.pc = 2128848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207bd0));
    } else {
        emu.pc = 2128816u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207bb0));
    }
}
#[inline(always)]
pub fn block_0x00207bb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 10usize, 1u32, 2128820u32);
    emu.xri_no_count(12usize, 9usize, 63u32, 2128824u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2128828u32);
    emu.apc_no_count(1usize, 2128828u32, 8192u32, 2128832u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128836u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966192u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207bc4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(10usize, 9usize, 56u32, 2128840u32);
    emu.adi_no_count(11usize, 0usize, 7u32, 2128844u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2128960u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207c40));
    } else {
        emu.pc = 2128848u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207bd0));
    }
}
#[inline(always)]
pub fn block_0x00207bd0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 848u32, 2128852u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2128856u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2128860u32);
    emu.apc_no_count(1usize, 2128860u32, 45056u32, 2128864u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128868u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965784u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207be4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 736u32, 2128872u32);
    emu.adi_no_count(12usize, 0usize, 56u32, 2128876u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2128880u32);
    emu.apc_no_count(1usize, 2128880u32, 8192u32, 2128884u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128888u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966140u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00207bf8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 19usize, 24u32, 2128892u32);
    emu.sri_no_count(11usize, 19usize, 16u32, 2128896u32);
    emu.sri_no_count(12usize, 19usize, 8u32, 2128900u32);
    emu.sri_no_count(13usize, 18usize, 24u32, 2128904u32);
    emu.sri_no_count(14usize, 18usize, 16u32, 2128908u32);
    emu.sb_no_count(19usize, 2usize, 796u32, 2128912u32);
    emu.sb_no_count(12usize, 2usize, 797u32, 2128916u32);
    emu.sb_no_count(11usize, 2usize, 798u32, 2128920u32);
    emu.sb_no_count(10usize, 2usize, 799u32, 2128924u32);
    emu.sri_no_count(10usize, 18usize, 8u32, 2128928u32);
    emu.sb_no_count(18usize, 2usize, 792u32, 2128932u32);
    emu.sb_no_count(10usize, 2usize, 793u32, 2128936u32);
    emu.sb_no_count(14usize, 2usize, 794u32, 2128940u32);
    emu.sb_no_count(13usize, 2usize, 795u32, 2128944u32);
    emu.adi_no_count(10usize, 2usize, 848u32, 2128948u32);
    emu.adi_no_count(11usize, 2usize, 736u32, 2128952u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2128956u32);
    emu.add_memory_rw_events(18usize);
    let return_addr = 2128960u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2128980u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207c54));
}
#[inline(always)]
pub fn block_0x00207c40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 2usize, 944u32, 2128964u32)?;
    emu.sw_no_count(19usize, 2usize, 948u32, 2128968u32)?;
    emu.adi_no_count(10usize, 2usize, 848u32, 2128972u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2128976u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2128980u32);
    emu.add_memory_rw_events(5usize);
    emu.pc = 2128980u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207c54));
}
#[inline(always)]
pub fn block_0x00207c54(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2128980u32, 45056u32, 2128984u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2128988u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965664u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00207c5c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(9usize, 27usize, 0u32, 2128992u32);
    emu.lw_no_count(10usize, 2usize, 848u32, 2128996u32)?;
    emu.sw_no_count(10usize, 2usize, 116u32, 2129000u32)?;
    emu.lw_no_count(10usize, 2usize, 852u32, 2129004u32)?;
    emu.sw_no_count(10usize, 2usize, 120u32, 2129008u32)?;
    emu.lw_no_count(18usize, 2usize, 856u32, 2129012u32)?;
    emu.lw_no_count(24usize, 2usize, 860u32, 2129016u32)?;
    emu.lw_no_count(23usize, 2usize, 864u32, 2129020u32)?;
    emu.lw_no_count(27usize, 2usize, 868u32, 2129024u32)?;
    emu.lw_no_count(25usize, 2usize, 872u32, 2129028u32)?;
    emu.lw_no_count(22usize, 2usize, 876u32, 2129032u32)?;
    emu.adi_no_count(19usize, 2usize, 776u32, 2129036u32);
    emu.adi_no_count(12usize, 0usize, 65u32, 2129040u32);
    emu.adi_no_count(10usize, 19usize, 0u32, 2129044u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2129048u32);
    emu.apc_no_count(1usize, 2129048u32, 8192u32, 2129052u32);
    emu.add_memory_rw_events(17usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129056u32;
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
#[inline(never)]
pub fn block_0x00207ca0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 29u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(1779032064u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2129060u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(3144134656u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2129064u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1013903360u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2129068u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2773479424u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2129072u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(1359892480u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2129076u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(2600824832u32);
    emu.write_reg_no_count(15usize, a);
    emu.pc = 2129080u32;
    emu.update_insn_clock();
    let a = 0u32.wrapping_add(528736256u32);
    emu.write_reg_no_count(16usize, a);
    emu.pc = 2129084u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 1639u32, 2129088u32);
    emu.adi_no_count(11usize, 11usize, 4294966917u32, 2129092u32);
    emu.adi_no_count(12usize, 12usize, 882u32, 2129096u32);
    emu.adi_no_count(13usize, 13usize, 1338u32, 2129100u32);
    emu.sw_no_count(10usize, 2usize, 736u32, 2129104u32)?;
    emu.sw_no_count(11usize, 2usize, 740u32, 2129108u32)?;
    emu.sw_no_count(12usize, 2usize, 744u32, 2129112u32)?;
    emu.sw_no_count(13usize, 2usize, 748u32, 2129116u32)?;
    let a = 0u32.wrapping_add(1541459968u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2129120u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 14usize, 639u32, 2129124u32);
    emu.adi_no_count(12usize, 15usize, 4294965388u32, 2129128u32);
    emu.adi_no_count(13usize, 16usize, 4294965675u32, 2129132u32);
    emu.adi_no_count(10usize, 10usize, 4294966553u32, 2129136u32);
    emu.sw_no_count(11usize, 2usize, 752u32, 2129140u32)?;
    emu.sw_no_count(12usize, 2usize, 756u32, 2129144u32)?;
    emu.sw_no_count(13usize, 2usize, 760u32, 2129148u32)?;
    emu.sw_no_count(10usize, 2usize, 764u32, 2129152u32)?;
    emu.sri_no_count(12usize, 8usize, 6u32, 2129156u32);
    emu.sw_no_count(0usize, 2usize, 768u32, 2129160u32)?;
    emu.sw_no_count(0usize, 2usize, 772u32, 2129164u32)?;
    emu.sw_no_count(18usize, 2usize, 124u32, 2129168u32)?;
    emu.add_memory_rw_events(28usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(12usize);
    if a == b {
        emu.pc = 2129220u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207d44));
    } else {
        emu.pc = 2129172u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207d14));
    }
}
#[inline]
pub fn block_0x00207d14(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 9u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.ani_no_count(20usize, 8usize, 4294967232u32, 2129176u32);
    emu.ani_no_count(8usize, 8usize, 63u32, 2129180u32);
    emu.adr_no_count(20usize, 9usize, 20usize, 2129184u32);
    emu.sw_no_count(12usize, 2usize, 768u32, 2129188u32)?;
    emu.sw_no_count(0usize, 2usize, 772u32, 2129192u32)?;
    emu.adi_no_count(10usize, 2usize, 736u32, 2129196u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2129200u32);
    emu.apc_no_count(1usize, 2129200u32, 45056u32, 2129204u32);
    emu.add_memory_rw_events(9usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129208u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965444u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207d38(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 19usize, 0u32, 2129212u32);
    emu.adi_no_count(11usize, 20usize, 0u32, 2129216u32);
    emu.add_memory_rw_events(3usize);
    let return_addr = 2129220u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2129228u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207d4c));
}
#[inline(always)]
pub fn block_0x00207d44(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 19usize, 0u32, 2129224u32);
    emu.adi_no_count(11usize, 9usize, 0u32, 2129228u32);
    emu.add_memory_rw_events(2usize);
    emu.pc = 2129228u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207d4c));
}
#[inline(always)]
pub fn block_0x00207d4c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(12usize, 8usize, 0u32, 2129232u32);
    emu.apc_no_count(1usize, 2129232u32, 8192u32, 2129236u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129240u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966036u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207d58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sb_no_count(8usize, 2usize, 840u32, 2129244u32);
    emu.adi_no_count(10usize, 2usize, 848u32, 2129248u32);
    emu.adi_no_count(11usize, 2usize, 736u32, 2129252u32);
    emu.adi_no_count(12usize, 0usize, 112u32, 2129256u32);
    emu.apc_no_count(1usize, 2129256u32, 8192u32, 2129260u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129264u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966012u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00207d70(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 35u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lbu_no_count(8usize, 2usize, 952u32, 2129268u32);
    emu.lw_no_count(10usize, 2usize, 880u32, 2129272u32)?;
    emu.lw_no_count(11usize, 2usize, 884u32, 2129276u32)?;
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2129280u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 0usize, 128u32, 2129284u32);
    emu.adi_no_count(12usize, 12usize, 4294967040u32, 2129288u32);
    emu.sli_no_count(14usize, 11usize, 9u32, 2129292u32);
    emu.sli_no_count(15usize, 10usize, 9u32, 2129296u32);
    emu.sli_no_count(16usize, 8usize, 3u32, 2129300u32);
    emu.sli_no_count(17usize, 10usize, 1u32, 2129304u32);
    emu.sli_no_count(11usize, 11usize, 1u32, 2129308u32);
    emu.orr_no_count(16usize, 15usize, 16usize, 2129312u32);
    emu.anr_no_count(17usize, 17usize, 12usize, 2129316u32);
    emu.sri_no_count(15usize, 15usize, 24u32, 2129320u32);
    emu.anr_no_count(11usize, 11usize, 12usize, 2129324u32);
    emu.orr_no_count(15usize, 17usize, 15usize, 2129328u32);
    emu.sri_no_count(17usize, 14usize, 24u32, 2129332u32);
    emu.orr_no_count(11usize, 11usize, 17usize, 2129336u32);
    emu.adi_no_count(17usize, 0usize, 63u32, 2129340u32);
    emu.sri_no_count(5usize, 10usize, 23u32, 2129344u32);
    emu.orr_no_count(10usize, 14usize, 5usize, 2129348u32);
    emu.anr_no_count(14usize, 16usize, 12usize, 2129352u32);
    emu.anr_no_count(12usize, 10usize, 12usize, 2129356u32);
    emu.sli_no_count(10usize, 8usize, 27u32, 2129360u32);
    emu.orr_no_count(15usize, 10usize, 15usize, 2129364u32);
    emu.adr_no_count(10usize, 21usize, 8usize, 2129368u32);
    emu.sli_no_count(5usize, 5usize, 24u32, 2129372u32);
    emu.sli_no_count(14usize, 14usize, 8u32, 2129376u32);
    emu.sli_no_count(12usize, 12usize, 8u32, 2129380u32);
    emu.orr_no_count(9usize, 5usize, 11usize, 2129384u32);
    emu.orr_no_count(18usize, 15usize, 14usize, 2129388u32);
    emu.orr_no_count(9usize, 9usize, 12usize, 2129392u32);
    emu.sb_no_count(13usize, 10usize, 0u32, 2129396u32);
    emu.sw_no_count(24usize, 2usize, 112u32, 2129400u32)?;
    emu.add_memory_rw_events(34usize);
    let b = emu.read_reg_b_tracked(17usize);
    let a = emu.read_reg_a_tracked(8usize);
    if a == b {
        emu.pc = 2129436u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207e1c));
    } else {
        emu.pc = 2129404u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207dfc));
    }
}
#[inline(always)]
pub fn block_0x00207dfc(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 10usize, 1u32, 2129408u32);
    emu.xri_no_count(12usize, 8usize, 63u32, 2129412u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2129416u32);
    emu.apc_no_count(1usize, 2129416u32, 8192u32, 2129420u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129424u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294965604u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207e10(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.xri_no_count(10usize, 8usize, 56u32, 2129428u32);
    emu.adi_no_count(11usize, 0usize, 7u32, 2129432u32);
    emu.add_memory_rw_events(2usize);
    let b = emu.read_reg_b_tracked(10usize);
    let a = emu.read_reg_a_tracked(11usize);
    if a < b {
        emu.pc = 2129552u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207e90));
    } else {
        emu.pc = 2129436u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207e1c));
    }
}
#[inline(always)]
pub fn block_0x00207e1c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 23usize, 0u32, 2129440u32);
    emu.adi_no_count(10usize, 2usize, 848u32, 2129444u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2129448u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2129452u32);
    emu.apc_no_count(1usize, 2129452u32, 40960u32, 2129456u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129460u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1992u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207e34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1004u32, 2129464u32);
    emu.adi_no_count(12usize, 0usize, 56u32, 2129468u32);
    emu.adi_no_count(11usize, 0usize, 0u32, 2129472u32);
    emu.apc_no_count(1usize, 2129472u32, 8192u32, 2129476u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129480u32;
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
#[inline]
pub fn block_0x00207e48(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 18u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 18usize, 24u32, 2129484u32);
    emu.sri_no_count(11usize, 18usize, 16u32, 2129488u32);
    emu.sri_no_count(12usize, 18usize, 8u32, 2129492u32);
    emu.sri_no_count(13usize, 9usize, 24u32, 2129496u32);
    emu.sri_no_count(14usize, 9usize, 16u32, 2129500u32);
    emu.sb_no_count(18usize, 2usize, 1064u32, 2129504u32);
    emu.sb_no_count(12usize, 2usize, 1065u32, 2129508u32);
    emu.sb_no_count(11usize, 2usize, 1066u32, 2129512u32);
    emu.sb_no_count(10usize, 2usize, 1067u32, 2129516u32);
    emu.sri_no_count(10usize, 9usize, 8u32, 2129520u32);
    emu.sb_no_count(9usize, 2usize, 1060u32, 2129524u32);
    emu.sb_no_count(10usize, 2usize, 1061u32, 2129528u32);
    emu.sb_no_count(14usize, 2usize, 1062u32, 2129532u32);
    emu.sb_no_count(13usize, 2usize, 1063u32, 2129536u32);
    emu.adi_no_count(10usize, 2usize, 848u32, 2129540u32);
    emu.adi_no_count(11usize, 2usize, 1004u32, 2129544u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2129548u32);
    emu.add_memory_rw_events(18usize);
    let return_addr = 2129552u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2129576u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207ea8));
}
#[inline(always)]
pub fn block_0x00207e90(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(8usize, 23usize, 0u32, 2129556u32);
    emu.sw_no_count(9usize, 2usize, 944u32, 2129560u32)?;
    emu.sw_no_count(18usize, 2usize, 948u32, 2129564u32)?;
    emu.adi_no_count(10usize, 2usize, 848u32, 2129568u32);
    emu.adi_no_count(12usize, 0usize, 1u32, 2129572u32);
    emu.adi_no_count(11usize, 21usize, 0u32, 2129576u32);
    emu.add_memory_rw_events(6usize);
    emu.pc = 2129576u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00207ea8));
}
#[inline(always)]
pub fn block_0x00207ea8(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.apc_no_count(1usize, 2129576u32, 40960u32, 2129580u32);
    emu.add_memory_rw_events(2usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129584u32;
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
#[inline(never)]
pub fn block_0x00207eb0(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 26u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(23usize, 2usize, 848u32, 2129588u32)?;
    emu.lw_no_count(20usize, 2usize, 852u32, 2129592u32)?;
    emu.lw_no_count(21usize, 2usize, 856u32, 2129596u32)?;
    emu.lw_no_count(24usize, 2usize, 860u32, 2129600u32)?;
    emu.lw_no_count(18usize, 2usize, 864u32, 2129604u32)?;
    emu.lw_no_count(19usize, 2usize, 868u32, 2129608u32)?;
    emu.lw_no_count(26usize, 2usize, 872u32, 2129612u32)?;
    emu.lw_no_count(9usize, 2usize, 876u32, 2129616u32)?;
    emu.lw_no_count(10usize, 2usize, 176u32, 2129620u32)?;
    emu.lw_no_count(11usize, 2usize, 180u32, 2129624u32)?;
    emu.lw_no_count(12usize, 2usize, 184u32, 2129628u32)?;
    emu.lw_no_count(13usize, 2usize, 188u32, 2129632u32)?;
    emu.sw_no_count(10usize, 2usize, 1020u32, 2129636u32)?;
    emu.sw_no_count(11usize, 2usize, 1024u32, 2129640u32)?;
    emu.sw_no_count(12usize, 2usize, 1028u32, 2129644u32)?;
    emu.sw_no_count(13usize, 2usize, 1032u32, 2129648u32)?;
    emu.lw_no_count(10usize, 2usize, 160u32, 2129652u32)?;
    emu.lw_no_count(11usize, 2usize, 164u32, 2129656u32)?;
    emu.lw_no_count(12usize, 2usize, 168u32, 2129660u32)?;
    emu.lw_no_count(13usize, 2usize, 172u32, 2129664u32)?;
    emu.sw_no_count(10usize, 2usize, 1004u32, 2129668u32)?;
    emu.sw_no_count(11usize, 2usize, 1008u32, 2129672u32)?;
    emu.sw_no_count(12usize, 2usize, 1012u32, 2129676u32)?;
    emu.sw_no_count(13usize, 2usize, 1016u32, 2129680u32)?;
    emu.apc_no_count(1usize, 2129680u32, 4096u32, 2129684u32);
    emu.add_memory_rw_events(26usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129688u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967284u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207f18(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2129692u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965652u32, 2129696u32);
    emu.adi_no_count(11usize, 0usize, 4u32, 2129700u32);
    emu.adi_no_count(12usize, 0usize, 12u32, 2129704u32);
    emu.apc_no_count(1usize, 2129704u32, 8192u32, 2129708u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129712u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966628u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207f30(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
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
        emu.pc = 2131496u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00208628));
    } else {
        emu.pc = 2129716u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207f34));
    }
}
#[inline(always)]
pub fn block_0x00207f34(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 3u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 108u32, 2129720u32)?;
    emu.apc_no_count(1usize, 2129720u32, 4096u32, 2129724u32);
    emu.add_memory_rw_events(3usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129728u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294967244u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207f40(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2129732u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965652u32, 2129736u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2129740u32);
    emu.adi_no_count(12usize, 0usize, 11u32, 2129744u32);
    emu.apc_no_count(1usize, 2129744u32, 8192u32, 2129748u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2129752u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966588u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00207f58(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(10usize, 2usize, 104u32, 2129756u32)?;
    emu.add_memory_rw_events(1usize);
    let b = emu.read_reg_b_tracked(0usize);
    let a = emu.read_reg_a_tracked(10usize);
    if a == b {
        emu.pc = 2131636u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x002086b4));
    } else {
        emu.pc = 2129760u32;
        emu.update_insn_clock();
        emu.check_chunk_boundary_fast();
        if emu.should_yield() {
            return Ok(crate::NextStep::Dynamic(emu.pc));
        }
        return Ok(crate::NextStep::Direct(block_0x00207f60));
    }
}
#[inline(never)]
pub fn block_0x00207f60(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 300u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sri_no_count(10usize, 9usize, 8u32, 2129764u32);
    let a = 0u32.wrapping_add(65536u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2129768u32;
    emu.update_insn_clock();
    emu.sri_no_count(12usize, 9usize, 24u32, 2129772u32);
    emu.sri_no_count(13usize, 26usize, 8u32, 2129776u32);
    emu.adi_no_count(11usize, 11usize, 4294967040u32, 2129780u32);
    emu.anr_no_count(10usize, 10usize, 11usize, 2129784u32);
    emu.orr_no_count(10usize, 10usize, 12usize, 2129788u32);
    emu.sw_no_count(10usize, 2usize, 100u32, 2129792u32)?;
    emu.sri_no_count(10usize, 26usize, 24u32, 2129796u32);
    emu.anr_no_count(13usize, 13usize, 11usize, 2129800u32);
    emu.orr_no_count(10usize, 13usize, 10usize, 2129804u32);
    emu.sw_no_count(10usize, 2usize, 96u32, 2129808u32)?;
    emu.sri_no_count(10usize, 19usize, 8u32, 2129812u32);
    emu.anr_no_count(10usize, 10usize, 11usize, 2129816u32);
    emu.sri_no_count(12usize, 19usize, 24u32, 2129820u32);
    emu.orr_no_count(10usize, 10usize, 12usize, 2129824u32);
    emu.sw_no_count(10usize, 2usize, 92u32, 2129828u32)?;
    emu.sri_no_count(10usize, 18usize, 8u32, 2129832u32);
    emu.anr_no_count(10usize, 10usize, 11usize, 2129836u32);
    emu.sri_no_count(12usize, 18usize, 24u32, 2129840u32);
    emu.orr_no_count(10usize, 10usize, 12usize, 2129844u32);
    emu.sw_no_count(10usize, 2usize, 88u32, 2129848u32)?;
    emu.sri_no_count(10usize, 24usize, 8u32, 2129852u32);
    emu.anr_no_count(10usize, 10usize, 11usize, 2129856u32);
    emu.sri_no_count(12usize, 24usize, 24u32, 2129860u32);
    emu.orr_no_count(10usize, 10usize, 12usize, 2129864u32);
    emu.sw_no_count(10usize, 2usize, 84u32, 2129868u32)?;
    emu.sri_no_count(10usize, 21usize, 8u32, 2129872u32);
    emu.anr_no_count(10usize, 10usize, 11usize, 2129876u32);
    emu.sri_no_count(12usize, 21usize, 24u32, 2129880u32);
    emu.orr_no_count(10usize, 10usize, 12usize, 2129884u32);
    emu.sw_no_count(10usize, 2usize, 80u32, 2129888u32)?;
    emu.sri_no_count(10usize, 20usize, 8u32, 2129892u32);
    emu.anr_no_count(10usize, 10usize, 11usize, 2129896u32);
    emu.sri_no_count(12usize, 20usize, 24u32, 2129900u32);
    emu.orr_no_count(10usize, 10usize, 12usize, 2129904u32);
    emu.sw_no_count(10usize, 2usize, 76u32, 2129908u32)?;
    emu.sri_no_count(10usize, 23usize, 8u32, 2129912u32);
    emu.anr_no_count(10usize, 10usize, 11usize, 2129916u32);
    emu.sri_no_count(12usize, 23usize, 24u32, 2129920u32);
    emu.orr_no_count(10usize, 10usize, 12usize, 2129924u32);
    emu.sw_no_count(10usize, 2usize, 72u32, 2129928u32)?;
    emu.sri_no_count(10usize, 22usize, 8u32, 2129932u32);
    emu.anr_no_count(10usize, 10usize, 11usize, 2129936u32);
    emu.sri_no_count(12usize, 22usize, 24u32, 2129940u32);
    emu.orr_no_count(10usize, 10usize, 12usize, 2129944u32);
    emu.sw_no_count(10usize, 2usize, 68u32, 2129948u32)?;
    emu.sri_no_count(10usize, 25usize, 8u32, 2129952u32);
    emu.anr_no_count(10usize, 10usize, 11usize, 2129956u32);
    emu.sri_no_count(12usize, 25usize, 24u32, 2129960u32);
    emu.orr_no_count(10usize, 10usize, 12usize, 2129964u32);
    emu.sw_no_count(10usize, 2usize, 64u32, 2129968u32)?;
    emu.sri_no_count(10usize, 27usize, 8u32, 2129972u32);
    emu.anr_no_count(10usize, 10usize, 11usize, 2129976u32);
    emu.sri_no_count(12usize, 27usize, 24u32, 2129980u32);
    emu.orr_no_count(10usize, 10usize, 12usize, 2129984u32);
    emu.sw_no_count(10usize, 2usize, 60u32, 2129988u32)?;
    emu.sw_no_count(27usize, 2usize, 20u32, 2129992u32)?;
    emu.adi_no_count(27usize, 8usize, 0u32, 2129996u32);
    emu.sri_no_count(10usize, 8usize, 8u32, 2130000u32);
    emu.anr_no_count(10usize, 10usize, 11usize, 2130004u32);
    emu.sri_no_count(12usize, 8usize, 24u32, 2130008u32);
    emu.orr_no_count(10usize, 10usize, 12usize, 2130012u32);
    emu.sw_no_count(10usize, 2usize, 56u32, 2130016u32)?;
    emu.lw_no_count(28usize, 2usize, 112u32, 2130020u32)?;
    emu.sri_no_count(10usize, 28usize, 8u32, 2130024u32);
    emu.anr_no_count(10usize, 10usize, 11usize, 2130028u32);
    emu.sri_no_count(12usize, 28usize, 24u32, 2130032u32);
    emu.orr_no_count(10usize, 10usize, 12usize, 2130036u32);
    emu.sw_no_count(10usize, 2usize, 52u32, 2130040u32)?;
    emu.lw_no_count(29usize, 2usize, 124u32, 2130044u32)?;
    emu.sri_no_count(10usize, 29usize, 8u32, 2130048u32);
    emu.anr_no_count(10usize, 10usize, 11usize, 2130052u32);
    emu.sri_no_count(12usize, 29usize, 24u32, 2130056u32);
    emu.orr_no_count(10usize, 10usize, 12usize, 2130060u32);
    emu.sw_no_count(10usize, 2usize, 48u32, 2130064u32)?;
    emu.lw_no_count(30usize, 2usize, 120u32, 2130068u32)?;
    emu.sri_no_count(10usize, 30usize, 8u32, 2130072u32);
    emu.anr_no_count(10usize, 10usize, 11usize, 2130076u32);
    emu.sri_no_count(12usize, 30usize, 24u32, 2130080u32);
    emu.orr_no_count(10usize, 10usize, 12usize, 2130084u32);
    emu.sw_no_count(10usize, 2usize, 44u32, 2130088u32)?;
    emu.lw_no_count(31usize, 2usize, 116u32, 2130092u32)?;
    emu.sri_no_count(10usize, 31usize, 8u32, 2130096u32);
    emu.anr_no_count(10usize, 10usize, 11usize, 2130100u32);
    emu.sri_no_count(12usize, 31usize, 24u32, 2130104u32);
    emu.orr_no_count(10usize, 10usize, 12usize, 2130108u32);
    emu.sw_no_count(10usize, 2usize, 40u32, 2130112u32)?;
    emu.lw_no_count(1usize, 2usize, 128u32, 2130116u32)?;
    emu.sri_no_count(10usize, 1usize, 8u32, 2130120u32);
    emu.anr_no_count(10usize, 10usize, 11usize, 2130124u32);
    emu.sri_no_count(12usize, 1usize, 24u32, 2130128u32);
    emu.orr_no_count(10usize, 10usize, 12usize, 2130132u32);
    emu.sw_no_count(10usize, 2usize, 36u32, 2130136u32)?;
    emu.lw_no_count(7usize, 2usize, 132u32, 2130140u32)?;
    emu.sri_no_count(10usize, 7usize, 8u32, 2130144u32);
    emu.anr_no_count(10usize, 10usize, 11usize, 2130148u32);
    emu.sri_no_count(12usize, 7usize, 24u32, 2130152u32);
    emu.orr_no_count(10usize, 10usize, 12usize, 2130156u32);
    emu.sw_no_count(10usize, 2usize, 32u32, 2130160u32)?;
    emu.lw_no_count(6usize, 2usize, 136u32, 2130164u32)?;
    emu.sri_no_count(10usize, 6usize, 8u32, 2130168u32);
    emu.anr_no_count(10usize, 10usize, 11usize, 2130172u32);
    emu.sri_no_count(12usize, 6usize, 24u32, 2130176u32);
    emu.orr_no_count(10usize, 10usize, 12usize, 2130180u32);
    emu.sw_no_count(10usize, 2usize, 28u32, 2130184u32)?;
    emu.lw_no_count(5usize, 2usize, 140u32, 2130188u32)?;
    emu.sri_no_count(10usize, 5usize, 8u32, 2130192u32);
    emu.anr_no_count(10usize, 10usize, 11usize, 2130196u32);
    emu.sri_no_count(12usize, 5usize, 24u32, 2130200u32);
    emu.orr_no_count(10usize, 10usize, 12usize, 2130204u32);
    emu.sw_no_count(10usize, 2usize, 24u32, 2130208u32)?;
    emu.lw_no_count(17usize, 2usize, 144u32, 2130212u32)?;
    emu.sri_no_count(10usize, 17usize, 8u32, 2130216u32);
    emu.anr_no_count(10usize, 10usize, 11usize, 2130220u32);
    emu.sri_no_count(15usize, 17usize, 24u32, 2130224u32);
    emu.orr_no_count(10usize, 10usize, 15usize, 2130228u32);
    emu.sw_no_count(10usize, 2usize, 16u32, 2130232u32)?;
    emu.lw_no_count(16usize, 2usize, 148u32, 2130236u32)?;
    emu.sri_no_count(10usize, 16usize, 8u32, 2130240u32);
    emu.anr_no_count(10usize, 10usize, 11usize, 2130244u32);
    emu.sri_no_count(14usize, 16usize, 24u32, 2130248u32);
    emu.orr_no_count(10usize, 10usize, 14usize, 2130252u32);
    emu.sw_no_count(10usize, 2usize, 12u32, 2130256u32)?;
    emu.lw_no_count(15usize, 2usize, 152u32, 2130260u32)?;
    emu.sri_no_count(10usize, 15usize, 8u32, 2130264u32);
    emu.anr_no_count(10usize, 10usize, 11usize, 2130268u32);
    emu.sri_no_count(13usize, 15usize, 24u32, 2130272u32);
    emu.orr_no_count(10usize, 10usize, 13usize, 2130276u32);
    emu.sw_no_count(10usize, 2usize, 8u32, 2130280u32)?;
    emu.lw_no_count(13usize, 2usize, 156u32, 2130284u32)?;
    emu.sri_no_count(10usize, 13usize, 8u32, 2130288u32);
    emu.anr_no_count(10usize, 10usize, 11usize, 2130292u32);
    emu.sri_no_count(12usize, 13usize, 24u32, 2130296u32);
    emu.orr_no_count(14usize, 10usize, 12usize, 2130300u32);
    emu.sli_no_count(10usize, 9usize, 24u32, 2130304u32);
    emu.anr_no_count(8usize, 9usize, 11usize, 2130308u32);
    emu.sli_no_count(8usize, 8usize, 8u32, 2130312u32);
    emu.orr_no_count(12usize, 10usize, 8usize, 2130316u32);
    emu.sli_no_count(10usize, 26usize, 24u32, 2130320u32);
    emu.anr_no_count(8usize, 26usize, 11usize, 2130324u32);
    emu.sli_no_count(8usize, 8usize, 8u32, 2130328u32);
    emu.orr_no_count(26usize, 10usize, 8usize, 2130332u32);
    emu.sli_no_count(10usize, 19usize, 24u32, 2130336u32);
    emu.anr_no_count(8usize, 19usize, 11usize, 2130340u32);
    emu.sli_no_count(8usize, 8usize, 8u32, 2130344u32);
    emu.orr_no_count(8usize, 10usize, 8usize, 2130348u32);
    emu.sli_no_count(10usize, 18usize, 24u32, 2130352u32);
    emu.anr_no_count(9usize, 18usize, 11usize, 2130356u32);
    emu.sli_no_count(9usize, 9usize, 8u32, 2130360u32);
    emu.orr_no_count(18usize, 10usize, 9usize, 2130364u32);
    emu.sli_no_count(10usize, 24usize, 24u32, 2130368u32);
    emu.anr_no_count(9usize, 24usize, 11usize, 2130372u32);
    emu.sli_no_count(9usize, 9usize, 8u32, 2130376u32);
    emu.orr_no_count(24usize, 10usize, 9usize, 2130380u32);
    emu.sli_no_count(10usize, 21usize, 24u32, 2130384u32);
    emu.anr_no_count(9usize, 21usize, 11usize, 2130388u32);
    emu.sli_no_count(9usize, 9usize, 8u32, 2130392u32);
    emu.orr_no_count(21usize, 10usize, 9usize, 2130396u32);
    emu.sli_no_count(10usize, 20usize, 24u32, 2130400u32);
    emu.anr_no_count(9usize, 20usize, 11usize, 2130404u32);
    emu.sli_no_count(9usize, 9usize, 8u32, 2130408u32);
    emu.orr_no_count(20usize, 10usize, 9usize, 2130412u32);
    emu.sli_no_count(10usize, 23usize, 24u32, 2130416u32);
    emu.anr_no_count(9usize, 23usize, 11usize, 2130420u32);
    emu.sli_no_count(9usize, 9usize, 8u32, 2130424u32);
    emu.orr_no_count(19usize, 10usize, 9usize, 2130428u32);
    emu.sli_no_count(10usize, 22usize, 24u32, 2130432u32);
    emu.anr_no_count(9usize, 22usize, 11usize, 2130436u32);
    emu.sli_no_count(9usize, 9usize, 8u32, 2130440u32);
    emu.orr_no_count(10usize, 10usize, 9usize, 2130444u32);
    emu.sli_no_count(9usize, 25usize, 24u32, 2130448u32);
    emu.anr_no_count(23usize, 25usize, 11usize, 2130452u32);
    emu.sli_no_count(23usize, 23usize, 8u32, 2130456u32);
    emu.orr_no_count(9usize, 9usize, 23usize, 2130460u32);
    emu.lw_no_count(22usize, 2usize, 20u32, 2130464u32)?;
    emu.sli_no_count(23usize, 22usize, 24u32, 2130468u32);
    emu.anr_no_count(25usize, 22usize, 11usize, 2130472u32);
    emu.sli_no_count(25usize, 25usize, 8u32, 2130476u32);
    emu.orr_no_count(23usize, 23usize, 25usize, 2130480u32);
    emu.sli_no_count(25usize, 27usize, 24u32, 2130484u32);
    emu.anr_no_count(27usize, 27usize, 11usize, 2130488u32);
    emu.sli_no_count(27usize, 27usize, 8u32, 2130492u32);
    emu.orr_no_count(25usize, 25usize, 27usize, 2130496u32);
    emu.sli_no_count(27usize, 28usize, 24u32, 2130500u32);
    emu.anr_no_count(28usize, 28usize, 11usize, 2130504u32);
    emu.sli_no_count(28usize, 28usize, 8u32, 2130508u32);
    emu.orr_no_count(27usize, 27usize, 28usize, 2130512u32);
    emu.sli_no_count(28usize, 29usize, 24u32, 2130516u32);
    emu.anr_no_count(29usize, 29usize, 11usize, 2130520u32);
    emu.sli_no_count(29usize, 29usize, 8u32, 2130524u32);
    emu.orr_no_count(28usize, 28usize, 29usize, 2130528u32);
    emu.sli_no_count(29usize, 30usize, 24u32, 2130532u32);
    emu.anr_no_count(30usize, 30usize, 11usize, 2130536u32);
    emu.sli_no_count(30usize, 30usize, 8u32, 2130540u32);
    emu.orr_no_count(29usize, 29usize, 30usize, 2130544u32);
    emu.sli_no_count(30usize, 31usize, 24u32, 2130548u32);
    emu.anr_no_count(31usize, 31usize, 11usize, 2130552u32);
    emu.sli_no_count(31usize, 31usize, 8u32, 2130556u32);
    emu.orr_no_count(30usize, 30usize, 31usize, 2130560u32);
    emu.sli_no_count(31usize, 1usize, 24u32, 2130564u32);
    emu.anr_no_count(1usize, 1usize, 11usize, 2130568u32);
    emu.sli_no_count(1usize, 1usize, 8u32, 2130572u32);
    emu.orr_no_count(31usize, 31usize, 1usize, 2130576u32);
    emu.sli_no_count(1usize, 7usize, 24u32, 2130580u32);
    emu.anr_no_count(7usize, 7usize, 11usize, 2130584u32);
    emu.sli_no_count(7usize, 7usize, 8u32, 2130588u32);
    emu.orr_no_count(7usize, 1usize, 7usize, 2130592u32);
    emu.sli_no_count(1usize, 6usize, 24u32, 2130596u32);
    emu.anr_no_count(6usize, 6usize, 11usize, 2130600u32);
    emu.sli_no_count(6usize, 6usize, 8u32, 2130604u32);
    emu.orr_no_count(6usize, 1usize, 6usize, 2130608u32);
    emu.sli_no_count(1usize, 5usize, 24u32, 2130612u32);
    emu.anr_no_count(5usize, 5usize, 11usize, 2130616u32);
    emu.sli_no_count(5usize, 5usize, 8u32, 2130620u32);
    emu.orr_no_count(5usize, 1usize, 5usize, 2130624u32);
    emu.sli_no_count(1usize, 17usize, 24u32, 2130628u32);
    emu.anr_no_count(17usize, 17usize, 11usize, 2130632u32);
    emu.sli_no_count(17usize, 17usize, 8u32, 2130636u32);
    emu.orr_no_count(17usize, 1usize, 17usize, 2130640u32);
    emu.sli_no_count(1usize, 16usize, 24u32, 2130644u32);
    emu.anr_no_count(16usize, 16usize, 11usize, 2130648u32);
    emu.sli_no_count(16usize, 16usize, 8u32, 2130652u32);
    emu.orr_no_count(16usize, 1usize, 16usize, 2130656u32);
    emu.sli_no_count(1usize, 15usize, 24u32, 2130660u32);
    emu.anr_no_count(15usize, 15usize, 11usize, 2130664u32);
    emu.sli_no_count(15usize, 15usize, 8u32, 2130668u32);
    emu.orr_no_count(15usize, 1usize, 15usize, 2130672u32);
    emu.anr_no_count(11usize, 13usize, 11usize, 2130676u32);
    emu.sli_no_count(1usize, 13usize, 24u32, 2130680u32);
    emu.sli_no_count(11usize, 11usize, 8u32, 2130684u32);
    emu.orr_no_count(11usize, 1usize, 11usize, 2130688u32);
    emu.lw_no_count(13usize, 2usize, 100u32, 2130692u32)?;
    emu.orr_no_count(12usize, 12usize, 13usize, 2130696u32);
    emu.sw_no_count(12usize, 2usize, 140u32, 2130700u32)?;
    emu.lw_no_count(12usize, 2usize, 96u32, 2130704u32)?;
    emu.orr_no_count(12usize, 26usize, 12usize, 2130708u32);
    emu.sw_no_count(12usize, 2usize, 132u32, 2130712u32)?;
    emu.lw_no_count(12usize, 2usize, 92u32, 2130716u32)?;
    emu.orr_no_count(12usize, 8usize, 12usize, 2130720u32);
    emu.sw_no_count(12usize, 2usize, 124u32, 2130724u32)?;
    emu.lw_no_count(12usize, 2usize, 88u32, 2130728u32)?;
    emu.orr_no_count(12usize, 18usize, 12usize, 2130732u32);
    emu.sw_no_count(12usize, 2usize, 116u32, 2130736u32)?;
    emu.lw_no_count(12usize, 2usize, 84u32, 2130740u32)?;
    emu.orr_no_count(12usize, 24usize, 12usize, 2130744u32);
    emu.sw_no_count(12usize, 2usize, 100u32, 2130748u32)?;
    emu.lw_no_count(12usize, 2usize, 80u32, 2130752u32)?;
    emu.orr_no_count(21usize, 21usize, 12usize, 2130756u32);
    emu.lw_no_count(12usize, 2usize, 76u32, 2130760u32)?;
    emu.orr_no_count(18usize, 20usize, 12usize, 2130764u32);
    emu.lw_no_count(12usize, 2usize, 72u32, 2130768u32)?;
    emu.orr_no_count(8usize, 19usize, 12usize, 2130772u32);
    emu.lw_no_count(12usize, 2usize, 68u32, 2130776u32)?;
    emu.orr_no_count(10usize, 10usize, 12usize, 2130780u32);
    emu.sw_no_count(10usize, 2usize, 156u32, 2130784u32)?;
    emu.lw_no_count(10usize, 2usize, 64u32, 2130788u32)?;
    emu.orr_no_count(10usize, 9usize, 10usize, 2130792u32);
    emu.sw_no_count(10usize, 2usize, 152u32, 2130796u32)?;
    emu.lw_no_count(10usize, 2usize, 60u32, 2130800u32)?;
    emu.orr_no_count(10usize, 23usize, 10usize, 2130804u32);
    emu.sw_no_count(10usize, 2usize, 148u32, 2130808u32)?;
    emu.lw_no_count(10usize, 2usize, 56u32, 2130812u32)?;
    emu.orr_no_count(10usize, 25usize, 10usize, 2130816u32);
    emu.sw_no_count(10usize, 2usize, 144u32, 2130820u32)?;
    emu.lw_no_count(10usize, 2usize, 52u32, 2130824u32)?;
    emu.orr_no_count(10usize, 27usize, 10usize, 2130828u32);
    emu.sw_no_count(10usize, 2usize, 136u32, 2130832u32)?;
    emu.lw_no_count(10usize, 2usize, 48u32, 2130836u32)?;
    emu.orr_no_count(10usize, 28usize, 10usize, 2130840u32);
    emu.sw_no_count(10usize, 2usize, 128u32, 2130844u32)?;
    emu.lw_no_count(10usize, 2usize, 44u32, 2130848u32)?;
    emu.orr_no_count(10usize, 29usize, 10usize, 2130852u32);
    emu.sw_no_count(10usize, 2usize, 120u32, 2130856u32)?;
    emu.lw_no_count(10usize, 2usize, 40u32, 2130860u32)?;
    emu.orr_no_count(10usize, 30usize, 10usize, 2130864u32);
    emu.sw_no_count(10usize, 2usize, 112u32, 2130868u32)?;
    emu.lw_no_count(10usize, 2usize, 36u32, 2130872u32)?;
    emu.orr_no_count(10usize, 31usize, 10usize, 2130876u32);
    emu.sw_no_count(10usize, 2usize, 96u32, 2130880u32)?;
    emu.lw_no_count(10usize, 2usize, 32u32, 2130884u32)?;
    emu.orr_no_count(19usize, 7usize, 10usize, 2130888u32);
    emu.lw_no_count(10usize, 2usize, 28u32, 2130892u32)?;
    emu.orr_no_count(25usize, 6usize, 10usize, 2130896u32);
    emu.lw_no_count(10usize, 2usize, 24u32, 2130900u32)?;
    emu.orr_no_count(27usize, 5usize, 10usize, 2130904u32);
    emu.lw_no_count(10usize, 2usize, 16u32, 2130908u32)?;
    emu.orr_no_count(9usize, 17usize, 10usize, 2130912u32);
    emu.lw_no_count(10usize, 2usize, 12u32, 2130916u32)?;
    emu.orr_no_count(26usize, 16usize, 10usize, 2130920u32);
    emu.lw_no_count(10usize, 2usize, 8u32, 2130924u32)?;
    emu.orr_no_count(22usize, 15usize, 10usize, 2130928u32);
    emu.orr_no_count(23usize, 11usize, 14usize, 2130932u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2130936u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965464u32, 2130940u32);
    emu.adi_no_count(12usize, 0usize, 11u32, 2130944u32);
    emu.lw_no_count(24usize, 2usize, 104u32, 2130948u32)?;
    emu.adi_no_count(10usize, 24usize, 0u32, 2130952u32);
    emu.apc_no_count(1usize, 2130952u32, 4096u32, 2130956u32);
    emu.add_memory_rw_events(300usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2130960u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1116u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00208410(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(20usize, 2usize, 108u32, 2130964u32)?;
    emu.adi_no_count(10usize, 0usize, 11u32, 2130968u32);
    emu.sw_no_count(10usize, 20usize, 0u32, 2130972u32)?;
    emu.sw_no_count(24usize, 20usize, 4u32, 2130976u32)?;
    emu.sw_no_count(10usize, 20usize, 8u32, 2130980u32)?;
    emu.adi_no_count(10usize, 2usize, 860u32, 2130984u32);
    emu.adi_no_count(11usize, 2usize, 1004u32, 2130988u32);
    emu.adi_no_count(12usize, 0usize, 32u32, 2130992u32);
    emu.apc_no_count(1usize, 2130992u32, 4096u32, 2130996u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131000u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1076u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(never)]
pub fn block_0x00208438(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 45u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 1u32, 2131004u32);
    emu.sw_no_count(10usize, 2usize, 848u32, 2131008u32)?;
    emu.sw_no_count(20usize, 2usize, 852u32, 2131012u32)?;
    emu.sw_no_count(10usize, 2usize, 856u32, 2131016u32)?;
    emu.sw_no_count(8usize, 2usize, 892u32, 2131020u32)?;
    emu.sw_no_count(18usize, 2usize, 896u32, 2131024u32)?;
    emu.sw_no_count(21usize, 2usize, 900u32, 2131028u32)?;
    emu.lw_no_count(10usize, 2usize, 100u32, 2131032u32)?;
    emu.sw_no_count(10usize, 2usize, 904u32, 2131036u32)?;
    emu.lw_no_count(10usize, 2usize, 116u32, 2131040u32)?;
    emu.sw_no_count(10usize, 2usize, 908u32, 2131044u32)?;
    emu.lw_no_count(10usize, 2usize, 124u32, 2131048u32)?;
    emu.sw_no_count(10usize, 2usize, 912u32, 2131052u32)?;
    emu.lw_no_count(10usize, 2usize, 132u32, 2131056u32)?;
    emu.sw_no_count(10usize, 2usize, 916u32, 2131060u32)?;
    emu.lw_no_count(10usize, 2usize, 140u32, 2131064u32)?;
    emu.sw_no_count(10usize, 2usize, 920u32, 2131068u32)?;
    emu.sw_no_count(23usize, 2usize, 924u32, 2131072u32)?;
    emu.sw_no_count(22usize, 2usize, 928u32, 2131076u32)?;
    emu.sw_no_count(26usize, 2usize, 932u32, 2131080u32)?;
    emu.sw_no_count(9usize, 2usize, 936u32, 2131084u32)?;
    emu.sw_no_count(27usize, 2usize, 940u32, 2131088u32)?;
    emu.sw_no_count(25usize, 2usize, 944u32, 2131092u32)?;
    emu.sw_no_count(19usize, 2usize, 948u32, 2131096u32)?;
    emu.lw_no_count(10usize, 2usize, 96u32, 2131100u32)?;
    emu.sw_no_count(10usize, 2usize, 952u32, 2131104u32)?;
    emu.lw_no_count(10usize, 2usize, 112u32, 2131108u32)?;
    emu.sw_no_count(10usize, 2usize, 956u32, 2131112u32)?;
    emu.lw_no_count(10usize, 2usize, 120u32, 2131116u32)?;
    emu.sw_no_count(10usize, 2usize, 960u32, 2131120u32)?;
    emu.lw_no_count(10usize, 2usize, 128u32, 2131124u32)?;
    emu.sw_no_count(10usize, 2usize, 964u32, 2131128u32)?;
    emu.lw_no_count(10usize, 2usize, 136u32, 2131132u32)?;
    emu.sw_no_count(10usize, 2usize, 968u32, 2131136u32)?;
    emu.lw_no_count(10usize, 2usize, 144u32, 2131140u32)?;
    emu.sw_no_count(10usize, 2usize, 972u32, 2131144u32)?;
    emu.lw_no_count(10usize, 2usize, 148u32, 2131148u32)?;
    emu.sw_no_count(10usize, 2usize, 976u32, 2131152u32)?;
    emu.lw_no_count(10usize, 2usize, 152u32, 2131156u32)?;
    emu.sw_no_count(10usize, 2usize, 980u32, 2131160u32)?;
    emu.lw_no_count(10usize, 2usize, 156u32, 2131164u32)?;
    emu.sw_no_count(10usize, 2usize, 984u32, 2131168u32)?;
    emu.adi_no_count(10usize, 2usize, 848u32, 2131172u32);
    emu.apc_no_count(1usize, 2131172u32, 4294955008u32, 2131176u32);
    emu.add_memory_rw_events(45usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131180u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(4294966944u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002084ec(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 15u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(1usize, 2usize, 1116u32, 2131184u32)?;
    emu.lw_no_count(8usize, 2usize, 1112u32, 2131188u32)?;
    emu.lw_no_count(9usize, 2usize, 1108u32, 2131192u32)?;
    emu.lw_no_count(18usize, 2usize, 1104u32, 2131196u32)?;
    emu.lw_no_count(19usize, 2usize, 1100u32, 2131200u32)?;
    emu.lw_no_count(20usize, 2usize, 1096u32, 2131204u32)?;
    emu.lw_no_count(21usize, 2usize, 1092u32, 2131208u32)?;
    emu.lw_no_count(22usize, 2usize, 1088u32, 2131212u32)?;
    emu.lw_no_count(23usize, 2usize, 1084u32, 2131216u32)?;
    emu.lw_no_count(24usize, 2usize, 1080u32, 2131220u32)?;
    emu.lw_no_count(25usize, 2usize, 1076u32, 2131224u32)?;
    emu.lw_no_count(26usize, 2usize, 1072u32, 2131228u32)?;
    emu.lw_no_count(27usize, 2usize, 1068u32, 2131232u32)?;
    emu.adi_no_count(2usize, 2usize, 1120u32, 2131236u32);
    emu.add_memory_rw_events(15usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131240u32;
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
pub fn block_0x00208528(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 19u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 2usize, 1004u32, 2131244u32);
    let a = 0u32.wrapping_add(2207744u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2131248u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 1064u32, 2131252u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2131256u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965520u32, 2131260u32);
    emu.adi_no_count(13usize, 0usize, 1u32, 2131264u32);
    emu.sw_no_count(0usize, 2usize, 864u32, 2131268u32)?;
    emu.sw_no_count(10usize, 2usize, 736u32, 2131272u32)?;
    emu.sw_no_count(11usize, 2usize, 740u32, 2131276u32)?;
    emu.adi_no_count(10usize, 2usize, 736u32, 2131280u32);
    emu.sw_no_count(12usize, 2usize, 848u32, 2131284u32)?;
    emu.sw_no_count(13usize, 2usize, 852u32, 2131288u32)?;
    emu.sw_no_count(10usize, 2usize, 856u32, 2131292u32)?;
    emu.sw_no_count(13usize, 2usize, 860u32, 2131296u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(11usize, a);
    emu.pc = 2131300u32;
    emu.update_insn_clock();
    emu.adi_no_count(11usize, 11usize, 4294965528u32, 2131304u32);
    emu.adi_no_count(10usize, 2usize, 848u32, 2131308u32);
    emu.apc_no_count(1usize, 2131308u32, 77824u32, 2131312u32);
    emu.add_memory_rw_events(19usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131316u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1520u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00208574(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 852u32, 2131320u32)?;
    emu.lw_no_count(11usize, 2usize, 856u32, 2131324u32)?;
    emu.sw_no_count(10usize, 2usize, 736u32, 2131328u32)?;
    emu.sw_no_count(11usize, 2usize, 740u32, 2131332u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2131336u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965592u32, 2131340u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2131344u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294965544u32, 2131348u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2131352u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965608u32, 2131356u32);
    emu.adi_no_count(11usize, 0usize, 13u32, 2131360u32);
    emu.adi_no_count(12usize, 2usize, 736u32, 2131364u32);
    emu.apc_no_count(1usize, 2131364u32, 81920u32, 2131368u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131372u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1900u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x002085ac(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 14u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.lw_no_count(10usize, 2usize, 852u32, 2131376u32)?;
    emu.lw_no_count(11usize, 2usize, 856u32, 2131380u32)?;
    emu.sw_no_count(10usize, 2usize, 736u32, 2131384u32)?;
    emu.sw_no_count(11usize, 2usize, 740u32, 2131388u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2131392u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965560u32, 2131396u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2131400u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294965544u32, 2131404u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2131408u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965576u32, 2131412u32);
    emu.adi_no_count(11usize, 0usize, 15u32, 2131416u32);
    emu.adi_no_count(12usize, 2usize, 736u32, 2131420u32);
    emu.apc_no_count(1usize, 2131420u32, 81920u32, 2131424u32);
    emu.add_memory_rw_events(14usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131428u32;
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
#[inline]
pub fn block_0x002085e4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 17u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2131432u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965440u32, 2131436u32);
    emu.adi_no_count(11usize, 0usize, 1u32, 2131440u32);
    emu.adi_no_count(12usize, 0usize, 4u32, 2131444u32);
    emu.sw_no_count(10usize, 2usize, 848u32, 2131448u32)?;
    emu.sw_no_count(11usize, 2usize, 852u32, 2131452u32)?;
    emu.sw_no_count(12usize, 2usize, 856u32, 2131456u32)?;
    emu.sw_no_count(0usize, 2usize, 860u32, 2131460u32)?;
    emu.sw_no_count(0usize, 2usize, 864u32, 2131464u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2131468u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965448u32, 2131472u32);
    emu.adi_no_count(11usize, 2usize, 616u32, 2131476u32);
    emu.adi_no_count(12usize, 2usize, 620u32, 2131480u32);
    emu.adi_no_count(13usize, 2usize, 848u32, 2131484u32);
    emu.adi_no_count(10usize, 0usize, 0u32, 2131488u32);
    emu.apc_no_count(1usize, 2131488u32, 77824u32, 2131492u32);
    emu.add_memory_rw_events(17usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131496u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1528u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x00208628(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 4u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.adi_no_count(10usize, 0usize, 4u32, 2131500u32);
    emu.adi_no_count(11usize, 0usize, 12u32, 2131504u32);
    emu.apc_no_count(1usize, 2131504u32, 73728u32, 2131508u32);
    emu.add_memory_rw_events(4usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131512u32;
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
pub fn block_0x00208638(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 5u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2131516u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965476u32, 2131520u32);
    emu.adi_no_count(10usize, 21usize, 0u32, 2131524u32);
    emu.apc_no_count(1usize, 2131524u32, 77824u32, 2131528u32);
    emu.add_memory_rw_events(5usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131532u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1396u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x0020864c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(18usize, 2usize, 848u32, 2131536u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2131540u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2131552u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208660));
}
#[inline(always)]
pub fn block_0x00208654(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 2u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(27usize, 2usize, 848u32, 2131544u32)?;
    emu.add_memory_rw_events(2usize);
    let return_addr = 2131548u32;
    emu.write_reg_no_count(0usize, return_addr);
    emu.pc = 2131552u32;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208660));
}
#[inline(always)]
pub fn block_0x0020865c(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 1u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(19usize, 2usize, 848u32, 2131552u32)?;
    emu.add_memory_rw_events(1usize);
    emu.pc = 2131552u32;
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Direct(block_0x00208660));
}
#[inline]
pub fn block_0x00208660(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 10u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2131556u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 8u32, 2131560u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2131564u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294967252u32, 2131568u32);
    let a = 0u32.wrapping_add(2232320u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2131572u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 32u32, 2131576u32);
    emu.adi_no_count(11usize, 0usize, 22u32, 2131580u32);
    emu.adi_no_count(12usize, 2usize, 848u32, 2131584u32);
    emu.apc_no_count(1usize, 2131584u32, 81920u32, 2131588u32);
    emu.add_memory_rw_events(10usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131592u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1680u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline]
pub fn block_0x00208688(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 11u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    emu.sw_no_count(0usize, 2usize, 848u32, 2131596u32)?;
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(10usize, a);
    emu.pc = 2131600u32;
    emu.update_insn_clock();
    emu.adi_no_count(10usize, 10usize, 4294965624u32, 2131604u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(13usize, a);
    emu.pc = 2131608u32;
    emu.update_insn_clock();
    emu.adi_no_count(13usize, 13usize, 4294965544u32, 2131612u32);
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(14usize, a);
    emu.pc = 2131616u32;
    emu.update_insn_clock();
    emu.adi_no_count(14usize, 14usize, 4294965636u32, 2131620u32);
    emu.adi_no_count(11usize, 0usize, 12u32, 2131624u32);
    emu.adi_no_count(12usize, 2usize, 848u32, 2131628u32);
    emu.apc_no_count(1usize, 2131628u32, 81920u32, 2131632u32);
    emu.add_memory_rw_events(11usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131636u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1636u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
#[inline(always)]
pub fn block_0x002086b4(emu: &mut AotEmulatorCore) -> Result<crate::NextStep, String> {
    const BLOCK_INSNS: u32 = 6u32;
    if !emu.can_fit_instructions(BLOCK_INSNS) {
        return emu.interpret_from_current_pc();
    }
    if emu.is_unconstrained_mode() {
        return emu.interpret_from_current_pc();
    }
    let a = 0u32.wrapping_add(2236416u32);
    emu.write_reg_no_count(12usize, a);
    emu.pc = 2131640u32;
    emu.update_insn_clock();
    emu.adi_no_count(12usize, 12usize, 4294965404u32, 2131644u32);
    emu.adi_no_count(10usize, 0usize, 1u32, 2131648u32);
    emu.adi_no_count(11usize, 0usize, 11u32, 2131652u32);
    emu.apc_no_count(1usize, 2131652u32, 73728u32, 2131656u32);
    emu.add_memory_rw_events(6usize);
    let base = emu.read_reg_b_tracked(1usize);
    let return_addr = 2131660u32;
    emu.write_reg_no_count(1usize, return_addr);
    let target = base.wrapping_add(1072u32);
    emu.pc = target;
    emu.update_insn_clock();
    emu.check_chunk_boundary_fast();
    if emu.should_yield() {
        return Ok(crate::NextStep::Dynamic(emu.pc));
    }
    return Ok(crate::NextStep::Dynamic(target));
}
