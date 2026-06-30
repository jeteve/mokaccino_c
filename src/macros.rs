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

macro_rules! ensure_ptr_not_null {
    ($ptr:expr, $msg:expr) => {
        if $ptr.is_null() {
            eprintln!("ERROR: {}", $msg);
            return crate::MOKACCINO_ERROR;
        }
    };
}

macro_rules! ensure_ptr_ptr_not_null {
    ($ptr:expr, $ptr_name:expr, $type_name:expr) => {{
        if $ptr.is_null() {
            eprintln!("ERROR: given {} pointer is null", $ptr_name);
            return crate::MOKACCINO_ERROR;
        }
        let qq = unsafe { *$ptr };
        if qq.is_null() {
            eprintln!(
                "ERROR: given {} pointer points to a NULL *{}",
                $ptr_name, $type_name
            );
            return crate::MOKACCINO_ERROR;
        }
        qq
    }};
}

macro_rules! ensure_ptr_ptr_is_null {
    ($ptr:expr, $ptr_name:expr, $type_name:expr) => {
        if $ptr.is_null() {
            eprintln!("ERROR: given {} pointer is null", $ptr_name);
            return crate::MOKACCINO_ERROR;
        }
        if !unsafe { *$ptr }.is_null() {
            eprintln!(
                "ERROR: given {} pointer is NOT a null *{}. Calling this would lead to a memory leak",
                $ptr_name, $type_name
            );
            return crate::MOKACCINO_ERROR;
        }
    };
}
