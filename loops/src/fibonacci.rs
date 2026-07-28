pub fn fibonacci(n: u32) -> u128 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut i: u32 = 2;
            let mut a: u128 = 1;
            let mut b: u128 = 1;
            while i < n {
                let sum = a + b;
                a = b;
                b = sum;
                i += 1;
            }
            b
        }
    }
}
