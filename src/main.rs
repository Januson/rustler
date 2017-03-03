mod euler;

use std::fmt::Display;
use euler::p001::sum_multiples;
use euler::p002::sum_even_fib;
use euler::p003::largest_prime_factor;
use euler::p004::max_palindrome;
use euler::p005::smallest_multiple;

fn main() {
    print_solutions();
}


fn print_solutions() {
    print_problem(1, "Multiples of 3 and 5", sum_multiples(1000));
    print_problem(2, "Even Fibonacci numbers", sum_even_fib());
    print_problem(3, "Largest prime factor", largest_prime_factor(600851475143));
    print_problem(4, "Largest palindrome product", max_palindrome());
    print_problem(5, "Smallest multiple from 1 to 20", smallest_multiple());

}

fn print_problem<N: Display>(index: i16, name: &str, solution: N) {
    println!("| {} | {} | {} |", index, name, solution);
}
