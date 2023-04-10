impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let mut ans = s.to_string().clone();

        for (i, v) in indices.iter().enumerate() {
            let c = s.chars().nth(i).unwrap().to_string();
            let iv = *v as usize;
            ans.replace_range(iv..iv+1, &c);
        }

        return ans.to_string();
    }
}
