pub fn max_palindrome() -> u32 {
    for i in (0..1000).rev() {
        for j in (900..i).rev() {
            let x = i * j;
            if is_palindrome(&x.to_string()) {
                println!("{} * {} = {}", i, j, x);
                return x;
            }
        }
    }
    return 0;
}

fn is_palindrome(word: &str) -> bool {
    let drow = word.chars().rev().collect::<String>();
    word == drow
}
