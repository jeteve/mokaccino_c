use std::ffi::c_int;

use super::MOKACCINO_ERROR;

pub struct Percolator(mokaccino::prelude::Percolator);

/// Builds an empty *Percolator given a **Percolator to a NULL *Percolator
///
/// # Safety
/// - p MUST be a NON-NULL **Percolator, pointing to a NULL *Percolator
/// - Use `mokacino_p_free` when you're done with the percolator to avoid memory leaks.
///
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mokaccino_p_new(p: *mut *mut Percolator) -> c_int {
    if p.is_null() {
        eprintln!("ERROR: given p pointer is null");
        return MOKACCINO_ERROR;
    }

    if !unsafe { *p }.is_null() {
        eprintln!(
            "ERROR: given p pointer is NOT a null *Percolator. Calling this would lead to a memory leak"
        );
        return MOKACCINO_ERROR;
    }

    unsafe {
        *p = Box::into_raw(Box::new(Percolator(
            mokaccino::prelude::Percolator::default(),
        )));
    }

    0
}

/// Frees the given *Percolator, setting the **Percolator to NULL
///
/// # Safety
/// - Calling with null is safe.
///
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mokaccino_p_free(p: *mut *mut Percolator) {
    if p.is_null() {
        return;
    }

    let pp = unsafe { *p };
    if pp.is_null() {
        return;
    }

    unsafe {
        drop(Box::from_raw(pp));
        *p = std::ptr::null_mut();
    }
}
