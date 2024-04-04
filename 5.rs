fn find_median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len == 0 {
        panic!("Empty array!");
    }

    if len % 2 == 0 {
        // If the array has an even number of elements, return the average of the middle two elements
        let mid_right = len / 2;
        let mid_left = mid_right - 1;
        return (arr[mid_left] as f64 + arr[mid_right] as f64) / 2.0;
    } else {
        // If the array has an odd number of elements, return the middle element
        let mid = len / 2;
        return arr[mid] as f64;
    }
}

fn main() {
    let arr1 = vec![1, 2, 3, 4, 5];
    let arr2 = vec![1, 2, 3, 4, 5, 6];
    println!("Median of arr1: {}", find_median(&arr1)); // 3
    println!("Median of arr2: {}", find_median(&arr2)); // 3.5
}
