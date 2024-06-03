impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let mut mx: i32 = 1;
        let mut up: i32 = 1;
        let mut dn: i32 = 1;

        for i in 1..nums.len() {
            if nums[i - 1] > nums[i] {
                dn += 1;
                up = 1;
            } else if nums[i - 1] < nums[i] {
                up += 1;
                dn = 1;
            } else {
                up = 1;
                dn = 1;
            }

            mx = mx.max(up).max(dn);
        }

        mx
    }
}
