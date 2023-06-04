impl Solution {
    pub fn maximize_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        let mx = *nums.iter().max().unwrap();

        for i in 0..k {
            ans += mx;
            ans += i;
        }

        return ans;
    }
}
