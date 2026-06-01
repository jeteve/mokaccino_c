use std::ffi::c_int;

use super::MOKACCINO_ERROR;

pub struct Document(mokaccino::prelude::Document);

/// Builds an empty document given a **Document to a NULL *Document
///
/// # Safety
/// - d MUST be a NON-NULL **Document, pointing to a NULL *Document
/// - Use `mokacino_d_free` when you're done with the document.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mokaccino_d_new(d: *mut *mut Document) -> c_int {
    if d.is_null() {
        eprintln!("ERROR: given q pointer is null");
        return MOKACCINO_ERROR;
    }

    if !unsafe { *d }.is_null() {
        eprintln!(
            "ERROR: given q pointer is NOT a null *Document. Calling this would lead to a memory leak"
        );
        return MOKACCINO_ERROR;
    }

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
/// - *d is not NULL
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
/// - do not save the value d* yourself.
///
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mokaccino_d_add_value(
    d: *mut *mut Document,
    field: *const std::ffi::c_char,
    value: *const std::ffi::c_char,
) -> c_int {
    if d.is_null() {
        eprintln!("ERROR: given q pointer is null");
        return MOKACCINO_ERROR;
    }
    let dd = unsafe { *d };
    if dd.is_null() {
        eprintln!("ERROR: given q pointer points to a NULL *Document");
        return MOKACCINO_ERROR;
    }

    if field.is_null() || value.is_null() {
        eprintln!("ERROR: Either field or value is null");
        return MOKACCINO_ERROR;
    }

    let field_c = unsafe { std::ffi::CStr::from_ptr(field) }.to_str();
    if field_c.is_err() {
        eprintln!("ERROR: Invalid UTF8 field string {field_c:?}");
        return MOKACCINO_ERROR;
    }
    let field_c = field_c.unwrap();

    let value_c = unsafe { std::ffi::CStr::from_ptr(value) }.to_str();

    if value_c.is_err() {
        eprintln!("ERROR: Invalid UTF8 value bytes {value_c:?}");
        return MOKACCINO_ERROR;
    }
    let value_c = value_c.unwrap();

    let mut rust_d = unsafe { Box::from_raw(dd) };
    rust_d.0 = std::mem::take(&mut rust_d.0).with_value(field_c, value_c);
    unsafe {
        *d = Box::into_raw(rust_d);
    }

    0
}

/// Frees the memory at *Document, and sets *Document to NULL.
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
