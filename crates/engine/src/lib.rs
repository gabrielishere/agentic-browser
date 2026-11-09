/// Placeholder returning the crate's name.
pub fn name() -> &'static str {
    "engine"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_is_engine() {
        assert_eq!(name(), "engine");
    }
}
