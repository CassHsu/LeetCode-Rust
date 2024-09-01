impl Solution {
    pub fn final_position_of_snake(n: i32, commands: Vec<String>) -> i32 {
        let mut d: Vec<Vec<i32>> = vec![vec![0; n as usize]; n as usize]; 
        let mut x = 0;

        for i in 0..n as usize {
            for j in 0..n as usize {
                d[i][j] = x;
                x += 1;
            }
        }

        let mut i = 0;
        let mut j = 0;
        for c in &commands {
            match c.as_str() {
                "RIGHT" => j += 1,
                "DOWN" => i += 1,
                "LEFT" => j -= 1,
                "UP" => i -= 1,
                _ => {}
            }
        }

        return d[i][j];
    }
}
