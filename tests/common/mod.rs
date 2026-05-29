use std::env;
use std::path::PathBuf;
use std::process::Command;

pub(crate) fn assert_run_c(c_prog: &str) {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // Paths to our source and header files
    let include_dir = PathBuf::from(&manifest_dir).join("target").join("include");
    let test_c_path = PathBuf::from(&manifest_dir)
        .join("tests")
        .join("c")
        .join(c_prog);
    let out_exe_path = PathBuf::from(&manifest_dir).join("target").join("test_app");

    // Dynamically find out if Cargo is building in debug or release mode
    let mut target_dir = env::current_exe().unwrap();
    target_dir.pop(); // Remove the test executable name
    if target_dir.ends_with("deps") {
        target_dir.pop(); // Step out of the "deps" directory
    }

    // 1. Compile the C program
    let compile_status = Command::new("gcc")
        .arg(&test_c_path)
        .arg("-o")
        .arg(&out_exe_path)
        .arg(format!("-I{}", include_dir.display()))
        .arg(format!("-L{}", target_dir.display()))
        .arg("-lmokaccino_c")
        .arg("-lpthread")
        .arg("-ldl")
        .status()
        .expect("Failed to execute gcc compiler. Is gcc installed?");

    assert!(
        compile_status.success(),
        "Failed to compile {test_c_path:?}"
    );

    // 2. Run the compiled C executable
    let run_status = Command::new(&out_exe_path)
        .env("LD_LIBRARY_PATH", &target_dir)
        .status()
        .expect("Failed to run the compiled C program");

    assert!(
        run_status.success(),
        "The compiled C program from {test_c_path:?} failed to run or crashed"
    );
}
