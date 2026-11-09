/// Placeholder returning the crate's name.
pub fn name() -> &'static str {
    "net"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_is_net() {
        assert_eq!(name(), "net");
    }
}
