pub fn sum_multiples(limit: u32) -> u32 {
    (1..limit)
        .filter(|x| x % 3 == 0 || x % 5 == 0)
        .sum::<u32>()
}


#[test]
fn sum_up_to_10() {
    assert_eq!(sum_multiples(10), 23);
}

#[test]
fn sum_up_to_1000() {
    assert_eq!(sum_multiples(1000), 233168);
}

