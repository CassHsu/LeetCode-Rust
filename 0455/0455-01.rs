impl Solution {
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let mut g = g;
        let mut s = s;

        g.sort();
        s.sort();

        let mut i = 0;

        for &c in s.iter() {
            if i < g.len() && c >= g[i] {
                i += 1;
            }
        }

        return i as i32;
    }
}
