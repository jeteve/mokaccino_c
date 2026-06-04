use std::env;
use std::path::PathBuf;
use std::process::Command;

pub(crate) fn assert_run_c(c_prog: &str) {
    let has_valgrind = Command::new("valgrind")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    let has_leaks = Command::new("leaks")
        .arg("-h")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // Paths to our source and header files
    let include_dir = PathBuf::from(&manifest_dir).join("target").join("include");
    let test_c_path = PathBuf::from(&manifest_dir)
        .join("tests")
        .join("c")
        .join(c_prog);

    // Remove extension from c_prog.
    let c_prog_bin = c_prog
        .strip_suffix(".c")
        .expect("{c_prog:?} does not end with .c");

    let out_exe_path = PathBuf::from(&manifest_dir).join("target").join(c_prog_bin);

    // Dynamically find out if Cargo is building in debug or release mode
    let mut target_dir = env::current_exe().unwrap();
    target_dir.pop(); // Remove the test executable name
    if target_dir.ends_with("deps") {
        target_dir.pop(); // Step out of the "deps" directory
    }
    println!("Linking into {target_dir:?}");

    // 1. Compile the C program
    let mut clang_cmd = Command::new("clang");
    clang_cmd
        .arg(&test_c_path)
        .arg("-o")
        .arg(&out_exe_path)
        .arg(format!("-I{}", include_dir.display()))
        .arg(format!("-L{}", target_dir.display()))
        .arg("-lmokaccino");

    let compile_status = clang_cmd
        .status()
        .expect("Failed to execute clang compiler. Is gcc installed?");

    assert!(
        compile_status.success(),
        "Failed to compile {test_c_path:?}"
    );

    // 2. Run the compiled C executable
    let run_status = Command::new(&out_exe_path)
        .env("LD_LIBRARY_PATH", &target_dir)
        .status()
        .expect("Failed to run the compiled C program {out_exe_path:?}");

    assert!(
        run_status.success(),
        "The compiled C program {out_exe_path:?} from {test_c_path:?} failed to run or crashed: {run_status}"
    );

    // 3. Check for memory leaks
    if has_valgrind {
        // Inspired by https://github.com/cathay4t/librabc/blob/main/Makefile
        let run_status = Command::new("valgrind")
            .arg("--trace-children=yes")
            .arg("--leak-check=full")
            .arg("--show-leak-kinds=all")
            .arg("--error-exitcode=1")
            .arg(&out_exe_path)
            .env("LD_LIBRARY_PATH", &target_dir)
            .status()
            .expect("Failed to run valgrind. Is valgrind installed and in the PATH?");

        assert!(
            run_status.success(),
            "The compiled C program {out_exe_path:?} from {test_c_path:?} failed to pass memory leak status: {run_status}"
        );
    }

    if has_leaks {
        let run_status = Command::new("leaks")
            .arg("--atExit")
            .arg("--")
            .arg(&out_exe_path)
            .env("DYLD_LIBRARY_PATH", &target_dir)
            .status()
            .expect("Failed to run leaks. Is leaks installed and in the PATH?");

        assert!(
            run_status.success(),
            "The compiled C program {out_exe_path:?} from {test_c_path:?} failed to pass leaks status: {run_status}"
        );
    }
}
