use std::process::Command;
fn main() {
    let mut cmd = Command::new("echo");
    let mut c = cmd.arg("hello");
    let c = c.arg("world");
    c.status().unwrap();
}
