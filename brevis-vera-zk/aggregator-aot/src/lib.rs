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
        pc_max: 2101996u32,
        lookup: agg_aot_chunk_000::lookup,
    },
    ChunkDesc {
        pc_min: 2102012u32,
        pc_max: 2105340u32,
        lookup: agg_aot_chunk_001::lookup,
    },
    ChunkDesc {
        pc_min: 2105400u32,
        pc_max: 2109824u32,
        lookup: agg_aot_chunk_002::lookup,
    },
    ChunkDesc {
        pc_min: 2109932u32,
        pc_max: 2112284u32,
        lookup: agg_aot_chunk_003::lookup,
    },
    ChunkDesc {
        pc_min: 2112324u32,
        pc_max: 2114712u32,
        lookup: agg_aot_chunk_004::lookup,
    },
    ChunkDesc {
        pc_min: 2114860u32,
        pc_max: 2120688u32,
        lookup: agg_aot_chunk_005::lookup,
    },
    ChunkDesc {
        pc_min: 2120792u32,
        pc_max: 2124804u32,
        lookup: agg_aot_chunk_006::lookup,
    },
    ChunkDesc {
        pc_min: 2124844u32,
        pc_max: 2127572u32,
        lookup: agg_aot_chunk_007::lookup,
    },
    ChunkDesc {
        pc_min: 2127624u32,
        pc_max: 2130704u32,
        lookup: agg_aot_chunk_008::lookup,
    },
    ChunkDesc {
        pc_min: 2130732u32,
        pc_max: 2134332u32,
        lookup: agg_aot_chunk_009::lookup,
    },
    ChunkDesc {
        pc_min: 2134336u32,
        pc_max: 2137072u32,
        lookup: agg_aot_chunk_010::lookup,
    },
    ChunkDesc {
        pc_min: 2137140u32,
        pc_max: 2143584u32,
        lookup: agg_aot_chunk_011::lookup,
    },
    ChunkDesc {
        pc_min: 2143692u32,
        pc_max: 2149020u32,
        lookup: agg_aot_chunk_012::lookup,
    },
    ChunkDesc {
        pc_min: 2149160u32,
        pc_max: 2157856u32,
        lookup: agg_aot_chunk_013::lookup,
    },
    ChunkDesc {
        pc_min: 2157940u32,
        pc_max: 2166080u32,
        lookup: agg_aot_chunk_014::lookup,
    },
    ChunkDesc {
        pc_min: 2166124u32,
        pc_max: 2188072u32,
        lookup: agg_aot_chunk_015::lookup,
    },
    ChunkDesc {
        pc_min: 2188188u32,
        pc_max: 2191328u32,
        lookup: agg_aot_chunk_016::lookup,
    },
    ChunkDesc {
        pc_min: 2191352u32,
        pc_max: 2193804u32,
        lookup: agg_aot_chunk_017::lookup,
    },
    ChunkDesc {
        pc_min: 2193860u32,
        pc_max: 2195972u32,
        lookup: agg_aot_chunk_018::lookup,
    },
    ChunkDesc {
        pc_min: 2196016u32,
        pc_max: 2198660u32,
        lookup: agg_aot_chunk_019::lookup,
    },
    ChunkDesc {
        pc_min: 2198716u32,
        pc_max: 2201544u32,
        lookup: agg_aot_chunk_020::lookup,
    },
    ChunkDesc {
        pc_min: 2201556u32,
        pc_max: 2204168u32,
        lookup: agg_aot_chunk_021::lookup,
    },
    ChunkDesc {
        pc_min: 2204192u32,
        pc_max: 2206032u32,
        lookup: agg_aot_chunk_022::lookup,
    },
    ChunkDesc {
        pc_min: 2206044u32,
        pc_max: 2209612u32,
        lookup: agg_aot_chunk_023::lookup,
    },
    ChunkDesc {
        pc_min: 2209628u32,
        pc_max: 2211784u32,
        lookup: agg_aot_chunk_024::lookup,
    },
    ChunkDesc {
        pc_min: 2211792u32,
        pc_max: 2214088u32,
        lookup: agg_aot_chunk_025::lookup,
    },
    ChunkDesc {
        pc_min: 2214096u32,
        pc_max: 2216692u32,
        lookup: agg_aot_chunk_026::lookup,
    },
    ChunkDesc {
        pc_min: 2216696u32,
        pc_max: 2219780u32,
        lookup: agg_aot_chunk_027::lookup,
    },
    ChunkDesc {
        pc_min: 2219808u32,
        pc_max: 2221908u32,
        lookup: agg_aot_chunk_028::lookup,
    },
    ChunkDesc {
        pc_min: 2221944u32,
        pc_max: 2223476u32,
        lookup: agg_aot_chunk_029::lookup,
    },
];
const GLOBAL_PC_MIN: u32 = 2099200u32;
const PAGE_SHIFT: u32 = 12u32;
const PAGE_HINT: [u16; 31usize] = [
    0u16, 1u16, 2u16, 3u16, 5u16, 5u16, 6u16, 8u16, 9u16, 10u16, 11u16, 12u16, 12u16,
    13u16, 13u16, 14u16, 14u16, 15u16, 15u16, 15u16, 15u16, 15u16, 16u16, 17u16, 19u16,
    21u16, 22u16, 24u16, 25u16, 27u16, 29u16,
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
