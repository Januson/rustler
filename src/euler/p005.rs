pub fn smallest_multiple() -> u32 {
    let mut i = 25;
    'a: loop {
        i += 1;
        for d in 2..21 {
            if i % d != 0 {
                continue 'a;
            }
        }
        return i as u32;
    }
}
