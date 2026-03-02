pub use pico_aot_runtime::AotEmulatorCore;
use pico_aot_runtime::{set_lookup_block_fn, BlockFn, NextStep};
pub fn run_aot(emu: &mut AotEmulatorCore) -> Result<(), String> {
    set_lookup_block_fn(lookup_block);
    let mut next = if emu.pc == 0 {
        NextStep::Halt
    } else if let Some(func) = lookup_block(emu.pc) {
        NextStep::Direct(func)
    } else {
        NextStep::Dynamic(emu.pc)
    };
    loop {
        if emu.should_yield() {
            break;
        }
        match next {
            NextStep::Direct(func) => {
                next = func(emu)?;
            }
            NextStep::Dynamic(pc) => {
                emu.pc = pc;
                if emu.pc == 0 {
                    next = NextStep::Halt;
                } else if let Some(func) = lookup_block(pc) {
                    next = NextStep::Direct(func);
                } else {
                    next = emu.interpret_from_current_pc()?;
                }
            }
            NextStep::Halt => break,
        }
    }
    Ok(())
}
type ChunkLookupFn = fn(u32) -> Option<BlockFn>;
#[repr(C)]
struct ChunkDesc {
    pc_min: u32,
    pc_max: u32,
    lookup: ChunkLookupFn,
}
const CHUNKS: &[ChunkDesc] = &[
    ChunkDesc {
        pc_min: 2099200u32,
        pc_max: 2102996u32,
        lookup: shard_aot_chunk_000::lookup,
    },
    ChunkDesc {
        pc_min: 2103040u32,
        pc_max: 2105412u32,
        lookup: shard_aot_chunk_001::lookup,
    },
    ChunkDesc {
        pc_min: 2105436u32,
        pc_max: 2108496u32,
        lookup: shard_aot_chunk_002::lookup,
    },
    ChunkDesc {
        pc_min: 2108512u32,
        pc_max: 2111212u32,
        lookup: shard_aot_chunk_003::lookup,
    },
    ChunkDesc {
        pc_min: 2111228u32,
        pc_max: 2130740u32,
        lookup: shard_aot_chunk_004::lookup,
    },
    ChunkDesc {
        pc_min: 2130756u32,
        pc_max: 2132840u32,
        lookup: shard_aot_chunk_005::lookup,
    },
    ChunkDesc {
        pc_min: 2132844u32,
        pc_max: 2135016u32,
        lookup: shard_aot_chunk_006::lookup,
    },
    ChunkDesc {
        pc_min: 2135072u32,
        pc_max: 2137416u32,
        lookup: shard_aot_chunk_007::lookup,
    },
    ChunkDesc {
        pc_min: 2137428u32,
        pc_max: 2140096u32,
        lookup: shard_aot_chunk_008::lookup,
    },
    ChunkDesc {
        pc_min: 2140116u32,
        pc_max: 2142484u32,
        lookup: shard_aot_chunk_009::lookup,
    },
    ChunkDesc {
        pc_min: 2142488u32,
        pc_max: 2144712u32,
        lookup: shard_aot_chunk_010::lookup,
    },
    ChunkDesc {
        pc_min: 2144752u32,
        pc_max: 2147996u32,
        lookup: shard_aot_chunk_011::lookup,
    },
    ChunkDesc {
        pc_min: 2148112u32,
        pc_max: 2150372u32,
        lookup: shard_aot_chunk_012::lookup,
    },
    ChunkDesc {
        pc_min: 2150380u32,
        pc_max: 2152364u32,
        lookup: shard_aot_chunk_013::lookup,
    },
    ChunkDesc {
        pc_min: 2152392u32,
        pc_max: 2154412u32,
        lookup: shard_aot_chunk_014::lookup,
    },
    ChunkDesc {
        pc_min: 2154424u32,
        pc_max: 2156944u32,
        lookup: shard_aot_chunk_015::lookup,
    },
    ChunkDesc {
        pc_min: 2156948u32,
        pc_max: 2160048u32,
        lookup: shard_aot_chunk_016::lookup,
    },
    ChunkDesc {
        pc_min: 2160072u32,
        pc_max: 2162576u32,
        lookup: shard_aot_chunk_017::lookup,
    },
    ChunkDesc {
        pc_min: 2162628u32,
        pc_max: 2165068u32,
        lookup: shard_aot_chunk_018::lookup,
    },
    ChunkDesc {
        pc_min: 2165072u32,
        pc_max: 2167704u32,
        lookup: shard_aot_chunk_019::lookup,
    },
    ChunkDesc {
        pc_min: 2167732u32,
        pc_max: 2170428u32,
        lookup: shard_aot_chunk_020::lookup,
    },
    ChunkDesc {
        pc_min: 2170452u32,
        pc_max: 2172380u32,
        lookup: shard_aot_chunk_021::lookup,
    },
    ChunkDesc {
        pc_min: 2172384u32,
        pc_max: 2174152u32,
        lookup: shard_aot_chunk_022::lookup,
    },
    ChunkDesc {
        pc_min: 2174176u32,
        pc_max: 2175916u32,
        lookup: shard_aot_chunk_023::lookup,
    },
    ChunkDesc {
        pc_min: 2175968u32,
        pc_max: 2177484u32,
        lookup: shard_aot_chunk_024::lookup,
    },
    ChunkDesc {
        pc_min: 2177504u32,
        pc_max: 2179516u32,
        lookup: shard_aot_chunk_025::lookup,
    },
];
const GLOBAL_PC_MIN: u32 = 2099200u32;
const PAGE_SHIFT: u32 = 12u32;
const PAGE_HINT: [u16; 20usize] = [
    0u16, 1u16, 2u16, 4u16, 4u16, 4u16, 4u16, 4u16, 5u16, 7u16, 9u16, 10u16, 12u16,
    14u16, 15u16, 17u16, 18u16, 20u16, 22u16, 24u16,
];
fn lookup_block(pc: u32) -> Option<BlockFn> {
    if CHUNKS.is_empty() {
        return None;
    }
    if pc < GLOBAL_PC_MIN {
        return None;
    }
    let off = pc - GLOBAL_PC_MIN;
    let page = (off >> PAGE_SHIFT) as usize;
    let mut idx = if page < PAGE_HINT.len() {
        PAGE_HINT[page] as usize
    } else {
        CHUNKS.len().saturating_sub(1)
    };
    while idx < CHUNKS.len() && pc > CHUNKS[idx].pc_max {
        idx += 1;
    }
    if idx == CHUNKS.len() {
        return None;
    }
    let c = &CHUNKS[idx];
    if pc < c.pc_min {
        return None;
    }
    (c.lookup)(pc)
}
