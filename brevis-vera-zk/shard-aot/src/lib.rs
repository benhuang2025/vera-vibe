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
        pc_max: 2102000u32,
        lookup: shard_aot_chunk_000::lookup,
    },
    ChunkDesc {
        pc_min: 2102020u32,
        pc_max: 2105036u32,
        lookup: shard_aot_chunk_001::lookup,
    },
    ChunkDesc {
        pc_min: 2105040u32,
        pc_max: 2107420u32,
        lookup: shard_aot_chunk_002::lookup,
    },
    ChunkDesc {
        pc_min: 2107432u32,
        pc_max: 2110572u32,
        lookup: shard_aot_chunk_003::lookup,
    },
    ChunkDesc {
        pc_min: 2110576u32,
        pc_max: 2130724u32,
        lookup: shard_aot_chunk_004::lookup,
    },
    ChunkDesc {
        pc_min: 2130740u32,
        pc_max: 2133060u32,
        lookup: shard_aot_chunk_005::lookup,
    },
    ChunkDesc {
        pc_min: 2133064u32,
        pc_max: 2135048u32,
        lookup: shard_aot_chunk_006::lookup,
    },
    ChunkDesc {
        pc_min: 2135072u32,
        pc_max: 2137564u32,
        lookup: shard_aot_chunk_007::lookup,
    },
    ChunkDesc {
        pc_min: 2137572u32,
        pc_max: 2140500u32,
        lookup: shard_aot_chunk_008::lookup,
    },
    ChunkDesc {
        pc_min: 2140520u32,
        pc_max: 2142888u32,
        lookup: shard_aot_chunk_009::lookup,
    },
    ChunkDesc {
        pc_min: 2142892u32,
        pc_max: 2145116u32,
        lookup: shard_aot_chunk_010::lookup,
    },
    ChunkDesc {
        pc_min: 2145156u32,
        pc_max: 2148400u32,
        lookup: shard_aot_chunk_011::lookup,
    },
    ChunkDesc {
        pc_min: 2148516u32,
        pc_max: 2150776u32,
        lookup: shard_aot_chunk_012::lookup,
    },
    ChunkDesc {
        pc_min: 2150784u32,
        pc_max: 2152768u32,
        lookup: shard_aot_chunk_013::lookup,
    },
    ChunkDesc {
        pc_min: 2152796u32,
        pc_max: 2154816u32,
        lookup: shard_aot_chunk_014::lookup,
    },
    ChunkDesc {
        pc_min: 2154828u32,
        pc_max: 2157352u32,
        lookup: shard_aot_chunk_015::lookup,
    },
    ChunkDesc {
        pc_min: 2157364u32,
        pc_max: 2160452u32,
        lookup: shard_aot_chunk_016::lookup,
    },
    ChunkDesc {
        pc_min: 2160476u32,
        pc_max: 2162980u32,
        lookup: shard_aot_chunk_017::lookup,
    },
    ChunkDesc {
        pc_min: 2163032u32,
        pc_max: 2165472u32,
        lookup: shard_aot_chunk_018::lookup,
    },
    ChunkDesc {
        pc_min: 2165476u32,
        pc_max: 2168108u32,
        lookup: shard_aot_chunk_019::lookup,
    },
    ChunkDesc {
        pc_min: 2168136u32,
        pc_max: 2170832u32,
        lookup: shard_aot_chunk_020::lookup,
    },
    ChunkDesc {
        pc_min: 2170856u32,
        pc_max: 2172784u32,
        lookup: shard_aot_chunk_021::lookup,
    },
    ChunkDesc {
        pc_min: 2172788u32,
        pc_max: 2174556u32,
        lookup: shard_aot_chunk_022::lookup,
    },
    ChunkDesc {
        pc_min: 2174580u32,
        pc_max: 2176376u32,
        lookup: shard_aot_chunk_023::lookup,
    },
    ChunkDesc {
        pc_min: 2176388u32,
        pc_max: 2177888u32,
        lookup: shard_aot_chunk_024::lookup,
    },
    ChunkDesc {
        pc_min: 2177908u32,
        pc_max: 2179920u32,
        lookup: shard_aot_chunk_025::lookup,
    },
];
const GLOBAL_PC_MIN: u32 = 2099200u32;
const PAGE_SHIFT: u32 = 12u32;
const PAGE_HINT: [u16; 20usize] = [
    0u16, 1u16, 2u16, 4u16, 4u16, 4u16, 4u16, 4u16, 5u16, 7u16, 8u16, 10u16, 11u16,
    13u16, 15u16, 17u16, 18u16, 20u16, 22u16, 24u16,
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
