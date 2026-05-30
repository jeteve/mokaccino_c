use std::ptr::null_mut;

static VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), "\0");

/// An opaque Query structure
pub struct Query(mokaccino::prelude::Query);

/// Returns the mokaccino_version string.
///
/// # Safety
/// Safe to display the returned *char as a plain ASCII string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mokaccino_version() -> *const std::ffi::c_char {
    VERSION.as_ptr() as *const std::ffi::c_char
}

/// Returns a debug C string representation of the query.
///
/// Do NOT use C-space free to deallocate the returned string.
///
/// # Safety
/// - `q` must be a valid pointer to a `Query`.
/// - The caller takes ownership of the returned string and must free it using `mokaccino_string_free(&s)`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mokaccino_q_debug(q: *const Query) -> *mut std::ffi::c_char {
    if q.is_null() {
        return std::ptr::null_mut();
    }
    let query = unsafe { &*q };
    let debug_str = format!("{:?}", query.0);
    match std::ffi::CString::new(debug_str) {
        Ok(c_str) => c_str.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Frees a C string previously returned by this library (for example `mokaccino_q_debug`).
///
/// This will make the passed pointer to *char null.
///
/// # Safety
/// - `s` must be a pointer previously returned by this library via `CString::into_raw()`.
/// - Passing a null pointer is allowed and is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mokaccino_string_free(s: *mut *mut std::ffi::c_char) {
    if s.is_null() {
        return;
    }
    if unsafe { *s }.is_null() {
        return;
    }

    unsafe {
        let _ = std::ffi::CString::from_raw(*s);
        *s = std::ptr::null_mut();
    }
}

unsafe fn build_two_qs<F>(q1: *mut *mut Query, q2: *mut *mut Query, builder: F) -> i32
where
    F: Fn(mokaccino::prelude::Query, mokaccino::prelude::Query) -> mokaccino::prelude::Query,
{
    if q1.is_null() || q2.is_null() {
        eprintln!("Either q1 or q2 is null");
        return -1;
    }

    let qq1 = unsafe { *q1 };
    let qq2 = unsafe { *q2 };

    if qq1.is_null() || qq2.is_null() {
        eprintln!("Either q1 or q2 points to a NULL *Query");
        return -1;
    }

    let bq1 = unsafe { Box::from_raw(qq1) };
    let bq2 = unsafe { Box::from_raw(qq2) };

    let new_q = builder(bq1.0, bq2.0);

    unsafe {
        *q1 = Box::into_raw(Box::new(Query(new_q)));
        *q2 = null_mut();
    }

    0
}

/// A logical AND between q1 and q2.
///
/// This consumes q1 and q2 and replaces q1 with the new Query*.
/// q2 is nullified.
///
///
/// # Safety
/// - q1, q2 must be not null and pointing to not null *Query values.
/// - If you copy and keep the value of q1 or q2 (of type Query*), they will be dandling pointers
///   after this operation.
///   
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mokaccino_q_and(q1: *mut *mut Query, q2: *mut *mut Query) -> i32 {
    unsafe { build_two_qs(q1, q2, |q1, q2| q1 & q2) }
}

/// A logical OR between q1 and q2.
///
/// This consumes q1 and q2 and replaces q1 with the new Query*.
/// q2 is nullified.
///
///
/// # Safety
/// - q1, q2 must be not null and pointing to not null *Query values.
/// - If you copy and keep the value of q1 or q2 (of type Query*), they will be dandling pointers
///   after this operation.
///   
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mokaccino_q_or(q1: *mut *mut Query, q2: *mut *mut Query) -> i32 {
    unsafe { build_two_qs(q1, q2, |q1, q2| q1 | q2) }
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

unsafe fn two_values_build<F>(
    q: *mut *mut Query,
    field: *const std::ffi::c_char,
    value: *const std::ffi::c_char,
    builder: F,
) -> i32
where
    F: Fn(&str, &str) -> mokaccino::prelude::Query,
{
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
        *q = Box::into_raw(Box::new(Query(builder(field_c, value_c))));
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
    unsafe {
        two_values_build(q, field, value, |f, v| {
            mokaccino::prelude::Query::term(f, v)
        })
    }
}

///
/// Returns -1 in case of error. Sets the given pointer otherwise.
///
/// # Safety
/// - q must be a valid pointer to a Query *
/// - field and value must be NULL terminated valid UTF8 bytes or an ASCII string.
///
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mokaccino_q_prefix(
    q: *mut *mut Query,
    field: *const std::ffi::c_char,
    value: *const std::ffi::c_char,
) -> i32 {
    unsafe {
        two_values_build(q, field, value, |f, v| {
            mokaccino::prelude::Query::prefix(f, v)
        })
    }
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
