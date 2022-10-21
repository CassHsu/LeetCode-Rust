impl Solution {
    pub fn common_factors(a: i32, b: i32) -> i32 {
        return (1..=a.min(b)).filter(|n| a % n == 0 && b % n == 0).count() as i32;
    }
}
