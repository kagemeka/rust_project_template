#[allow(dead_code)]
pub(super) fn pub_in_super() {
    println!("called `pub_in_super()`");
}

#[cfg(test)]
mod tests {
    #[test]
    fn visibility() { super::pub_in_super(); }
}
