impl Solution {
    pub fn maximum_number_of_string_pairs(words: Vec<String>) -> i32 {
        let mut count = 0;
        let n = words.len();

        for i in 0..n {
            for j in i + 1..n {
                let r: String = words[j].chars().rev().collect();
                if words[i] == r {
                    count += 1;
                    break;
                }
            }
        }

        return count;
    }
}
