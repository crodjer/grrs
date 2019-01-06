pub fn find_matches<'a, I: 'a>(lines: I, pattern: &'a str) -> impl Iterator<Item = String> + 'a
where
    I: Iterator<Item = String>,
{
    lines.filter(move |line| line.contains(pattern))
}

#[cfg(test)]
mod tests {
    use super::find_matches;

    #[test]
    fn test_find_matches() {
        let lines = ["A test", "Actual content", "More content", "Another test"]
            .iter()
            .map(|s| s.to_string());
        let results: Vec<_> = find_matches(lines, "test").collect();
        assert_eq!(results, ["A test", "Another test"]);
    }
}
