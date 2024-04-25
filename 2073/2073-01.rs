impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let mut seconds = 0;
        let q = tickets[k as usize];

        for (index, ticket) in tickets.iter().enumerate() {
            if index <= k as usize {
                seconds += q.min(*ticket);
            } else {
                seconds += (q - 1).min(*ticket);
            }
        }

        seconds
    }
}
