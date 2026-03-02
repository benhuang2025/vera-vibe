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
        pc_max: 2105692u32,
        lookup: agg_aot_chunk_001::lookup,
    },
    ChunkDesc {
        pc_min: 2105760u32,
        pc_max: 2110184u32,
        lookup: agg_aot_chunk_002::lookup,
    },
    ChunkDesc {
        pc_min: 2110292u32,
        pc_max: 2116088u32,
        lookup: agg_aot_chunk_003::lookup,
    },
    ChunkDesc {
        pc_min: 2116140u32,
        pc_max: 2118508u32,
        lookup: agg_aot_chunk_004::lookup,
    },
    ChunkDesc {
        pc_min: 2118532u32,
        pc_max: 2121452u32,
        lookup: agg_aot_chunk_005::lookup,
    },
    ChunkDesc {
        pc_min: 2121472u32,
        pc_max: 2126348u32,
        lookup: agg_aot_chunk_006::lookup,
    },
    ChunkDesc {
        pc_min: 2126388u32,
        pc_max: 2131636u32,
        lookup: agg_aot_chunk_007::lookup,
    },
    ChunkDesc {
        pc_min: 2131660u32,
        pc_max: 2133788u32,
        lookup: agg_aot_chunk_008::lookup,
    },
    ChunkDesc {
        pc_min: 2133808u32,
        pc_max: 2137196u32,
        lookup: agg_aot_chunk_009::lookup,
    },
    ChunkDesc {
        pc_min: 2137200u32,
        pc_max: 2139920u32,
        lookup: agg_aot_chunk_010::lookup,
    },
    ChunkDesc {
        pc_min: 2139988u32,
        pc_max: 2146432u32,
        lookup: agg_aot_chunk_011::lookup,
    },
    ChunkDesc {
        pc_min: 2146540u32,
        pc_max: 2151868u32,
        lookup: agg_aot_chunk_012::lookup,
    },
    ChunkDesc {
        pc_min: 2152008u32,
        pc_max: 2160704u32,
        lookup: agg_aot_chunk_013::lookup,
    },
    ChunkDesc {
        pc_min: 2160788u32,
        pc_max: 2168928u32,
        lookup: agg_aot_chunk_014::lookup,
    },
    ChunkDesc {
        pc_min: 2168972u32,
        pc_max: 2190920u32,
        lookup: agg_aot_chunk_015::lookup,
    },
    ChunkDesc {
        pc_min: 2191036u32,
        pc_max: 2193796u32,
        lookup: agg_aot_chunk_016::lookup,
    },
    ChunkDesc {
        pc_min: 2193808u32,
        pc_max: 2196136u32,
        lookup: agg_aot_chunk_017::lookup,
    },
    ChunkDesc {
        pc_min: 2196148u32,
        pc_max: 2198472u32,
        lookup: agg_aot_chunk_018::lookup,
    },
    ChunkDesc {
        pc_min: 2198496u32,
        pc_max: 2200988u32,
        lookup: agg_aot_chunk_019::lookup,
    },
    ChunkDesc {
        pc_min: 2200996u32,
        pc_max: 2203924u32,
        lookup: agg_aot_chunk_020::lookup,
    },
    ChunkDesc {
        pc_min: 2203944u32,
        pc_max: 2206244u32,
        lookup: agg_aot_chunk_021::lookup,
    },
    ChunkDesc {
        pc_min: 2206268u32,
        pc_max: 2208240u32,
        lookup: agg_aot_chunk_022::lookup,
    },
    ChunkDesc {
        pc_min: 2208280u32,
        pc_max: 2211884u32,
        lookup: agg_aot_chunk_023::lookup,
    },
    ChunkDesc {
        pc_min: 2212000u32,
        pc_max: 2214088u32,
        lookup: agg_aot_chunk_024::lookup,
    },
    ChunkDesc {
        pc_min: 2214112u32,
        pc_max: 2216876u32,
        lookup: agg_aot_chunk_025::lookup,
    },
    ChunkDesc {
        pc_min: 2216928u32,
        pc_max: 2218892u32,
        lookup: agg_aot_chunk_026::lookup,
    },
    ChunkDesc {
        pc_min: 2218928u32,
        pc_max: 2220780u32,
        lookup: agg_aot_chunk_027::lookup,
    },
    ChunkDesc {
        pc_min: 2220808u32,
        pc_max: 2222320u32,
        lookup: agg_aot_chunk_028::lookup,
    },
    ChunkDesc {
        pc_min: 2222336u32,
        pc_max: 2224792u32,
        lookup: agg_aot_chunk_029::lookup,
    },
    ChunkDesc {
        pc_min: 2224800u32,
        pc_max: 2224800u32,
        lookup: agg_aot_chunk_030::lookup,
    },
];
const GLOBAL_PC_MIN: u32 = 2099200u32;
const PAGE_SHIFT: u32 = 12u32;
const PAGE_HINT: [u16; 31usize] = [
    0u16, 1u16, 2u16, 3u16, 3u16, 5u16, 6u16, 7u16, 8u16, 9u16, 11u16, 11u16, 12u16,
    13u16, 13u16, 13u16, 14u16, 14u16, 15u16, 15u16, 15u16, 15u16, 15u16, 16u16, 18u16,
    20u16, 21u16, 23u16, 24u16, 26u16, 28u16,
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
