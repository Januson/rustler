mod euler;

use euler::p001::sum_multiples;
use euler::p002::sum_even_fib;


fn main() {
    println!("{}", sum_even_fib());
    print_results(sum_multiples(1000));
}

fn print_results(sum: i32) {
    println!("{}", sum);
}
