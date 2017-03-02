struct Prime {
    curr: u64,
}

impl Iterator for Prime {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        self.curr += 2;
        while !is_prime(&self.curr) {
            self.curr += 2;
        }

        Some(self.curr)
    }
}

fn primes() -> Prime {
    Prime { curr: 1 }
}

fn is_prime(x: &u64) -> bool {
    for i in 2..(x / 2 + 1) {
        if x % i == 0 {
            return false;
        }
    }
    true
}

pub fn largest_prime_factor(num: u64) -> u64 {
    let mut n: u64 = num;
    let mut max_factor: u64 = 1;

    if n % 2 == 0 {
        max_factor = 2;
        while n % 2 == 0 {
            n /= 2;
        }
    }

    for prime in primes() {
        while n % prime == 0 && n > 1 {
            if max_factor != prime {
                max_factor = prime;
            }
           n /= prime;
        }
        if n == 1 {
            break;
        }
    }
    max_factor
}
