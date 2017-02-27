pub fn sum_multiples(limit: i32) -> i32 {
    let mut sum: i32 = 0;
    for i in 1..limit {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    return sum;
}


#[test]
fn sum_up_to_10() {
    assert_eq!(sum_multiples(10), 23);
}

#[test]
fn sum_up_to_1000() {
    assert_eq!(sum_multiples(1000), 233168);
}

