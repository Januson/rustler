struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Iterator for Fibonacci {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let new_next = self.curr + self.next;
        
        self.curr = self.next;
        self.next = new_next;
        
        Some(self.curr)
    }
}

pub fn sum_even_fib() ->u64 {
    return Fibonacci { curr: 1, next: 1}
        .take_while(|&x| x < 4_000_000)
        .filter(|&x| x % 2 == 0)
        .sum::<u64>();
}

#[test]
fn sum_even_up_to_4_000_000() {
    assert_eq!(sum_even_fib(), 4613732);
}

