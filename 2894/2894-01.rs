impl Solution {
    pub fn difference_of_sums(n: i32, m: i32) -> i32 {
        let mut s1 = 0;
        let mut s2 = 0;

        for i in 1..n + 1 {
            if i % m == 0 {
                s2 += i;
            } else {
                s1 += i;
            }
        }

        return s1 - s2;
    }
}
