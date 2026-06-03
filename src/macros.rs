macro_rules! cstr_to_str {
    ($ptr:expr, $err_msg:expr) => {
        match unsafe { std::ffi::CStr::from_ptr($ptr) }.to_str() {
            Ok(s) => s,
            Err(e) => {
                eprintln!("ERROR: {} {e:?}", $err_msg);
                return crate::MOKACCINO_ERROR;
            }
        }
    };
}
