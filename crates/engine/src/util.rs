pub fn strip_text_indent(s: &str) -> String {
    s.lines()
        .map(|line| {
            let result = if line.starts_with('\n') {
                line.strip_prefix('\n').unwrap().trim()
            } else {
                line.trim()
            };

            if result.len() == line.len() {
                result.to_string()
            } else {
                format!(" {result}")
            }
        })
        .collect()
}
