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
        pc_max: 2102488u32,
        lookup: shard_aot_chunk_000::lookup,
    },
    ChunkDesc {
        pc_min: 2102512u32,
        pc_max: 2105272u32,
        lookup: shard_aot_chunk_001::lookup,
    },
    ChunkDesc {
        pc_min: 2105276u32,
        pc_max: 2108892u32,
        lookup: shard_aot_chunk_002::lookup,
    },
    ChunkDesc {
        pc_min: 2108908u32,
        pc_max: 2111644u32,
        lookup: shard_aot_chunk_003::lookup,
    },
    ChunkDesc {
        pc_min: 2111660u32,
        pc_max: 2131480u32,
        lookup: shard_aot_chunk_004::lookup,
    },
    ChunkDesc {
        pc_min: 2131484u32,
        pc_max: 2133788u32,
        lookup: shard_aot_chunk_005::lookup,
    },
    ChunkDesc {
        pc_min: 2133792u32,
        pc_max: 2136068u32,
        lookup: shard_aot_chunk_006::lookup,
    },
    ChunkDesc {
        pc_min: 2136120u32,
        pc_max: 2138572u32,
        lookup: shard_aot_chunk_007::lookup,
    },
    ChunkDesc {
        pc_min: 2138620u32,
        pc_max: 2141044u32,
        lookup: shard_aot_chunk_008::lookup,
    },
    ChunkDesc {
        pc_min: 2141064u32,
        pc_max: 2143432u32,
        lookup: shard_aot_chunk_009::lookup,
    },
    ChunkDesc {
        pc_min: 2143436u32,
        pc_max: 2145660u32,
        lookup: shard_aot_chunk_010::lookup,
    },
    ChunkDesc {
        pc_min: 2145700u32,
        pc_max: 2148944u32,
        lookup: shard_aot_chunk_011::lookup,
    },
    ChunkDesc {
        pc_min: 2149060u32,
        pc_max: 2151320u32,
        lookup: shard_aot_chunk_012::lookup,
    },
    ChunkDesc {
        pc_min: 2151328u32,
        pc_max: 2154168u32,
        lookup: shard_aot_chunk_013::lookup,
    },
    ChunkDesc {
        pc_min: 2154196u32,
        pc_max: 2156436u32,
        lookup: shard_aot_chunk_014::lookup,
    },
    ChunkDesc {
        pc_min: 2156440u32,
        pc_max: 2159176u32,
        lookup: shard_aot_chunk_015::lookup,
    },
    ChunkDesc {
        pc_min: 2159200u32,
        pc_max: 2161752u32,
        lookup: shard_aot_chunk_016::lookup,
    },
    ChunkDesc {
        pc_min: 2161776u32,
        pc_max: 2164428u32,
        lookup: shard_aot_chunk_017::lookup,
    },
    ChunkDesc {
        pc_min: 2164432u32,
        pc_max: 2166656u32,
        lookup: shard_aot_chunk_018::lookup,
    },
    ChunkDesc {
        pc_min: 2166684u32,
        pc_max: 2168652u32,
        lookup: shard_aot_chunk_019::lookup,
    },
    ChunkDesc {
        pc_min: 2168680u32,
        pc_max: 2171376u32,
        lookup: shard_aot_chunk_020::lookup,
    },
    ChunkDesc {
        pc_min: 2171400u32,
        pc_max: 2173328u32,
        lookup: shard_aot_chunk_021::lookup,
    },
    ChunkDesc {
        pc_min: 2173332u32,
        pc_max: 2175100u32,
        lookup: shard_aot_chunk_022::lookup,
    },
    ChunkDesc {
        pc_min: 2175124u32,
        pc_max: 2176936u32,
        lookup: shard_aot_chunk_023::lookup,
    },
    ChunkDesc {
        pc_min: 2176952u32,
        pc_max: 2178972u32,
        lookup: shard_aot_chunk_024::lookup,
    },
    ChunkDesc {
        pc_min: 2179008u32,
        pc_max: 2180464u32,
        lookup: shard_aot_chunk_025::lookup,
    },
];
const GLOBAL_PC_MIN: u32 = 2099200u32;
const PAGE_SHIFT: u32 = 12u32;
const PAGE_HINT: [u16; 20usize] = [
    0u16, 1u16, 2u16, 3u16, 4u16, 4u16, 4u16, 4u16, 5u16, 6u16, 8u16, 10u16, 11u16,
    13u16, 15u16, 16u16, 18u16, 20u16, 21u16, 24u16,
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
