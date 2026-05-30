use std::ffi::c_int;

mod queries;

/***
 * This is mokaccino, the percolator library.
 *
 * This C binding source code can be found here:
 * https://github.com/jeteve/mokaccino_c
 *
 *
 */

static VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), "\0");

/// A generic error return code for the C API.
#[unsafe(no_mangle)]
pub static MOKACCINO_ERROR: c_int = -1;

/// Returns the mokaccino_version string.
///
/// # Safety
/// Safe to display the returned *char as a plain ASCII string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mokaccino_version() -> *const std::ffi::c_char {
    VERSION.as_ptr() as *const std::ffi::c_char
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
