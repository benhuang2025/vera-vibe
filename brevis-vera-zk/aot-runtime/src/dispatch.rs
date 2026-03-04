use crate::BlockFn;
use std::sync::atomic::{AtomicPtr, Ordering};

type LookupFn = fn(u32) -> Option<BlockFn>;
static LOOKUP_BLOCK_FN: AtomicPtr<()> = AtomicPtr::new(std::ptr::null_mut());

pub fn set_lookup_block_fn(func: LookupFn) {
    LOOKUP_BLOCK_FN.store(func as *mut (), Ordering::Relaxed);
}

pub fn lookup_block_fn(pc: u32) -> Option<BlockFn> {
    let ptr = LOOKUP_BLOCK_FN.load(Ordering::Relaxed);
    if ptr.is_null() {
        None
    } else {
        let func: LookupFn = unsafe { std::mem::transmute(ptr) };
        func(pc)
    }
}
