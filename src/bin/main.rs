// use rust_in_competitive_programming::*;

struct Solution();
impl Solution {
    fn get_dp(nums: &Vec<i32>, len: usize) -> (Vec<i32>, Vec<i32>) {
        let mut sum = 0 ;

        let mut dp1: Vec<i32> = vec![0; nums.len()];
        for i in 0..len {
            sum += nums[i];
        }
        dp1[len - 1] = sum;
        for i in len..nums.len() {
            sum -= nums[i - len];
            sum += nums[i];
            dp1[i] = dp1[i - 1].max(sum);
        }

        let mut dp2: Vec<i32> = vec![0; nums.len()];
        sum = 0;
        for i in (nums.len() - len ..= nums.len() - 1).rev()  {
            sum += nums[i];
        }
        dp2[nums.len() - len] = sum;
        for i in (0 ..= nums.len() - len - 1).rev() {
            sum -= nums[i + len];
            sum += nums[i];
            dp2[i] = dp2[i + 1].max(sum);
        }
        (dp1, dp2)
    }

    pub fn max_sum_two_no_overlap(nums: Vec<i32>, first_len: i32, second_len: i32) -> i32 {
        let first_len = first_len as usize;
        let second_len = second_len as usize;
        let (pre1, suf1) = Self::get_dp(&nums, first_len);
        let (pre2, suf2) = Self::get_dp(&nums, second_len);
        
        let mut ans = 0;
        for i in first_len - 1..= nums.len() - second_len - 1 {
            ans = ans.max(pre1[i] + suf2[i + 1]);
        }

        for i in second_len - 1 ..= nums.len() - first_len - 1 {
            ans = ans.max(pre2[i] + suf1[i + 1]);
        }
        // println!("{:?} {} {}", nums, first_len, second_len);
        // println!("{:?}", pre1);
        // println!("{:?}", pre2);
        // println!("{:?}", suf1);
        // println!("{:?}", suf2);
        
        ans
    }
}

fn main() {
    println!("{}", Solution::max_sum_two_no_overlap(vec![1,0,3], 1, 2));
}
