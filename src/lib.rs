use std::mem::take;

static VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), "\0");

pub struct Query(mokaccino::prelude::Query);

/// Returns the mokaccino_version string.
///
/// # Safety
/// Safe to display the returned *char as a plain ASCII string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mokaccino_version() -> *const std::ffi::c_char {
    VERSION.as_ptr() as *const std::ffi::c_char
}

///
/// # Safety
/// - q is not NULL
/// - *q is not NULL
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mokaccino_q_negation(q: *mut *mut Query) -> i32 {
    if q.is_null() {
        eprintln!("given q pointer is null");
        return -1;
    }

    let qq = unsafe { *q };
    if qq.is_null() {
        eprintln!("given q pointer points to a NULL *Query");
        return -1;
    }

    let qq = unsafe { Box::from_raw(qq) };
    let new_q = mokaccino::prelude::Query::negation((*qq).0);

    unsafe {
        *q = Box::into_raw(Box::new(Query(new_q)));
    }

    0
}

///
/// Returns -1 in case of error. Sets the given pointer otherwise.
///
/// # Safety
/// - q must be a valid pointer to a Query *
/// - field and value must be NULL terminated valid UTF8 bytes or an ASCII string.
///
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mokaccino_q_term(
    q: *mut *mut Query,
    field: *const std::ffi::c_char,
    value: *const std::ffi::c_char,
) -> i32 {
    if q.is_null() {
        eprintln!("given q pointer is null");
        return -1;
    }

    if field.is_null() || value.is_null() {
        eprintln!("Either field or value is null");
        return -1;
    }

    let field_c = unsafe { std::ffi::CStr::from_ptr(field) }.to_str();

    if field_c.is_err() {
        eprintln!("Invalid UTF8 field string {field_c:?}");
        return -1;
    }
    let field_c = field_c.unwrap();

    let value_c = unsafe { std::ffi::CStr::from_ptr(value) }.to_str();

    if value_c.is_err() {
        eprintln!("Invalid UTF8 value bytes {value_c:?}");
        return -1;
    }
    let value_c = value_c.unwrap();

    unsafe {
        *q = Box::into_raw(Box::new(Query(mokaccino::prelude::Query::term(
            field_c, value_c,
        ))));
    }

    0
}

///
/// # Safety
/// Calling with null is safe.
///
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mokaccino_q_free(q: *mut *mut Query) {
    if q.is_null() {
        return;
    }

    let qq = unsafe { *q };
    if qq.is_null() {
        return;
    }

    unsafe {
        drop(Box::from_raw(qq));
        *q = std::ptr::null_mut();
    }
}
