fn longest_common_prefix(strs: &[String]) -> String {
    if strs.is_empty() {
        return String::new();
    }

    let mut prefix = String::new();
    let first_str = &strs[0];

    for (i, c) in first_str.chars().enumerate() {
        for s in strs.iter().skip(1) {
            if let Some(ch) = s.chars().nth(i) {
                if ch != c {
                    return prefix;
                }
            } else {
                return prefix;
            }
        }
        prefix.push(c);
    }

    prefix
}

fn main() {
    let strings = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    ];
    println!("Longest common prefix: {}", longest_common_prefix(&strings)); // Output: "fl"
}
