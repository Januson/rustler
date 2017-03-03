pub fn smallest_multiple(max: u32) -> u32 {
    let mut i = 0;
    'a: loop {
        i += max;
        for d in (2..max).rev() {
            if i % d != 0 {
                continue 'a;
            }
        }
        return i as u32;
    }
}
