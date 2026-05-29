static VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), "\0");

/// Returns the mokaccino_version string.
#[unsafe(no_mangle)]
pub extern "C" fn mokaccino_version() -> *const std::ffi::c_char {
    VERSION.as_ptr() as *const std::ffi::c_char
}
