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
        pc_max: 2110588u32,
        lookup: shard_aot_chunk_003::lookup,
    },
    ChunkDesc {
        pc_min: 2110592u32,
        pc_max: 2130556u32,
        lookup: shard_aot_chunk_004::lookup,
    },
    ChunkDesc {
        pc_min: 2130600u32,
        pc_max: 2132956u32,
        lookup: shard_aot_chunk_005::lookup,
    },
    ChunkDesc {
        pc_min: 2132968u32,
        pc_max: 2135292u32,
        lookup: shard_aot_chunk_006::lookup,
    },
    ChunkDesc {
        pc_min: 2135316u32,
        pc_max: 2137808u32,
        lookup: shard_aot_chunk_007::lookup,
    },
    ChunkDesc {
        pc_min: 2137816u32,
        pc_max: 2140744u32,
        lookup: shard_aot_chunk_008::lookup,
    },
    ChunkDesc {
        pc_min: 2140764u32,
        pc_max: 2143132u32,
        lookup: shard_aot_chunk_009::lookup,
    },
    ChunkDesc {
        pc_min: 2143136u32,
        pc_max: 2145360u32,
        lookup: shard_aot_chunk_010::lookup,
    },
    ChunkDesc {
        pc_min: 2145400u32,
        pc_max: 2148644u32,
        lookup: shard_aot_chunk_011::lookup,
    },
    ChunkDesc {
        pc_min: 2148760u32,
        pc_max: 2151020u32,
        lookup: shard_aot_chunk_012::lookup,
    },
    ChunkDesc {
        pc_min: 2151028u32,
        pc_max: 2153012u32,
        lookup: shard_aot_chunk_013::lookup,
    },
    ChunkDesc {
        pc_min: 2153040u32,
        pc_max: 2155060u32,
        lookup: shard_aot_chunk_014::lookup,
    },
    ChunkDesc {
        pc_min: 2155072u32,
        pc_max: 2157608u32,
        lookup: shard_aot_chunk_015::lookup,
    },
    ChunkDesc {
        pc_min: 2157612u32,
        pc_max: 2160696u32,
        lookup: shard_aot_chunk_016::lookup,
    },
    ChunkDesc {
        pc_min: 2160720u32,
        pc_max: 2163224u32,
        lookup: shard_aot_chunk_017::lookup,
    },
    ChunkDesc {
        pc_min: 2163276u32,
        pc_max: 2165716u32,
        lookup: shard_aot_chunk_018::lookup,
    },
    ChunkDesc {
        pc_min: 2165720u32,
        pc_max: 2168352u32,
        lookup: shard_aot_chunk_019::lookup,
    },
    ChunkDesc {
        pc_min: 2168380u32,
        pc_max: 2171076u32,
        lookup: shard_aot_chunk_020::lookup,
    },
    ChunkDesc {
        pc_min: 2171100u32,
        pc_max: 2173028u32,
        lookup: shard_aot_chunk_021::lookup,
    },
    ChunkDesc {
        pc_min: 2173032u32,
        pc_max: 2174800u32,
        lookup: shard_aot_chunk_022::lookup,
    },
    ChunkDesc {
        pc_min: 2174824u32,
        pc_max: 2176636u32,
        lookup: shard_aot_chunk_023::lookup,
    },
    ChunkDesc {
        pc_min: 2176652u32,
        pc_max: 2178168u32,
        lookup: shard_aot_chunk_024::lookup,
    },
    ChunkDesc {
        pc_min: 2178176u32,
        pc_max: 2180164u32,
        lookup: shard_aot_chunk_025::lookup,
    },
];
const GLOBAL_PC_MIN: u32 = 2099200u32;
const PAGE_SHIFT: u32 = 12u32;
const PAGE_HINT: [u16; 20usize] = [
    0u16, 1u16, 2u16, 4u16, 4u16, 4u16, 4u16, 4u16, 5u16, 7u16, 8u16, 10u16, 11u16,
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
