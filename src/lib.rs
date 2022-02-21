//! This is a template crate for Rust project
//! `//!` is a enclosing doc comment for the whole crate.

pub mod sample_mod {
    //! `//!` comment can be used for the module.

    /// `///` is a doc comment for the function, struct, or
    /// trait ...
    pub fn public_fn_in_mod(arg: i64) {
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

/// `pub` and `pub(crate)` is different.
/// `pub` is visible in user code,
/// `pub(crate)` is only visible in this crate.
/// thus `pub(crate)` item won't be shown in https://docs.rs
pub mod another_mod;
pub(crate) mod sub_mod;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn access_to_child_mod() {
        super::another_mod::pub_in_crate();
        // super means the parent of `tests` module.

        // super::sub_mod::another_mod::pub_in_super();
        // compile error because another_mod is only public in
        // sub_mod;
    }
}
