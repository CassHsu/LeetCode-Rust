impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let mut l = 0;
        let mut r = mat.len() - 1;

        for row in mat {
            if l == r {
                ans += row[l];
            } else {
                ans += row[l];
                ans += row[r];
            }

            l += 1;
            r -= 1;
        }

        return ans;
    }
}
