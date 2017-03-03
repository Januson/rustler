mod euler;

use euler::p001::sum_multiples;
use euler::p002::sum_even_fib;
use euler::p003::largest_prime_factor;
use euler::p004::max_palindrome;

fn main() {
    println!("{}", max_palindrome());
    println!("{}", largest_prime_factor(600851475143));
    println!("{}", sum_even_fib());
    print_results(sum_multiples(1000));
}

fn print_results(sum: i32) {
    println!("{}", sum);
}
