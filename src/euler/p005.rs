pub fn smallest_multiple(max: u32) -> u32 {
    let mut i = 0;
    'a: loop {
        i += max;
        for d in (2..max).rev() {
            if i % d != 0 {
                continue 'a;
            }
        }
        return i as u32;
    }
}

fn prime_factors(n: u32) -> Vec<u32> {
    let mut result = vec![];
    let mut i = 2;
    let mut num = n;
    while num > 1 {
        while num % i == 0 {
            result.push(i);
            num /= i;
        }
        i += 1;
    }
    result
}

#[test]
fn prime_factors_of_four() {
    assert_eq!(prime_factors(4), [2, 2]);
}
