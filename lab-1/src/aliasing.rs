/// Lab 1C - Aliasing (Input must not be overwritten mid-computation)
///
/// Implicit assumption:
///   "src does not change while I'm computing pow(src, p)."
///
/// Why aliasing breaks this:
///   If `dst` aliases `src` (i.e., src == dst), then writing intermediate results to `dst`
///   overwrites the base value stored at `src`. That changes the meaning of the computation
///   while it is still running.
///
/// Your tasks:
///   1) Detect aliasing: if src == dst, return Err(PowError::AliasingNotAllowed).
///   2) Do not modify memory on error (i.e., `*dst` must remain unchanged).
///   3) Add a correct call where src and dst are distinct (so the computation is meaningful).
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowError {
    AliasingNotAllowed,
}

unsafe fn pow(src: *const u64, dst: *mut u64, p: u32) -> Result<(), PowError> {
    // BUG: assumes src and dst are distinct.
    // TODO: if src == dst, return Err(PowError::AliasingNotAllowed) and do not write anything.

    unsafe {
        *dst = 1;
        for _ in 0..p {
            *dst = *dst * *src;
        }
    }

    Ok(())
}

pub fn run() {
    println!("\n== aliasing: pow(src, dst, p) assumes src is stable ==");

    let p: u32 = 3;

    // Caller mistake (aliasing: src == dst)
    let mut same: u64 = 3;
    let r_bad = unsafe { pow(&same as *const u64, &mut same as *mut u64, p) };
    println!("(aliased): res={:?}, same={}", r_bad, same);

    // TODO: Add a correct call where src and dst are distinct, e.g.:
    // let base: u64 = 3;
    // let mut out: u64 = 0;
    // let r_ok = unsafe { pow(&base as *const u64, &mut out as *mut u64, p) };
    // println!("(distinct): res={:?}, base={}, out={}", r_ok, base, out);

    // Expected behavior after student fix:
    // - aliased call returns Err(PowError::AliasingNotAllowed)
    // - `same` remains unchanged
}
