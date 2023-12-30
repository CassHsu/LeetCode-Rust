impl Solution {
    pub fn buy_choco(prices: Vec<i32>, money: i32) -> i32 {
        let mut ans = money;
        let mut step = 0;
        let mut sorted = prices;
        sorted.sort();

        for p in sorted {
            if p <= ans {
                ans -= p;
                step += 1;

                if step == 2 {
                    return ans;
                }
            } else {
                return money;
            }
        }

        return money;
    }
}
