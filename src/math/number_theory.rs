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

/// 计算组合数 C(n, m)
/// NOTE: 请调用者自行保证数值不会溢出u64
pub fn comb(n: u64, m: u64) -> u64 {
    assert_eq!(n >= m, true);
    let mut ans: u64 = 1;
    let mut tmp: u64 = 1;
    for i in 1..=m.min(n - m) {
        ans = ans.checked_mul(n - i + 1).unwrap();
        tmp = tmp.checked_mul(i).unwrap();
    }
    return ans / tmp;
}

/// 计算组合数 C(n, m) % p
/// 每次计算时均单独计算逆元, 如需加速请使用`comb_fast`
pub fn comb_p(n: u64, m: u64, p: u64) -> u64 {
    <u64 as TryInto<u32>>::try_into(p).unwrap();
    assert_eq!(n >= m, true);
    let mut ans = 1;
    for i in 1..=m.min(n - m) {
        ans = ans * (n - i + 1) % p;
        ans = ans * quick_pow(i, p - 2, p) % p;
    }
    return ans;
}

static mut COMB_FAST_INV: Option<&mut Vec<u64>> = None;
/// 第一次执行时会预处理`1-n`在`p`下的逆元, 后续使用逆元加速运算.
/// NOTE: 由于预处理逆元, 因此**<调用者>**需要确保每次对`n`的调用一致
pub fn comb_fast(n: u64, m: u64, p: u64) -> u64 {
    <u64 as TryInto<u32>>::try_into(p).unwrap();
    assert_eq!(n >= m, true);
    unsafe {
        if COMB_FAST_INV.is_none() {
            COMB_FAST_INV = Some(Box::leak(Box::new(vec![0; (n + 1) as usize])));
            COMB_FAST_INV.as_mut().unwrap()[1] = 1;
            for i in 2..=n {
                COMB_FAST_INV.as_mut().unwrap()[i as usize] = quick_pow(i, p - 2, p);
            }
        }
    }
    let mut ans = 1;
    for i in 1..=m.min(n - m) {
        ans = ans * (n - i + 1) % p;
        ans = ans * unsafe {COMB_FAST_INV.as_ref().unwrap()[i as usize] } % p;
    }
    return ans;
}

/// 计算排列数 A(n, m) % p
pub fn perm(mut n: u64, m: u64, p: u64) -> u64 {
    assert_eq!(n >= m, true);
    let mut ans = 1;
    for _ in 1..=m {
        ans = ans * n % p;
        n -= 1;
    }
    return ans;
}

#[cfg(test)]
mod number_theory_tests {
    use super::*;

    #[test]
    fn comb_test_small() {
        for n in 1..=10 {
            for m in 1..=n {
                assert!(comb(n, m) == comb_p(n, m, 1e9 as u64 + 7));
            }
        }
    }

    #[test]
    #[should_panic]
    fn comb_test_too_big() {
        comb(10000, 5);
    }

    #[test]
    fn comb_test_p() {
        let p = 1e9 as u64 + 7;
        let n = 5000;
        for m in 1..=n {
            assert!(comb_fast(n, m, p) == comb_p(n, m, p), "Mismatch: n={}, m={}", n, m);
        }
    }

    #[test]
    #[should_panic]
    fn comb_test_p_bigger_than_u32() {
        comb_fast(10000, 5000, 1e18 as u64 + 7);
    }

}
