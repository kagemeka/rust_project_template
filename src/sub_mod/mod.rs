mod another_mod;

#[cfg(test)]
mod tests {
    #[test]
    fn visibility() { super::another_mod::pub_in_super(); }
}
