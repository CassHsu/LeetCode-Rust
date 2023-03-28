use std::collections::HashMap;

impl Solution {
    pub fn remove_vowels(s: String) -> String {
        let mut m = HashMap::new();
        let mut res = String::from("");

        m.insert('a', true);
        m.insert('e', true);
        m.insert('i', true);
        m.insert('o', true);
        m.insert('u', true);

        for c in s.chars() {
            if m.contains_key(&c) == false {
                res.push(c);
            }
        }

        return res;
    }
}
