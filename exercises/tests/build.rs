//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

// use std::env;
use std::time::SystemTime;

fn main() {
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);
    println!("cargo:rustc-cfg=pass");
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
