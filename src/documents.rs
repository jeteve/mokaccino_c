use std::ffi::c_int;

use super::MOKACCINO_ERROR;

pub struct Document(pub(crate) mokaccino::prelude::Document);

/// Builds an empty document given a **Document to a NULL *Document
///
/// # Safety
/// - d MUST be a NON-NULL **Document, pointing to a NULL *Document
/// - Use `mokaccino_d_free` when you're done with the document.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mokaccino_d_new(d: *mut *mut Document) -> c_int {
    ensure_ptr_ptr_is_null!(d, "d", "Document");

    let new_doc = mokaccino::prelude::Document::new();
    unsafe {
        *d = Box::into_raw(Box::new(Document(new_doc)));
    }

    0
}

/// Returns a debug representation of this document.
///
/// # Safety
/// - d is not NULL
/// - Free the returned char* with `mokaccino_string_free`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mokaccino_d_debug(d: *const Document) -> *mut std::ffi::c_char {
    if d.is_null() {
        return std::ptr::null_mut();
    }
    let document = unsafe { &*d };
    let debug_str = format!("{:?}", document.0);
    match std::ffi::CString::new(debug_str) {
        Ok(c_str) => c_str.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Adds a field,value to this Document*
///
/// After this call, *d is replaced by a new value.
///
/// # Safety
/// - d is not NULL
/// - *d is not NULL
/// - do not save the value *d yourself.
/// - field and value must be NULL terminated valid UTF8 bytes or an ASCII string.
///
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mokaccino_d_add_value(
    d: *mut *mut Document,
    field: *const std::ffi::c_char,
    value: *const std::ffi::c_char,
) -> c_int {
    let dd = ensure_ptr_ptr_not_null!(d, "d", "Document");

    if field.is_null() || value.is_null() {
        eprintln!("ERROR: Either field or value is null");
        return MOKACCINO_ERROR;
    }

    let field_c = cstr_to_str!(field, "Invalid UTF8 field string");
    let value_c = cstr_to_str!(value, "Invalid UTF8 value bytes");

    let mut rust_d = unsafe { Box::from_raw(dd) };
    rust_d.0 = std::mem::take(&mut rust_d.0).with_value(field_c, value_c);
    unsafe {
        *d = Box::into_raw(rust_d);
    }

    0
}

/// Frees the memory at *d, and sets *d to NULL.
///
/// # Safety
/// - Calling with null is safe.
///
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mokaccino_d_free(d: *mut *mut Document) {
    if d.is_null() {
        return;
    }

    let dd = unsafe { *d };
    if dd.is_null() {
        return;
    }

    unsafe {
        drop(Box::from_raw(dd));
        *d = std::ptr::null_mut();
    }
}
