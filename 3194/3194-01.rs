impl Solution {
    pub fn minimum_average(nums: Vec<i32>) -> f64 {
        let mut sorted = nums;
        sorted.sort();
        let mut l = 0;
        let mut r = sorted.len() - 1;
        let mut ans = sorted[r] as f64;

        while l < r {
            let avg = (sorted[l] + sorted[r]) as f64 / 2.0;
            ans = ans.min(avg);
            l += 1;
            r -= 1;
        }

        ans
    }
}
