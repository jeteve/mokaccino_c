use h3o::CellIndex;
use mokaccino::prelude::*;
use std::{ffi::c_int, ptr::null_mut};

use super::MOKACCINO_ERROR;

/// An opaque Query structure
pub struct Query(pub(crate) mokaccino::prelude::Query);

/// Returns a debug C string representation of the query.
///
/// Do NOT use C-space `free` to deallocate the returned string.
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

/// Returns a human friendly C string representation of the query.
///
/// Do NOT use C-space `free` to deallocate the returned string.
///
/// # Safety
/// - `q` must be a valid pointer to a `Query`.
/// - The caller takes ownership of the returned string and must free it using `mokaccino_string_free(&s)`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mokaccino_q_tostring(q: *const Query) -> *mut std::ffi::c_char {
    if q.is_null() {
        return std::ptr::null_mut();
    }
    let query = unsafe { &*q };
    match std::ffi::CString::new(query.0.to_string()) {
        Ok(c_str) => c_str.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

unsafe fn build_two_qs<F>(q1: *mut *mut Query, q2: *mut *mut Query, builder: F) -> c_int
where
    F: Fn(mokaccino::prelude::Query, mokaccino::prelude::Query) -> mokaccino::prelude::Query,
{
    if q1.is_null() || q2.is_null() {
        eprintln!("Either q1 or q2 is null");
        return MOKACCINO_ERROR;
    }

    let qq1 = unsafe { *q1 };
    let qq2 = unsafe { *q2 };

    if qq1.is_null() || qq2.is_null() {
        eprintln!("Either q1 or q2 points to a NULL *Query");
        return MOKACCINO_ERROR;
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
pub unsafe extern "C" fn mokaccino_q_and(q1: *mut *mut Query, q2: *mut *mut Query) -> c_int {
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
pub unsafe extern "C" fn mokaccino_q_or(q1: *mut *mut Query, q2: *mut *mut Query) -> c_int {
    unsafe { build_two_qs(q1, q2, |q1, q2| q1 | q2) }
}

///
/// # Safety
/// - q is not NULL
/// - *q is not NULL
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mokaccino_q_negation(q: *mut *mut Query) -> c_int {
    if q.is_null() {
        eprintln!("ERROR: given q pointer is null");
        return MOKACCINO_ERROR;
    }

    let qq = unsafe { *q };
    if qq.is_null() {
        eprintln!("ERROR: given q pointer points to a NULL *Query");
        return MOKACCINO_ERROR;
    }

    let qq = unsafe { Box::from_raw(qq) };
    let new_q = mokaccino::prelude::Query::negation((*qq).0);

    unsafe {
        *q = Box::into_raw(Box::new(Query(new_q)));
    }

    0
}

unsafe fn field_int_build<F>(
    q: *mut *mut Query,
    field: *const std::ffi::c_char,
    value: i64,
    builder: F,
) -> c_int
where
    F: Fn(&str, i64) -> mokaccino::prelude::Query,
{
    if q.is_null() {
        eprintln!("ERROR: given q pointer is null");
        return MOKACCINO_ERROR;
    }

    let qq = unsafe { *q };
    if !qq.is_null() {
        eprintln!(
            "ERROR: given q pointer is NOT a null *Query. Calling this would lead to a memory leak"
        );
        return MOKACCINO_ERROR;
    }

    if field.is_null() {
        eprintln!("ERROR: Field is null");
        return MOKACCINO_ERROR;
    }

    let field_c = unsafe { std::ffi::CStr::from_ptr(field) }.to_str();
    if field_c.is_err() {
        eprintln!("ERROR: Invalid UTF8 field string {field_c:?}");
        return MOKACCINO_ERROR;
    }
    let field_c = field_c.unwrap();

    unsafe {
        *q = Box::into_raw(Box::new(Query(builder(field_c, value))));
    }

    0
}

/// A query where the field `k` as an integer must be lower than the given `value`
///
/// # Safety
/// - q must be a valid Query** pointing to a NULL Query* to avoid memory leaks.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mokaccino_q_klt(
    q: *mut *mut Query,
    field: *const std::ffi::c_char,
    value: i64,
) -> c_int {
    unsafe { field_int_build(q, field, value, |f, v| f.i64_lt(v)) }
}

/// A query where the field `k` as an integer must be lower than or equal to the given `value`
///
/// # Safety
/// - q must be a valid Query** pointing to a NULL Query* to avoid memory leaks.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mokaccino_q_kle(
    q: *mut *mut Query,
    field: *const std::ffi::c_char,
    value: i64,
) -> c_int {
    unsafe { field_int_build(q, field, value, |f, v| f.i64_le(v)) }
}

/// A query where the field `k` as an integer must be greater than the given `value`
///
/// # Safety
/// - q must be a valid Query** pointing to a NULL Query* to avoid memory leaks.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mokaccino_q_kgt(
    q: *mut *mut Query,
    field: *const std::ffi::c_char,
    value: i64,
) -> c_int {
    unsafe { field_int_build(q, field, value, |f, v| f.i64_gt(v)) }
}

/// A query where the field `k` as an integer must be greater than or equal to the given `value`
///
/// # Safety
/// - q must be a valid Query** pointing to a NULL Query* to avoid memory leaks.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mokaccino_q_kge(
    q: *mut *mut Query,
    field: *const std::ffi::c_char,
    value: i64,
) -> c_int {
    unsafe { field_int_build(q, field, value, |f, v| f.i64_ge(v)) }
}

/// A query where the field `k` as an integer must be equal to the given `value`
///
/// # Safety
/// - q must be a valid Query** pointing to a NULL Query* to avoid memory leaks.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mokaccino_q_keq(
    q: *mut *mut Query,
    field: *const std::ffi::c_char,
    value: i64,
) -> c_int {
    unsafe { field_int_build(q, field, value, |f, v| f.i64_eq(v)) }
}

unsafe fn two_strings_build<F>(
    q: *mut *mut Query,
    field: *const std::ffi::c_char,
    value: *const std::ffi::c_char,
    builder: F,
) -> c_int
where
    F: Fn(&str, &str) -> mokaccino::prelude::Query,
{
    if q.is_null() {
        eprintln!("ERROR: given q pointer is null");
        return MOKACCINO_ERROR;
    }

    let qq = unsafe { *q };
    if !qq.is_null() {
        eprintln!(
            "ERROR: given q pointer is NOT a null *Query. Calling this would lead to a memory leak"
        );
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

    unsafe {
        *q = Box::into_raw(Box::new(Query(builder(field_c, value_c))));
    }

    0
}

/// Builds a simple term query `field=value`
///
///
/// Returns MOKACCINO_ERROR in case of error. Sets the given pointer otherwise.
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
) -> c_int {
    unsafe { two_strings_build(q, field, value, |f, v| f.has_value(v)) }
}

/// Builds a query where when the field is a value H3 cell index, the document values
/// are checked for their inclusion in the given H3 index.
///
/// See (h3 documentation)[https://h3geo.org/].
///
/// Falls back to a plain term query if the give H3 index is not correct.
///
/// Returns MOKACCINO_ERROR in case of error. Sets the given pointer otherwise.
///
/// # Safety
/// - q must be a valid pointer to a Query *
/// - field and value must be NULL terminated valid UTF8 bytes or an ASCII string.
///
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mokaccino_q_h3in(
    q: *mut *mut Query,
    field: *const std::ffi::c_char,
    value: *const std::ffi::c_char,
) -> c_int {
    let builder = |f: &str, v: &str| {
        // Try building an h3 cell index. Fallback to a plain term query if invalid.
        if let Ok(ci) = v.parse::<CellIndex>() {
            f.h3in(ci)
        } else {
            eprintln!("WARNING: Given value {v} is not a correct H3 Cell Index.");
            f.has_value(v)
        }
    };

    unsafe { two_strings_build(q, field, value, builder) }
}

///
/// Returns MOKACCINO_ERROR in case of error. Sets the given pointer otherwise.
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
) -> c_int {
    unsafe { two_strings_build(q, field, value, |f, v| f.has_prefix(v)) }
}

/// Frees the memory at *Query, and sets *Query to NULL.
///
/// # Safety
/// - Calling with null is safe.
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
