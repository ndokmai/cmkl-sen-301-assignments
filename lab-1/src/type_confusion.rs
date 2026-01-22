use std::alloc::{alloc, dealloc, Layout};

/// Lab 1D - Type / Structure Integrity (Allocate the right layout)
///
/// Implicit assumption:
///   "A *mut Node points to memory that is large enough and correctly laid out for Node."
///
/// Bug:
///   We allocate ONLY 8 bytes (u64-sized), then cast it to *mut Node.
///   Writing Node fields writes past the allocation boundary (UB).
///
/// Your tasks:
///   1) Fix by allocating the correct amount of memory for Node:
///        Layout::new::<Node>()
///   2) Ensure dealloc uses the matching Layout (same type / size / alignment).
///   3) Keep it minimal: allocate, initialize fields, read back, deallocate.
///
/// Note:
///   This is undefined behavior (UB). It may appear to work while still being wrong.
#[repr(C)]
struct Node {
    value: u64,
    next: *mut Node,
}

pub fn run() {
    println!("\n== type integrity: allocate correct layout for Node ==");

    unsafe {
        // BUG: allocate too little memory (only enough for a u64).
        // TODO: change this to Layout::new::<Node>()
        let layout = Layout::new::<u64>();
        let p = alloc(layout) as *mut Node;
        if p.is_null() {
            panic!("alloc failed");
        }

        // UB: Node is larger than u64, so this write goes out of bounds.
        (*p).value = 42;
        (*p).next = std::ptr::null_mut();

        println!("Node.value = {}", (*p).value);

        // TODO: dealloc must use the SAME layout that was used for alloc.
        dealloc(p as *mut u8, layout);
    }
}
