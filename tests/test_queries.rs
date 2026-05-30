mod common;

#[test]
fn test_queries() {
    common::assert_run_c("queries.c");
}

#[test]
fn test_queries_num() {
    common::assert_run_c("queries_num.c");
}
