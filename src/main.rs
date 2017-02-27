mod euler;

use euler::p001::sum_multiples;

fn main() {
    print_results(sum_multiples(1000));
}

fn print_results(sum: i32) {
    println!("{}", sum);
}
