fn reverse_string(s: &str) -> String {
    let mut reversed = String::new();
    for c in s.chars().rev() {
        reversed.push(c);
    }
    reversed
}

fn main() {
    let original = "Hello, world!";
    let reversed = reverse_string(original);
    println!("Original: {}", original);
    println!("Reversed: {}", reversed);
}
