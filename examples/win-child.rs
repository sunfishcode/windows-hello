#![windows_subsystem = "windows"]

use std::io::Write;
use std::os::windows::io::AsRawHandle;

fn main() {
    println!("Child: Hello world from println in the child!");
    writeln!(
        std::io::stdout(),
        "Hello world from write to stdout in the child!"
    )
    .unwrap();

    assert!(std::io::stdin().as_raw_handle().is_null());
    assert!(std::io::stdout().as_raw_handle().is_null());
    assert!(std::io::stderr().as_raw_handle().is_null());
}
