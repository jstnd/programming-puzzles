pub fn extract_integer(input: &str) -> usize {
    let output: String = input.chars().filter(|c| c.is_numeric()).collect();

    output.parse().unwrap()
}
