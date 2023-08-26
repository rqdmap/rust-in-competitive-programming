pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { return a; }
    return gcd(b, a % b);
}

pub fn exgcd(a: i64, b: i64) -> (i64, i64) {
    if b == 0 { return (1, 0); }
    let (x, y) = exgcd(b, a % b);
    return (y, x - a / b * y);
}


