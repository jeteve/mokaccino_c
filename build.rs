use std::env;
use std::fs;

fn main() {
    println!("cargo:rerun-if-changed=README.md");
    println!("cargo:rerun-if-changed=LICENSE");

    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let readme_path = std::path::Path::new(&crate_dir).join("README.md");
    let readme_content = fs::read_to_string(readme_path).expect("Unable to read README.md");

    let license_content = fs::read_to_string(std::path::Path::new(&crate_dir).join("LICENSE"))
        .expect("Unable to read LICENSE file");
    let header_comment = format!(
        "/*\n{}\n\n#LICENSE\n\n{}\n*/",
        readme_content, license_content
    );

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::C)
        .with_include_guard("MOKACCINO_H")
        .with_header(header_comment)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("target/include/mokaccino.h");
}
