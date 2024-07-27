impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut count = 0;

        for n in nums {
            if n % 3 != 0 {
                count += 1;
            }
        }

        return count;
    }
}
