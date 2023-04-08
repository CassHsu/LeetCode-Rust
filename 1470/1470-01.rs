impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut ans = Vec::new();

        let mut j = 0;
        let mut k = n as usize;
        for i in 0..nums.len() {
            if i % 2 == 0 {
                ans.push(nums[j]);
                j += 1;
            } else {
                ans.push(nums[k]);
                k += 1;
            }
        }

        return ans;
    }
}
