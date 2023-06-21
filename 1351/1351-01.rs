impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let mut count = 0;

        for row in grid {
            for n in row {
                if n < 0 {
                    count += 1;
                }
            }
        }

        return count;
    }
}
