fn main() {
    let mut arr = [5, 2, 7, 1, 9, 10];

    println!("Original array: {:?}", arr);

    arr.sort();

    let ans;

    if arr.len() % 2 == 0 {
        let a = arr.len() / 2 - 1; // Corrected index calculation
        let b = arr.len() / 2;     // Corrected index calculation
        ans = (arr[a] + arr[b]) / 2;
    } else {
        ans = arr[arr.len() / 2];
    }

    println!("Median of the array is {}", ans);
}
