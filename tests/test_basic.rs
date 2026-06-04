mod common;

#[test]
fn test_version() {
    common::assert_run_c("version.c");
}

#[test]
#[should_panic]
fn test_leaker() {
    common::assert_run_c("leaker.c");
}
