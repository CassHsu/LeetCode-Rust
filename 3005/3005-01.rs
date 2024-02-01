impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let mut m = [0; 101];
        let mut max: i32 = 0;
        let mut count: i32 = 0;

        for n in nums {
            m[n as usize] += 1;
            if (m[n as usize] > max) {
                max = m[n as usize];
            }
        }

        for i in m {
            if i as i32 == max {
                count += 1;
            }
        }

        count * max
    }
}
