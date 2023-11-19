impl Solution {
    pub fn find_champion(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();

        for i in 0..n {
            let mut sum = 0;
            for j in 0..m {
                sum += grid[i][j] as i32;
            }

            if sum == (n - 1) as i32 {
                return i as i32;
            }
        }

        return 0;
    }
}
