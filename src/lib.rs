//! This is a template crate for Rust project
//! `//!` is a enclosing doc comment for the whole crate.

pub mod sample_module {
    //! `//!` comment can be used for the module.

    /// `///` is a doc comment for the function, struct, or
    /// trait ...
    pub fn public_fn_in_module(arg: i64) {
        // `//` comment is not a doc comment.
        println!(
            "called `sample_module::public_fn_in_module()`, arg: {}",
            arg
        );
    }
}

/// This is a public function.
pub fn public_function() {
    println!("called `public_function()`");
}

/// This is a private function.
/// and won't be shown in the documentation on docs.rs.
#[allow(dead_code)]
fn private_function() {
    println!("called `private_function()`",);
    println!(
        "called `private_function()` `private_function()` `private_function` \
         `private_function()` `private_function()` `private_function()` \
         `private_function()`"
    );
}
