impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut last = 0;

        for g in gain {
            last += g;
            if last > max {
                max = last;
            }
        }

        return max;
    }
}
