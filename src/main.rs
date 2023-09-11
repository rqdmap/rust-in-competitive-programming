use rust_in_competitive_programming::*;

use std::collections::*;

static mut PRE_HSH1: Vec<u64> = Vec::new();
static mut PRE_HSH2: Vec<u64> = Vec::new();
// static P: u64 =  999_983;
static P: u64 =  13;
static MOD: u64 = 1_000_000_007;
impl Solution {
    fn get_hash(s: &String, hash: &mut Vec<u64>){ 
        let s = s.as_bytes();
        let n = s.len();
        for _ in 0..n { hash.push(0); }

        hash[0] = s[0] as u64;
        for i in 1..n {
            hash[i] = (hash[i - 1] + s[i] as u64 * quick_pow(P, i as u64, MOD) % MOD) % MOD;
        }
    }

    fn get_hash_intval(hash: &Vec<u64>, left: usize, right: usize) -> u64 {
        assert!(left <= right);
        assert!(right < hash.len());

        let mut res = match left {
            0 => hash[right],
            _ => (hash[right] + MOD - hash[left - 1]) % MOD
        };

        let div: u64 = quick_pow(P, left as u64, MOD);
        // println!("res = {}, div = {}", res, div);
        res = res * quick_pow(div, MOD - 2, MOD) % MOD;

        res
    }

    // 将start作为新的头部, 循环拼接
    fn calc_hash_from(hash: &Vec<u64>, start: usize) -> u64 {
        let n = hash.len();
        let mut res = Self::get_hash_intval(hash, start, n - 1);
        if start > 0 {
            res = (res + Self::get_hash_intval(hash, 0, start - 1) * quick_pow(P, n as u64 - start as u64, MOD) % MOD) % MOD;
        }

        res
    }
    unsafe fn foo(s: String, t: String, k: i64) -> i32 {
        PRE_HSH1 = Vec::new();
        PRE_HSH2 = Vec::new();
        Self::get_hash(&s, &mut PRE_HSH1);
        Self::get_hash(&t, &mut PRE_HSH2);
        

        let n = s.len();
        let mut m = 0;
        let mut start_flag = 0;
        for i in 0..n {
            // println!("i = {}, hash = {}, taget = {}", i, Self::calc_hash_from(&PRE_HSH1, i), PRE_HSH2[s.len() - 1]);
            if Self::calc_hash_from(&PRE_HSH1, i) == PRE_HSH2[s.len() - 1] {
                m += 1;
                if i == 0 { start_flag = 1;}
                // println!("entry: {}", i);
            }
        }

        if m == 0 { return 0; }

        let x;
        if start_flag == 1 {
            x = Matrix::<u64>::new(vec![vec![0], vec![1]]);
        }
        else {
            x = Matrix::<u64>::new(vec![vec![1], vec![0]]);
        }

        let matrix = Matrix::new(vec![
            vec![(n - m - 1).try_into().unwrap(), (n - m).try_into().unwrap()],
            vec![m.try_into().unwrap(), (m - 1).try_into().unwrap()]
        ]);
        let matrix = quick_pow(matrix, k as u64, MOD);
        // println!("{}{}", matrix, x);
        let y = matrix * x;
        let y = y.get();

        return y[1][0].try_into().unwrap()
    }
    pub fn number_of_ways(s: String, t: String, k: i64) -> i32 {
        return unsafe { Self::foo(s, t, k) }
    }
}

struct Solution { }
fn main() {
    println!("{}", Solution::number_of_ways("abcd".to_string(), "cdab".to_string(), 1));
    println!("{}", Solution::number_of_ways("ababab".to_string(), "ababab".to_string(), 1));
    // println!("{}", Solution::minimum_moves(vec![vec![1, 2, 2], vec![1, 1, 0], vec![0, 1, 1]]));
    // println!("{}", Solution::minimum_moves(vec![vec![1, 3, 0], vec![1, 0, 0], vec![1, 0, 3]]));
    // println!("{}", Solution::minimum_moves(vec![vec![1, 1, 0], vec![1, 1, 1], vec![1, 2, 1]]));
    // let tmp = Matrix::new(vec![vec![1, 2, 3, 4], vec![7, 8, 9, 10], vec![13, 14, 15, 16], vec![19, 20, 21, 22]]);
    // println!("{:?}", tmp);
    // println!("{}", tmp.clone() * tmp.clone() * tmp.clone());
    // println!("{}", quick_pow(tmp, 3, 1e9 as u64 + 7));
}
