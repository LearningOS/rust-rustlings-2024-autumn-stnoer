//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // For tests7: Set up the `TEST_FOO` environment variable.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Set the `TEST_FOO` environment variable to the current timestamp
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // For tests8: Enable the "pass" feature to make the test return early.
    // This tells Cargo to enable a specific feature when compiling.
    println!("cargo:rustc-cfg=feature=\"pass\"");
}

