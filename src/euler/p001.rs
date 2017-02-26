use std::collections::HashSet;

pub fn sum_multiples(divisors: Vec<i32>, limit: i32) -> i32 {
    let mut numbers = HashSet::new();
    for i in 1..limit {
        for divisor in divisors.iter() {
            if i % divisor == 0 {
                numbers.insert(i);    
            }
        }
    }
    return sum(numbers);
}

fn sum(numbers: HashSet<i32>) -> i32 {
    //let sum = numbers.iter().fold(0u64, |sum, val| sum += val);
    let sum = numbers.iter().sum::<i32>();
    return sum;
}

#[test]
fn sum_of_3_5_multiples_in_10() {
    assert_eq!(sum_multiples(vec!(3, 5), 10), 23);
}

#[test]
fn correct_answer() {
    assert_eq!(sum_multiples(vec!(3, 5), 1000), 233168);
}

