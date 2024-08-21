impl Solution {
    pub fn can_alice_win(nums: Vec<i32>) -> bool {
        let mut s = 0;
        let mut d = 0;

        for n in nums {
            if n.to_string().len() == 1 {
                s += n;
            } else {
                d += n;
            }
        }

        s != d
    }
}
