fn is_palindrome(s: &str) -> bool {
    // Convert the string to lowercase and remove whitespace
    let s = s.to_lowercase().chars().filter(|c| c.is_alphanumeric()).collect::<String>();

    // Check if the string is equal to its reverse
    s == s.chars().rev().collect::<String>()
}

fn main() {
    // Test cases
    println!("{}", is_palindrome("racecar")); // true
    println!("{}", is_palindrome("hello"));   // false
}
