pub fn sum_square(n: u32) -> u32 {
    let mut sum_squares = 0;
    let sum_n = n * (n + 1) / 2;
    for i in 1..(n + 1) {
        sum_squares += i * i;
    }
    sum_n * sum_n - sum_squares
}
