pub fn max_palindrome() -> u32 {
    let mut max_pal = 0;
    let mut a = 999;
    while a >= 100 {
        let mut b;
        let db;
        if a % 11 == 0 {
            b = 999;
            db = 1;
        } else {
            b = 990;
            db = 11;
        }
        while b >= a {
            let prod = a * b;
            if prod <= max_pal {
                break;
            }
            if is_palindrome(prod) {
                max_pal = prod;
            }
            b -= db;
        }
        a -= 1;
    }
    return max_pal;
}

fn is_palindrome(num: u32) -> bool {
    let mut rev = 0;
    let mut n = num;
    while n > 0 {
        rev = rev * 10 + n % 10;
        n /= 10;
    }
    rev == num
}
