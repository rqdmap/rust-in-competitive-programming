use super::*;

/// 带有预处理的阶乘
///
/// # Usage
/// ```
/// use rust_in_competitive_programming::Fac;
/// // 预处理1-1_000_000的阶乘及逆元, 对1_000_000_007取模
/// let fac = Fac::new(1_000_000, 1_000_000_007);
///
/// fac.get(100);            // 100! mod p
/// fac.get_inv(100);            // 1/100! mod p
/// fac.get_range(50, 100);  // 50*...*100 mod p
/// ```
pub struct Fac {
    n: u32, 
    fac: Vec<u32>,
    p: u32,
    inv: Vec<u32>,
}

impl Fac {
    /// Initialize using `Fac::new (n, p)`
    ///
    /// result will be mod p and inv will be calculated
    ///     Time complexity: 带模乘法的O(n)
    pub fn new(n: u32, p: u32) -> Self {
        // 0!=1
        let mut fac: Vec<u32> = vec![1; n as usize + 1];

        for i in 1..=n {
            fac[i as usize]= (fac[i as usize - 1] as u64 * i as u64 % p as u64).try_into().unwrap();
        }

        let n: usize = n.try_into().unwrap();
        let mut inv = vec![1; n + 1];
        inv[n] = quick_pow(fac[n] as u64, (p - 2) as u64, p).try_into().unwrap();
        for i in (2..=n).rev() {
            inv[i - 1] = (inv[i] as u64 * i as u64 % p as u64).try_into().unwrap();
        }

        return Fac { n: n.try_into().unwrap(), fac, p, inv, };
    }

    /// return k! mod p
    pub fn get(&self, k: u32) -> u32 {
        return self.fac[k as usize];
    }
    
    /// return 1/k! mod p
    pub fn get_inv(&self, k: u32) -> u32 {
        return self.inv[k as usize];
    }

    /// return l * (l+1) * ... * (r - 1) * r mod p
    pub fn get_range(&self, l: u32, r: u32) -> u32 {
        assert!(r >= l && r <= self.n && l >= 1, "l = {}, r = {}, n = {}", l, r, self.n);
        return (self.fac[r as usize] as u64 * self.inv[l as usize - 1] as u64 % self.p as u64)
            .try_into()
            .unwrap();
    }
}

/// 计算组合数 C(n, m)
/// NOTE: 溢出u64时直接panic
pub fn comb_raw(n: u64, m: u64) -> u64 {
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
/// NOTE: 每次调用时均重新计算逆元, 如需加速请使用`Comb`类
pub fn comb(n: u64, m: u64, p: u32) -> u32 {
    assert_eq!(n >= m, true);
    let mut ans = 1;
    for i in 1..=m.min(n - m) {
        ans = ans * (n - i + 1) % p as u64;
        ans = ans * quick_pow(i, (p - 2) as u64, p) % p as u64;
    }
    return ans.try_into().unwrap();
}

pub struct Comb {
    n: u32,
    p: u32,
    fac: Fac
}

impl Comb {
    pub fn new(n: u32, p: u32) -> Self {
        return Comb { n, p, fac: Fac::new(n.try_into().unwrap(), p) };
    }

    pub fn get(&self, m: u32) -> u32 {
        assert!(self.n >= m);
        let m = m.min(self.n - m);
        return (
            self.fac.get(self.n) as u64
            * self.fac.get_inv(m) as u64
            % self.p as u64
            * self.fac.get_inv(self.n - m) as u64
            % self.p as u64
        ).try_into().unwrap();
    }
}

/// 计算排列数 A(n, m)
/// NOTE: 溢出u64时直接panic
pub fn perm_raw(mut n: u64, m: u64) -> u64 {
    assert!(n >= m);
    let mut ans: u64 = 1;
    for _ in 1..=m {
        ans = ans.checked_mul(n).unwrap();
        n -= 1;
    }
    return ans.try_into().unwrap();
}

/// 计算排列数 A(n, m)
pub fn perm(mut n: u64, m: u64, p: u32) -> u32 {
    assert!(n >= m);
    let mut ans = 1;
    for _ in 1..=m {
        ans = ans * n % p as u64;
        n -= 1;
    }
    return ans.try_into().unwrap();
}

pub struct Perm {
    n: u32,
    p: u32,
    fac: Fac
}

impl Perm {
    pub fn new(n: u32, p: u32) -> Self {
        return Perm { n, p, fac: Fac::new(n.try_into().unwrap(), p) };
    }
    pub fn get(&self, m: u32) -> u32 {
        assert!(self.n >= m);
        return (
            self.fac.get(self.n) as u64
            * self.fac.get_inv(self.n - m) as u64
            % self.p as u64
        ).try_into().unwrap();
    }
}

mod perm_transfrom_mod {
    pub fn dfs<T: FnMut(&[usize])>(vec: &mut [usize], now: usize, foo: &mut T) {
        if now == vec.len() {
            foo(vec);
            return;
        }
        for i in now..vec.len() {
            vec.swap(now, i);
            dfs(vec, now + 1, foo);
            vec.swap(now, i);
        }
    }
}

pub fn iter_all_permutations<T: FnMut(&[usize])>(vec: &[usize], foo: &mut T) {
    let mut tmp = vec.to_vec();
    perm_transfrom_mod::dfs(&mut tmp, 0, foo);
}

pub fn get_all_permutations(vec: &[usize]) -> Vec<Vec<usize>> {
    let mut ans: Vec<Vec<usize>> = vec![];
    iter_all_permutations(vec, &mut |vec: &[usize]| ans.push(vec.to_vec()));
    return ans;
}

/// Time complexity: O(n)
///
/// # Example
/// ```
/// use rust_in_competitive_programming::get_next_permtation;
/// assert_eq!(get_next_permtation(&vec![1, 2, 3]), vec![1, 3, 2]);
/// assert_eq!(get_next_permtation(&vec![3, 2, 1]), vec![1, 2, 3]);
/// ```
pub fn get_next_permtation(vec: &Vec<usize>) -> Vec<usize> {
    let n = vec.len();
    for i in (0..n - 1).rev() {
        if vec[i] < vec[i + 1] {
            let mut ans = vec[0..=i].to_vec();
            let mut tmp = vec[i+1..].to_vec();
            for j in (0..tmp.len()).rev() {
                if tmp[j] > ans[i] {
                    let x = tmp[j];
                    tmp[j] = ans[i];
                    ans[i] = x;
                    break;
                }
            }
            tmp.reverse();
            ans.append(&mut tmp);
            return ans;
        }
    }
    let mut ans = vec.to_vec();
    ans.reverse();
    return ans;
}

// pub fn get_prev_permtation<T: Clone>(vec: &Vec<T>) -> Vec<T> {
// }


#[cfg(test)]
mod comb_tests {
    use super::*;

    #[test]
    fn comb_test_small() {
        for n in 1..=10 {
            for m in 1..=n {
                assert!(comb_raw(n, m) == comb(n, m, 1e9 as u32 + 7) as u64);
            }
        }
    }

    #[test]
    #[should_panic]
    fn comb_test_too_big() {
        comb_raw(10000, 5);
    }

    #[test]
    fn comb_test_p() {
        let p = 1e9 as u32 + 7;
        let n = 5000;
        let cmb = Comb::new(n, p);
        for m in 1..=n {
            assert!(
                cmb.get(m) == comb(n as u64, m as u64, p),
                "Mismatch: n={}, m={}",
                n, m
            );
        }
    }

    #[test]
    fn perm_test() {
        let p = 1e9 as u32 + 7;
        let n = 5000;
        let prm = Perm::new(n, p);
        for i in 1..=n {
            assert_eq!(perm(n as u64, i as u64, p), prm.get(i));
        }
    }
}

