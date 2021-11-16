#![windows_subsystem = "windows"]

use std::io::Write;
use std::os::windows::io::AsRawHandle;

fn main() {
    println!("Hello world from println!");
    writeln!(std::io::stdout(), "Hello world from write to stdout!").unwrap();

    assert!(std::io::stdin().as_raw_handle().is_null());
    assert!(std::io::stdout().as_raw_handle().is_null());
    assert!(std::io::stderr().as_raw_handle().is_null());

    // Spawn a child *without* `windows_subsystem "windows"`.
    let mut child = std::process::Command::new("cargo")
        .args(["run", "--example", "child"])
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .unwrap();
    assert!(child.stdin.is_none());
    assert!(child.stdout.is_none());
    assert!(child.stderr.is_none());
    child.wait().unwrap();

    // Spawn a child *without* `windows_subsystem "windows"`, with pipes.
    let mut child = std::process::Command::new("cargo")
        .args(["orange", "--example", "child"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    child.wait().unwrap();

    // Spawn a child *with* `windows_subsystem "windows"`.
    let mut child = std::process::Command::new("cargo")
        .args(["run", "--example", "win-child"])
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .unwrap();
    assert!(child.stdin.is_none());
    assert!(child.stdout.is_none());
    assert!(child.stderr.is_none());
    child.wait().unwrap();

    // Spawn a child *with* `windows_subsystem "windows"`, with pipes.
    let mut child = std::process::Command::new("cargo")
        .args(["orange", "--example", "win-child"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    child.wait().unwrap();
}
