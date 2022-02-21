pub(self) mod private_mod {
    pub(super) mod pub_in_super {
        pub(in crate::another_mod) fn pub_in_file() {
            println!("called `pub_in_super::pub_in_file()`");
        }

        pub fn pub_in_anywhere() {
            println!("called `pub_in_super::pub_in_anywhere()`");
        }
    }

    /// pub_in_user::pub_in_anywhere is visible anywhere.
    /// it's visible in user code if and only if
    /// all the ancestor modules are also visible in user code.
    pub use pub_in_super::pub_in_anywhere;

    // compile error because pub_in_file is public in file but
    // in private_mod.
    // pub use pub_in_super::pub_in_file;
}

#[allow(dead_code)]
pub(crate) fn pub_in_crate() {
    private_mod::pub_in_super::pub_in_file();
    self::private_mod::pub_in_super::pub_in_file();
    super::another_mod::private_mod::pub_in_super::pub_in_file();
    crate::another_mod::private_mod::pub_in_super::pub_in_file();
    self::private_mod::pub_in_super::pub_in_anywhere();
    self::private_mod::pub_in_anywhere();
    super::private_function();
}

/// tests should be private in moodule.
#[cfg(test)]
pub(self) mod tests {
    // use super::eat_at_restaurant;

    #[test]
    fn visibility() {
        use super::pub_in_crate;
        pub_in_crate();
        // this function is  public in super because it's
        // public in crate.
    }
}
