use super::traits::*;

pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    return gcd(b, a % b);
}

pub fn exgcd(a: i64, b: i64) -> (i64, i64) {
    if b == 0 {
        return (1, 0);
    }
    let (x, y) = exgcd(b, a % b);
    return (y, x - a / b * y);
}

pub fn quick_pow<T>(a: T, mut b: u64, p: u32) -> T
where
    T: Mul<Output = T> + Rem<u64, Output = T> + One + Clone,
{
    let mut ans = a.one();
    let mut a = a.clone();
    while b > 0 {
        if b & 1 == 1 {
            ans = ans * a.clone() % p as u64;
        }
        a = a.clone() * a.clone() % p as u64;
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
            if i * ans[j] > n {
                break;
            }
            prime[(i * ans[j]) as usize] = false;
            if i % ans[j] == 0 {
                break;
            }
        }
    }
    return ans;
}

