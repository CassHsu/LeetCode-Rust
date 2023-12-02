use std::collections::HashSet;

impl Solution {
    pub fn minimized_string_length(s: String) -> i32 {
        let set: HashSet<_> = s.chars().collect();
        return set.len() as i32;
    }
}
