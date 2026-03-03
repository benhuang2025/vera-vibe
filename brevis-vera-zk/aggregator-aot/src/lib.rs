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
        lookup: pico_aot_chunk_000::lookup,
    },
    ChunkDesc {
        pc_min: 2102012u32,
        pc_max: 2105340u32,
        lookup: pico_aot_chunk_001::lookup,
    },
    ChunkDesc {
        pc_min: 2105400u32,
        pc_max: 2109812u32,
        lookup: pico_aot_chunk_002::lookup,
    },
    ChunkDesc {
        pc_min: 2109824u32,
        pc_max: 2112244u32,
        lookup: pico_aot_chunk_003::lookup,
    },
    ChunkDesc {
        pc_min: 2112256u32,
        pc_max: 2114344u32,
        lookup: pico_aot_chunk_004::lookup,
    },
    ChunkDesc {
        pc_min: 2114360u32,
        pc_max: 2120812u32,
        lookup: pico_aot_chunk_005::lookup,
    },
    ChunkDesc {
        pc_min: 2120832u32,
        pc_max: 2124784u32,
        lookup: pico_aot_chunk_006::lookup,
    },
    ChunkDesc {
        pc_min: 2124824u32,
        pc_max: 2127572u32,
        lookup: pico_aot_chunk_007::lookup,
    },
    ChunkDesc {
        pc_min: 2127624u32,
        pc_max: 2130680u32,
        lookup: pico_aot_chunk_008::lookup,
    },
    ChunkDesc {
        pc_min: 2130704u32,
        pc_max: 2134312u32,
        lookup: pico_aot_chunk_009::lookup,
    },
    ChunkDesc {
        pc_min: 2134316u32,
        pc_max: 2137036u32,
        lookup: pico_aot_chunk_010::lookup,
    },
    ChunkDesc {
        pc_min: 2137104u32,
        pc_max: 2143548u32,
        lookup: pico_aot_chunk_011::lookup,
    },
    ChunkDesc {
        pc_min: 2143656u32,
        pc_max: 2148984u32,
        lookup: pico_aot_chunk_012::lookup,
    },
    ChunkDesc {
        pc_min: 2149124u32,
        pc_max: 2157736u32,
        lookup: pico_aot_chunk_013::lookup,
    },
    ChunkDesc {
        pc_min: 2157820u32,
        pc_max: 2166044u32,
        lookup: pico_aot_chunk_014::lookup,
    },
    ChunkDesc {
        pc_min: 2166088u32,
        pc_max: 2171204u32,
        lookup: pico_aot_chunk_015::lookup,
    },
    ChunkDesc {
        pc_min: 2171284u32,
        pc_max: 2191016u32,
        lookup: pico_aot_chunk_016::lookup,
    },
    ChunkDesc {
        pc_min: 2191040u32,
        pc_max: 2193492u32,
        lookup: pico_aot_chunk_017::lookup,
    },
    ChunkDesc {
        pc_min: 2193548u32,
        pc_max: 2195660u32,
        lookup: pico_aot_chunk_018::lookup,
    },
    ChunkDesc {
        pc_min: 2195704u32,
        pc_max: 2198348u32,
        lookup: pico_aot_chunk_019::lookup,
    },
    ChunkDesc {
        pc_min: 2198404u32,
        pc_max: 2201232u32,
        lookup: pico_aot_chunk_020::lookup,
    },
    ChunkDesc {
        pc_min: 2201244u32,
        pc_max: 2203856u32,
        lookup: pico_aot_chunk_021::lookup,
    },
    ChunkDesc {
        pc_min: 2203880u32,
        pc_max: 2205720u32,
        lookup: pico_aot_chunk_022::lookup,
    },
    ChunkDesc {
        pc_min: 2205732u32,
        pc_max: 2209300u32,
        lookup: pico_aot_chunk_023::lookup,
    },
    ChunkDesc {
        pc_min: 2209316u32,
        pc_max: 2211472u32,
        lookup: pico_aot_chunk_024::lookup,
    },
    ChunkDesc {
        pc_min: 2211480u32,
        pc_max: 2213776u32,
        lookup: pico_aot_chunk_025::lookup,
    },
    ChunkDesc {
        pc_min: 2213784u32,
        pc_max: 2216380u32,
        lookup: pico_aot_chunk_026::lookup,
    },
    ChunkDesc {
        pc_min: 2216384u32,
        pc_max: 2219468u32,
        lookup: pico_aot_chunk_027::lookup,
    },
    ChunkDesc {
        pc_min: 2219496u32,
        pc_max: 2221596u32,
        lookup: pico_aot_chunk_028::lookup,
    },
    ChunkDesc {
        pc_min: 2221632u32,
        pc_max: 2223164u32,
        lookup: pico_aot_chunk_029::lookup,
    },
];
const GLOBAL_PC_MIN: u32 = 2099200u32;
const PAGE_SHIFT: u32 = 12u32;
const PAGE_HINT: [u16; 31usize] = [
    0u16, 1u16, 2u16, 3u16, 5u16, 5u16, 6u16, 8u16, 9u16, 10u16, 11u16, 12u16, 12u16,
    13u16, 13u16, 14u16, 14u16, 15u16, 16u16, 16u16, 16u16, 16u16, 16u16, 17u16, 19u16,
    21u16, 22u16, 24u16, 26u16, 27u16, 29u16,
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
