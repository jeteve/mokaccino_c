use std::ffi::{CString, c_char, c_int};
use std::time::Instant;

enum Document {}

#[link(name = "mokaccino")]
unsafe extern "C" {
    fn mokaccino_d_new(d: *mut *mut Document) -> c_int;
    fn mokaccino_d_add_value(
        d: *mut *mut Document,
        field: *const c_char,
        value: *const c_char,
    ) -> c_int;
    fn mokaccino_d_debug(d: *const Document) -> *mut c_char;
    fn mokaccino_d_free(d: *mut *mut Document);
    fn mokaccino_string_free(s: *mut *mut c_char);
}

#[test]
fn bench_d_debug() {
    let mut doc_ptr: *mut Document = std::ptr::null_mut();
    unsafe {
        mokaccino_d_new(&mut doc_ptr);
        let f1 = CString::new("field1").unwrap();
        let v1 = CString::new("value1").unwrap();
        let f2 = CString::new("field2").unwrap();
        let v2 = CString::new("value2").unwrap();
        let f3 = CString::new("field3").unwrap();
        let v3 = CString::new("value3").unwrap();

        mokaccino_d_add_value(&mut doc_ptr, f1.as_ptr(), v1.as_ptr());
        mokaccino_d_add_value(&mut doc_ptr, f2.as_ptr(), v2.as_ptr());
        mokaccino_d_add_value(&mut doc_ptr, f3.as_ptr(), v3.as_ptr());
    }

    let iterations = 200_000;
    let start = Instant::now();
    for _ in 0..iterations {
        unsafe {
            let str_ptr = mokaccino_d_debug(doc_ptr);
            let mut p = str_ptr;
            mokaccino_string_free(&mut p);
        }
    }
    let duration = start.elapsed();
    println!(
        "Benchmark `mokaccino_d_debug`: {:?} for {} iterations ({:.2?} / op)",
        duration,
        iterations,
        duration / iterations
    );

    unsafe {
        mokaccino_d_free(&mut doc_ptr);
    }
}
