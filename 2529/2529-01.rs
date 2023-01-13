impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        let mut p = 0;
        let mut n = 0;

        for num in nums {
            if (num > 0) { p += 1 };
            if (num < 0) { n += 1 };
        }

        if (p > n) {
            return p;
        }
        return n;
    }
}
