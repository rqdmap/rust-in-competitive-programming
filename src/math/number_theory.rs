pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { return a; }
    return gcd(b, a % b);
}

pub fn exgcd(a: i64, b: i64) -> (i64, i64) {
    if b == 0 { return (1, 0); }
    let (x, y) = exgcd(b, a % b);
    return (y, x - a / b * y);
}

pub fn quick_pow(mut a: u64, mut b: u64, p: u64) -> u64 {
    let mut ans = 1;
    while b > 0 {
        if b & 1 == 1 { ans = ans * a % p; }
        a = a * a % p;
        b >>= 1;
    }
    return ans;
}

pub fn euler_prime(n: u32) -> Vec<u32> {
    let mut prime = vec![true; n as usize + 1];
    let mut ans = vec![];
    for i in 2..=n {
        if prime[i as usize] {
            ans.push(i);
        }
        for j in 0..ans.len() {
            if i * ans[j] > n { break; }
            prime[(i * ans[j]) as usize] = false;
            if i % ans[j] == 0 { break; }
        }
    }
    return ans;
}

/// 计算组合数 C(n, m) % p
/// 每次计算时均单独计算逆元, 如需加速请使用`combine_fast`
pub fn combine(n: u64, m: u64, p: u64) -> u64 {
    let mut ans = 1;
    for i in 1..=m.min(n - m) {
        ans = ans * (n - i + 1) % p;
        ans = ans * quick_pow(i, p - 2, p) % p;
    }
    return ans;
}

static mut __INV: Option<Vec<u64>> = None;
/// 第一次执行时会预处理`1-n`在`p`下的逆元, 后续使用逆元加速运算.
/// NOTE: 由于预处理逆元, 因此**<调用者>**需要确保每次对`n`的调用一致
pub fn combine_fast(n: u64, m: u64, p: u64) -> u64 {
    if unsafe { __INV.is_none() } {
        unsafe {
            __INV = Some(vec![0; (n + 1) as usize]);
            __INV.as_mut().unwrap()[1] = 1;
            for i in 2..=n {
                __INV.as_mut().unwrap()[i as usize] = quick_pow(i, p - 2, p);
            }
        }
    }
    
    assert_eq!(unsafe { __INV.is_none()}, false);
    let mut ans = 1;
    for i in 1..=m.min(n - m) {
        ans = ans * (n - i + 1) % p;
        ans = ans * unsafe { __INV.as_ref().unwrap()[i as usize] } % p;
    }
    return ans;
}
