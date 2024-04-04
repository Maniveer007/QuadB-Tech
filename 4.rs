fn is_prime(n: u64) -> bool {
    // Check if the number is less than 2
    if n < 2 {
        return false;
    }

    // Check for divisibility by numbers from 2 to the square root of n
    for i in 2..=((n as f64).sqrt() as u64) {
        if n % i == 0 {
            return false;
        }
    }

    true
}

fn main() {
    // Test cases
    println!("{}", is_prime(7));   // true
    println!("{}", is_prime(10));  // false
    println!("{}", is_prime(23));  // true
    println!("{}", is_prime(1));   // false
}
