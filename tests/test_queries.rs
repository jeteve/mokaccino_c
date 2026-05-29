mod common;

#[test]
fn test_version() {
    common::assert_run_c("queries.c");
}
