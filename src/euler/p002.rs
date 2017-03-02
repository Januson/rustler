//#![feature(test)]

//extern crate test;

struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Iterator for Fibonacci {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let result = self.curr;
	let new_next = self.curr + 4 * self.next; 
        self.curr = self.next;
        self.next = new_next;
        
        Some(result)
    }
}

pub fn sum_even_fib() ->u64 {
    return Fibonacci { curr: 2, next: 8}
        .take_while(|&x| x < 4_000_000)
        .sum::<u64>();
}

#[cfg(test)]
mod tests {
    use super::*;
//    use test::Bencher;

    #[test]
    fn sum_even_up_to_4_000_000() {
        assert_eq!(sum_even_fib(), 4613732);
    }

//    #[bench]
//    fn bench_sum_even_up_to_4_000_000(b: &mut Bencher) {
//        b.iter(|| sum_even_fib());
//    }    
}
