impl Solution {
    pub fn is_ugly(n: i32) -> bool {
        if n == 0 { return false }

        let mut num = n;

        let prime: [i32; 3] = [2, 3, 5];
        for p in prime {
            while num % p == 0 {
                num /= p;
            }
        }

        return num == 1;
    }
}
