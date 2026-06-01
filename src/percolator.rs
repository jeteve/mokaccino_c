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

/// Index the given query under the given id.
///
/// # Safety
/// - p cannot be NULL
/// - q cannot be NULL
/// - *q cannot by NULL
/// - *q will be NULL after this call, as this consumes the Query.
///   You don't need to free the query as it will now live in the Percolator.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mokaccino_p_index_id(
    p: *mut Percolator,
    q: *mut *mut super::queries::Query,
    id: u32,
) -> c_int {
    if p.is_null() {
        eprintln!("ERROR given *p is NULL");
        return MOKACCINO_ERROR;
    }

    if q.is_null() {
        eprintln!("ERROR given *q is NULL");
        return MOKACCINO_ERROR;
    }

    let qq = unsafe { *q };
    if qq.is_null() {
        eprintln!("ERROR given *q points to a NULL *Query");
        return MOKACCINO_ERROR;
    }

    let qq: super::queries::Query = *{ unsafe { Box::from_raw(qq) } };
    let rust_p = unsafe { &mut *p };

    let index_res = {
        unsafe {
            *q = std::ptr::null_mut();
        }
        rust_p.0.index_query_uid(qq.0, id)
    };

    match index_res {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("ERROR adding query: {e:?}");
            MOKACCINO_ERROR
        }
    }
}

/// Percolate the document and emit the matching query IDs
/// in the given callback. You can give a pointer to some of your data
/// to the callback too.
///
/// # Safety
/// - p cannot be NULL
/// - d cannot be NULL
/// - cb can be NULL. If not NULL, it will be called for each matching query ID.
///
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mokaccino_p_percolate(
    p: *mut Percolator,
    d: *const super::documents::Document,
    cb: Option<unsafe extern "C" fn(u32, *mut std::ffi::c_void)>,
    user_data: *mut std::ffi::c_void,
) -> c_int {
    if p.is_null() {
        eprintln!("ERROR given *p is NULL");
        return MOKACCINO_ERROR;
    }

    if d.is_null() {
        eprintln!("ERROR given *d is NULL");
        return MOKACCINO_ERROR;
    }

    let dd = unsafe { &*d };
    let rust_p = unsafe { &*p };
    let matches = rust_p.0.percolate(&dd.0);

    if let Some(callback) = cb {
        for id in matches {
            unsafe { callback(id, user_data) };
        }
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
