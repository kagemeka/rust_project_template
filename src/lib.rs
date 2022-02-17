//! This is a template crate for Rust project

/// This is a public function.
pub fn public_function() {
    println!("called `public_function()`");
}

// This is a private function.
// and won't be shown in the documentation on docs.rs.
#[allow(dead_code)]
fn private_function() {
    println!("called `private_function()`");
    println!(
        "called `private_function()` `private_function()` `private_function` `private_function()` `private_function()` `private_function()` `private_function()`"
    );
}

