pub(self) mod private_mod {
    pub(super) mod pub_in_super {
        pub(in crate::another_mod) fn pub_in_file() {
            println!("called `pub_in_super::pub_in_file()`");
        }
    }
}

#[allow(dead_code)]
pub(crate) fn pub_in_crate() {
    private_mod::pub_in_super::pub_in_file();
    self::private_mod::pub_in_super::pub_in_file();
    super::another_mod::private_mod::pub_in_super::pub_in_file();
    crate::another_mod::private_mod::pub_in_super::pub_in_file();
    super::private_function();
}

#[cfg(test)]
mod tests {
    // use super::eat_at_restaurant;

    #[test]
    fn visibility() {
        super::pub_in_crate();
        // this function is  public in super because it's
        // public in crate.
    }
}
