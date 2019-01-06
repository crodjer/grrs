// TODO: Genrate an iterator instead of Vec
pub fn find_matches<I>(lines: I, pattern: &str) -> Vec<String>
where
    I: Iterator<Item = String>,
{
    lines.filter(|line| line.contains(&pattern)).collect()
}
