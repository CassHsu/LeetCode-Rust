impl Solution {
    pub fn common_factors(a: i32, b: i32) -> i32 {
        let mut count: i32 = 1;
        
        for n in 2..a.min(b) + 1 {
            if a % n == 0 && b % n == 0 {
                count += 1;
            }
        }
        
        return count;
    }
}
